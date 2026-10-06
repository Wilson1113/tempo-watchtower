# Tech-lead review, round 1: `tempo-watchtower-build-guide.md`

**Reviewer:** tech lead (staff Rust / chain infra). **Date:** 2026-09-28.
**Reviewed:** the build guide (2,289 lines), `reference/testnet-probe/`, `reference/workspace-skeleton/`,
the study guide Parts 4-6, the deck's Appendix A, and `evidence-round2.md` (CORRECTIONS first).

---

## Verdict: **APPROVE WITH REQUIRED CHANGES**

The Tempo integration research is excellent. Every load-bearing chain fact I re-checked is true,
and the given plumbing compiles and matches the precompile source. The capture rule, the
"keep retrying until `withdraw` is mined" rule and the one-brain actor are the right calls.

The problems are elsewhere, in three areas:

1. **Sequencing and scope.** The plan is sequenced for an experienced Rust engineer, not a
   first-timer. The first *automatic* save lands on day 8, after six layers of infrastructure
   (ledger, actor, HTTP, payee server, WebSocket, FakeChain). The escape hatch opens on day 8,
   which is too late to use it.
2. **State-machine bugs.** Four edge-case bugs would break the restart demo or put wrong numbers
   on camera.
3. **Honesty and security traps.** Three of them: a key leak through `--help`, an Appendix A
   claim the build doesn't implement, and AI reference solutions that contradict the planned
   "core logic is my own work" disclosure.

All of this is fixable by editing the guide. None of it needs new research. Hence the verdict is
not REWORK.

## Dimension scores

| # | Dimension | Score | One-line justification |
|---|---|---|---|
| 1 | Feasibility for a Rust beginner in ~10 days | **2 / 5** | About 80 h of plan, with the hardest async work on days 5-8. The first automatic save is on day 8, the MVD decision is at day-8 EOD, and M2 spends a whole day on something the demo doesn't need. |
| 2 | Technical correctness | **3 / 5** | Chain facts and the capture math are right. The state-machine edges are not: `in_flight` survives a crash, there is a `topUp`-cancel race, `last_tx` misattributes outcomes, and the poll path misses the fresh deadline. |
| 3 | Architecture fit | **3 / 5** | The actor and the pure core are right. It is over-built for day 10: six crates, `ChainClient` + `async-trait` + `FakeChain`, a block cursor with back-fill, and a shared `known` closure that contradicts "the brain owns state". |
| 4 | Learning design | **3 / 5** | The just-in-time table, the pitfalls list and the book pointers are good. Day 1 is overloaded. Full reference solutions for the core exercises undercut both learning and the disclosure. There are no tests-as-spec. |
| 5 | CI/CD quality | **4 / 5** | The YAML is correct and every action ref resolves to the current version (verified). It is too heavy for day 1, and the gitleaks job lacks `pull-requests` permission. The skeleton fails the guide's own clippy command. |
| 6 | Security | **3 / 5** | Good `.env` and gitignore hygiene and good testnet-only messaging. It misses that clap prints env secrets in `--help` (verified), and it slightly overstates what secret scanning catches. |
| 7 | Open source, repo practice, Colosseum rules | **4 / 5** | Thorough: license, SECURITY, CHANGELOG, a visible history and a disclosure section. The disclosure wording needs to match what actually happens with the AI sketches. |
| 8 | Demo readiness | **4 / 5** | It maps step by step to Appendix A: split screen, measured latency, WS-kill, restart and a time-lapsed 900 s. Two gaps: the "threshold settle backstop" line in Appendix A has no build or wording, and the restart demo only covers the easy crash window. |
| 9 | Risk management | **3 / 5** | The risk table is honest and it correctly names time as the biggest risk. But the mitigation (MVD at day-8 EOD) arrives too late, and the MVD is framed as a fallback rather than as the spine. |

---

## Required changes (blocking)

