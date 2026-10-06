//! GIVEN spec for the M2 exercises (`plan_close`, `step`, `verify_voucher`).
//! Your job: make every test pass without editing this file. Fixtures come from real Tempo
//! testnet transactions (2026-09-28).

use alloy_primitives::{Address, B256, address, b256, hex};
use tw_core::*;

const CHAIN_ID: u64 = 42431;
const P: Policy = Policy {
    close_when_nothing_to_claim: false,
    deadline_alert_secs: 120,
    base_backoff_ms: 500,
    max_backoff_ms: 5_000,
    alert_after_attempts: 5,
};
const T0: u64 = 1_000_000; // "now" in ms for most tests
const GRACE_END: u64 = T0 / 1000 + 900; // unix seconds

fn desc() -> Descriptor {
    Descriptor {
        payer: Address::repeat_byte(0xa1),
        payee: Address::repeat_byte(0xb2),
        operator: Address::repeat_byte(0xc3),
        token: address!("0x20c0000000000000000000000000000000000000"),
        salt: B256::repeat_byte(1),
        authorized_signer: Address::ZERO,
        expiring_nonce_hash: B256::repeat_byte(2),
    }
}
fn v(cumulative: Amount) -> Voucher {
    Voucher {
        cumulative,
        signature: vec![7; 65],
    }
}
/// deposit 1.00, spent 0.25, voucher 0.26, settled 0
fn rec() -> ChannelRecord {
    let mut r = ChannelRecord::new(B256::repeat_byte(9), desc(), 1_000_000);
    r.spent = 250_000;
    r.best_voucher = Some(v(260_000));
    r
}
fn request(r: &mut ChannelRecord, now: u64, grace_end: u64) -> Vec<Action> {
    step(
        r,
        Input::CloseRequested {
            grace_end,
            requested_at_ms: now,
        },
        now,
        &P,
    )
}
fn submits(a: &[Action]) -> usize {
    a.iter()
        .filter(|x| matches!(x, Action::SubmitClose(_)))
        .count()
}
fn alerts(a: &[Action]) -> usize {
    a.iter().filter(|x| matches!(x, Action::Alert(_))).count()
}
fn closing(r: &ChannelRecord) -> &Closing {
    match &r.phase {
        Phase::Closing(c) => c,
        other => panic!("expected Closing, got {other:?}"),
    }
}

// ---------- plan_close ----------

#[test]
fn plan_worked_example_from_study_guide() {
    // deposit 5.00, spent 3.00, voucher 3.20, settled 1.00 -> capture 3.00, claim 2.00
    let mut r = ChannelRecord::new(B256::ZERO, desc(), 5_000_000);
    r.spent = 3_000_000;
    r.settled = 1_000_000;
    r.best_voucher = Some(v(3_200_000));
    let p = plan_close(&r).unwrap();
    assert_eq!(
        (p.capture, p.claimable, p.cumulative),
        (3_000_000, 2_000_000, 3_200_000)
    );
    assert_eq!(p.signature, vec![7; 65]);
    assert!(!p.clamped);
}

#[test]
fn plan_nothing_to_claim_needs_no_voucher() {
    let mut r = ChannelRecord::new(B256::ZERO, desc(), 1_000_000);
    r.settled = 400_000;
    r.spent = 300_000; // spent < settled: capture = settled, no signature needed (verified on testnet)
    let p = plan_close(&r).unwrap();
    assert_eq!(
        (p.capture, p.cumulative, p.claimable),
        (400_000, 400_000, 0)
    );
    assert!(p.signature.is_empty());
}

#[test]
fn plan_without_voucher_is_an_error_when_something_is_claimable() {
    let mut r = rec();
    r.best_voucher = None;
    assert_eq!(plan_close(&r), Err(PlanError::NoVoucher));
}

