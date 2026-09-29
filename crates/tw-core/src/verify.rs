use crate::{Descriptor, Voucher};
use alloy_primitives::B256;

pub fn verify_voucher(d: &Descriptor, id: B256, v: &Voucher, chain_id: u64) -> bool {
    let _ = (d, id, v, chain_id);
    todo!("M2")
}