### R1. Re-sequence into a walking skeleton: first automatic save by day 4 (§0.2, §0.3, §4)
**Problem.** The plan builds M4 (ledger), M5 (actor + HTTP + payee server) and M6 (WS + poll +
reconcile + FakeChain) before M7 sends the first automatic `close()` on day 8. Slips in async Rust
compound. If M5-M6 slip by two days, which is likely for a first-timer, the first save lands on
day 10: feature-freeze day.

**Change §0.2 to this schedule** (same dates and total hours, reordered so that each day ships
something demoable):

| Day | Date | Focus | End-of-day proof |
|---|---|---|---|
| 1 | Mon 9/28 | **M0-lite.** Repo, licenses, CI with fmt + clippy + test (ubuntu) + gitleaks. Paste the verified probe as `crates/tw-chain/examples/probe.rs` in a labelled commit and run it on testnet. | CI green, plus probe tx links in the devlog. **You touch the chain on day 1.** |
| 2 | Tue 9/29 | **M1.** `tw-demo` CLI: keygen, fund, open, pay (records `spent` + voucher in `runs/<label>.json`), request-close, operator-close, withdraw, status. Start one withdraw channel in the morning so the 15 min passes in the background. | An operator-key close tx link and a withdraw tx link |
| 3 | Wed 9/30 | **M3.** Pure `tw-core`: `plan_close`, `step`, `channel_id`, `verify_voucher`. The tests are given as a spec (see R11). | ≥15 unit tests + 2 proptests green |
| 4 | Thu 10/1 | **Walking skeleton, `tempo-watchtower` v0.** One `async fn main` loop with no HTTP, no WS and no DB. It loads channels from `runs/*.json`, polls `getChannelStatesBatch` every 1 s, feeds `step()`, and sends `close()` with the given `send_close`. | **First automatic save**, with its explorer link. Celebrate here, not on day 8. |
| 5 | Fri 10/2 | **M4.** redb ledger, plus the restart-resume rule from R4. Refactor the loop into the brain actor (poll task → brain → closer task). | `kill -9` / restart / close works, poll-only. **This is the MVD.** |
| 6 | Sat 10/3 | **M5.** Ingest API (axum) and the demo payee server with `spent` forwarding (L = 1 s). | payer → payee → tower; `GET /v1/channels/{id}` shows `spent` |
| 7 | Sun 10/4 | **M6-lite.** A WS task (logs filtered by the payee topic, plus a `newHeads` heartbeat) layered on top. The poller stays. **Weekly video #1.** | `via=ws` within ~1 s; `--no-ws` still works |
| 8 | Mon 10/5 | **M7.** Closer hardening: error classification, backoff, the cancel-race pre-check (R6), outcome attribution (R5) and the fee-balance alert | The failure-path dry runs pass once |
| 9 | Tue 10/6 | **M8.** The `scenario` harness: control vs protected | Two full runs in a row |
| 10 | Wed 10/7 | **M9.** Metrics, WS-kill (`--no-ws` or chaos) and restart demos. **Freeze at 22:00.** | `docs/results.md` with 3 runs |
| 11 | Thu 10/8 | **M10.** Docs; add the deny, macOS, audit and release workflows; tag `v0.1.0`. M2 stats **only if everything above is done.** | Tagged release |

**Decision checkpoints** (these replace the current list):
- Day 2 EOD: the tracer bullet works.
- **Day 4 EOD: an automatic save via poll.** If it's missing, nothing else starts until it lands.
- **Day 7 at 18:00:** if WS isn't stable, ship poll-only and label it on screen.
- Day 10: freeze.

**Change §0.3:** say that the MVD *is* days 1-5, and that everything after day 5 is additive.

### R2. Take M2 (mainnet stats, `tw-stats`) off the critical path (§0.2 day 3, §4 M2, §2.1)
- M2 costs a full day in week 1. It needs ~171 throttle-prone `eth_getLogs` calls, ~465 MB of
  logs and ~344 `get_transaction_by_hash` lookups.
- It produces numbers that already exist and are reproducible in Python. Appendix A doesn't need
  them.
