//! GIVEN test for the GIVEN `precheck` (copy on day 4 with M3).
use tw_chain::{OnChainState, precheck};
use tw_core::Input;

#[test]
fn precheck_sends_only_when_still_requested_and_open() {
    let st = |deposit, close_requested_at| OnChainState {
        settled: 0,
        deposit,
        close_requested_at,
    };
    assert_eq!(
        precheck(&st(0, 0)),
        Some(Input::CloseSkipped { cancelled: false })
    );
    assert_eq!(
        precheck(&st(0, 1_790_000_000)),
        Some(Input::CloseSkipped { cancelled: false })
    );
    assert_eq!(
        precheck(&st(1_000_000, 0)),
        Some(Input::CloseSkipped { cancelled: true })
    );
    assert_eq!(precheck(&st(1_000_000, 1_790_000_000)), None);
}