#[test]
fn plan_never_captures_more_than_the_voucher_or_deposit() {
    let mut r = rec();
    r.spent = 300_000; // served more than authorized (voucher 0.26)
    let p = plan_close(&r).unwrap();
    assert_eq!((p.capture, p.clamped), (260_000, true));

    let mut r = rec();
    r.deposit = 200_000;
    let p = plan_close(&r).unwrap();
    assert_eq!((p.capture, p.clamped), (200_000, true));
}

// ---------- step: the happy path ----------

#[test]
fn close_request_submits_one_close_with_capture_equal_to_spent() {
    let mut r = rec();
    let a = request(&mut r, T0, GRACE_END);
    assert_eq!(submits(&a), 1);
    let Some(Action::SubmitClose(plan)) = a.iter().find(|x| matches!(x, Action::SubmitClose(_)))
    else {
        unreachable!()
    };
    assert_eq!((plan.capture, plan.cumulative), (250_000, 260_000)); // capture = spent, not voucher
    let c = closing(&r);
    assert!(c.in_flight);
    assert_eq!((c.grace_end, c.detected_at_ms), (GRACE_END, T0));
}

#[test]
fn duplicate_close_request_from_ws_and_poll_is_a_no_op() {
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let before = r.clone();
    let a = request(&mut r, T0 + 700, GRACE_END);
    assert_eq!(submits(&a), 0);
    assert_eq!(r, before);
}

#[test]
fn saved_when_the_closing_tx_is_any_of_ours_and_claimed_is_the_delta() {
    // R5: attempt 1 times out, attempt 2 is sent, then attempt 1 lands.
    let mut r = rec();
    r.settled = 100_000;
    request(&mut r, T0, GRACE_END);
    let (tx1, tx2) = (B256::repeat_byte(0x11), B256::repeat_byte(0x22));
    step(&mut r, Input::CloseSent { tx: tx1 }, T0 + 100, &P);
    step(
        &mut r,
        Input::CloseFailed {
            retryable: true,
            reason: "timeout".into(),
        },
        T0 + 20_000,
        &P,
    );
    step(&mut r, Input::Tick, T0 + 21_000, &P);
    step(&mut r, Input::CloseSent { tx: tx2 }, T0 + 21_100, &P);
    let closed = Input::Closed {
        tx: Some(tx1),
        settled_to_payee: Some(250_000),
        refunded: Some(750_000),
    };
    step(&mut r, closed, T0 + 22_000, &P);
    // settledToPayee in ChannelClosed is CUMULATIVE; this close earned 250_000 - 100_000.
    assert_eq!(
        r.phase,
        Phase::Done(Outcome::Saved {
            tx: tx1,
            claimed: 150_000,
            settled_to_payee: 250_000,
            refunded: 750_000
        })
    );
}

// ---------- step: failures, retries, deadlines ----------

#[test]
fn retryable_failure_backs_off_then_retries() {
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let a = step(
        &mut r,
        Input::CloseFailed {
            retryable: true,
            reason: "rpc".into(),
        },
        T0 + 1_000,
        &P,
    );
    assert_eq!((submits(&a), alerts(&a)), (0, 0));
    assert_eq!(submits(&step(&mut r, Input::Tick, T0 + 1_499, &P)), 0); // backoff = 500 ms
    assert_eq!(submits(&step(&mut r, Input::Tick, T0 + 1_500, &P)), 1);
}

#[test]
fn non_retryable_failure_alerts_and_still_retries_later() {
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let a = step(
        &mut r,
        Input::CloseFailed {
            retryable: false,
            reason: "bad key".into(),
        },
        T0,
        &P,
    );
    assert_eq!(alerts(&a), 1);
    assert!(!closing(&r).in_flight);
}

#[test]
fn alerts_once_after_n_failed_attempts_whatever_the_error() {
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let mut total_alerts = 0;
    let mut now = T0;
    for _ in 0..7 {
        now += 10_000;
        total_alerts += alerts(&step(
            &mut r,
            Input::CloseFailed {
                retryable: true,
                reason: "?".into(),
            },
            now,
            &P,
        ));
        step(&mut r, Input::Tick, now + 6_000, &P);
    }
    assert_eq!(total_alerts, 1); // exactly at attempt 5 (Policy::alert_after_attempts)
}