- Move M2 to "stretch, day 11+ only".
- Delete `tw-stats` from the §2.1 layout (it can come back later).
- The one piece M6 needs, `to_event(&Log) -> Option<ChainEvent>` tested against 5 saved testnet
  logs, moves into M6 as a 1-2 h task.
- **Publish the Python `onchain/` scripts in the repo under `analysis/`**, in a commit whose
  message states their original dates, so judges can reproduce 304/344 without `tw-stats`.

### R3. Cut the over-engineering from the MVP scope (§2.1, §2.6, §4 M6, §7 "testing pyramid")
Mark each of the following as **stretch**, and rewrite M6 without them:
- **`ChainClient` trait + `async-trait` + `FakeChain`.**
  - Your own design already makes the brain chain-free: its inputs are `Input` messages and its
    output is `CloseJob`. So the brain is fully testable by sending it messages. That is the
    point of §2.5.
  - `FakeChain` re-implements the precompile. That's a day of work and a second source of bugs,
    for testing thin I/O shells that testnet tests better.
  - It also forces a beginner into `dyn` + async-trait + `Send + Sync + 'static` on day 7.
- **Block cursor + `eth_getLogs` back-fill + `put_with_cursor`.**
  - The tower only needs `closeRequestedAt`, `settled` and `deposit` to act, and
    `getChannelStatesBatch` over `ledger.active()` is authoritative for all three.
  - Startup reconcile = **one poll pass**. That is also exactly what the restart demo shows.
  - Keep back-fill as a stretch item for discovering `ChannelOpened` and for exact
    `ChannelClosed` amounts.
- **The shared `known: Arc<dyn Fn(&B256) -> bool>` in the WS sketch.** It contradicts "the brain
  exclusively owns state". Replace it with an RPC-side filter on the indexed **payee** topic
  (`watched_filter().topic3(payee.into_word())`; `payee` is `topic[3]` in every event). Forward
  everything that matches to the brain and let the brain drop unknown ids.
- **Crates (optional but recommended).** Fold `tw-store` into `tw-daemon` as `mod ledger`. That
  leaves 4 crates: `tw-core` (pure), `tw-chain`, `tw-daemon`, `tw-demo`. Keeping `tw-core`
  separate is the part that signals discipline to judges; the rest is ceremony.
- **One message type.** The §4 M5 sketch uses `mpsc::Receiver<(B256, Input)>`, while the §4 M6
  sketch uses `BrainMsg::Event / Head / BackfillFromCursor`. Define one `enum BrainMsg` once, in
  §2.5, and use it everywhere. Beginners copy types literally.

### R4. A crash while `in_flight = true` strands the channel forever (§4 M3 types, §4 M5 `Brain::apply`, §4 M6 reconcile)
**Bug.**
- `decide` sets `in_flight = true`, and `apply` persists the record **before** handing the job to
  the closer. That part is correct.
- But if the tower is killed after that commit and before `CloseSent` / `CloseFailed` comes back,
  the ledger holds `Closing { in_flight: true }`.
- After a restart, `decide` never fires again (`!c.in_flight` is false) and no closer job exists.
  The channel is never closed, and the payer's `withdraw` wins.
- A second path leads to the same state: `self.close_tx.try_send(job)` silently drops the job
  when the queue is full.

**Change.**
- **(a)** When the brain loads records, set `in_flight = false` for every `Closing` record. It's
  safe: the closer pre-checks chain state, and `close` reverts with `ChannelNotFound` if the
  channel is already closed.
- **(b)** Make the closer queue `mpsc::unbounded_channel()`. It is naturally bounded by the
  number of channels. Or, if you keep a bounded queue, revert `in_flight` when `try_send` fails.
  Never `.send().await` from the brain into the closer: the brain ↔ closer loop can deadlock.
- **(c)** Add the test "record persisted with `in_flight = true` → new brain on the same ledger
  → `SubmitClose` emitted".
- **(d)** Add a second restart demo in M9: `kill -9` *after* `close sent` and before the receipt.

