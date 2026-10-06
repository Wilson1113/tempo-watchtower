//! GIVEN spec for the M7 exercise `classify_revert`. Make these pass.
use tw_chain::{CloseError, Reserve, Reserve::ITIP20ChannelReserveErrors as E, classify_revert};

#[test]
fn channel_not_found_means_already_closed() {
    let e = E::ChannelNotFound(Reserve::ChannelNotFound {});
    assert_eq!(classify_revert(Some(&e), "x"), CloseError::AlreadyClosed);
    assert!(!CloseError::AlreadyClosed.retryable());
}

#[test]
fn capture_and_deposit_errors_mean_stale_state_and_retry() {
    for e in [
        E::CaptureAmountInvalid(Reserve::CaptureAmountInvalid {}),
        E::AmountExceedsDeposit(Reserve::AmountExceedsDeposit {}),
    ] {
        let c = classify_revert(Some(&e), "x");
        assert_eq!(c, CloseError::StaleState);
        assert!(c.retryable());
    }
}

#[test]
fn bad_signature_and_wrong_key_are_not_retryable() {
    let sig = classify_revert(
        Some(&E::InvalidSignature(Reserve::InvalidSignature {})),
        "x",
    );
    let key = classify_revert(
        Some(&E::NotPayeeOrOperator(Reserve::NotPayeeOrOperator {})),
        "x",
    );
    assert_eq!(
        (sig.clone(), key.clone()),
        (CloseError::BadVoucher, CloseError::WrongKey)
    );
    assert!(!sig.retryable() && !key.retryable());
}

#[test]
fn unknown_or_transport_errors_are_transient_and_keep_the_message() {
    assert_eq!(
        classify_revert(None, "timeout"),
        CloseError::Transient("timeout".into())
    );
    let other = E::CloseNotReady(Reserve::CloseNotReady {});
    assert!(classify_revert(Some(&other), "x").retryable());
}
