# tempo-watchtower

A watchtower for **Tempo v2 payment channels**, written in Rust. **Testnet only.**

> **Status: paused (October 2026).** I stopped working on this. The project scaffold, chain plumbing
> and protocol research are done and tested against the Moderato testnet. The watchtower logic itself
> is not written yet. Contributions and forks are welcome; this README is the handover.
>
> It started as an entry for the Colosseum Crypto World's Fair (Tempo track, deadline 2026-10-12).
> All the background work is in [`research/`](research/README.md): why this idea was picked, the
> on-chain market analysis, the build guide and a study guide.

## The problem

Tempo has a built-in payment-channel contract, the **TIP-20 Channel Reserve** precompile:

1. A **payer** opens a channel and locks a stablecoin deposit (PathUSD).
2. The payer pays a **payee** off-chain by signing EIP-712 **vouchers** ("you may claim up to X in total").
3. The channel closes in one of two ways:
   - The payee, or an **operator** acting for it, calls `close()` with the best voucher and is paid.
   - The payer calls `requestClose()`, waits out a **grace period**, then calls `withdraw()` and gets
     back everything that isn't settled.

If a payee is offline during that grace period, a dishonest payer can withdraw money it already
spent. A **watchtower** is the operator. It stores the payee's best voucher, watches the chain, and
calls `close()` within the grace period.

## Where it stands

| Area | State |
|---|---|
| Workspace, toolchain, lints, `make` shortcuts | Done |
| `tw-chain`: every chain call the tower needs | Done; marked verified on testnet 2026-09-28 (`make probe`) |
| `tw-core`: data types, EIP-712 voucher digest, channel id | Done, both checked against the chain in `probe` |
| `tw-core`: `plan_close`, `step` (state machine), `verify_voucher` | **Not started** (`todo!()`) |
| `tw-chain`: `classify_revert` (close error triage) | Safe stub: treats every error as retryable |
| `tw-daemon`: config, secret redaction, `/healthz`, task supervisor | Done |
| `tw-daemon`: brain, poller, closer, WebSocket, ingest API, ledger | **Not started** |
| `tw-demo`: `keygen`, `grace` | Done |
| `tw-demo`: payer/payee/scenario commands | **Not started** |

On 2026-10-01, `make ci` (fmt check, clippy with `-D warnings`, tests) passed with 10 tests, and
`make grace` returned `900` from testnet.

**AI disclosure:** the code in `crates/` and the root config files were copied from the
AI-drafted build guide (`research/build-guide/`). The guide marks these pieces GIVEN, and each was
added in its own commit ending in `(plumbing from AI-drafted build guide)`. The tests and the `probe`
run are what back it up. Read it critically.

## Research findings

These are the protocol facts the code depends on. Each one is in code and most are checked against the
chain by `make probe`.

**Network**

- Moderato testnet: chain id `42431`, RPC `https://rpc.moderato.tempo.xyz`, WebSocket
  `wss://rpc.moderato.tempo.xyz`.
- Testnet faucet: the JSON-RPC method `tempo_fundAddress(address)` mints PathUSD, AlphaUSD, BetaUSD
  and ThetaUSD (`tw_chain::fund`).
- PathUSD is the TIP-20 token at `0x20c0000000000000000000000000000000000000`. TIP-20 tokens have 6
  decimals (`1_000_000` = $1.00), and the Reserve stores amounts as `uint96`.
- A plain secp256k1 key sends EIP-1559 transactions. Fees are paid in the sender's preferred USD
  token, falling back to PathUSD.
- Rust bindings come from the `tempo-alloy` crate (`ITIP20`, `ITIP20ChannelReserve`, the
  `TempoNetwork` type for alloy providers).

**Channel Reserve**

- `CLOSE_GRACE_PERIOD()` returns **900 seconds** (15 minutes).
- Channel id = `keccak256(abi.encode(payer, payee, operator, token, salt, authorizedSigner,
  expiringNonceHash, RESERVE, chainId))`. It can be computed locally (`tw_core::channel_id`).