### R5. Outcome attribution is wrong after retries or timeouts (§4 M3 "How to tell Saved…", §4 M6 poller, §4 M7)
**Bug.**
- "It's ours if `tx == last_tx`" fails whenever attempt 1 times out after 20 s but still lands,
  while attempt 2 sets a new `last_tx`.
- Separately, if the poller sees `deposit == 0` before the closer's receipt returns, the plan
  records `ClosedByOther` "with unknown amounts". The protected run would then show as **lost**
  on camera.

**Change.**
- Replace `last_tx: Option<B256>` with `our_txs: Vec<B256>`. `CloseSent` pushes onto it.
  `Saved` iff `closed_tx ∈ our_txs`.
- In the poller's `deposit == 0` path, while `our_txs` is non-empty, first fetch those receipts
  and look for `ChannelClosed`. Only fall back to `ClosedByOther` if none is found.
- Also state in §3.3 that **`ChannelClosed.settledToPayee` is the cumulative total, not the
  delta**. Verified in `crates/precompiles/src/tip20_channel_reserve/mod.rs`: `close` emits
  `settledToPayee: capture`, and `withdraw` emits `state.settled`. So "payee received on this
  close" = `settledToPayee − settled_before`. The demo table (M8 step 8) and `Outcome::Saved`
  must use that.

### R6. `topUp`-cancel race and stale deadline (§2.4 rules, §4 M3 behaviour table, §4 M7 pre-check)
**Bug A: the tower can close a channel the payer just rescued.**
- Verified in the precompile source: `close` has **no** `close_requested_at` check.
- The sequence is: `CloseRequested` → `SubmitClose` is queued → the payer calls `topUp`
  (`CloseRequestCancelled`) → the queued close still lands.
- The tower would kill a channel the payer topped up precisely to keep using it.

**Change the M7 pre-check** from "skip if `deposit == 0`" to:
- skip if `deposit == 0` (already closed); **or**
- skip if `closeRequestedAt == 0` (cancelled). Report `CloseSkipped`, and let the brain decide
  again.

Also update the "at most one in flight" property test. Cancel then re-request can queue a second
job while the first is still queued, and the pre-check is what makes that safe.

**Bug B: the fresh-deadline rule is lost in poll-only mode.**
- If cancel + re-request both happen between two polls, the poller never sees
  `closeRequestedAt == 0`. It sees a *new* `closeRequestedAt`.
- The table maps "Closing + CloseRequested" to "unchanged", so `grace_end` stays stale.

**Change:** "Closing + CloseRequested with `grace_end > c.grace_end` → update `grace_end`, reset
`deadline_alerted`". An equal `grace_end` stays a no-op, so idempotency holds. Add this as the
M3 test "re-request seen only via poll gets a fresh deadline". Appendix A explicitly claims this
behaviour, and M3 test names are shown on camera (§8 step 3c).

### R7. Private keys leak through `--help` and `Debug` (§4 M0 `main.rs`, §4 M1 keys, §6.3, §7 "Secrets")
**Verified.** I built a clap 4.6 binary with `#[arg(long, env = "TW_OPERATOR_KEY")]`.
`--help` prints `[env: TW_OPERATOR_KEY=0xdeadbeefSECRET]`, and `{:?}` prints the value.

With `dotenvy::dotenv()` at the top of `main` (§4 M1 hint), running `tempo-watchtower --help`
prints the operator key. The M0 DoD tells the builder to run `--help`, and a demo recording is
exactly where someone would run it.

**Change.**
- **(a)** Keys are **never** clap flags. A flag lands in shell history and in `ps`.
- **(b)** If a key is read through clap `env`, add `hide_env_values = true`. Simpler: read it with
  `std::env::var` into a `struct Secret(PrivateKeySigner)` whose manual `impl Debug` prints only
  the address.
- **(c)** Add a unit test asserting that `format!("{:?}", config)` doesn't contain the key hex.
- **(d)** §8 honesty checklist: never `cat .env` or run `env` on camera. Before recording,
  `grep` the recording-session terminal scrollback for `0x` followed by 64 hex characters.

