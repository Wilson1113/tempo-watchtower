//! SPOILER: reference implementation of the M2 exercises (`plan_close`, `step`).
//! In the builder's repo this file starts as two `todo!()` bodies. If you open this file before
//! the spec passes on your own code, say so in your README disclosure.

use crate::{Action, ChannelRecord, ClosePlan, Closing, Input, Outcome, Phase, PlanError, Policy};

pub fn plan_close(r: &ChannelRecord) -> Result<ClosePlan, PlanError> {
    let target = r.spent.max(r.settled);
    let voucher_cap = r.best_voucher.as_ref().map_or(r.settled, |v| v.cumulative);
    let capture = r.settled.max(target.min(voucher_cap).min(r.deposit));
    let clamped = capture < target;
    if capture == r.settled {
        if target > r.settled && r.best_voucher.is_none() {
            return Err(PlanError::NoVoucher);
        }
        return Ok(ClosePlan {
            cumulative: r.settled,
            capture,
            signature: Vec::new(),
            claimable: 0,
            clamped,
        });
    }
    let v = r.best_voucher.as_ref().ok_or(PlanError::NoVoucher)?;
    Ok(ClosePlan {
        cumulative: v.cumulative,
        capture,
        signature: v.signature.clone(),
        claimable: capture - r.settled,
        clamped,
    })
}

fn backoff(attempts: u32, p: &Policy) -> u64 {
    let shift = attempts.saturating_sub(1).min(16);
    p.base_backoff_ms
        .saturating_mul(1u64 << shift)
        .min(p.max_backoff_ms)
}

pub fn step(r: &mut ChannelRecord, input: Input, now_ms: u64, p: &Policy) -> Vec<Action> {
    if matches!(r.phase, Phase::Done(_)) {
        return Vec::new();
    }
    let mut out = Vec::new();
    match input {
        Input::Voucher(v) => {
            if r.best_voucher
                .as_ref()
                .is_none_or(|b| v.cumulative > b.cumulative)
            {
                r.best_voucher = Some(v);
            }
        }
        Input::Spent(s) => r.spent = r.spent.max(s),
        Input::Settled(s) => r.settled = r.settled.max(s),
        Input::Deposit(d) => r.deposit = r.deposit.max(d),
        Input::CloseRequested {
            grace_end,
            requested_at_ms,
        } => match &mut r.phase {
            Phase::Watching => {
                r.phase = Phase::Closing(Closing {
                    grace_end,
                    requested_at_ms,
                    detected_at_ms: now_ms,
                    attempts: 0,
                    next_attempt_ms: now_ms,
                    in_flight: false,
                    deadline_alerted: false,
                });
            }
            Phase::Closing(c) => {
                if grace_end > c.grace_end {
                    // cancel + re-request that we only saw as a newer closeRequestedAt (R6 B)
                    c.grace_end = grace_end;
                    c.requested_at_ms = requested_at_ms;
                    c.deadline_alerted = false;
                    r.cancellations += 1;
                }
            }
            Phase::Done(_) => {}
        },
        Input::CloseRequestCancelled => {
            if matches!(r.phase, Phase::Closing(_)) {
                r.phase = Phase::Watching;
                r.cancellations += 1;
            }
        }
        Input::GoneOnChain => {
            if r.our_txs.is_empty() {
                r.phase = Phase::Done(not_ours(r, None, None, None));
            } else {
                out.push(Action::CheckReceipts(r.our_txs.clone()));
            }
        }
        Input::Closed {
            tx,
            settled_to_payee,
            refunded,
        } => {
            let outcome = match tx {
                Some(t) if r.our_txs.contains(&t) => {
                    let total = settled_to_payee.unwrap_or(r.settled);
                    Outcome::Saved {
                        tx: t,
                        claimed: total.saturating_sub(r.settled),
                        settled_to_payee: total,
                        refunded: refunded.unwrap_or(0),
                    }
                }
                _ => not_ours(r, tx, settled_to_payee, refunded),
            };
            r.phase = Phase::Done(outcome);
        }
        Input::CloseSent { tx } => {
            if !r.our_txs.contains(&tx) {
                r.our_txs.push(tx);
            }
        }
        Input::CloseFailed { retryable, reason } => {
            if let Phase::Closing(c) = &mut r.phase {
                c.in_flight = false;
                c.attempts += 1;
                c.next_attempt_ms = now_ms + backoff(c.attempts, p);
                if !retryable {
                    out.push(Action::Alert(format!(
                        "close failed (not retryable): {reason}"
                    )));
                }
                if c.attempts == p.alert_after_attempts {
                    out.push(Action::Alert(format!(
                        "{} close attempts failed: {reason}",
                        c.attempts
                    )));
                }
            }
        }
        Input::CloseSkipped { cancelled } => {
            if let Phase::Closing(c) = &mut r.phase {
                c.in_flight = false;
                if cancelled {
                    r.phase = Phase::Watching;
                    r.cancellations += 1;
                } else {
                    // already gone on-chain: the poller will report GoneOnChain / Closed
                    c.next_attempt_ms = now_ms + p.max_backoff_ms;
                }
            }
        }
        Input::Restarted => {
            if let Phase::Closing(c) = &mut r.phase {
                c.in_flight = false; // R4: the job that was in flight died with the process
                c.next_attempt_ms = now_ms;
            }
        }
        Input::Tick => {}
    }
    out.extend(decide(r, now_ms, p));
    out
}

fn not_ours(
    r: &ChannelRecord,
    tx: Option<alloy_primitives::B256>,
    settled_to_payee: Option<u128>,
    refunded: Option<u128>,
) -> Outcome {
    if r.spent > r.settled {
        Outcome::ClosedByOther {
            tx,
            settled_to_payee,
            refunded,
        }
    } else {
        Outcome::NothingToClaim { tx }
    }
}

fn decide(r: &mut ChannelRecord, now_ms: u64, p: &Policy) -> Vec<Action> {
    let plan = plan_close(r); // borrow of `r` ends here
    let mut out = Vec::new();
    let Phase::Closing(c) = &mut r.phase else {
        return out;
    };
    if !c.deadline_alerted && now_ms / 1000 + p.deadline_alert_secs >= c.grace_end {
        c.deadline_alerted = true;
        out.push(Action::Alert("close deadline near; still retrying".into()));
    }
    if !c.in_flight && now_ms >= c.next_attempt_ms {
        match plan {
            Ok(plan) if plan.claimable > 0 || p.close_when_nothing_to_claim => {
                if plan.clamped {
                    out.push(Action::Alert(
                        "spent exceeds voucher/deposit; capture clamped".into(),
                    ));
                }
                c.in_flight = true;
                out.push(Action::SubmitClose(plan));
            }
            Ok(_) => {}
            Err(e) => {
                c.next_attempt_ms = now_ms + p.max_backoff_ms;
                out.push(Action::Alert(format!("cannot plan close: {e}")));
            }
        }
    }
    out
}
