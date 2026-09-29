use crate::{Action, ChannelRecord, ClosePlan, Input, PlanError, Policy};

pub fn plan_close(r: &ChannelRecord) -> Result<ClosePlan, PlanError> {
    let _ = r;
    todo!("M2")
}

pub fn step(r: &mut ChannelRecord, input: Input, now_ms: u64, p: &Policy) -> Vec<Action> {
    let _ = (r, input, now_ms, p);
    todo!("M2")
}