#[test]
fn keeps_retrying_past_grace_end_and_deadline_alert_fires_once() {
    // close() has no grace check: the real deadline is the payer's withdraw being mined.
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let late = (GRACE_END + 60) * 1000;
    let a = step(
        &mut r,
        Input::CloseFailed {
            retryable: true,
            reason: "rpc".into(),
        },
        late,
        &P,
    );
    assert_eq!(alerts(&a), 1); // deadline-near alert
    let a = step(&mut r, Input::Tick, late + 5_000, &P);
    assert_eq!((submits(&a), alerts(&a)), (1, 0));
}

#[test]
fn restart_with_a_close_in_flight_resubmits() {
    // R4: persisted Closing{in_flight: true}, process killed before CloseSent/CloseFailed.
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    assert!(closing(&r).in_flight);
    let a = step(&mut r, Input::Restarted, T0 + 60_000, &P);
    assert_eq!(submits(&a), 1);
}

// ---------- step: cancel, re-request, races ----------

#[test]
fn cancel_returns_to_watching_and_a_new_request_gets_a_fresh_deadline() {
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    step(&mut r, Input::CloseRequestCancelled, T0 + 5_000, &P);
    assert_eq!((r.phase.clone(), r.cancellations), (Phase::Watching, 1));
    let a = request(&mut r, T0 + 60_000, GRACE_END + 60);
    assert_eq!(submits(&a), 1);
    assert_eq!(closing(&r).grace_end, GRACE_END + 60);
}

#[test]
fn re_request_seen_only_via_poll_gets_a_fresh_deadline() {
    // R6 B: cancel + re-request both happened between two polls; we only see a newer grace_end.
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let late = (GRACE_END - 100) * 1000;
    step(&mut r, Input::Tick, late, &P); // deadline-near alert fires for the OLD deadline
    assert!(closing(&r).deadline_alerted);
    request(&mut r, late + 1_000, GRACE_END + 500);
    let c = closing(&r);
    assert_eq!((c.grace_end, c.deadline_alerted), (GRACE_END + 500, false));
    assert_eq!(r.cancellations, 1);
}

#[test]
fn closer_pre_check_saw_cancel_goes_back_to_watching_without_sending() {
    // R6 A: topUp rescued the channel while our job sat in the queue.
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let a = step(
        &mut r,
        Input::CloseSkipped { cancelled: true },
        T0 + 500,
        &P,
    );
    assert_eq!(submits(&a), 0);
    assert_eq!(r.phase, Phase::Watching);
}

#[test]
fn gone_on_chain_with_our_txs_asks_for_receipts_before_deciding() {
    let mut r = rec();
    request(&mut r, T0, GRACE_END);
    let tx = B256::repeat_byte(0x33);
    step(&mut r, Input::CloseSent { tx }, T0 + 100, &P);
    let a = step(&mut r, Input::GoneOnChain, T0 + 900, &P); // poll beat the receipt
    assert_eq!(a, vec![Action::CheckReceipts(vec![tx])]);
    assert!(matches!(r.phase, Phase::Closing(_)));
}

#[test]
fn gone_on_chain_without_our_txs_is_a_loss_if_there_was_something_to_claim() {
    let mut r = rec();
    step(&mut r, Input::GoneOnChain, T0, &P);
    assert_eq!(
        r.phase,
        Phase::Done(Outcome::ClosedByOther {
            tx: None,
            settled_to_payee: None,
            refunded: None
        })
    );
}

#[test]
fn closed_by_someone_else_with_nothing_unsettled_is_nothing_to_claim() {
    let mut r = rec();
    r.spent = 0;
    let a = request(&mut r, T0, GRACE_END);
    assert_eq!(submits(&a), 0); // Policy::close_when_nothing_to_claim = false
    let tx = Some(B256::repeat_byte(0x44));
    step(
        &mut r,
        Input::Closed {
            tx,
            settled_to_payee: Some(0),
            refunded: Some(1_000_000),
        },
        T0 + 1,
        &P,
    );
    assert_eq!(r.phase, Phase::Done(Outcome::NothingToClaim { tx }));
}

