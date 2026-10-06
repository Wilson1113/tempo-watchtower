//! EXERCISE (M7): `classify_revert`. The wrapper `classify` is GIVEN.
//! SPOILER below. In the builder's repo, from day 1 until M7, the body is this SAFE DEFAULT, never
//! `todo!()`, because `broadcast_close` calls it on every send error from day 4 on:
//!
//! ```ignore
//! pub fn classify_revert(decoded: Option<&E>, message: &str) -> CloseError {
//!     let _ = decoded; // TODO(M7): classify the decoded precompile error
//!     CloseError::Transient(message.to_string())
//! }
//! ```
//! Everything is retryable until M7. The spec (`tests/classify.rs`) arrives on day 8 and goes red
//! against this stub; that is the exercise.

use crate::Reserve::ITIP20ChannelReserveErrors as E;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloseError {
    /// `ChannelNotFound`: a withdraw, the payee, or our earlier tx already closed it.
    AlreadyClosed,
    /// `CaptureAmountInvalid` / `AmountExceedsDeposit`: a settle or topUp raced us. Re-poll, replan.
    StaleState,
    /// `InvalidSignature`: the stored voucher is bad. Alert.
    BadVoucher,
    /// `NotPayeeOrOperator`: wrong operator key (config bug). Alert, stop retrying.
    WrongKey,
    /// Anything else (RPC error, timeout, nonce/replacement, unknown revert). Retry.
    Transient(String),
}

impl CloseError {
    pub fn retryable(&self) -> bool {
        matches!(self, Self::StaleState | Self::Transient(_))
    }
}

/// GIVEN: decode the precompile's custom error (if any) and classify it.
pub fn classify(e: &alloy::contract::Error) -> CloseError {
    classify_revert(e.as_decoded_interface_error::<E>().as_ref(), &e.to_string())
}

/// EXERCISE. SPOILER:
pub fn classify_revert(decoded: Option<&E>, message: &str) -> CloseError {
    match decoded {
        Some(E::ChannelNotFound(_)) => CloseError::AlreadyClosed,
        Some(E::CaptureAmountInvalid(_) | E::AmountExceedsDeposit(_)) => CloseError::StaleState,
        Some(E::InvalidSignature(_)) => CloseError::BadVoucher,
        Some(E::NotPayeeOrOperator(_)) => CloseError::WrongKey,
        _ => CloseError::Transient(message.to_string()),
    }
}
