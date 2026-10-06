# Tech-lead review, round 2: `tempo-watchtower-build-guide.md` (revision 2)

**Reviewer:** tech lead. **Date:** 2026-09-29.
**Reviewed:**
- the revised guide (1,960 lines);
- `reference/README.md`;
- `reference/workspace-skeleton/` (every source, test and manifest);
- the deck's Appendix A (for R10).

---

## Verdict: **APPROVE WITH REQUIRED CHANGES**

This is a strong revision. All 11 round-1 blockers are addressed, and the ones I could re-verify
hold up:
- the skeleton is clean on the exact CI commands;
- keys can't reach `--help` or `Debug`;
- all 7 bugs I reintroduced were caught by the spec (details below);
- day 4 now delivers a real first save.

**Three new blocking items remain** (N1-N3). They share a root cause: code that runs **before**
its exercise day hits a `todo!()` or a silent no-op. The planner verified that the day-1 file set
*compiles*, but never *ran* it end to end. Each fix is small; together they are well under an
hour. Once they're made I'd approve without another round.

## Dimension scores (round 1 → round 2)

| # | Dimension | R1 | R2 | Why |
|---|---|---|---|---|
| 1 | Feasibility (beginner, ~10 days) | 2 | **4** | A walking skeleton with the save on day 4, the MVD on day 5, and later layers cut bottom-up. Remaining risk: M2 (a 22-test state machine on day 3) is the true bottleneck; see rec. 1. |
| 2 | Technical correctness | 3 | **4** | R4/R5/R6 are fixed and mutation-proven. It loses a point for N1: the closer panics on any send error until day 8. Attribution still depends on cross-task ordering (rec. 2). |
| 3 | Architecture fit | 3 | **4** | Four crates, one `BrainMsg`, an unbounded closer queue, the payee topic filter, and FakeChain/back-fill moved to stretch. Right-sized. |
| 4 | Learning design | 3 | **4** | Tests-as-spec, a hint ladder, and spoilers moved out of the guide. One sequencing slip, and it was mine in round 1: M1 on day 2 needs Book ch. 5-6/9, which are now scheduled for *after* it (rec. 3). |
| 5 | CI/CD | 4 | **5** | Day-1 CI-lite is correct. gitleaks has `pull-requests: read`. The rest waits for day 11. Every ref resolves (re-verified in round 1). |
| 6 | Security | 3 | **4** | Keys never go through clap, `Debug` is redacted, there are leak tests and an on-screen checklist. It loses a point for N3: the documented `.env` flow generates no keys. |
| 7 | Open source / Colosseum | 4 | **4** | A per-file disclosure template tied to `git log`, which is excellent. It omits the given `keygen` (rec. 6). |
| 8 | Demo readiness | 4 | **4** | Both crash windows, the cancel race and the chain-derived `grace_end` are on screen, and the backstop claim is fixed in the deck. The attribution race could still print "lost" for a real save (rec. 2). |
| 9 | Risk management | 3 | **4** | Checkpoints on days 2, 4, 7 and 10, and an honest cut order. The residual cancel race is documented. |

---

## R1-R11 status

