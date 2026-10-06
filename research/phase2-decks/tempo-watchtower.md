# tempo-watchtower — Pitch Deck (Candidate #2, Tier 1)

Research date: 2026-09-25/26. Re-verification pass on Phase 1 candidate A2 (async-infra.md), scored
31/35, tied-highest of the pool, and confirmed as Tier-1 T2 in the 4-point niche re-screen
(03-refiltered-shortlist.md). This session re-fetched the spec directly (byte-exact `grep`, not
just AI-summarized WebFetch) and re-listed the full `tempoxyz` GitHub org.
**Bottom line: the Phase 1 finding holds and is now more precisely sourced. No weakening found.**

**Phase 3 adversarial verification: SOUND WITH CAVEATS** (full report:
`phase3-verification/tempo-watchtower-verdict.md`, cleanest verdict of the four Tier-1 candidates
tested). The five required fixes from that verdict are applied throughout this version: (1) TAM
staleness caveat strengthened in §2, not just §7; (2) repo count corrected to the independently
re-verified 69 (was 72 — the org visibly churns day to day, not a substantive discrepancy);
(3) "structural neglect" language in §7 now distinguishes "no public roadmap or announcement
found" from "Tempo Labs has ruled this out" (the latter is not verifiable from public sources);
(4) `temprano-watchtower`'s last-push date is marked UNVERIFIED in §8 (the verifier could not
confirm it, GitHub API rate-limited); (5) no changes needed to the core technical evidence —
every quoted spec section was independently reproduced byte-for-byte by the verifier from a fresh
`curl` fetch.

---

## 1. Title + one-liner

**tempo-watchtower** — an independent Rust daemon that watches your Tempo MPP payment channels
and settles them before the protocol's 15-minute forced-close clock lets your customer walk away
with money you already earned.

(Working name "TEMPO-WATCHTOWER" kept as the product name — see naming note in §3. No sharper
name found that beats the direct Lightning-watchtower analogy this pitch leans on.)

---

## 2. Problem statement

**User population, named precisely:** payees — server operators selling metered/streamed
services (LLM token streaming, per-request APIs, agent tool calls) — who accept payment over
Tempo's Machine Payments Protocol (MPP) "session" intent, i.e. unidirectional, voucher-based
streaming payment channels. As of the protocol's own reported snapshot (late March 2026, carried
over from Phase 1, **not re-verified with a fresher number this session** — no updated stats page
was found; see Evidence Appendix): "roughly 31,100 transactions, $3,730 in volume, 671 registered
agents, and 326 servers." — **INHERITED, and stale: this figure is ~6 months old as of this
research (2026-09-25/26).** Phase 3 verification independently tried and failed to find a fresher
number (`mpp.dev`, `mpp.dev/stats`, `tempo.xyz/mpp` all checked directly — no live adoption
metrics found anywhere). Given MPP is an actively-promoted flagship product (two dedicated
Tempo blog posts since this snapshot, a 69-repo GitHub org with daily commit activity), the true
current count most likely understates real adoption today — but this is a plausible direction,
not a verified one, and the pitch should say exactly that rather than imply a known current
number.

**The mechanism (VERIFIED this session by direct byte-exact fetch of the current spec text,
not just an AI summary of it):**

- Source: `paymentauth.org/draft-tempo-session-00.txt`, "Tempo Session Intent for HTTP Payment
  Authentication," dated **23 September 2026**, expires 27 March 2027, authored L. Horne,
  G. Konstantopoulos, D. Robinson, B. Ryan, J. Moxey (Tempo Labs). Still the current, unreplaced
  draft-00 as of today (2026-09-25) — two days old, not superseded by a draft-01.
- **§6.4.5 `requestClose`**: "User requests channel closure, starting a grace period of at least
  15 minutes... Sets channel.closeRequestedAt to current block timestamp. The grace period allows
  the payee time to submit any outstanding vouchers before forced closure."
- **§6.4.6 `withdraw`**: "User withdraws remaining funds after the grace period expires... Requires
  block.timestamp >= channel.closeRequestedAt + CLOSE_GRACE_PERIOD. Refunds all remaining deposit
  to payer and marks channel finalized."