### R8. Fix the "verified clean" claims and the skeleton / guide drift (§4 M0 "[compiled; clippy -D warnings … pass]", appendix)
**Verified:** `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
(the exact CI command in §5.1) **fails** on the skeleton:

```
error: used `unwrap_err()` on a `Result` value
 --> crates/tw-chain/examples/revert.rs:7:15
```

`build`, `test` (3/3 pass) and `+1.95.0 check` do pass.

The skeleton has also drifted from the guide:
- its crates are named `demo` / `watchtower`, where the guide uses `tw-demo` / `tw-daemon`;
- its workspace deps lack the `dotenvy`, `tower` and `http-body-util` that the guide's
  `Cargo.toml` lists;
- the guide's "[compiled]" `Cargo.toml` is therefore not the one that was compiled.

**Change.**
- Fix `revert.rs` to use `expect_err`.
- Rename the skeleton crates to match the guide.
- Re-run the guide's exact root `Cargo.toml` through the §5.1 commands, and only then keep the
  "[compiled]" tags.
- A beginner who pastes the examples on day 1 gets red CI with a lint they don't understand.

### R9. Make day 1 survivable (§0.2 day 1, §1.1 "Before M1", §4 M0 tasks 3-4, §5.1)
**Problem.** Day 1 currently asks for all of the following, which is 8-9 h for someone who has
never written Rust:
- toolchain installs;
- 6 CI jobs (including a macOS matrix, MSRV and cargo-deny across alloy's tree);
- secret scanning and branch protection with required checks;
- a PR workflow;
- then, "about 3 hours", Book chapters 1-6 **plus** Rustlings 00-08 **plus** Tokio "Async in
  depth".

**Change.**
- Day 1 CI = `fmt`, `clippy`, `test` (ubuntu only) and `gitleaks`. Move `deny`, `msrv`, the
  macOS matrix, `audit.yml`, `release.yml`, templates and required-status branch protection to
  day 11.
- Drop the MSRV job entirely. This is a binary with a pinned toolchain, and the job doubles CI
  time.
- Remove `rust-analyzer` from `rust-toolchain.toml` `components`: every CI job then installs it
  for nothing. Install it locally with rustup instead.
- Push straight to `main` during the hackathon: small commits, no PR ceremony. Many small
  commits on `main` *is* the visible history Colosseum wants.
- Day-1 reading = Book ch. 1-4 plus Rustlings 00-06. Replace "Async in depth" with Tokio
  "Hello Tokio" only; "Async in depth" is about hand-writing futures, which is the wrong page
  for day 1. Book ch. 5-6 and Rustlings 07-08 move to the evening of day 2.

### R10. Don't claim a threshold-settle backstop the build doesn't have (§8 step 3c; deck Appendix A §3 "Keys and health")
- Appendix A says "An amount-threshold settle stays on as a backstop". Nothing in the plan
  builds it.
- It also shouldn't be in the tower: `settle` sets `settled = cumulativeAmount` (verified). Your
  vouchers run one price ahead of `spent`, so the tower settling a voucher would capture
  pre-authorized, unconsumed credit, which is the thing §13.2 forbids.

**Change.** In §8 step 3c, add the line: *"Recommended payee setup: keep your SDK's
settlement threshold (mppx `SettlementSchedule`) on as a backstop; the tower doesn't settle, it
only closes on request."* Say it that way in the video.

### R11. Make the AI-code disclosure true, and turn the core exercises into tests-as-spec (§4 M3/M6/M7 `<details>` reference solutions, §6.6, §10)
**Problem.**
- §6.6 tells the builder to disclose that "the core logic (state machine, ledger, watcher,
  closer policy) is your own work".
- But the guide ships complete, AI-written solutions for exactly that core: the `plan_close` and
  `decide` sketch, `classify`, and the WS loop.
- Under deadline pressure they will be opened and pasted. The disclosure then becomes false, and
  a judge asking "walk me through `decide`" exposes it.

**On "pre-existing code".** The reference code was written 2026-09-28, inside the contest period
(which started 2026-09-14), so it is **not** pre-existing. It **is** third-party (AI-authored)
code under rules §9 ("inform Administrator of the status and ownership of any open-source or
other third party code"). That calls for disclosure, not avoidance.

**Change.**
- **(a)** Replace the M3 `<details>` solution with a **given test file**, `tw-core/tests/spec.rs`,
  containing the ~15 behaviour-table tests as `#[test]`s. The builder makes them pass. Keep a
  3-step hint ladder: signature → pseudocode → one tricky line. Do the same for `classify` (give
  the tests; the `match` is theirs). The WS loop can stay as given plumbing, but label it as
  such in the disclosure.