| Item | Status | How I checked |
|---|---|---|
| **R1** Walking skeleton, first save on day 4 | **Closed** | §0.2/§0.3/M3 read. The M3 loop is concrete: pure `state_to_inputs`, a work-list, and reuse of M1's GIVEN calls. **Realistic for a beginner if M2 finishes on day 3.** M3 itself is about 150-200 lines that recombine calls the builder already used by hand in M1. The risk is upstream (rec. 1). |
| **R2** M2 stats off the critical path | **Closed** | Now S1. `analysis/` is published with dates in M0. |
| **R3** Over-engineering | **Closed** | 4 crates; FakeChain → S2, back-fill → S3; `payee_filter` uses `topic3`, which is correct because payee is the third indexed field in all six events; one `BrainMsg` (§2.5). |
| **R4** Crash with `in_flight` | **Closed** | `Input::Restarted` clears it. The ledger test `crash_with_close_in_flight_resubmits_after_restart` passes. **Mutation:** I deleted the `in_flight = false` in `Restarted`, and the spec went 21/22. The closer queue is unbounded, with the no-`.send().await` rule stated. |
| **R5** Attribution + cumulative `settledToPayee` | **Closed**, with a residual race (rec. 2) | `our_txs`, `GoneOnChain` → `CheckReceipts`, and `claimed = settledToPayee − settled` are all implemented and tested. **Mutations:** last-tx-only attribution (21/22), `claimed = settledToPayee` (21/22), and a `GoneOnChain` loss despite our txs (21/22). All caught. |
| **R6** Cancel race + stale deadline | **Closed** | A pre-check before every send; `CloseSkipped{cancelled}`; a later `grace_end` refreshes the deadline. **Mutations:** no fresh deadline (21/22); skip-cancel stays Closing (21/22). The daemon-side pre-check itself has no given test (rec. 4). |
| **R7** Key leaks | **Closed** | `config.rs` has no key fields in `Args`; `Secret` / `IngestToken` have manual `Debug`; `debug_never_prints_the_key_or_token` and `help_has_no_key_arguments` pass. `keygen` prints addresses only. The §8 checklist includes the scrollback grep. |
| **R8** Skeleton drift + clippy | **Closed** | I copied the skeleton to `/tmp/twr2` and ran it on Rust **1.98.1** (auto-installed from `rust-toolchain.toml`). `cargo fmt --all -- --check`: pass. `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`: clean. `cargo test --workspace --all-features --locked`: **all pass** (spec 22, props 4, classify 4, daemon lib 4, daemon bin 2). `cargo +1.95.0 check --workspace --all-targets --all-features --locked`: pass. Crate names match the guide. The orphan-rule, dead-code and EIP-712 fixes are real improvements. |
| **R9** Day 1 survivable | **Closed**, with one sequencing slip (rec. 3) | CI-lite, no MSRV job, `rust-analyzer` out of the toolchain file, push to `main`. |
| **R10** Threshold backstop | **Closed** | Guide §3.4/§8 3c updated. The deck's Appendix A line 243 now reads "The payee keeps its SDK's amount-threshold settlement on as a backstop. The tower never calls `settle`". |
| **R11** Tests-as-spec + disclosure | **Closed**, with N1/N2 as side effects | The guide contains no solutions; spoilers are isolated and labelled in `reference/README.md`; Appendix A has the per-file template. **Capture-from-voucher mutation** (the cardinal sin): caught by `capture_stays_within_protocol_bounds`. |

**Mutation summary:** 7 of 7 reintroduced bugs fail at least one test. Six fail exactly one spec
test (21/22). The capture-from-voucher bug fails the property test. The planner's claim holds.

---

## New blocking items

### N1. `classify_revert` is `todo!()` from day 1 to day 8, but `broadcast_close` calls it on every send error (M0 task 4, M3, M4, M7)
**What happens.** M0 tells the builder to copy `classify.rs` with the `classify_revert` body
replaced by `todo!("M7")`. The GIVEN `broadcast_close` maps **every** `.send()` failure through
`classify(&e)` → `classify_revert` (`tw-chain/src/lib.rs`). So on days 4-7, any failed send
panics.

The failure causes are ordinary:
- an RPC hiccup;
- an `eth_estimateGas` revert (`ChannelNotFound` after a withdraw, `CaptureAmountInvalid` from a
  stale `settled`);
- "replacement underpriced" when a restarted tower re-sends while its earlier tx is still pending.

The effect depends on the day:
- In the day-4 inline loop, it crashes the process.
- From day 5, the closer is a spawned task. A panic there kills **only that task**. The brain's
  unbounded `send` to it then fails. If that error is ignored, `in_flight` stays `true` and the
  running tower silently never closes again.

That is the MVD (day 5), the thing §0.3 says you can demo if everything after it slips.

**Change.**
- **(a)** The day-1 stub for `classify_revert` must be a safe default, not `todo!()`:
  ```rust
  pub fn classify_revert(decoded: Option<&E>, message: &str) -> CloseError {
      let _ = decoded;                       // M7: classify the decoded precompile error
      CloseError::Transient(message.to_string())
  }
  ```
  Everything is then retryable until M7. That is safe: the poller turns an already-closed
  channel into `GoneOnChain`, and `alert_after_attempts` fires after 5 failures. The spec file
  still arrives on day 8, when its tests go red against this stub. That's the exercise.
- **(b)** Add one rule to M4: *"if any core task (brain, poller, closer) exits or panics,
  `main` exits non-zero"* (`tokio::select!` over the `JoinHandle`s). The design is restart-safe,
  so crash-and-restart beats a zombie. Also say explicitly: *"if `close_tx.send` returns `Err`,
  log at `error` and exit"*. Never write `let _ =` on that send.
- **(c)** Add to §0.1 as a general rule: **a stub reachable from code that runs before its
  exercise day returns a safe default, never `todo!()`**.

### N2. The day-1 probe panics: it calls `verify_voucher`, which is a `todo!()` stub on day 1 (M0 task 4, `examples/probe.rs:45`)
**What happens.** M0 copies `examples/probe.rs` as is, along with the `verify.rs` stub whose
body is `todo!("M2")`. The probe's line 45 prints `tw_core::verify_voucher(&d, id, &v, CHAIN_ID)`.