// ---------- step: bookkeeping ----------

#[test]
fn amounts_only_ever_increase() {
    let mut r = rec();
    step(&mut r, Input::Spent(100), T0, &P);
    step(&mut r, Input::Settled(5), T0, &P);
    step(&mut r, Input::Settled(3), T0, &P);
    step(&mut r, Input::Voucher(v(1)), T0, &P);
    assert_eq!(
        (
            r.spent,
            r.settled,
            r.best_voucher.as_ref().map(|b| b.cumulative)
        ),
        (250_000, 5, Some(260_000))
    );
}

#[test]
fn done_is_absorbing() {
    let mut r = rec();
    step(
        &mut r,
        Input::Closed {
            tx: None,
            settled_to_payee: None,
            refunded: None,
        },
        T0,
        &P,
    );
    let before = r.clone();
    for i in [
        Input::Spent(9_999_999),
        Input::Restarted,
        Input::GoneOnChain,
        Input::Tick,
    ] {
        assert!(step(&mut r, i, T0 + 1, &P).is_empty());
    }
    let a = request(&mut r, T0 + 2, GRACE_END);
    assert!(a.is_empty());
    assert_eq!(r, before);
}

// ---------- protocol helpers (fixtures from Tempo testnet, 2026-09-28) ----------

fn testnet_descriptor() -> (Descriptor, B256) {
    let d = Descriptor {
        payer: address!("0x659910621b79a5b9ce86e3e8411e9ef435e81300"),
        payee: address!("0x6a5ccf03d667978beb7ba52c6f260bc851f398f9"),
        operator: address!("0xc542a6763e0e614aab8b5f46dd631b7eb31a3cc9"),
        token: address!("0x20c0000000000000000000000000000000000000"),
        salt: b256!("0xbe4eeb3bfa45a1546cfbf5a7b3476fac52d4492f5a66141e37afb6af8719560d"),
        authorized_signer: Address::ZERO,
        expiring_nonce_hash: b256!(
            "0xb0e232f0ec07e3dcc16025841959be1e610848876b1db181580354dcafa21e23"
        ),
    };
    (
        d,
        b256!("0x61483b78e5c18eeb1ac15fec91ee43af68b1a405cf9463129132cbdbfcafc458"),
    )
}
const SIG: [u8; 65] = hex!(
    "9954f7401364c25199ad9185bc4c0f03197495fa53f4bd72ed4791a47e9094d848cd64ee566560eb160c7a97374af87d6bd527d68f223dedba2365469583d8ad1b"
);

#[test]
fn channel_id_and_voucher_digest_match_the_chain() {
    let (d, id) = testnet_descriptor();
    assert_eq!(channel_id(&d, CHAIN_ID), id);
    // getVoucherDigest(id, 300000) on the precompile
    assert_eq!(
        voucher_digest(CHAIN_ID, id, 300_000),
        b256!("0xb40b1d0ffdb53c657c234bcc09911a8212bc8fe19ce17a1ab692ce4dd80206dd")
    );
}

#[test]
fn verify_voucher_accepts_the_payers_signature_and_rejects_tampering() {
    let (d, id) = testnet_descriptor();
    let good = Voucher {
        cumulative: 300_000,
        signature: SIG.to_vec(),
    };
    assert!(verify_voucher(&d, id, &good, CHAIN_ID));
    assert!(!verify_voucher(
        &d,
        id,
        &Voucher {
            cumulative: 300_001,
            ..good.clone()
        },
        CHAIN_ID
    ));
    assert!(!verify_voucher(
        &d,
        id,
        &Voucher {
            signature: vec![0; 65],
            ..good.clone()
        },
        CHAIN_ID
    ));
    let mut signer_set = d.clone();
    signer_set.authorized_signer = Address::repeat_byte(5); // then the payer's sig is not enough
    assert!(!verify_voucher(&signer_set, id, &good, CHAIN_ID));
}