- **(b)** Never copy `reference/` wholesale. Every given snippet lands in its own commit, with a
  message suffix like `(plumbing from AI-drafted build guide)`.
- **(c)** Put a README "Prior work, third-party code & AI assistance" section listing, **by
  file**, what was given vs written.
- **(d)** Also give the proptest `Input` strategy (`prop_oneof![…]`) as given code. Writing
  strategies isn't the learning goal, and it costs a beginner about 2 h.

---

## Recommended changes (non-blocking)

**Chain and closer**
1. **Nonce on retry (§3.5 `send_close`).**
   - `.pending()` nonce plus a 20 s timeout: if attempt 1 is still in the mempool, attempt 2 takes
     nonce n+1. If attempt 1 is then dropped, attempt 2 is stuck behind a gap.
   - Use `.latest()` for retries. A still-pending attempt 1 then makes attempt 2 fail fast as
     "underpriced replacement", which is Transient; when attempt 1 lands, the pre-check sees
     `deposit == 0`.
   - The explicit `.nonce()` correctly bypasses alloy's `CachedNonceManager` (verified in
     `alloy-provider` 2.5 `NonceFiller::status`).
2. **Receipt timeouts.** Add `.with_timeout(Some(30 s))` to `open_channel` and `request_close`
   too. Without one, `get_receipt()` can hang forever.
3. **Unknown errors.** After N attempts (e.g. 5), alert regardless of error class. The
   precompile on `main` has T12 transfer-policy rejections (TIP-1028) that aren't in the 1.11
   error enum, so they decode as `Transient` and would retry silently forever.
4. **Separate payer keys.** Use two payer keys for control (A) and protected (B). It avoids
   nonce interleaving when both `requestClose` calls fire in the same second, and makes the
   explorer view unambiguous.
5. **Overflow checks.** Add `overflow-checks = true` to `[profile.release]`. For a money daemon,
   panicking beats wrapping, and it removes the "release builds wrap" caveat in §1.3.
6. **`eth_getLogs` range.** The cap is inclusive: `to − from ≤ 100_000` works, and 100,002
   blocks fails (verified). The guide's `< 100_000` is fine; just say it's conservative.

**Config and code quality**
7. **Ingest token.** Refuse to start if `TW_INGEST_TOKEN` is still `change-me`.
8. **`expect()` loophole.** Consider `clippy::expect_used = "warn"` plus
   `allow-expect-in-tests = true`. Otherwise beginners replace every `unwrap()` with `expect()`
   in daemon code.
9. **Shutdown.** Point to the Tokio topics page `tokio.rs/tokio/topics/shutdown`, not the
   tutorial. Verified: `tutorial/graceful-shutdown` returns 404.
10. **Features.** Book ch. 14 doesn't cover cargo features. Point to the Cargo Book
    "Features" chapter instead.

**CI and repo**
11. **gitleaks permissions.** Give the gitleaks job
    `permissions: { contents: read, pull-requests: read }`. On `pull_request` events, v3 calls
    `GET /repos/{o}/{r}/pulls/{n}/commits` (verified in `src/gitleaks.js`). Or set
    `GITLEAKS_ENABLE_COMMENTS: false`.