So `cargo run -p tw-chain --example probe` funds two keys, opens a channel, signs a voucher, and
then panics with `not yet implemented: M2`. It never reaches `requestClose` or the operator
`close`. The day-1 DoD ("probe tx links: open with operator, the operator's close") can't be met.

The appendix claims "the builder's day-1 state … passes fmt, clippy and test". That is true, but
it doesn't cover **running** the probe. The two day-1 runs listed (`0x48781975…`, `0x46aced1e…`)
must have used the reference `verify.rs`: the first one explicitly used the reference
`plan_close`.

*Verification:* by code reading. `verify.rs` as printed in M2 is an unconditional `todo!()`, and
the probe calls it on the success path. My attempt to run the day-1 set on testnet was blocked by
a tool outage.

**Change.**
- Replace line 45 with a check that needs no exercise code. For example, compare the local
  `tw_core::voucher_digest` (GIVEN) with the on-chain `getVoucherDigest`, which also teaches why
  the struct name matters.
- Or drop the line entirely.
- Then **run** the exact day-1 file set on testnet before claiming "[ran on testnet]". Apply the
  same rule to the probe's use of `broadcast_close` (N1): its error path also reaches `todo!()`.

### N3. The documented `.env` flow silently generates no keys (`.env.example` header, M1 `keygen`)
**What happens.** `.env.example` says *"copy to .env; `tw-demo keygen` fills them"*, and it
contains `TW_OPERATOR_KEY=` with an empty value. dotenvy loads that as an empty string, so
`std::env::var` returns `Ok("")`, and the GIVEN `keygen` skips any var that is `Ok`.

**Verified:** I built `tw-demo` from the skeleton, copied the exact `.env.example` to `.env` and
ran `tw-demo keygen`. It exits **0**, prints nothing and writes nothing. The daemon then fails
with "TW_OPERATOR_KEY is not a valid private key". With no `.env` at all, `keygen` works.

Even if it did append, dotenvy keeps the **first** occurrence, so the empty line would still
win. `keygen` also never generates `TW_INGEST_TOKEN`, which `IngestToken::from_env` requires
(≥16 chars) and the skeleton's `main.rs` reads at startup.

This is a first-hour trap that nudges a beginner into hand-pasting keys.

**Change.** Pick one:
- **(a)** `keygen` treats empty values as missing, **rewrites** `.env` (replacing `VAR=` lines
  rather than appending), and also generates a 32-char random `TW_INGEST_TOKEN`; or
- **(b)** change the header to "don't copy this file; run `tw-demo keygen`, which creates `.env`",
  and have `keygen` write the non-secret defaults too.

Either way, add a unit test: *"`.env` with `TW_OPERATOR_KEY=` → keygen fills it"*.

---

## Remaining recommendations (non-blocking)

1. **Tier the M2 spec so day 4 can't be starved (M2, §0.2).**
   - For a three-day Rust beginner, M2 is the hardest day in the plan: 13 input arms, a `decide`
     function, backoff, `Option` plumbing and the E0502 trap, all against 22 tests plus 4
     properties. That's plausibly 10-12 h, not 7-8.
   - Mark the tests M3 needs as **tier 1**: the 4 `plan_close` tests, request → submit, the
     duplicate no-op, saved/claimed, the retry backoff, cancel, and gone-on-chain.
   - Ship the rest with `#[ignore = "tier 2: finish by day 5"]`: restart, fresh deadline via
     poll, alert-after-N, skipped-cancel and deadline-alert-once.
   - Un-ignore them on day 5, before M4 needs `Restarted`.
   - CI stays green overnight, and the day-4 save isn't blocked by the alert-counting test.
2. **Attribute a close by its sender, not only by `our_txs` (§2.4 rules, M3/M6/M7).**
   - `our_txs` is filled when the brain processes `CloseSent`. Two paths can finalize before
     that and absorb `Done(ClosedByOther)`, printing "lost" for a real save:
     - a WS `ChannelClosed` or a poll `deposit == 0` that the brain handles first. Tasks send to
       the brain unordered, with about 0.5 s of margin (one block);
     - a crash between broadcast and the persist of `CloseSent`.
   - The chain already knows the answer. On `Closed{tx: Some(t)}` with `t ∉ our_txs`, or on
     `GoneOnChain` with empty `our_txs`, emit a lookup action. The daemon fetches the closing tx
     (`get_transaction_by_hash(t).from()`, which M2-stats already uses, or `eth_getLogs` with
     `topic1 = id` over recent blocks). If `from == operator`, feed `CloseSent{t}` first.
   - That is one extra `Action` variant plus one spec test, and it makes the headline metric
     robust.
   - Related: persist only when `step` changed the record (compare before and after).
     Otherwise the 1 s poller forces about 3 redb fsyncs per channel per second. On macOS those
     are `F_FULLFSYNC`, which widens the ordering gap above.