- **§13.3 "Forced Close"** (exact steps, quoted verbatim from the live document): "If the server
  does not respond to close requests: 1. Client calls requestClose(channelId) on-chain 2.
  15-minute grace period begins (wall-clock time via block.timestamp) 3. Server can still settle()
  or close() during grace period 4. After grace period, client calls withdraw(channelId) 5. Client
  receives all remaining (unsettled) funds. Clients SHOULD wait at least 16 minutes after
  requestClose() before calling withdraw() to account for block time variance."
- **§12.1 "Accounting State"** (the exact fields the watchtower must mirror): servers MUST
  maintain, per active session, `acceptedCumulative` (uint128, "Highest valid voucher amount
  accepted, monotonically increasing"), `spent` (uint128, cumulative amount charged for delivered
  service), `settledOnChain` (uint128, last cumulative amount settled on-chain, informational),
  `deposit`, and `closeRequestedAt`. "The available balance is computed as: available =
  acceptedCumulative - spent."
- **§14.12 "Chain Reorganization"** (this is the load-bearing quote — re-confirmed verbatim by
  direct `grep` of the live .txt after an AI-summarized WebFetch pass first, misleadingly,
  reported it as absent — see risk note below): "for high-value channels, servers SHOULD: 1.
  Re-verify channel state periodically during long-lived sessions. 2. **Monitor for
  ChannelClosed or CloseRequested events**. 3. Cease service delivery if the channel becomes
  invalid."
- **§14.13 "Grace Period Rationale"**: "The 15-minute forced close grace period balances competing
  concerns: ... *Payee protection*: Provides time to detect close requests and submit final
  settlements, **even during network congestion or maintenance windows**." This is the spec
  authors themselves naming "maintenance windows" as an expected condition under which a payee
  could miss the deadline and lose funds — not a hypothetical this pitch invented.
