# A tour of the workspace skeleton, file by file

What every file in `reference/workspace-skeleton/` is for, what's inside, and when you'll use it.
Keep this open next to the build guide.

**Spoiler-safe.** The exercise files (`machine.rs`, `verify.rs`, the `classify_revert` body,
`Ledger::all`) are described by their *contract* only (inputs, outputs, rules). How the reference
solves them isn't described here.

---

## 1. The big picture: four crates, one direction of dependency

```text
                 ┌────────────────────────────┐
                 │  tw-core   (pure logic)     │  types + the money state machine
                 │  no async, no I/O, no clock │  ← your day-3 exercises live here
                 └──────────────▲─────────────┘
                                │ uses
                 ┌──────────────┴─────────────┐
                 │  tw-chain  (chain I/O)      │  talk to Tempo: read, sign, send, decode
                 └───────▲──────────────▲─────┘
                         │ uses         │ uses
      ┌──────────────────┴───┐   ┌──────┴────────────────────┐
      │ tw-daemon (binary:   │   │ tw-demo (binary: tw-demo) │
      │  tempo-watchtower)   │   │ payer/payee tools, keygen, │
      │ the watchtower itself│   │ scenario harness           │
      └──────────────────────┘   └────────────────────────────┘
```

**Why split it this way:**
- `tw-core` knows nothing about networks or time: `now_ms` is always passed in. So every money
  decision is testable in microseconds, with no testnet and no waiting 15 minutes.
- `tw-chain` is the only place that knows about RPC, ABIs and transactions.
- The two binaries glue them together.

**The runtime flow (from day 4 onward), one close request end to end:**
```text
poller ──states()──▶ OnChainState ──state_to_inputs()──▶ Input::CloseRequested
   ──▶ step(record, input, now) ──▶ Action::SubmitClose(ClosePlan)         [tw-core]
   ──▶ precheck(fresh state)? ──None──▶ broadcast_close() ──▶ Input::CloseSent{tx}  [tw-chain]
   ──▶ wait_receipt() ──▶ Input::Closed{tx,…} ──▶ step() ──▶ Phase::Done(Outcome::Saved)
   (every step's new record is written to the ledger BEFORE its actions run)   [tw-daemon]
```

**The day-1 state vs the full skeleton.** `make-day1-copy.sh` removes or stubs what you'll build
later. What you copy on day 1 (§1.4 of the build guide):
- **Stubbed:** `machine.rs`/`verify.rs` bodies become `todo!()`, and `classify_revert` becomes
  the safe `Transient` default.
- **Removed:** the spec test files (they arrive on days 3 and 8) and `ledger.rs` (day 5).

---

## 2. Root files

| File | Day | What it is / why it matters |
|---|---|---|
| `Cargo.toml` | 1 | **The workspace manifest.** Details below. |
| `Cargo.lock` | 1 | Exact resolved versions of every dependency. **Commit it** (binaries should), and CI uses `--locked` so builds can't silently drift. |
| `rust-toolchain.toml` | 1 | Pins the compiler to **1.98.1** with `rustfmt` + `clippy`. `rustup` installs it automatically the first time you run `cargo` in the repo. |
| `clippy.toml` | 1 | Allows `unwrap`/`expect` **inside tests only**. In real code they're warnings, and CI turns warnings into errors. |
| `rustfmt.toml` | 1 | Line width 100. `cargo fmt` does the rest. |
| `.env.example` | 1 | **The only env file you commit.** It lists `TW_RPC_URL`, `TW_WS_URL`, `TW_CHAIN_ID` (42431 = testnet), three empty keys (`TW_OPERATOR_KEY`, `TW_PAYER_A_KEY`, `TW_PAYER_B_KEY`) and `TW_INGEST_TOKEN`. You `cp` it to `.env`, then `tw-demo keygen` fills the blanks. |
| `deny.toml` | 11 | Policy for `cargo deny`: allowed licenses, fail on known vulnerabilities (with one documented ignore for the unmaintained `paste` crate that alloy pulls in), ban wildcard versions, only crates.io as a source. |

