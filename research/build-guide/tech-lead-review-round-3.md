# Tech-lead review, round 3 (final): `tempo-watchtower-build-guide.md` (revision 3)

**Reviewer:** tech lead. **Date:** 2026-09-29.

## Verdict: **APPROVE**

All three round-2 blockers (N1–N3) are fixed, and I verified each one independently, including
a live testnet run of the true day-1 state. No blocking items remain. The round-1 items (R1–R11)
stay closed.

---

## Round-2 blockers: status

| Item | Status | How I verified it |
|---|---|---|
| **N1** `classify_revert` stub reachable from day 4 | **Closed** | **Stub:** in the day-1 copy it returns `CloseError::Transient(message.to_string())`, marked `// TODO(M7)`; no `todo!()`. **`supervise.rs`:** read in full. One `JoinSet`. A task returning or panicking outside a requested shutdown causes an `error` log, cancels the others and returns `Err`, so `main` exits non-zero. A requested shutdown returns `Ok`. The others get a bounded 5 s drain. Its 3 tests pass. `main.rs` runs axum inside the set and documents "return `Err` from the brain if `close_tx.send` fails; never `let _ =`". **Live:** the probe's final step re-sent `close` on the closed channel and got `Err(Transient("…ChannelNotFound…0x1e07dd94"))`, with no panic. |
| **N2** Day-1 probe hit a `todo!()` | **Closed** | `reference/make-day1-copy.sh` generated `/tmp/twr3/day1`. `grep -rn 'todo!'` finds only `tw-core/src/machine.rs` (×2) and `verify.rs` (×1). Nothing outside `tw-core` calls `plan_close`, `step` or `verify_voucher`: the probe now compares `voucher_digest` against the on-chain `getVoucherDigest` and uses a hand-written plan. `tw-demo` on day 1 has only `keygen` and `grace`. **Ran the probe on Moderato** from the day-1 copy with throwaway faucet keys (below). |
| **N3** `.env` flow wrote no keys | **Closed** | Copied the guide's exact `.env.example` to `.env` and ran `tw-demo keygen` with a clean environment. It printed 3 `address 0x…` lines plus `TW_INGEST_TOKEN: (generated, hidden)`. **No 64-hex string on stdout.** `.env` is `-rw-------` (0600), all four values are filled in place, and comments are preserved. A second run prints "all keys already set (nothing changed)". `tempo-watchtower --help` with that `.env` present: 0 hex-64 strings and 0 `KEY` mentions. |

**The planner's candor on N2 is worth noting.** It said plainly that its round-2 "day-1 run"
had used reference code, because an edit silently didn't apply after `cargo fmt` re-wrapped the
line. It then built `make-day1-copy.sh` so the claim is reproducible. That is the right
response.

## What I ran (2026-09-29)

| Check | Result |
|---|---|
| `make-day1-copy.sh /tmp/twr3/day1` | OK |
| Day-1 copy: `cargo fmt --all -- --check` | exit 0 |
| Day-1 copy: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0 |
| Day-1 copy: `cargo test --workspace --all-features --locked` | exit 0 (config, supervise, keygen, precheck and daemon HTTP/time tests pass; 1 ignored network test) |
| Reachable `todo!()` on the day-1/day-2 paths (`probe`, `tw-demo keygen`/`grace`, `tempo-watchtower`, all `tw-chain` fns) | **None** |
| Full reference skeleton (fresh copy): fmt / clippy `-D warnings` / test | 0 / 0 / **43 passed, 0 failed** |
| `tw-demo keygen` on the copied `.env.example` | Keys and token written in place, mode 0600, addresses only, idempotent |
| **Day-1 probe on Tempo testnet** (throwaway faucet keys, day-1 stub state) | See the list after this table |

The probe run, step by step:
1. `CLOSE_GRACE_PERIOD = 900`.
2. Open tx `0x83cbb4d2…f053`. The locally recomputed `channel_id` matches, and the local voucher
   digest equals the on-chain one.
3. `requestClose` → `topUp(1)`. The pre-check reads `close_requested_at: 0`, giving
   `CloseSkipped{cancelled: true}`.
4. `requestClose` again. The pre-check gives `None` (send).
5. The close tx `0x7f28b72e…1af0` landed **3.65 s** after the request, with
   `ChannelClosed{settledToPayee: 250000, refunded: 750001}`.
6. A second close returned `Err(Transient(ChannelNotFound))`, with no panic.

## Remaining non-blocking notes
1. **Attribution by sender (my round-2 rec. 2) is deferred to M7 as optional.** I accept that,
   since it keeps M2's scope stable. The residual effect: if the brain processes a WS
   `ChannelClosed` or a poll `deposit == 0` before the closer's `CloseSent`, or the process
   crashes between broadcast and persist, a real save can be logged as `ClosedByOther`.
   - Before recording, run M7 dry-run 4 and check the protected run's log says `saved`.
   - If it ever says otherwise, implement the `from == operator` lookup. That's about an hour.
2. **keygen permissions nit.** `fs::write` creates a new `.env` with the umask mode (usually
   0644) and only then chmods it to 0600, so for a moment the keys are world-readable. That's
   harmless on a single-user laptop. Post-hackathon, create the file with
   `OpenOptions::new().mode(0o600)`.
3. **The probe prints the voucher signature.** It's not a secret. Just remember that the §8
   scrollback grep will flag it along with tx hashes, and that's expected.

## Final word to the builder
- The plan is now honest, reproducible and sequenced for a beginner:
  - a real chain transaction on day 1;
  - an automatic save on day 4;
  - a crash-safe MVD on day 5;
  - every later day adds one layer on top of something that already works.
- The spec pins every money rule that matters, and the spoilers are fenced off.
- Protect day 3: it's the hardest day. If the tier-1 tests aren't green by that evening, carry
  the tier-2 tests to day 5, as the plan allows.