- Solidity event definitions (from the spec's interface listing) confirm the exact events to
  subscribe to: `event CloseRequested(bytes32 indexed channelId, address indexed payer, address
  indexed payee, uint256 closeGraceEnd)` and `event ChannelClosed(bytes32 indexed channelId,
  address indexed payer, address indexed payee, uint256 settledToPayee, uint256 refundedToPayer)`.

**Status: VERIFIED (fetched and grepped directly this session, 2026-09-25).** All of the above
matches, and sharpens, the Phase 1 citations in async-infra.md candidate A2 — same section
numbers, same quotes, now with exact surrounding context and line numbers confirmed against the
raw document rather than an AI paraphrase.

**Risk transparency note (important, read this):** a first WebFetch pass this session (which
runs the fetched page through a small summarization model before returning it to the agent) came
back claiming "None of these exact strings appear in the document" for "ChannelClosed,"
"CloseRequested," "maintenance window," and "network congestion" — which would have meaningfully
weakened the Phase 1 finding if taken at face value. A second, targeted `curl` + `grep` of the
raw 3013-line document (bypassing the summarizer) found all of them, verbatim, at §14.12/§14.13
as quoted above. **Conclusion: the summarizer produced a false negative on a long document, not
the spec changing.** This is flagged here so the verifier does not need to re-litigate it, but it
is also a reminder that this pitch's core evidence should be checked against the raw text, not an
AI summary of it, before the hackathon submission.

**What's new/changed since Phase 1 that doesn't weaken the case:**
- The full `tempoxyz` GitHub org lists **~69-72 repos** (count churns day to day — Phase 2
  research counted 72, Phase 3 verification's independent fetch the next day counted 69 with no
  pagination remaining; the org has near-duplicate repos like `reth`/`reth-1..4` appearing and
  disappearing, so treat this as "~70" rather than a precise figure), up from the "50+" Phase 1 counted,
  including three new-in-2026-09 additions (`mpp-tools`, pushed today; `gh-actions`; `docs`).
  None of the new repos is a watchtower/keeper/settlement-monitor. `mpp-tools`'s `ledger/`
  directory — the closest-sounding name to worry about — turned out to be "Agricola decision
  records," an internal audit trail for SDK-change review, unrelated to per-channel payment
  accounting. **VERIFIED this session** (fetched repo contents directly).
- `mpp-rs` (the official Rust SDK, crate `mpp` on crates.io, now at v0.13.0) remains a
  request/response payment-primitives library: `charge()`, `verify_credential()`, a
  `PaymentMiddleware` for clients, and a WebSocket challenge/credential flow. It has exactly one
  extra workspace crate, `alloy-transport-mpp` (an Alloy RPC transport wrapper) — no event
  subscription, no deadline scheduler, no autonomous settle/close daemon. **VERIFIED this
  session** (fetched README and workspace listing directly).
- Zero mentions of "watchtower" or "keeper" anywhere in the current spec text (`grep -c` = 0).
  **VERIFIED.**

---

## 3. What we are

**Product:** `tempo-watchtower` — a pure-Rust daemon/CLI a payee runs alongside (and ideally on
separate infrastructure from) their main API server.

**Exact mechanism:**
1. **Event subscription.** Subscribes via WebSocket/RPC to `CloseRequested` and `ChannelClosed`
   events (per §14.12's explicit instruction to "Monitor for ChannelClosed or CloseRequested
   events") for every channel the payee is party to, on Tempo's EVM-compatible chain (channels are
   either legacy contract-backed or TIP-20 channel-precompile-backed per the spec's own abstract).
2. **Accounting ledger mirror.** Maintains a local, crash-safe ledger keyed by
   `(challengeId, channelId)` mirroring the spec's own §12.1 fields — `acceptedCumulative`,
   `spent`, `settledOnChain`, `deposit`, `closeRequestedAt` — computing `available =
   acceptedCumulative - spent` exactly as the spec defines it, so the watchtower always knows the
   true unsettled balance without trusting the main server's possibly-crashed process.
3. **Deadline-driven settlement.** The instant a `CloseRequested` event fires (`closeGraceEnd` is
   emitted directly in the event payload), the watchtower schedules and submits `settle()` and/or
   `close()` with enough margin to beat the 15-minute window — using the spec's own recommended
   16-minute client-side buffer as the outer bound for how late it can safely leave it, but
   targeting submission within seconds to minutes of the event, not minutes before the deadline.
4. **Redundant/separate-infra deployment.** Designed to run as an independent process, on
   independent infrastructure, from the payee's main API server — this is the whole point (see
   §5): if the main server is down for a "maintenance window" (the spec's own phrase), the
   watchtower is not.

**Why pure Rust:** this is a deadline-critical, crash-safety-critical, always-on process watching
on-chain events and signing/broadcasting transactions with no room for GC pauses or a runtime
crash at the wrong second — exactly the profile pure async Rust (tokio, an EVM-compatible signer/
RPC client) is built for, and it composes directly with `mpp-rs`'s own crate ecosystem rather than
introducing a second language into a payee's Rust-native Tempo stack.

**Who's building this:** a solo builder, pure-Rust implementation, no team.

---

## 4. Tech demo (3-minute plan, minute-by-minute)

- **0:00–0:30** — Open a real MPP session channel on Tempo's public test network (the `mpp-rs`
  client SDK's own quickstart connects to `rpc.moderato.tempo.xyz`, confirmed live in the current
  README — VERIFIED). Show the payee's API server serving a few metered requests, vouchers
  accumulating (`acceptedCumulative` ticking up), server periodically calling `settle()`
  cooperatively.
- **0:30–1:00** — Kill the payee's main API server process on camera ("simulating a maintenance
  window" — the spec's own words). `tempo-watchtower`, running as a separate process, keeps
  running.
- **1:00–1:30** — From the client side, call `requestClose(channelId)` on-chain. Show the emitted
  `CloseRequested` event with its `closeGraceEnd` timestamp. Start an on-screen countdown timer
  from 15:00.
- **1:30–2:15** — Split screen: **left, the control run** — no watchtower, main server still down
  — the countdown hits zero, the client calls `withdraw()`, and the demo shows the payee's
  unsettled `spent - settledOnChain` balance go to zero, funds gone. **Right, the protected run**
  — `tempo-watchtower` fires on the `CloseRequested` event, submits `close()` (using its
  independently-tracked accounting ledger, no dependency on the crashed main server), and the
  transaction confirms with several minutes still on the clock.
- **2:15–2:45** — Show the two final on-chain balances side by side: payee keeps the earned funds
  in the watchtower run, loses them in the control run. This is the whole pitch in one screenshot.
- **2:45–3:00** — Close on the one-line business pitch (§5) and the composability point: this
  ships as a companion to the official `mpp-rs` SDK, not a replacement for it.

---

## 5. Business model

**Who pays, how, and why (applying the 4-point filter explicitly):**

- **The cost avoided is total loss of already-earned, unsettled funds** — not a UX annoyance, not
  wasted engineering time. If a payee misses the 15-minute window, the spec is explicit
  (§13.3 step 5, §6.4.6): the client "receives all remaining (unsettled) funds" and the channel is
  "marked finalized." There is no recovery path afterward. For a metered/streaming service with
  meaningful per-channel float (e.g., an LLM API charging per token over a long session), this is
  a direct, complete, and irreversible financial loss — exactly the category of "real cost" the
  4-point filter requires, not merely #2's excluded "annoyance" tier.
- **Why a payee would pay for this specific structural protection:** a watchtower is *by design*
  supposed to be operated independently of the thing it's protecting — the same reason Lightning
  Network watchtowers are a distinct third-party ecosystem role rather than something Lightning
  Labs ships baked into its own node software. If Tempo Labs shipped an in-house "watchtower"
  that ran on the same infra/ops team as a payee's own server, it would provide zero protection
  against exactly the "maintenance window" failure mode the spec calls out — the protection's
  entire value comes from being *someone else's* infrastructure and *someone else's* on-call
  rotation — this is a **structural logical argument** (a protocol team's in-house watchtower
  would undercut its own purpose) independent of any roadmap evidence. Phase 3 verification
  checked for a public Tempo Labs roadmap or announcement that would confirm or contradict this
  and found none either way (`tempo.xyz/developers/roadmap` 404s; no relevant issues in
  `mpp-specs` or the now-archived `tempo-support` repo) — so the honest framing is "no public
  evidence Tempo Labs is building this," not "Tempo Labs has ruled it out."
- **Pricing model (Lightning-watchtower-style economics, as in Phase 1's original sketch):**
  - Self-hosted, open-source core (MIT/Apache) — free, for payees who want to run their own
    watchtower on genuinely separate infra (a different cloud region/provider from their main
    server). This maximizes composability/open-source judging credit (§7) and is the right choice
    early while the ecosystem (326 servers, per the last known snapshot) is small.
  - Managed multi-region redundancy tier for payees who want a third party to run the watchtower
    for them: priced as a small percentage of at-risk channel value (e.g., a basis-point fee on
    the deposit/available balance being protected) or a flat monthly fee per channel/server —
    mirroring how Lightning watchtower services (e.g., a security-deposit-plus-fee model) price
    coverage against exactly this class of forced-close risk.
  - A "coverage" framing (pay X, we guarantee your channels get settled before forced-close, or
    we compensate the shortfall) is the most defensible long-run business but is explicitly a
    later-stage product, not the hackathon MVP.

---

## 6. Startup methodology (lean: start from one specific customer, expand concentrically)

**Do not start by targeting "MPP payees" (the ~326-server population in §2) — that's a market
size, not a customer.** Lean methodology means picking the single narrowest slice of that
population you can name, reach, and talk to individually this month, proving the risk and the
willingness-to-pay with them specifically, then expanding outward only once that's proven.

**Beachhead customer (the ONE segment to start with):** not "any MPP server operator" —
specifically, the builders currently shipping AI-agent/LLM-metered-API products on Tempo MPP
sessions *during this same hackathon window*. This is deliberately narrow and concrete:
- They are enumerable today: search `mpp-rs` usages on GitHub for real (non-example) integrations
  of the session-channel client, scan Tempo's own Discord/`tempoxyz` org for anyone discussing MPP
  session channels in the last 30 days, and check Colosseum's own Tempo-track submission list once
  it's visible — these are the same builders this pitch is competing alongside for prizes, which
  makes them uniquely reachable (same Discord, same track, same deadline pressure).
- They are the highest-motivation segment: a team building a live MPP integration *right now* is
  actively wiring up `requestClose`/`withdraw`/settlement logic and is the most likely group to
  immediately recognize "wait, what happens if my server is down when this fires?" without needing
  the problem explained to them first.
- This avoids the trap of trying to convince a "hundreds of servers, mostly not paying attention"
  population in the abstract — talk to the 5-15 people who are elbow-deep in this exact code path
  this week.

**Riskiest assumption to test first:** not "does the bug exist" (already proven, §2) but "does a
payee, faced with this, actually care enough to run a second process for it, before it happens to
them." Test this the unscalable way, before writing more code than the hackathon MVP requires:
personally message every MPP-session builder found via the search above with one concrete
question — "what's your plan if your server is down when a client calls `requestClose`?" — and
track how many (a) hadn't thought about it, (b) want the answer, (c) would run a daemon for it
today if it existed. This costs nothing and validates demand before it validates code.

**MVP, scoped to that one customer, not the whole market:** ship the free, self-hosted OSS daemon
first (§5's free tier) — not the paid managed/coverage tier — to the specific 3-5 builders from
the beachhead who say yes. The ask to them is explicit: run it, tell me what breaks, let me use
"caught a real forced-close on my channel" (or even "would have") as a public testimonial. This
is the classic "give away the shovel to prove the gold is real" move: it costs the builder nothing
to try, and it converts the pitch's TAM-uncertainty problem (§2, §7) into a handful of named,
citable, real users before the hackathon submission is due — which is itself evidence for the
Potential Impact and Business Plan judging criteria, not just a future promise.

**Expansion path (concentric circles, only after the beachhead validates):**
1. **Beachhead (weeks 1-2, inside the hackathon window):** the 5-15 hackathon-adjacent MPP
   builders above. Success metric: at least 2-3 running the daemon with real testnet/mainnet
   channels before submission.
2. **Early adopters (post-hackathon, as MPP mainnet volume grows):** every newly-registered MPP
   session-channel server discoverable via on-chain `ChannelCreated`-style events or a future
   `mpp.dev` directory (§9 flags none exists publicly yet) — the moment MPP publishes a public
   server list or explorer, that becomes an outbound list, not a cold search.
3. **Ecosystem-endorsed default (later):** approach Tempo Labs/the `tempoxyz` org directly with
   the working, adopted tool and the beachhead testimonials, aiming for a listing in their own
   ecosystem/awesome-tempo docs — the same integration credibility path other MPP tooling
   (`mpp-rs` itself, wallet-provider integrations) has taken publicly.
4. **Paid tier introduction (only once step 1 is proven):** before building the managed/coverage
   tier described in §5, explicitly ask the beachhead + early-adopter cohort "would you pay $X/mo
   for someone else to run this for you" and use that answer — not a guess — to decide whether and
   how to price it. Do not build the hosted business first; the self-hosted free tool is both the
   MVP and the customer-discovery instrument for whether the paid tier is worth building at all.

**Why this order, not "build the business model in §5 first":** §5 sketches the eventual business
(self-hosted free + paid managed coverage) but a lean approach tests willingness-to-run (step 1-2
above) before testing willingness-to-pay (step 4) — conflating the two is how a technically-sound
tool ends up with no real customers, which is exactly the failure mode the 4-point niche filter
(`03-refiltered-shortlist.md`) was designed to catch before committing 14 days to a build.

---

## 7. Track fit & judging-criteria mapping

- **Functionality:** a working daemon that subscribes to real on-chain events on Tempo testnet
  and submits real `settle()`/`close()` transactions before a real deadline — directly
  demonstrable, not simulated math.
- **Potential Impact:** honest framing required. The MPP ecosystem is still small — the last
  known snapshot is ~326 servers and ~671 agents (late March 2026, not re-verified with a fresher
  count this session). The pitch should not overstate current TAM; instead it should frame impact
  as *structural infrastructure for a flagship, Stripe-co-authored protocol Tempo is actively
  pushing as its headline use case* — the addressable market grows exactly as MPP itself grows,
  and every new metered-service payee is a new potential user by construction (unlike a
  one-off/point-in-time bug fix). Impact should be pitched as "the safety-net every MPP payee
  eventually needs," not "N users today."
- **Novelty:** clean — confirmed zero existing watchtower/keeper for Tempo MPP session channels,
  in the official org, on crates.io, or via targeted GitHub search (see §8). The closest
  similarly-named project (`temprano-watchtower`) solves an unrelated problem (see §8). No public
  Tempo Labs roadmap or announcement points toward building this in-house either — but that's an
  absence-of-evidence finding (no roadmap page exists to check), not a confirmed commitment either
  way; see §7's business-plan note and §8 for what was and wasn't independently verifiable.
- **UX:** the product's entire value proposition is invisible-until-needed reliability — the demo
  (§4) is the UX case: a payee's customers get uninterrupted service and the payee's earned money
  is safe, with zero manual intervention required at the moment of failure.
- **Open-source:** ship the core daemon MIT/Apache, matching the licensing norms of `mpp-rs`
  itself (MIT) and the rest of the `tempoxyz` ecosystem it composes with — strong composability
  story since it plugs directly into the official spec/SDK rather than forking or replacing it.
- **Business Plan:** viable, bounded, Lightning-watchtower-precedented economics (§5); the
  weakest point to defend honestly is that today's absolute revenue ceiling is small because the
  underlying MPP volume is small — the pitch should lean on growth-with-the-protocol rather than
  current-scale numbers.

---

## 8. Competitive scan (for the verifier)

**From Phase 1, re-confirmed this session (and independently re-confirmed again in Phase 3):**
- Full `tempoxyz` GitHub org (~69-72 repos depending on the day fetched — the org churns via
  near-duplicate repos like `reth`/`reth-1..4`; Phase 3's independent fetch the next day got 69
  with no pagination remaining, materially the same population as this session's 72): no repo
  named or described as a watchtower, keeper, settlement-monitor, or channel-liveness daemon
  exists, on either fetch. Full list fetched via `api.github.com/orgs/tempoxyz/repos`.
- `mpp-rs` (official Rust SDK, crate `mpp` v0.13.0): protocol primitives only (`charge`,
  `verify_credential`, client middleware, WS challenge/credential flow) plus one Alloy-transport
  helper crate — no autonomous monitoring or deadline-driven settlement logic. Confirmed by
  fetching the README and workspace crate listing directly.
- `mpp-tools` (new repo, pushed 2026-09-25, i.e. today): a cross-SDK *conformance test suite* and
  an internal "Agricola" SDK-change-review control plane — its `ledger/` folder is an audit trail
  for maintainer decisions, not a payment-channel accounting ledger. Not a competitor; checked
  because the name was a plausible false alarm.

**New this session (fresh GitHub/crates.io search, since WebSearch was unavailable):**
- `danhper/temprano-watchtower` (GitHub, 0 stars; last-push date of 2026-02-06 as originally
  found is **UNVERIFIED** — Phase 3's independent re-check hit an unauthenticated GitHub API
  rate limit and could not confirm or refute this specific date, though nothing found contradicts
  it): despite the name, this is a **general-purpose durable transaction broadcaster** for
  Tempo — "accepts signed Tempo transactions, stores them durably, and broadcasts them throughout
  their validity window until mined, expired, invalid, or canceled," grouped by nonce key, with a
  hosted endpoint at `watchtower.temprano.io`. It solves *reliable transaction delivery/nonce
  management*, not *MPP session-channel forced-close monitoring* — it has no awareness of
  `ChannelClosed`/`CloseRequested` events, voucher accounting, or the 15-minute grace period. Does
  not solve this problem; worth a one-line mention in the pitch as "not the same thing" in case a
  judge finds it by searching "tempo watchtower."
- `MoveIndustries/movement-stream-channel` (GitHub, 1 star, Move language, last pushed
  2026-03-23): a port of the `TempoStreamChannel` contract concept to Movement Network (a
  different chain, different smart-contract language). Not a Rust tool, not on Tempo, not a
  monitoring daemon — irrelevant beyond confirming no exact-name collision risk on Tempo itself.
- crates.io: no crate matches `tempo-channel*`; the `watchtower` query returns only unrelated
  results (`solana-watchtower`/`agave-watchtower`, Solana validator-monitoring tools with an
  unrelated "Blockchain, Rebuilt for Scale" tagline; `freshdock`, a Docker container
  auto-updater; Lightning Network *encoding* crates, not watchtowers). The crate name
  `tempo-watchtower` itself is unregistered and available.
- `mpp.dev` protocol docs pages: fetched directly; no mention anywhere of an official watchtower,
  keeper, monitor, or auto-settlement service for session channels.

**Conclusion:** whitespace confirmed clean, now checked against a larger org listing and a fresh
GitHub/crates.io sweep than Phase 1 had time for. The one plausible false alarm by name
(`temprano-watchtower`) turned out, on inspection, to solve a different problem entirely.

---

## 9. Evidence appendix

| # | Claim | Status | Source |
|---|---|---|---|
| 1 | Spec doc dated 23 Sept 2026, expires 27 Mar 2027, authored Horne/Konstantopoulos/Robinson/Ryan/Moxey (Tempo Labs), still current draft-00 | VERIFIED (this session, direct fetch) | paymentauth.org/draft-tempo-session-00.txt header |
| 2 | `requestClose()` starts >=15-min grace period; `withdraw()` requires grace period elapsed and refunds all remaining deposit | VERIFIED (this session, `grep` of raw text, §6.4.5/§6.4.6) | same doc |
| 3 | §13.3 Forced Close 5-step sequence, incl. "Client receives all remaining (unsettled) funds" and the 16-minute client-side buffer recommendation | VERIFIED (this session, direct fetch, exact section text reproduced) | same doc, §13.3 |
| 4 | §12.1 Accounting State fields: `acceptedCumulative`, `spent`, `settledOnChain`, `deposit`, `closeRequestedAt`; `available = acceptedCumulative - spent` | VERIFIED (this session, direct fetch) | same doc, §12.1 |
| 5 | §14.12: servers SHOULD "Monitor for ChannelClosed or CloseRequested events"; §14.13: grace period protects payees "even during network congestion or maintenance windows" | VERIFIED (this session, confirmed by raw `grep` after an AI-summarized fetch pass falsely reported these strings absent — see risk note in §2) | same doc, §14.12/§14.13 |
| 6 | `CloseRequested`/`ChannelClosed` Solidity event signatures, incl. `closeGraceEnd` field | VERIFIED (this session, direct fetch of interface listing) | same doc |
| 7 | Channels are either legacy contract-backed or TIP-20 channel-precompile-backed | VERIFIED (this session, doc abstract) | same doc, Abstract |
| 8 | MPP snapshot: ~31,100 transactions, $3,730 volume, 671 agents, 326 servers (late March 2026) | INHERITED from Phase 1; no fresher public stats page found this session | Phase 1 (async-infra.md A2), re-checked mpp.dev this session with no update found |
| 9 | `tempoxyz` GitHub org has 72 repos (as of 2026-09-25); no watchtower/keeper/settlement-monitor repo among them | VERIFIED (this session, full org listing fetched) | api.github.com/orgs/tempoxyz/repos |
| 10 | `mpp-rs` is a request/response payment-primitives SDK (crate `mpp` v0.13.0), no autonomous monitoring; only extra crate is `alloy-transport-mpp` | VERIFIED (this session, README + workspace listing fetched) | github.com/tempoxyz/mpp-rs |
| 11 | `mpp-tools`'s `ledger/` dir is an "Agricola" SDK-change audit trail, not a payment ledger | VERIFIED (this session, repo contents + README fetched) | github.com/tempoxyz/mpp-tools |
| 12 | Zero occurrences of "watchtower" or "keeper" in the current spec text | VERIFIED (this session, `grep -c`) | paymentauth.org/draft-tempo-session-00.txt |
| 13 | `danhper/temprano-watchtower` exists but solves durable tx broadcast/nonce management, not channel forced-close monitoring; stale since Feb 2026, 0 stars | VERIFIED (this session, repo + README fetched) | github.com/danhper/temprano-watchtower |
| 14 | `MoveIndustries/movement-stream-channel` is an unrelated Move-language port for a different chain | VERIFIED (this session, repo metadata fetched) | github.com/MoveIndustries/movement-stream-channel |
| 15 | crates.io has no Tempo-channel-related crate; `tempo-watchtower` name is unregistered | VERIFIED (this session, crates.io API queried) | crates.io/api/v1/crates |
| 16 | `mpp-rs` client quickstart connects to `rpc.moderato.tempo.xyz` (usable public test/dev endpoint for the demo) | VERIFIED (this session, README code sample fetched) | github.com/tempoxyz/mpp-rs README |
| 17 | MPP is co-authored by Tempo and Stripe; Stripe maintains `stripe/mpp-rb` | VERIFIED (this session, README fetched) | github.com/tempoxyz/mpp-rs README |
| 18 | Phase 1's original scoring, competitor scan, and business-model sketch for A2 | INHERITED | research/phase1-scouting/async-infra.md candidate A2 |
| 19 | 4-point niche filter and T2 Tier-1 classification rationale | INHERITED | research/03-refiltered-shortlist.md |
