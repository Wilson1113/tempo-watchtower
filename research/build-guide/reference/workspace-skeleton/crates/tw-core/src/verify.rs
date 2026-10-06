//! SPOILER: reference implementation of `verify_voucher` (M2 exercise).

use crate::{Descriptor, Voucher, voucher_digest};
use alloy_primitives::{Address, B256, Signature};

/// True if `v.signature` is a valid secp256k1 signature by the channel's voucher signer
/// (`authorized_signer` if non-zero, else `payer`) over `voucher_digest(chain_id, id, v.cumulative)`.
pub fn verify_voucher(d: &Descriptor, id: B256, v: &Voucher, chain_id: u64) -> bool {
    let expected = if d.authorized_signer == Address::ZERO {
        d.payer
    } else {
        d.authorized_signer
    };
    let digest = voucher_digest(chain_id, id, v.cumulative);
    Signature::try_from(v.signature.as_slice())
        .ok()
        .and_then(|s| s.recover_address_from_prehash(&digest).ok())
        == Some(expected)
}