12. **What gitleaks catches.** State it honestly. Verified with gitleaks 8.30.1 built from
    source:
    - The default `generic-api-key` rule **catches** `TW_OPERATOR_KEY=0x<64 hex>` and a keyed
      JSON secret.
    - It **misses** a hex key in a keyword-less Rust literal (`let signer = "0x…"`).
    - It correctly ignores tx hashes.
    - GitHub push protection doesn't know raw Ethereum keys.

    So the real control is "keys only in `.env`, and tests use `PrivateKeySigner::random()`".
13. **Dependabot.** Turn it off until 10/13, or set it to monthly. Bump PRs during the freeze
    are noise.
14. **Release matrix.** Trim it to linux x86_64 + macOS arm64 for v0.1.0.

**Demo and data model**
15. **Grace end in the demo.** In M8, print `grace_end` from the `CloseRequested` event
    (chain-derived) next to the local countdown. That matches the "never hard-code 900" rule on
    screen.
16. **Unknown ids.** In `Phase` / `Input`, reject vouchers or `spent` for unknown ids with 404,
    not by silently creating a record.
17. **Metrics.** Keep `detect latency` as a secondary number and headline `close_block_ms −
    request_block_ms`, as §8 already says. Good. Also log `detected_via` in the same line as the
    tx hash so a single screenshot proves it.

---

## What I verified myself (2026-09-28)

