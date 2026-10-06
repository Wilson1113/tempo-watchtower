#!/usr/bin/env bash
# Recreate the builder's exact DAY-1 state from the reference skeleton, in a fresh directory:
# exercise bodies as the stubs printed in the guide (M2), classify_revert as the SAFE DEFAULT
# (never todo!), no spec files (they arrive on days 3 and 8), no ledger (day 5).
# Usage: reference/make-day1-copy.sh /tmp/day1   then: cd /tmp/day1 && cargo test ...
set -euo pipefail
src="$(cd "$(dirname "$0")/workspace-skeleton" && pwd)"
dst="${1:?usage: make-day1-copy.sh <new-dir>}"
mkdir -p "$dst" && (cd "$src" && tar --exclude=target -cf - .) | (cd "$dst" && tar -xf -)
cd "$dst"
rm -f crates/tw-core/tests/spec.rs crates/tw-core/tests/props.rs crates/tw-chain/tests/classify.rs \
      crates/tw-daemon/src/ledger.rs
cat > crates/tw-core/src/machine.rs <<'RS'
use crate::{Action, ChannelRecord, ClosePlan, Input, PlanError, Policy};

pub fn plan_close(r: &ChannelRecord) -> Result<ClosePlan, PlanError> {
    let _ = r;
    todo!("M2")
}

pub fn step(r: &mut ChannelRecord, input: Input, now_ms: u64, p: &Policy) -> Vec<Action> {
    let _ = (r, input, now_ms, p);
    todo!("M2")
}
RS
cat > crates/tw-core/src/verify.rs <<'RS'
use crate::{Descriptor, Voucher};
use alloy_primitives::B256;

pub fn verify_voucher(d: &Descriptor, id: B256, v: &Voucher, chain_id: u64) -> bool {
    let _ = (d, id, v, chain_id);
    todo!("M2")
}
RS
python3 - <<'PY'
p = 'crates/tw-chain/src/classify.rs'; s = open(p).read()
i = s.rindex("\npub fn classify_revert") + 1; j = s.index("{", i)
s = s[:j] + "{\n    let _ = decoded; // TODO(M7): classify the decoded precompile error\n    CloseError::Transient(message.to_string())\n}\n"
open(p, 'w').write(s)
p = 'crates/tw-daemon/src/lib.rs'; s = open(p).read(); open(p, 'w').write(s.replace("pub mod ledger;\n", ""))
p = 'crates/tw-daemon/src/main.rs'; s = open(p).read()
s = s.replace("    config, ledger,\n", "    config,\n").replace("    let _ledger = ledger::Ledger::open(&cfg.args.ledger)?;\n", "")
open(p, 'w').write(s)
PY
echo "day-1 state written to $dst"
