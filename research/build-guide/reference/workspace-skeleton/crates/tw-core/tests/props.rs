//! GIVEN property tests for M2 (strategy AND properties are given; make them pass).
use alloy_primitives::{Address, B256};
use proptest::prelude::*;
use tw_core::*;

fn desc() -> Descriptor {
    Descriptor {
        payer: Address::repeat_byte(1),
        payee: Address::repeat_byte(2),
        operator: Address::repeat_byte(3),
        token: Address::repeat_byte(4),
        salt: B256::ZERO,
        authorized_signer: Address::ZERO,
        expiring_nonce_hash: B256::ZERO,
    }
}

fn input() -> impl Strategy<Value = Input> {
    let tx = (0u8..4).prop_map(B256::repeat_byte);
    prop_oneof![
        (0u128..2_000_000).prop_map(|c| Input::Voucher(Voucher {
            cumulative: c,
            signature: vec![1; 65]
        })),
        (0u128..2_000_000).prop_map(Input::Spent),
        (0u128..2_000_000).prop_map(Input::Settled),
        (1_000_000u128..3_000_000).prop_map(Input::Deposit),
        (1_000u64..2_000).prop_map(|g| Input::CloseRequested {
            grace_end: g,
            requested_at_ms: 0
        }),
        Just(Input::CloseRequestCancelled),
        Just(Input::GoneOnChain),
        (
            proptest::option::of(tx.clone()),
            proptest::option::of(0u128..2_000_000)
        )
            .prop_map(|(tx, s)| Input::Closed {
                tx,
                settled_to_payee: s,
                refunded: None
            }),
        tx.prop_map(|tx| Input::CloseSent { tx }),
        any::<bool>().prop_map(|r| Input::CloseFailed {
            retryable: r,
            reason: "x".into()
        }),
        any::<bool>().prop_map(|c| Input::CloseSkipped { cancelled: c }),
        Just(Input::Restarted),
        Just(Input::Tick),
    ]
}

/// (input, ms since previous input)
fn script() -> impl Strategy<Value = Vec<(Input, u64)>> {
    prop::collection::vec((input(), 0u64..3_000), 1..60)
}

fn record() -> ChannelRecord {
    ChannelRecord::new(B256::repeat_byte(9), desc(), 1_000_000)
}

proptest! {
    #[test]
    fn capture_stays_within_protocol_bounds(
        deposit in 1u128..10_000_000, settled_frac in 0u128..=100, spent in 0u128..20_000_000,
        voucher_extra in 0u128..10_000_000,
    ) {
        let mut r = record();
        r.deposit = deposit;
        r.settled = deposit * settled_frac / 100;
        r.spent = spent;
        r.best_voucher = Some(Voucher { cumulative: (r.settled + voucher_extra).min(deposit), signature: vec![1; 65] });
        let p = plan_close(&r).unwrap();
        prop_assert!(r.settled <= p.capture);                // never below what's already settled
        prop_assert!(p.capture <= p.cumulative.max(r.settled)); // never above the voucher
        prop_assert!(p.capture <= r.deposit);
        prop_assert!(p.capture <= r.spent.max(r.settled));   // never more than consumed (spec 13.2)
    }

    #[test]
    fn amounts_never_decrease_and_done_never_changes(s in script()) {
        let (mut r, mut now, p) = (record(), 1_000_000u64, Policy::default());
        for (i, dt) in s {
            let before = r.clone();
            now += dt;
            step(&mut r, i, now, &p);
            prop_assert!(r.spent >= before.spent && r.settled >= before.settled);
            prop_assert!(r.best_voucher.as_ref().map(|v| v.cumulative) >= before.best_voucher.as_ref().map(|v| v.cumulative));
            if matches!(before.phase, Phase::Done(_)) { prop_assert_eq!(&r, &before); }
        }
    }

    #[test]
    fn never_submits_while_a_close_is_in_flight(s in script()) {
        let (mut r, mut now, p) = (record(), 1_000_000u64, Policy::default());
        for (i, dt) in s {
            now += dt;
            let was_in_flight = matches!(&r.phase, Phase::Closing(c) if c.in_flight);
            let releases = matches!(i, Input::CloseFailed { .. } | Input::CloseSkipped { .. } | Input::Restarted
                | Input::CloseRequestCancelled);
            let actions = step(&mut r, i, now, &p);
            let submitted = actions.iter().any(|a| matches!(a, Action::SubmitClose(_)));
            prop_assert!(!(was_in_flight && !releases && submitted));
        }
    }

    #[test]
    fn applying_an_input_twice_equals_applying_it_once(s in script(), last in input()) {
        prop_assume!(!matches!(last, Input::CloseFailed { .. }));   // counts attempts by design
        let (mut r, mut now, p) = (record(), 1_000_000u64, Policy::default());
        for (i, dt) in s { now += dt; step(&mut r, i, now, &p); }
        let mut once = r.clone();
        step(&mut once, last.clone(), now, &p);
        let mut twice = once.clone();
        step(&mut twice, last, now, &p);
        prop_assert_eq!(once, twice);
    }
}