| Claim | Method | Result |
|---|---|---|
| Testnet chain id 42431, node v1.15.0 | `eth_chainId`, `web3_clientVersion` on `rpc.moderato.tempo.xyz` | `0xa5bf`, `tempo/v1.15.0-464e519`. **TRUE** |
| `CLOSE_GRACE_PERIOD()` = 900 | `eth_call` to `0x4d50…0000`, data `0x956c8327`, testnet **and** mainnet | Both return `0x…0384` = 900; code at the address is `0xef`. **TRUE** |
| `tempo_fundAddress` works | Called on a fresh random address (testnet) | 4 tx hashes; PathUSD and AlphaUSD balances each `0xe8d4a51000` = 1,000,000.000000. **TRUE** |
| WS `eth_subscribe` | Python `websockets` to `wss://rpc.moderato.tempo.xyz`: `newHeads` + `logs{address: 0x4d50…}` | Both subscriptions accepted; 12 heads in 6 s (~0.5 s blocks); header carries `timestampMillis`. **TRUE** |
| `eth_getLogs` 100k cap | Spans of 99,999 / 100,000 / 100,001 | ≤100,000 OK; 100,001 → `-32602 query exceeds max block range 100000`. **TRUE** (inclusive) |
| mpp-rs server hard-codes `operator: None` | Shallow clone `tempoxyz/mpp-rs` @ `e834f5f` | `challenge_method_details()` L1456 `operator: None`; `SessionMethodConfig` (L329-336) has no operator field; verification uses the v1 `getChannel`. **TRUE** (line refs match) |
| Precompile semantics | Read `crates/precompiles/src/tip20_channel_reserve/mod.rs` (main) | `close`: no grace or close-request check; voucher checked only if `capture > settled`; `topUp` (even 0) cancels and emits `CloseRequestCancelled`; repeated `requestClose` is a no-op; `withdraw` needs `now ≥ requestedAt + 900`; **`settledToPayee` = cumulative capture** (see R5); operator check matches. **TRUE, with the R5 and R6 consequences** |
| Crate pins | crates.io API | tempo-alloy 1.11.0 (MSRV 1.95.0), alloy 2.5.0 (1.94.1), alloy-primitives/sol-types 1.7.3, redb 4.3.0, axum 0.8.9, tokio 1.53.1, reqwest 0.13.5, clap 4.6.7, thiserror 2.0.21, proptest 1.11.0, nextest 0.9.146, cargo-deny 0.20.2, bacon 3.26.0, mpp 0.13.0. **All TRUE and current** |
| Rust stable 1.98.1 | `static.rust-lang.org/dist/channel-rust-stable.toml` | `1.98.1 (48a229cea 2026-09-01)`. **TRUE** |
| GitHub Action versions | GitHub API, latest releases + ref existence | checkout v7.0.1, rust-cache v2.9.2, cargo-deny-action v2.1.1, install-action v2.87.21, audit-check v2.0.0 (`v2` is a *branch*, not a tag, so the pin to `@v2.0.0` is right), gitleaks-action v3.0.0, create-gh-release v1.11.0, upload-rust-binary v1.30.2, upload-artifact v7.0.1, `dtolnay/rust-toolchain@1.98.1` and `@1.95.0` branches exist. **All TRUE** |
| gitleaks-action needs no license for personal accounts | v3.0.0 README | **TRUE** (orgs need one) |
| Skeleton builds and tests | Copied to `/tmp/twreview`; `cargo build --all-targets --locked`; `cargo test`; `cargo +1.95.0 check --all-targets` | Build OK; 3/3 tests pass (incl. `start_paused` and the axum `oneshot`); MSRV check OK |
| Skeleton is clippy-clean | The §5.1 command `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | **FAILS**: `unwrap_used` on `examples/revert.rs:7` (R8) |
| clap hides env secrets | Built a clap 4.6 test binary | **FALSE**: `--help` prints `[env: TW_OPERATOR_KEY=<value>]` unless `hide_env_values = true` (R7) |
| gitleaks catches keys | gitleaks v8.30.1 built from source, scanning 4 fixtures | Catches `KEY=0x…` and keyed JSON; misses a keyword-less Rust literal; ignores tx hashes (rec. 12) |
| Rustlings 00-24 incl. `24_async`; Book chapter map; Tokio URLs | Cloned rustlings (6.5.0, locally installed too); Book `SUMMARY.md`; curl | All folder names correct; ch17.1/17.2/17.4/18.2/19/14.3 map correctly; Tokio `hello-tokio`, `async`, `select`, `streams`, `channels`, `shared-state`, `spawning` = 200; `graceful-shutdown` = 404 (rec. 9) |
| Contest dates / §9 disclosure rule | `research/hackathon-official-rules.txt` | Contest period 2026-09-14 06:00 PT → 10-12 23:59 PT; §9 requires informing the Administrator of third-party code (R11) |

Not re-verified: `cargo deny check` (not installed locally; the planner reports it passes), the
arm64 release runner, and the planner's own testnet transactions. I didn't send any transactions.

---

## Top 3 risks for this builder, and how the plan should mitigate them

1. **Async Rust eats the schedule before anything automatic works.**
   - *Where:* M5-M7 (actor + axum + WS + closer), for someone who learned `Result` two days
     earlier.
   - *Today's mitigation* is the MVD decision at day-8 EOD, when there's no time left to use it.
   - *Fix:* R1 + R2 + R3.
     - A poll-only walking skeleton gives the first automatic save on **day 4**, and it has no
       HTTP, WS, trait objects or DB.
     - Every later day adds one layer on top of something that already works.
     - Cut M2 and FakeChain.
     - Decision points on days 4 and 7.
   - This is also what makes it *enjoyable*: a real on-chain win on day 1 (probe) and day 4
     (auto-save), instead of a week of scaffolding.
2. **Edge-case state bugs that only show on a real chain, on camera.**
   - A crash mid-close strands the channel (R4).
   - A retry misreports a save as a loss (R5).
   - A `topUp` rescue gets closed anyway, and the poll path keeps a stale deadline (R6).
   - Each of these breaks either the restart demo or the numbers table.
   - *Fix:* the R4-R6 rule changes, each with a named test in the given M3 spec file, plus a
     dry run of every failure path on **day 8**, not day 10.
3. **Credibility loss from things that aren't true or aren't safe on screen.**
   - The operator key printed by `--help` during a recording (R7).
   - "Threshold backstop" claimed but not built (R10).
   - "Core logic is my own work" while pasting AI reference solutions (R11).
   - Judges score "how adept is the team", and one of these found in an interview costs more
     than any missing feature.
   - *Fix:* R7, R10 and R11, plus the §8 honesty checklist extended with "no secrets on screen"
     and "AI assistance disclosed per file".