**`Cargo.toml` in detail:**
- `members = ["crates/*"]`: every folder in `crates/` is a workspace member.
- `[workspace.package]`: shared metadata (edition 2024, `rust-version = "1.95"` as the MSRV,
  license `MIT OR Apache-2.0`). Change the `repository` URL to your GitHub.
- `[workspace.dependencies]`: **every version is pinned once here.** Each crate then says
  `alloy.workspace = true`, so all four crates always agree. Key ones:
  - `alloy` and `tempo-alloy`: Ethereum and Tempo types, providers, signers, and the precompile
    ABI bindings.
  - `tokio`: async runtime.
  - `thiserror` for library errors, `anyhow` for binary errors.
  - `tracing`: logs. `clap`: CLI. `dotenvy`: `.env`. `redb`: embedded database.
    `axum`: HTTP.
  - `proptest`: property tests.
- `[workspace.lints]`: `unsafe_code = "forbid"`, and `unwrap_used`/`expect_used` warn. An
  `unwrap()` in the tower could crash it mid-deadline; handle the error instead.
- `[profile.release]`: `overflow-checks = true`. **Money arithmetic must never silently wrap**,
  even in release builds.

---

## 3. `crates/tw-core`: the brain's rules (pure)

### `src/lib.rs` (GIVEN): read this file first, carefully
It's the vocabulary for everything else.

**Constants**
- `Amount = u128` is in token base units. TIP-20 stablecoins have 6 decimals, so `1_000_000` =
  $1.00.
- `MAX_U96`: on-chain amounts are `uint96`.
- `RESERVE = 0x4d505…0000`: the v2 channel precompile address.

**Channel identity**
- `Descriptor`: the 7 fields that *define* a channel: payer, payee, **operator**, token, salt,
  authorized_signer, expiring_nonce_hash. Every precompile call takes it.
- `channel_id(d, chain_id)` (GIVEN): recomputes the channel's id as the keccak of the
  ABI-encoded descriptor + precompile + chain id. The probe checks it matches the chain.

**Money evidence**
- `Voucher { cumulative, signature }` is the payer's signed running total.
- `voucher_digest(chain_id, id, amount)` (GIVEN) is the exact EIP-712 hash the payer signs.
  - **Trap, noted in the code:** the struct *name* `Voucher` is part of the hash. Rename it, and
    the chain rejects every signature.

**The tower's memory of one channel: `ChannelRecord`**
- `deposit`, `settled` (on-chain), `spent` (from the payee), `best_voucher`.
- `our_txs`: every close we ever sent. It's how we know a close was ours (R5).
- `cancellations`.
- `phase`: the state machine's current state.

**The state machine's states: `Phase`**
- `Watching`: normal.
- `Closing(Closing)`: a close request is live. `Closing` holds:
  - `grace_end`, chain-derived, and never hard-coded;
  - `requested_at_ms` and `detected_at_ms`, for latency metrics;
  - `attempts` and `next_attempt_ms`, for the retry backoff;
  - `in_flight`, so we don't double-send;
  - `deadline_alerted`.
- `Done(Outcome)`: finished forever.

**How it ended: `Outcome`**
- `Saved { tx, claimed, … }`: our close landed.
- `NothingToClaim`: closed by someone else, but nothing was owed.
- `ClosedByOther`: closed by someone else *while money was owed*.

**Everything that can happen to a channel: `Input`**
- New evidence: `Voucher`, `Spent`, `Settled`, `Deposit`.
- Close-request lifecycle: `CloseRequested{grace_end, requested_at_ms}`,
  `CloseRequestCancelled`.
- Channel ending: `GoneOnChain` (a poll saw `deposit == 0`), `Closed{…}`.
- Closer feedback: `CloseSent`, `CloseFailed{retryable, reason}`, `CloseSkipped{cancelled}`.
- Housekeeping: `Restarted` (loaded after a crash), `Tick`.

