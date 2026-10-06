//! `tw-core`: every money decision the watchtower makes, as pure functions.
//! No I/O, no async, no clock: time is always passed in as `now_ms`.
//!
//! GIVEN in the build guide: the data types, `Policy::default`, `ChannelRecord::new`,
//! `channel_id` and `voucher_digest` (protocol plumbing).
//! EXERCISES for the builder: `plan_close`, `step` (in `machine.rs`) and `verify_voucher`
//! (in `verify.rs`). In the builder's repo those start as `todo!()`; the spec lives in
//! `tests/spec.rs` and `tests/props.rs`.

use alloy_primitives::{Address, B256, U256, address, keccak256};
use alloy_sol_types::{SolStruct, SolValue, eip712_domain};
use serde::{Deserialize, Serialize};

mod machine;
mod verify;

pub use machine::{plan_close, step};
pub use verify::verify_voucher;

/// Token base units (TIP-20 stablecoins have 6 decimals: 1_000_000 = $1.00).
pub type Amount = u128;
/// On-chain amounts are `uint96`.
pub const MAX_U96: Amount = (1u128 << 96) - 1;
/// TIP-20 channel reserve precompile (v2 channels).
pub const RESERVE: Address = address!("0x4d50500000000000000000000000000000000000");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Descriptor {
    pub payer: Address,
    pub payee: Address,
    pub operator: Address,
    pub token: Address,
    pub salt: B256,
    pub authorized_signer: Address,
    pub expiring_nonce_hash: B256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Voucher {
    pub cumulative: Amount,
    /// 65-byte secp256k1 r||s||v signature over `voucher_digest`.
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelRecord {
    pub id: B256,
    pub descriptor: Descriptor,
    pub deposit: Amount,
    /// settledOnChain
    pub settled: Amount,
    /// Service actually delivered, forwarded by the payee.
    pub spent: Amount,
    /// Highest cumulative voucher seen.
    pub best_voucher: Option<Voucher>,
    /// Every close tx we ever broadcast for this channel (outcome attribution, R5).
    pub our_txs: Vec<B256>,
    pub cancellations: u32,
    pub phase: Phase,
}

impl ChannelRecord {
    pub fn new(id: B256, descriptor: Descriptor, deposit: Amount) -> Self {
        Self {
            id,
            descriptor,
            deposit,
            settled: 0,
            spent: 0,
            best_voucher: None,
            our_txs: Vec::new(),
            cancellations: 0,
            phase: Phase::Watching,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Watching,
    Closing(Closing),
    Done(Outcome),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Closing {
    /// Unix seconds, chain-derived: `closeGraceEnd`, or `closeRequestedAt + CLOSE_GRACE_PERIOD()`.
    pub grace_end: u64,
    /// Chain time (ms) of the `CloseRequested` block, for metrics.
    pub requested_at_ms: u64,
    /// Local time (ms) when we first learned about this request.
    pub detected_at_ms: u64,
    pub attempts: u32,
    pub next_attempt_ms: u64,
    pub in_flight: bool,
    pub deadline_alerted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    /// Our close landed. `settled_to_payee` is the cumulative total from `ChannelClosed`;
    /// `claimed` = `settled_to_payee - settled_before` is what this close earned the payee.
    Saved {
        tx: B256,
        claimed: Amount,
        settled_to_payee: Amount,
        refunded: Amount,
    },
    /// Closed by someone else, but there was nothing unsettled to claim.
    NothingToClaim { tx: Option<B256> },
    /// Closed by someone else (payer withdraw, or the payee itself) while `spent > settled`.
    ClosedByOther {
        tx: Option<B256>,
        settled_to_payee: Option<Amount>,
        refunded: Option<Amount>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Voucher(Voucher),
    Spent(Amount),
    Settled(Amount),
    Deposit(Amount),
    /// From a `CloseRequested` event or a poll that saw `closeRequestedAt != 0`.
    CloseRequested {
        grace_end: u64,
        requested_at_ms: u64,
    },
    CloseRequestCancelled,
    /// A poll saw `deposit == 0`: the channel is closed, but who closed it is unknown.
    GoneOnChain,
    /// A `ChannelClosed` event, or the daemon's receipt lookup result (`tx: None` = not ours).
    Closed {
        tx: Option<B256>,
        settled_to_payee: Option<Amount>,
        refunded: Option<Amount>,
    },
    CloseSent {
        tx: B256,
    },
    CloseFailed {
        retryable: bool,
        reason: String,
    },
    /// The closer's pre-send check found the request cancelled (`closeRequestedAt == 0`) or the
    /// channel already gone (`deposit == 0`); nothing was sent.
    CloseSkipped {
        cancelled: bool,
    },
    /// The record was just loaded from the ledger after a (re)start.
    Restarted,
    Tick,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosePlan {
    pub cumulative: Amount,
    pub capture: Amount,
    pub signature: Vec<u8>,
    /// `capture - settled`: what this close would earn the payee.
    pub claimable: Amount,
    /// True if `spent` exceeded the best voucher (or the deposit) and capture was clamped.
    pub clamped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PlanError {
    #[error("spent exceeds settled but no voucher is known")]
    NoVoucher,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    SubmitClose(ClosePlan),
    /// Look up these receipts for `ChannelClosed`, then send `Input::Closed` (R5).
    CheckReceipts(Vec<B256>),
    Alert(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub close_when_nothing_to_claim: bool,
    pub deadline_alert_secs: u64,
    pub base_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub alert_after_attempts: u32,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            close_when_nothing_to_claim: false,
            deadline_alert_secs: 120,
            base_backoff_ms: 500,
            max_backoff_ms: 5_000,
            alert_after_attempts: 5,
        }
    }
}

mod eip712 {
    // The struct NAME is part of the EIP-712 type hash: it must be exactly `Voucher`.
    alloy_sol_types::sol! {
        struct Voucher { bytes32 channelId; uint96 cumulativeAmount; }
    }
}

/// GIVEN. EIP-712 digest the payer signs; equals the precompile's `getVoucherDigest`.
pub fn voucher_digest(chain_id: u64, channel_id: B256, amount: Amount) -> B256 {
    let domain = eip712_domain! {
        name: "TIP20 Channel Reserve",
        version: "1",
        chain_id: chain_id,
        verifying_contract: RESERVE,
    };
    eip712::Voucher {
        channelId: channel_id,
        cumulativeAmount: alloy_primitives::aliases::U96::from(amount),
    }
    .eip712_signing_hash(&domain)
}

/// GIVEN. `keccak256(abi.encode(payer, payee, operator, token, salt, authorizedSigner,
/// expiringNonceHash, RESERVE, chainId))`.
pub fn channel_id(d: &Descriptor, chain_id: u64) -> B256 {
    keccak256(
        (
            d.payer,
            d.payee,
            d.operator,
            d.token,
            d.salt,
            d.authorized_signer,
            d.expiring_nonce_hash,
            RESERVE,
            U256::from(chain_id),
        )
            .abi_encode(),
    )
}