- Voucher = EIP-712 struct `Voucher { bytes32 channelId; uint96 cumulativeAmount; }`, domain
  name `"TIP20 Channel Reserve"`, version `"1"`, verifying contract = the Reserve. The struct name
  must be exactly `Voucher`. The local digest equals the precompile's `getVoucherDigest`.
- `getChannelStatesBatch(ids)` returns `(settled, deposit, closeRequestedAt)` for many channels in
  one call. `deposit == 0` means closed or never existed.
- `topUp()` **cancels** a pending close request.
- Every channel event has the **payee as indexed topic 3**, so a log filter can subscribe to one
  payee's channels only (`tw_chain::payee_filter`).

**Design consequences**

- **`close()` does not check for a close request.** If the payer cancels with `topUp` while a close
  is queued, the tower would still close a healthy channel. So the closer must take a fresh state
  read right before sending (`tw_chain::precheck`): skip if `deposit == 0` or `closeRequestedAt == 0`.
- **Nonces:** read the operator's `latest` nonce on every attempt. A retry while the first
  transaction is still pending fails fast as a replacement. After the first one lands, the precheck
  sees the channel closed.
- **Broadcast before waiting:** `broadcast_close` returns the tx hash as soon as it's sent. The
  intent is to persist "close sent" before waiting for the receipt, so a restart can look the
  receipt up instead of sending again.
- **Close errors fall into five classes** (`tw_chain::CloseError`): `AlreadyClosed`, `StaleState`
  (re-read and re-plan), `BadVoucher` and `WrongKey` (alert, stop), `Transient` (retry).
- **Secrets never go through `clap`.** A flag is visible in shell history and `ps`, and
  `env = ...` prints the value in `--help`. Keys are read with `std::env::var` into types whose
  `Debug` prints only the address. Tests check this.

## Architecture

```
crates/
  tw-core    pure logic: types, state machine, close planning, voucher checks (no I/O, no clock)
  tw-chain   everything that talks to Tempo, wrapped in tw-core types
  tw-daemon  the tower binary (`tempo-watchtower`)
  tw-demo    CLI for testnet demos: keys, payers, payees, scenarios
```

The planned daemon design, from the code comments:

- **Brain:** owns every `ChannelRecord` and feeds each `Input` (vouchers, `spent` updates, chain
  events, close results, ticks) through `tw_core::step`, which returns `Action`s:
  `SubmitClose`, `CheckReceipts`, `Alert`. Time is passed in as `now_ms`, so the whole state
  machine can be tested without a network or a clock.
- **Poller and WebSocket:** feed chain state and events to the brain. `--no-ws` runs on polling
  only.
- **Closer:** runs `precheck`, then `broadcast_close`, then `wait_receipt`, and reports back.
- **Ingest API:** the payee server posts vouchers and `spent` updates over HTTP, authenticated by
  `TW_INGEST_TOKEN`. A sketch of `POST /v1/channels/{id}/spent` is in `tw-daemon/src/main.rs` tests.
- **Ledger:** `redb` file (`TW_LEDGER`). After a restart, each record gets `Input::Restarted` and a
  fresh poll.
- **Supervisor:** if any core task exits or panics, everything stops with a non-zero exit. A
  restart is safe; a tower whose closer died silently is not.
- **Policy defaults:** alert 120 s before the grace deadline, retry backoff 500 ms to 5 s, alert
  after 5 failed attempts, don't close when there is nothing to claim.

## Milestones

The code comments use the build guide's milestone labels:

| Milestone | What | State |
|---|---|---|
| M0 | Scaffold, chain plumbing, testnet probe, daemon shell | Done |
| M1 | `tw-demo` CLI beyond `keygen` / `grace` | Open |
| M2 | `plan_close`, `step`, `verify_voucher` in `tw-core` | Open |
| M4 | Daemon tasks: brain, poller, closer, under the supervisor | Open |
| M7 | `classify_revert` | Open |

M3, M5 and M6 aren't referenced in the code; the build guide
(`research/build-guide/tempo-watchtower-build-guide.md`) has the full day-by-day plan.