**Output of the brain**
- `ClosePlan` holds the numbers to put in the `close()` call: `cumulative`, `capture`,
  `signature`, `claimable` (what this close earns the payee) and `clamped`.
- `Action` is what the brain wants done: `SubmitClose(plan)`, `CheckReceipts(txs)` or
  `Alert(msg)`.
- `Policy` holds tunable knobs, with defaults:
  - backoff of 500 ms → 5 s;
  - an alert 120 s before `grace_end`;
  - an alert after 5 failed attempts;
  - don't close when there's nothing to claim.

### `src/machine.rs` (EXERCISE, day 3; a `todo!()` stub until then)
Two functions, and the heart of the product:
- **`plan_close(&ChannelRecord) -> Result<ClosePlan, PlanError>`** answers "if we closed now,
  what exact numbers go on-chain?" It must obey the capture rule from spec §13.2: never capture
  more than was consumed. The full rules are in the build guide §4 M2, under "The spec in words".
- **`step(&mut ChannelRecord, Input, now_ms, &Policy) -> Vec<Action>`** is the whole state
  machine: one input in, the record updated, zero or more actions out. It's pure: no I/O and no
  clock. That's why 22 tests can simulate a crash, a retry storm or a payer cancel in
  milliseconds. The transition table is in the guide (M2).

### `src/verify.rs` (EXERCISE, day 3; a stub until then)
**`verify_voucher(d, id, v, chain_id) -> bool`**: is this signature really from the payer (or
its authorized signer) for this channel and amount? The ingest API (M5) uses it to reject forged
vouchers.

### `tests/spec.rs` (22 tests) and `tests/props.rs` (4 properties): GIVEN on day 3
**Your specification.** Each test name is a sentence. For example:
- `close_request_submits_one_close_with_capture_equal_to_spent`;
- `restart_with_a_close_in_flight_resubmits`;
- `cancel_returns_to_watching_and_a_new_request_gets_a_fresh_deadline`.

They go red when you copy them in, and you're done when they're green.

`props.rs` uses proptest to throw random input sequences at `step` and check invariants that
must *always* hold. An example: `amounts_only_ever_increase`.

---

## 4. `crates/tw-chain`: talking to Tempo

### `src/lib.rs` (GIVEN, verified on testnet): read it in sections

**Setup**
- `Reserve` / `RESERVE` / `ITIP20`: the precompile's ABI bindings, re-exported from
  `tempo-alloy`. You never hand-write ABI.
- `PATH_USD` (`0x20c0…`): the default fee and deposit token.
- `ChainError`: one error enum wrapping every alloy error. `#[from]` lets `?` convert
  automatically.

**Connections**
- `http_provider(url)`: read-only.
- `signing_provider(url, key)`: can send transactions. Transactions go out as normal EIP-1559;
  the fee falls back to PathUSD (verified).

**Reads**
- `grace_period()`: `CLOSE_GRACE_PERIOD()`, 900 today.
- `path_usd_balance()`: for the fee-balance alert in M7.
- `states(&[id])`: batch-reads `{settled, deposit, close_requested_at}` in chunks of 200.
  **`deposit == 0` means the channel is closed.**

**Faucet:** `fund(who)` calls `tempo_fundAddress`, then waits until the balance appears.

**Conversions:** `from_sol` / `to_sol` translate between `tw_core::Descriptor` and the ABI's
struct. These are plain functions rather than `impl From`, because Rust's **orphan rule** forbids
implementing a trait between two types that are both foreign to this crate.

**Payer-side actions** (used by `tw-demo` to *be* the agent):
- `open_channel(payer, payee, operator, deposit)` returns the id, descriptor and tx hash, taken
  from the `ChannelOpened` event.
- `payer_call(RequestClose | Withdraw | TopUp(a))`.
- `sign_voucher()`.

**`precheck(&OnChainState) -> Option<Input>`**: the closer's last look before sending.
- `close()` on-chain doesn't check whether a close was requested. So if the payer cancelled
  (`topUp`) while our close was queued, we'd wrongly close a live channel.