3. **Reading order for day 2 (§1.1, §1.2). This one is my own error from round 1.** M1 (day 2)
   needs structs, enums and `Result` + `?` (Book ch. 5, 6, 9.2), but R9 moved them to "the
   evening of day 2", after M1. Put Book ch. 5-6 and a skim of 9.2 (about 90 min) on the
   **morning** of day 2, and keep Rustlings 07/08/13 for the evening.
4. **Give the pre-check a pure function and a test (M3).** The cancel pre-check is on the "never
   cut" list, but it lives in builder-written daemon code with no given test. Add
   `fn precheck(s: &OnChainState) -> Option<Input>` (`deposit == 0` →
   `CloseSkipped{cancelled: false}`, `close_requested_at == 0` → `CloseSkipped{cancelled: true}`,
   else `None` → send), with a 3-case given test next to `state_to_inputs`.
5. **Feed amounts before the close request (M3 loop).** Emit `Settled` / `Deposit` **before**
   `CloseRequested` in `state_to_inputs`. Otherwise the first plan uses the record's initial
   `settled = 0`. That is harmless in the demo, but it becomes a `CaptureAmountInvalid` retry on
   any channel with a prior settle.
6. **Disclosure completeness (Appendix A).**
   - `keygen` is GIVEN (M1 says so) but falls under "Demo CLI … written by me"; add a row.
   - Also list the ledger's given tests (`survives_reopen`, `crash_with_close_in_flight…`) as
     given.
7. **Run-file writes (M1/M3).** The day-4 tower re-reads `runs/*.json` every second while
   `tw-demo pay` rewrites them. Write atomically (temp file + `rename`), and skip an unreadable
   file for one tick instead of `?`-ing out of the loop.
8. **Hard-coded RPC in `tw-demo grace`.** The skeleton's `grace` hard-codes the testnet URL. Use
   `TW_RPC_URL`, as the daemon does, so the mainnet check (`grace` = 900 on mainnet) is one env
   change.
9. **`wait_receipt` transient errors (M3 sketch).** `wait_receipt` returns `Err(ChainError)` on
   a transient RPC error mid-wait, and the M3 sketch only maps `broadcast_close` errors. Say "any
   `ChainError` while waiting → `CloseFailed{retryable: true}`".

---

## What I verified myself (2026-09-29)

| Check | Result |
|---|---|
| Skeleton on Rust 1.98.1 (`/tmp/twr2`): fmt / clippy `-D warnings` (exact §5.1 command) / test / `+1.95.0 check` | All pass. Tests: spec 22, props 4, classify 4, daemon lib 4 (config 2 + ledger 2), daemon bin 2 |
| Mutation testing: 7 bugs reintroduced one at a time into `machine.rs` | All 7 caught: R4 `in_flight`, R5 last-tx attribution, R5 `claimed` as the cumulative total, R5 `GoneOnChain` loss, R6A skip-cancel, R6B stale deadline, and capture-from-voucher (caught by a proptest) |
| Key handling (R7) | `Args` has no key fields; the `help_has_no_key_arguments` and `debug_never_prints_the_key_or_token` tests pass; `keygen` and the probe print addresses only |
| `keygen` with the documented `.env.example` | **Broken** (N3): exit 0, no keys written |
| Day-1 file set, by code reading | `probe.rs:45` → `verify_voucher` → `todo!("M2")` (**N2**); `broadcast_close` error path → `classify_revert` → `todo!("M7")` (**N1**). A live run of the day-1 set was blocked by a tool outage. The panics are unconditional given the stubs as printed in M0/M2. |
| Deck Appendix A (R10) | Updated: the payee's SDK threshold is the backstop and the tower never settles |
| Precompile semantics behind R5/R6 (from round 1's source read) | Unchanged: `close` has no close-request check; `settledToPayee` is cumulative |

Round-1 checks (grace 900 on both networks, the faucet, WS subscriptions, the `eth_getLogs` cap,
mpp-rs `operator: None`, crate and action versions) are still valid. The revision doesn't depend
on anything new from the chain beyond the planner's live cancel pre-check, which is consistent
with the precompile source: `top_up` sets `close_requested_at = 0`.

---

## Top risks now

1. **Stubs reachable from running code (N1/N2).** The plan's own escape hatch, the day-5 MVD,
   currently contains a panic path. After the N1/N2 fixes, add the §0.1 rule so it doesn't recur
   when the builder adapts the plan.
2. **Day 3 (M2) overrunning.** Everything downstream waits on it. Tier the spec (rec. 1), and
   use the day-4 checkpoint as written.
3. **One wrong word on camera.** A real save logged as "lost" through the attribution race
   (rec. 2). The sender check removes it for about an hour of work.
