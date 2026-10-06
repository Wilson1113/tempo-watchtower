# reference/: verified code behind the build guide

Everything here was written by an AI assistant (Claude) on 2026-09-28 while researching the build
guide. It is **third-party code** from the builder's point of view (hackathon rules section 9):
**never copy this directory wholesale into the tempo-watchtower repo.** Copy only the pieces the
guide marks GIVEN, one per commit, with the commit-message suffix
`(plumbing from AI-drafted build guide)`, and list them in the README disclosure.

| Path | What | Status |
|---|---|---|
| `workspace-skeleton/` | 4-crate workspace with the exact pinned versions | `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-features --locked`, `cargo deny check`, `cargo +1.95.0 check` all pass (2026-09-28) |
| `workspace-skeleton/crates/tw-chain/src/lib.rs` | GIVEN plumbing: providers, faucet, open, payer calls, voucher signing, batch state reads, `broadcast_close`, receipt lookup, payee log filter | ran on testnet via `examples/probe.rs` |
| `workspace-skeleton/crates/tw-chain/examples/` | GIVEN: `probe.rs` (M0 tracer bullet; uses GIVEN code only, no exercise function), `ws_heads.rs` (M6), `revert.rs` (M7) | ran on testnet; `probe.rs` ran on the **day-1 stub state** on 2026-09-29 |
| `workspace-skeleton/crates/tw-core/src/lib.rs` | GIVEN: types, `Policy::default`, `channel_id`, `voucher_digest` | tested against testnet fixtures |
| `workspace-skeleton/crates/tw-core/tests/spec.rs`, `tests/props.rs` | GIVEN spec for the M2 exercises (22 tests + 4 properties) | pass against the spoiler below; each review bug (R4, R5, R6) re-introduced makes a test fail |
| `workspace-skeleton/crates/tw-chain/tests/classify.rs` | GIVEN spec for the M7 exercise | passes against the spoiler below |
| **SPOILER** `workspace-skeleton/crates/tw-core/src/machine.rs`, `src/verify.rs` | Reference solutions for `plan_close`, `step`, `verify_voucher` | Do not open until your own code passes the spec. If you do, disclose it. |
| **SPOILER** `workspace-skeleton/crates/tw-chain/src/classify.rs` (`classify_revert` body) | Reference solution for M7 | same |
| **SPOILER** `workspace-skeleton/crates/tw-daemon/src/ledger.rs` (`all()` body) | Reference solution for M4 | same |
| `workspace-skeleton/crates/tw-daemon/src/config.rs` | GIVEN: redacted `Secret` / `IngestToken`, clap args without keys | `Debug` and `--help` leak tests pass |
| `workspace-skeleton/crates/tw-daemon/src/supervise.rs` | GIVEN: stop the process (non-zero exit, logged) if any core task exits or panics (N1) | 3 tests pass; `main.rs` uses it; Ctrl-C exits 0 |
| `workspace-skeleton/crates/tw-chain/src/lib.rs` `precheck` + `tests/precheck.rs` | GIVEN: the closer's pre-send check (R6) as a pure function | test passes; used by `probe.rs` live |
| `workspace-skeleton/.env.example` + `crates/tw-demo/src/main.rs` (`keygen`, `grace`) | GIVEN: `cp .env.example .env && tw-demo keygen` fills empty keys + ingest token in place, `.env` mode 0600, prints addresses only (N3) | 3 keygen tests (parse the exact `.env.example` with dotenvy); ran live |
| `make-day1-copy.sh` | Recreates the builder's exact day-1 state (stubs; `classify_revert` = safe `Transient` default; no spec files; no ledger) | output passes fmt / clippy `-D warnings` / test; no reachable `todo!()`; its probe and daemon ran live on 2026-09-29 |
| `testnet-probe/` | The first, throwaway probe from research (historical; uses `expect`, not CI-clean, not part of the builder's path) | ran on testnet |
| `logs_test.py`, `ws_test.py` | Python RPC/WS probes used during research | ran |