- Return values: `None` means send; `Some(CloseSkipped{…})` means don't.

**`broadcast_close(op_provider, operator, descriptor, plan)`**: the operator's `close()`.
- It reads a fresh nonce on every attempt.
- It returns the tx hash **without waiting**, so the brain can record `CloseSent` first. That
  ordering matters for restart safety.
- Errors go through `classify`.

**Receipts**
- `receipt_status(id, tx)` returns `Pending`, `Reverted`, `Closed(ClosedEvent)` or
  `MinedWithoutClose`.
- `wait_receipt()` polls it every 500 ms.
- **`ClosedEvent.settled_to_payee` is the lifetime total, not the delta**; `claimed` is computed
  in core.

**WebSocket helpers (M6)**
- `payee_filter(payee)` subscribes only to *your* channels' events.
- `decode(log)` turns raw logs into typed events.

### `src/classify.rs` (GIVEN wrapper + EXERCISE body on day 8)
- `CloseError` is the error taxonomy for a failed close: `Transient`, `AlreadyClosed`,
  `StaleState`, `BadVoucher`, `WrongKey`.
- `classify(&alloy::contract::Error)` (GIVEN) decodes the revert and calls `classify_revert`.
- **`classify_revert`** returns `Transient` for everything until M7, which is safe: it just
  retries. On day 8 you replace it with the real mapping, driven by `tests/classify.rs`.

### `examples/` (GIVEN; runnable programs, not part of the daemon)
- **`probe.rs`, day 1:** the end-to-end tracer bullet. Read it top to bottom; it tells the
  protocol's story in about 100 lines.
  1. Random throwaway keys, funded from the faucet.
  2. Open a channel with the operator set.
  3. Check `channel_id` and the voucher digest match the chain.
  4. `requestClose` → `topUp` (cancels) → `precheck` says don't send.
  5. `requestClose` again → `precheck` says send.
  6. The operator's `close(capture = 0.25)` → receipt.
  7. Send again → an error, not a panic.
- **`ws_heads.rs`, M6:** a minimal WebSocket subscription (payee logs + new block headers).
- **`revert.rs`, M7:** deliberately triggers a precompile revert, so you can see how errors
  decode.

### `tests/precheck.rs` (day 4) and `tests/classify.rs` (day 8): GIVEN specs.

---

## 5. `crates/tw-daemon`: the watchtower binary (`tempo-watchtower`)

| File | Day | What it is |
|---|---|---|
| `src/lib.rs` | 1 | Just `pub mod config; pub mod supervise;` (plus `ledger` from day 5). All logic lives in the library so tests can reach it, and so `pub` items don't trip "dead code" errors under `-D warnings`. |
| `src/main.rs` | 1 (grows daily) | **The M0 shell.** Details below. |
| `src/config.rs` | 1 | **How settings and secrets load.** Details below. |
| `src/supervise.rs` | 1 (used from day 5) | **A crash policy for the task group.** Details below. |
| `src/ledger.rs` | 5 | **Durable storage** in `redb`. Details below. |

**`src/main.rs`, the M0 shell, in order:**
1. Load `.env`.
2. Start `tracing` logs.
3. Build `Config`.
4. Connect and log the grace period (proves chain access).
5. Serve `GET /healthz` with axum.
6. Wire up graceful shutdown: a `CancellationToken`, and a Ctrl-C handler that cancels it.
7. Hand everything to `supervise`.

In M3–M6 you add the poller, brain, closer and WS tasks here. Its tests show two useful
patterns:
- testing an axum route without a network (`Router::oneshot`);
- `#[tokio::test(start_paused = true)]`, which makes a 15-minute `sleep` finish instantly in
  tests.

**`src/config.rs`:**
- `Args` (clap): RPC/WS URLs, chain id, listen address, ledger path, poll interval,
  `--no-ws`. **Keys are deliberately NOT clap arguments.** Clap would print their values in
  `--help`, and flags leak into shell history.