The spec tests for the open exercises aren't in `crates/` yet. They're in the reference skeleton
at `research/build-guide/reference/workspace-skeleton/`:

- `crates/tw-core/tests/spec.rs` and `tests/props.rs`: 22 tests and 4 properties for M2
- `crates/tw-chain/tests/classify.rs` and `tests/precheck.rs`: M7 and the pre-send check

Copy them into `crates/` to get a failing spec to work against.

**Spoiler warning:** the same skeleton also has finished versions of the exercises
(`tw-core/src/machine.rs`, `verify.rs`, `tw-chain/src/classify.rs`) plus code for later milestones
(`tw-daemon/src/ledger.rs`, `examples/ws_heads.rs`, `examples/revert.rs`). They were written by an
AI assistant. Its README asks you not to copy that directory wholesale; take one GIVEN piece at a
time and disclose it.

## Getting started

Needs Rust (the toolchain in `rust-toolchain.toml` installs itself) and `make`.

```sh
cp .env.example .env
cargo run -p tw-demo -- keygen   # fills empty keys/token in .env, prints addresses only

make            # fmt check + clippy + tests
make grace      # read CLOSE_GRACE_PERIOD from testnet (900)
make probe      # full channel lifecycle on testnet with throwaway keys
make daemon     # current shell: config, one chain call, /healthz on 127.0.0.1:8787
```

`make fund` and `make balances` call the faucet and read PathUSD balances for every address in
`/tmp/tw-addresses.txt`. `keygen` prints addresses only when it creates keys, so that file may need
filling by hand.

**Good first reading:**

1. `research/study-guide/tempo-watchtower-study-guide.md`: Tempo, payment channels and the problem,
   from zero.
2. `research/build-guide/skeleton-tour.md`: what each file in the workspace is for.
3. `crates/tw-chain/examples/probe.rs` from top to bottom, then each function it calls in
   `crates/tw-chain/src/lib.rs`, then the types in `crates/tw-core/src/lib.rs`.

**Good first contribution:** M2. `plan_close`, `step` and `verify_voucher` are pure functions with
no I/O, and the types and spec tests already exist (see Milestones).

## Research (`research/`)

| Folder | What's in it |
|---|---|
| `README.md` | Index, decision summary, judge score history, honest risks |
| `00-brief.md`, `hackathon-official-rules.txt` | Hackathon rules and judging criteria |
| `phase1-scouting/`, `01`–`04-*.md` | 26 candidate ideas, shortlists, and why this one won |
| `phase2-decks/`, `phase3-verification/` | Decks for each candidate and adversarial verdicts on them |
| `phase4-pitch/` | Pitch deck, judge rubric and rounds, competitor re-check, evidence |
| `phase4-pitch/onchain/` | Python scripts and outputs measuring real channel activity on Tempo |
| `build-guide/` | Day-by-day build guide, tech-lead reviews, reference skeleton |
| `study-guide/` | Learning guide for the whole domain |
| `social/` | Build-in-public posting guide |

Market findings in short, from `research/README.md`: real money in Tempo payment channels is still
small (about $4.2k lifetime deposits, at most $36 of provable unanswered exposure). 6 of the 16
payees that received close requests had already hand-rolled a responder. The case for this project
is a shared, open, reliable tool (a public good) rather than a large near-term business.

## Known gaps

- `Cargo.toml` says `MIT OR Apache-2.0`, but only `LICENSE-APACHE` is in the repo.
- The `repository` field in `Cargo.toml` is a placeholder (`YOUR_GH_USER`).
- `.github/` is empty: there is no CI workflow yet. `make ci` is the local equivalent.
- `IngestToken::matches` isn't constant-time. That's fine on localhost; use `subtle` before
  exposing the API.
- `keygen` writes `.env` before setting `0600` permissions, so a newly created file is
  briefly readable with default permissions.

## License

Intended as MIT OR Apache-2.0 (see Known gaps). `LICENSE-APACHE` is included.
