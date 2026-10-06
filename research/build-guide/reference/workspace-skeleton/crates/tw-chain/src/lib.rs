//! `tw-chain`: GIVEN plumbing (verified on Tempo testnet 2026-09-28) plus one exercise
//! (`classify_revert`, in `classify.rs`).

use std::time::Duration;

use alloy::{
    network::ReceiptResponse,
    primitives::{Address, B256, Bytes, U256, aliases::U96},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::types::{Filter, Log},
    signers::{Signer, local::PrivateKeySigner},
    sol_types::{SolEvent, SolEventInterface},
};
use tempo_alloy::TempoNetwork;
pub use tempo_alloy::contracts::precompiles::{
    ITIP20, ITIP20ChannelReserve as Reserve, TIP20_CHANNEL_RESERVE_ADDRESS as RESERVE,
};

mod classify;
pub use classify::{CloseError, classify, classify_revert};

pub const PATH_USD: Address =
    alloy::primitives::address!("0x20c0000000000000000000000000000000000000");
pub type TempoProvider = DynProvider<TempoNetwork>;

#[derive(Debug, thiserror::Error)]
pub enum ChainError {
    #[error("rpc: {0}")]
    Rpc(#[from] alloy::transports::TransportError),
    #[error("contract: {0}")]
    Contract(#[from] alloy::contract::Error),
    #[error("pending tx: {0}")]
    Pending(#[from] alloy::providers::PendingTransactionError),
    #[error("signer: {0}")]
    Signer(#[from] alloy::signers::Error),
    #[error("bad url: {0}")]
    Url(String),
    #[error("{0}")]
    Other(String),
}

pub fn http_provider(url: &str) -> Result<TempoProvider, ChainError> {
    let url = url.parse().map_err(|_| ChainError::Url(url.into()))?;
    Ok(ProviderBuilder::new_with_network::<TempoNetwork>()
        .connect_http(url)
        .erased())
}

/// Plain secp256k1 key: transactions go out as EIP-1559 (type 0x2); the fee is paid in the
/// sender's preferred USD token, falling back to PathUSD.
pub fn signing_provider(url: &str, key: PrivateKeySigner) -> Result<TempoProvider, ChainError> {
    let url = url.parse().map_err(|_| ChainError::Url(url.into()))?;
    Ok(ProviderBuilder::new_with_network::<TempoNetwork>()
        .wallet(key)
        .connect_http(url)
        .erased())
}

pub async fn grace_period(p: &TempoProvider) -> Result<u64, ChainError> {
    Ok(Reserve::new(RESERVE, p).CLOSE_GRACE_PERIOD().call().await?)
}

/// Testnet faucet: mints PathUSD, AlphaUSD, BetaUSD and ThetaUSD to `who`.
pub async fn fund(p: &TempoProvider, who: Address) -> Result<(), ChainError> {
    let _hashes: Vec<B256> = p.raw_request("tempo_fundAddress".into(), (who,)).await?;
    for _ in 0..30 {
        if ITIP20::new(PATH_USD, p).balanceOf(who).call().await? > U256::ZERO {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Err(ChainError::Other(
        "faucet funding not visible after 30 s".into(),
    ))
}

pub async fn path_usd_balance(p: &TempoProvider, who: Address) -> Result<u128, ChainError> {
    Ok(ITIP20::new(PATH_USD, p)
        .balanceOf(who)
        .call()
        .await?
        .saturating_to())
}

// Orphan rule: `From<Reserve::ChannelDescriptor> for tw_core::Descriptor` can't live here (both
// types are foreign to tw-chain), so the conversions are plain functions.
pub fn from_sol(d: &Reserve::ChannelDescriptor) -> tw_core::Descriptor {
    tw_core::Descriptor {
        payer: d.payer,
        payee: d.payee,
        operator: d.operator,
        token: d.token,
        salt: d.salt,
        authorized_signer: d.authorizedSigner,
        expiring_nonce_hash: d.expiringNonceHash,
    }
}

pub fn to_sol(d: &tw_core::Descriptor) -> Reserve::ChannelDescriptor {
    Reserve::ChannelDescriptor {
        payer: d.payer,
        payee: d.payee,
        operator: d.operator,
        token: d.token,
        salt: d.salt,
        authorizedSigner: d.authorized_signer,
        expiringNonceHash: d.expiring_nonce_hash,
    }
}

const RECEIPT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(30));

/// Payer opens a channel. Returns (channel_id, descriptor, open tx hash).
pub async fn open_channel(
    payer_p: &TempoProvider,
    payee: Address,
    operator: Address,
    deposit: u128,
) -> Result<(B256, tw_core::Descriptor, B256), ChainError> {
    let rcpt = Reserve::new(RESERVE, payer_p)
        .open(
            payee,
            operator,
            PATH_USD,
            U96::from(deposit),
            B256::random(),
            Address::ZERO,
        )
        .send()
        .await?
        .with_timeout(RECEIPT_TIMEOUT)
        .get_receipt()
        .await?;
    if !rcpt.status() {
        return Err(ChainError::Other(format!(
            "open reverted: {}",
            rcpt.transaction_hash
        )));
    }
    let ev = rcpt
        .decoded_log::<Reserve::ChannelOpened>()
        .ok_or_else(|| ChainError::Other("no ChannelOpened in receipt".into()))?
        .data;
    let d = tw_core::Descriptor {
        payer: ev.payer,
        payee: ev.payee,
        operator: ev.operator,
        token: ev.token,
        salt: ev.salt,
        authorized_signer: ev.authorizedSigner,
        expiring_nonce_hash: ev.expiringNonceHash,
    };
    Ok((ev.channelId, d, rcpt.transaction_hash))
}

/// Payer-only calls: `requestClose`, `withdraw` (after grace), `topUp` (cancels a close request).
pub enum PayerCall {
    RequestClose,
    Withdraw,
    TopUp(u128),
}

pub async fn payer_call(
    payer_p: &TempoProvider,
    d: &tw_core::Descriptor,
    call: PayerCall,
) -> Result<B256, ChainError> {
    let r = Reserve::new(RESERVE, payer_p);
    let pending = match call {
        PayerCall::RequestClose => r.requestClose(to_sol(d)).send().await?,
        PayerCall::Withdraw => r.withdraw(to_sol(d)).send().await?,
        PayerCall::TopUp(a) => r.topUp(to_sol(d), U96::from(a)).send().await?,
    };
    let rcpt = pending.with_timeout(RECEIPT_TIMEOUT).get_receipt().await?;
    if !rcpt.status() {
        return Err(ChainError::Other(format!(
            "payer call reverted: {}",
            rcpt.transaction_hash
        )));
    }
    Ok(rcpt.transaction_hash)
}

/// EIP-712 voucher signature (65 bytes r||s||v) over `tw_core::voucher_digest`.
pub async fn sign_voucher(
    s: &PrivateKeySigner,
    chain_id: u64,
    id: B256,
    amount: u128,
) -> Result<Bytes, ChainError> {
    let digest = tw_core::voucher_digest(chain_id, id, amount);
    Ok(Bytes::from(s.sign_hash(&digest).await?.as_bytes().to_vec()))
}

/// On-chain state; `deposit == 0` means closed (or never existed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnChainState {
    pub settled: u128,
    pub deposit: u128,
    pub close_requested_at: u64,
}

pub async fn states(p: &TempoProvider, ids: &[B256]) -> Result<Vec<OnChainState>, ChainError> {
    let mut out = Vec::with_capacity(ids.len());
    for chunk in ids.chunks(200) {
        let st = Reserve::new(RESERVE, p)
            .getChannelStatesBatch(chunk.to_vec())
            .call()
            .await?;
        out.extend(st.into_iter().map(|s| OnChainState {
            settled: s.settled.to(),
            deposit: s.deposit.to(),
            close_requested_at: u64::from(s.closeRequestedAt),
        }));
    }
    Ok(out)
}

/// GIVEN. The closer's pre-send check (R6), from a FRESH `states(&[id])` read taken immediately
/// before sending. `None` = go ahead and send. `Some(input)` = don't send; feed it to `step`.
/// Needed because `close()` has no close-request check: a queued close would otherwise kill a
/// channel the payer just rescued with `topUp`.
pub fn precheck(s: &OnChainState) -> Option<tw_core::Input> {
    if s.deposit == 0 {
        Some(tw_core::Input::CloseSkipped { cancelled: false }) // already closed
    } else if s.close_requested_at == 0 {
        Some(tw_core::Input::CloseSkipped { cancelled: true }) // payer cancelled (topUp)
    } else {
        None
    }
}

/// Broadcast `close()` from the operator key and return the tx hash (do NOT wait for the receipt
/// here: report `CloseSent` first, then wait). Nonce = `latest` from the chain on every attempt:
/// if an earlier attempt is still pending, this one fails fast as a replacement (Transient), and
/// once the earlier one lands the pre-check sees `deposit == 0`.
pub async fn broadcast_close(
    p: &TempoProvider,
    operator: Address,
    d: &tw_core::Descriptor,
    plan: &tw_core::ClosePlan,
) -> Result<B256, CloseError> {
    let nonce = p
        .get_transaction_count(operator)
        .latest()
        .await
        .map_err(|e| CloseError::Transient(e.to_string()))?;
    let pending = Reserve::new(RESERVE, p)
        .close(
            to_sol(d),
            U96::from(plan.cumulative),
            U96::from(plan.capture),
            Bytes::from(plan.signature.clone()),
        )
        .nonce(nonce)
        .send()
        .await
        .map_err(|e| classify(&e))?;
    Ok(*pending.tx_hash())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosedEvent {
    pub tx: B256,
    /// Cumulative total paid to the payee over the channel's life (NOT the delta).
    pub settled_to_payee: u128,
    pub refunded: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptStatus {
    Pending,
    Reverted,
    Closed(ClosedEvent),
    MinedWithoutClose,
}

/// Look up one tx's receipt and find `ChannelClosed` for `id` in it. Used by the closer after
/// broadcasting, and for `Action::CheckReceipts` (R5).
pub async fn receipt_status(
    p: &TempoProvider,
    id: B256,
    tx: B256,
) -> Result<ReceiptStatus, ChainError> {
    let Some(rcpt) = p.get_transaction_receipt(tx).await? else {
        return Ok(ReceiptStatus::Pending);
    };
    if !rcpt.status() {
        return Ok(ReceiptStatus::Reverted);
    }
    let found = rcpt
        .decoded_log::<Reserve::ChannelClosed>()
        .filter(|l| l.data.channelId == id)
        .map(|l| ClosedEvent {
            tx,
            settled_to_payee: l.data.settledToPayee.to(),
            refunded: l.data.refundedToPayer.to(),
        });
    Ok(found.map_or(ReceiptStatus::MinedWithoutClose, ReceiptStatus::Closed))
}

/// Poll `receipt_status` every 500 ms until it isn't `Pending`, or `timeout`.
pub async fn wait_receipt(
    p: &TempoProvider,
    id: B256,
    tx: B256,
    timeout: Duration,
) -> Result<ReceiptStatus, ChainError> {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let s = receipt_status(p, id, tx).await?;
        if s != ReceiptStatus::Pending || tokio::time::Instant::now() >= deadline {
            return Ok(s);
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// Precompile events for ONE payee (payee is indexed topic 3 in every channel event), so the
/// WebSocket only delivers your own channels. The brain still drops unknown channel ids.
pub fn payee_filter(payee: Address) -> Filter {
    Filter::new()
        .address(RESERVE)
        .event_signature(vec![
            Reserve::ChannelOpened::SIGNATURE_HASH,
            Reserve::Settled::SIGNATURE_HASH,
            Reserve::TopUp::SIGNATURE_HASH,
            Reserve::CloseRequested::SIGNATURE_HASH,
            Reserve::CloseRequestCancelled::SIGNATURE_HASH,
            Reserve::ChannelClosed::SIGNATURE_HASH,
        ])
        .topic3(payee.into_word())
}

pub fn decode(log: &Log) -> Option<Reserve::ITIP20ChannelReserveEvents> {
    Reserve::ITIP20ChannelReserveEvents::decode_log(&log.inner)
        .ok()
        .map(|l| l.data)
}