- `Secret` holds the operator key. Its `Debug` prints only the address.
- `IngestToken` refuses weak values, and its `Debug` prints `<redacted>`.
- Two tests guard this forever: `debug_never_prints_the_key_or_token` and
  `help_has_no_key_arguments`.

**`src/supervise.rs`:**
- All core tasks run in one `JoinSet`.
- If any task returns or panics while you're *not* shutting down, it logs the error, stops the
  others, and `main` exits non-zero.
- Rationale: a crash plus restart is safe (the ledger remembers), but a "zombie" tower whose
  closer silently died would miss deadlines while looking healthy.
- 3 tests: a panic → error; an early return → error; Ctrl-C → a clean exit.

**`src/ledger.rs`:**
- A single table in `redb`: `channel_id → ChannelRecord` as JSON.
- `open` / `put` / `get` are GIVEN. **`put` is fsync-durable when it returns**, so you always
  persist *before* acting on a decision.
- `all()`, which loads every record at startup, is your **EXERCISE**.
- Given tests: `survives_reopen` and `crash_with_close_in_flight_resubmits_after_restart`.

**Modules you'll create** (not in the skeleton; described in the guide):
- `state_to_inputs` + the poll loop (M3);
- `brain` / `poller` / `closer` tasks (M4);
- `api` (M5);
- `ws` and `to_input` (M6).

---

## 6. `crates/tw-demo`: your agent and payee simulator (binary `tw-demo`)

### `src/main.rs` (GIVEN parts: `keygen`, `grace`)
- `Cli` / `Cmd` is a clap subcommand enum. On day 2 you add `fund`, `open`, `pay`,
  `request-close`, `top-up`, `withdraw`, `operator-close` and `status`. Later come `payee serve`
  (M5) and `scenario` (M8).
- **`keygen`** reads `.env` and fills only **empty** keys and the token, in place. In-place
  matters: `dotenvy` keeps the first occurrence of a variable, so appending a second line
  wouldn't work.
  - It sets the file to `0600` (owner-only) and prints **addresses only**.
  - Running it again changes nothing.
  - The pure core is `fill_env()`, so it's unit-tested without touching your real `.env`.
- **`grace`** prints `CLOSE_GRACE_PERIOD()`. It's a one-line check that your RPC works.
- 3 tests:
  - it fills the real `.env.example` and parses it exactly as the daemon will;
  - it's idempotent;
  - it works with no file at all.

---

## 7. Helper scripts in `reference/`
- **`make-day1-copy.sh <dir>`** recreates your exact day-1 starting point from the full
  skeleton. It deletes the later spec files and `ledger.rs`, stubs the exercises, and strips the
  ledger lines from `main.rs`. It was verified: fmt/clippy/test all pass, and the probe ran on
  testnet.
- `testnet-probe/`, `logs_test.py`, `ws_test.py`: research leftovers. Not part of your path.

---

## 8. Suggested reading order (about 45 minutes, before day 1's coding)
1. `Cargo.toml`, then `.env.example`: what's in the box, and what you configure.
2. `tw-core/src/lib.rs`: the vocabulary. Read every doc comment.
3. `tw-chain/examples/probe.rs`: the whole protocol as one story. Then open `tw-chain/src/lib.rs`
   to see how each call it uses works.
4. `tw-daemon/src/config.rs` + `supervise.rs`: the safety rails.
5. `tw-daemon/src/main.rs`: where tomorrow's code will plug in.
6. `tw-demo/src/main.rs`: where day 2's code goes.

**Rust features you'll meet here that Rustlings may not have covered:**
- workspace dependency inheritance (`x.workspace = true`);
- `thiserror` + `#[from]` making `?` convert errors automatically;
- `let … else { return … }`;
- `impl Future<Output = …> + use<>` (Rust 2024 precise capturing, in `supervise.rs` tests);
- `tokio::select!`;
- `JoinSet`;
- `CancellationToken`;
- `#[tokio::test(start_paused = true)]`.
