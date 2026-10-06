# Judge round 1: tempo-watchtower pitch deck (v1)

Judge persona: Colosseum partner plus a Tempo-track judge who knows payment channels. Date: 2026-09-27.
Deck reviewed: `research/phase4-pitch/tempo-watchtower-pitch-deck.md` (v1).
Evidence checked: the phase2 deck, the phase3 verdict, the competitor re-check, winner-patterns and 00-brief. I also fetched the live spec (`curl paymentauth.org/draft-tempo-session-00.txt`, 3013 lines) and grepped it.

Placeholder assumption, as instructed: `[[FOUNDER]]` and `[[DEMO]]` get filled with modest, typical solo facts. That means a real testnet tx hash, a few builder conversations, 1-2 testnet users, and a credible but not famous background.

## TOTAL: 60 / 100 (pass = 90). Not shortlisted as-is.

In one sentence: the deck is clean, well structured and honest about market size. But its headline technical "insight" is out of date against the current spec. It never puts a dollar figure on the loss. It never answers the obvious "why not just settle every minute?" objection. As a result it reads like a well-formatted port of a known concept to a tiny market.

---

## A. Official contest criteria: 28 / 48

| # | Criterion | Score | Justification |
|---|---|---|---|
| A1 | Functionality | 5/8 | The demo design is right: a split-screen control vs. protected run, a real tx hash, and "nothing on screen is faked". But it shows one happy path only. There is no signal of code quality, crash-safety, missed-event handling or tests. The deck's own "pre-build checks" admit it is **unconfirmed** whether the escrow/precompile is even usable on testnet. |
| A2 | Potential Impact | 4/8 | "Small but growing" is honest, but the count is of *MPP servers* (~326, March 2026), not *session payees*, which is an unknown subset. The deck never states value at risk. The same snapshot says **$3,730 total MPP volume**, and a judge who finds that will discount everything. The big ecosystem argument is missing: payee-side liveness risk caps how large channels and deposits can safely get, so a watchtower is what lets sessions scale. |
| A3 | Novelty | 5/8 | The gap is real and sourced: 0 watchtowers in the tempoxyz org, and Fiber and MPP-Inspector are correctly distinguished. But the stated moat ("only the payee may settle, so a Lightning tower can't be reused") is weakened by the current spec's v2 `operator` field and by the pre-signed-transaction pattern (see Technical errors T1). The table also omits `danhper/temprano-watchtower`, the first hit a judge gets when searching "tempo watchtower". |
| A4 | UX | 5/8 | "Invisible until needed, zero manual intervention" is the right story, and the demo carries it. There is nothing on operator UX: how a payee installs it, how vouchers reach the tower (an mpp-rs/mppx hook?), what config it needs, and what they see when it fires (alert plus receipt). "How blockchain creates great UX" is not argued. |
| A5 | Open-source / composability | 5/8 | "Open source (Rust)" plus mpp-rs in the appendix is thin. There is no license and no concrete integration point (a middleware that tees vouchers to the tower, `getChannelsBatch` polling, `CloseRequested` / `CloseRequestCancelled` / `ChannelClosed` handling). It also misses the strongest composability story available: acting as the v2 `operator` in the channel descriptor, which the spec already defines (§6.2.1, `methodDetails.operator`). |
| A6 | Business Plan | 4/8 | Open core plus a hosted tier is sensible. But the price is a placeholder and "per protected channel" is the wrong unit: channels are many and small, so price per payee or server per month. The hosted tier depends on key delegation, which the deck calls unsolved and pushes to the roadmap, so as pitched the business sits on an open problem. There is no revenue math at all, not even "N payees x $X". |

## B. Colosseum internal dimensions: 15 / 28

| Dimension | Score | Justification |
|---|---|---|
| Founder-market fit | 2/4 | The slide structure and guidance are good: dense proof points, with the Tokamai and Zoneless examples. But the brief says the builder's background did *not* drive the idea, so with modest facts this becomes "competent engineer who read the spec". "I found this reading the spec line by line" is a decent how-it-was-uncovered answer; it is not founder-market fit. |
| Insight | 2/4 | "Forced close means payee loss during an outage, and the spec itself names maintenance windows" is real but readable by anyone who opens §13.3. The claimed non-obvious insight (payee-only settle) is the part a Tempo engineer will pick apart (T1). |
| Product + execution | 2/4 | Nothing is built yet and feasibility on testnet is unconfirmed. The scope is tight and demo-able, but the deck gives no build-progress signal (commits, weekly updates). |
| Market size | 2/4 | Honestly small, but the growth driver (Stripe backing, other chains) is asserted rather than shown. The multichain claim is not in the evidence files and, per the deck's own appendix, "semantics may differ". |
| Founder communication | 3/4 | One idea per slide, plain language, and a strong timeline visual. It loses a point for running over the word budget and for per-slide timings that can't be spoken (see C3). |
| Viability | 2/4 | There is no quantified reason a payee pays rather than settling more often, and the hosted tier is blocked on delegation. |
| Traction | 2/4 | The validation slide is well structured (counters plus a real quote), and "a few conversations, 1-2 testnet users" earns 2. The zero-user fallback line ("My first design partners are onboarding now") is itself a soft misrepresentation if no partner has agreed to anything (see Honesty). |

## C. Pitch craft: 11 / 16

| Item | Score | Justification |
|---|---|---|
| Winner patterns | 3/4 | It follows winner-patterns §E nearly slide for slide, which is good. It misses rule C6 (one quantified contrast with a $ figure on screen) and E1 (founder credential spoken in the first 10 s), and the "why now" is a draft date rather than a trend. |
| Hook | 2/4 | Slide 1 is a slogan plus "an MPP server". There is no named user and no concrete loss. The loss arrives at 0:10-0:35 and is abstract ("unsettled revenue"). Compare Tokamai's opening status-quo sentence. You need something like: "An agent streams $400 of inference, your region goes down for 20 minutes, and it takes all $400 back. Legally, per the spec." |
| Narrative flow | 3/4 | The order earns each next slide. But the timings are wrong. Slide 10 has 36 words in 10 s (216 wpm), slide 11 has 34 words in 10 s (204 wpm), slide 5 has 62 words in 20 s (186 wpm), and slide 4 has 38 words in 12 s (190 wpm). The aha demo gets only 28 s for two runs. The pitch is back-loaded with market and GTM slides that could merge. |
| Honesty | 3/4 | There is no fabricated traction and placeholders are clearly marked. The deductions: "Protocol authors prescribe it" and "tempo-watchtower is that 'should', shipped" over-read §14.12. The spec date is stale (the live header now reads 26 Sep 2026). The multichain claim is not traceable to the evidence files. The "~473 words" self-count is wrong. The zero-user fallback line implies partners that may not exist. |

## D. Build-readiness for an AI deck generator: 6 / 8

| Item | Score | Justification |
|---|---|---|
| Per-slide completeness | 3/4 | Every slide has a title, on-slide text, a visual and a script. But slides 2, 10, 11 and 12 break the deck's own "max ~20 words on-slide" rule (about 30-35 words each). |
| Self-contained | 3/4 | The style brief and visuals are strong. But "Guidance (delete before generating)" blocks and the appendices sit inline, so a generator could render them. The slide 3 *example* script is written as quoted speech a generator or TTS could read out verbatim as fact. Slide 6 needs a real screen recording the generator can't produce, which should be marked "insert recording". |

---

## Top 5 highest-leverage fixes (ordered)

1. **Rebuild the technical core against the current spec (v2), and turn the delegation "open problem" into the product.** The live spec (header 26 Sep 2026) says "New servers SHOULD emit `sessionProtocol: v2`" (TIP-20 channel precompile, §4). The v2 channel descriptor includes `operator`: "Optional payee-side operator authorized for channel operations" (§6.2.1). Servers advertise it via `methodDetails.operator`, and it is hashed into the channelId.
   - Verify on testnet/precompile docs exactly which calls the operator may make.
   - If it covers `settle`/`close`, rewrite slide 5 and the close as "tempo-watchtower is your channel's operator: it can settle for you, but it can never redirect funds." That removes key custody, unblocks the hosted tier and makes the composability story concrete.
   - Either way, delete "only the payee can settle, so you can't drop in a Lightning-style tower" as the headline insight. It is v1-only (§6.5 table), and even on v1 a payee could hand a tower *pre-signed* `settle` txs (the Lightning pattern, and exactly what temprano-watchtower already broadcasts).
   - In the same pass: mirror `spent` (not just `acceptedCumulative`/`settledOnChain`), and change "settle() the highest voucher" to "claim what you earned". §13.2 says servers "MUST NOT capture more than the amount actually consumed"; for v2, `captureAmount = max(spent, settledOnChain)`.
   - Add a `getChannelsBatch` / `closeRequestedAt` polling fallback and `CloseRequestCancelled` handling.

2. **Put a dollar loss and the adversarial threat model in the first 15 seconds, and verify which of two framings is true.** Check the actual server code of mppx and mpp-rs (`grep -r CloseRequested`), not just READMEs, to see whether any SDK server watches for close requests.
   - If none does, the pitch is much stronger: "every MPP session server that doesn't run a separate monitor loses unsettled revenue on *any* forced close, even while it's up."
   - If one does, the pitch must be "the in-process monitor dies with your process; ours doesn't."
   - Then open with a named user and a number. For example: "Maya runs an LLM API on MPP sessions. A $250 channel, a 20-minute region outage, and the agent takes it all back." Put the payer's ability to time `requestClose` to your outage on screen; that is theft of service, not bad luck.

3. **Answer the killer objection on-slide: "Why not just settle every minute?"** Tempo has ~500 ms finality (§14.12) and cheap stablecoin fees, so a Tempo judge will say frequent `settle()` shrinks exposure to about a minute of revenue. Quantify the fee per settle against revenue per interval, and show the break-even: when the tower beats the settle cadence and when it doesn't. Also name the failure mode frequent settling can't cover: a server whose settler is the thing that's down. If you can't win this argument with numbers, the business collapses. It belongs on slide 8 (competition: "DIY cron settle") and in Appendix C.

4. **Fix the claims a judge can falsify in two minutes.**
   - (a) "Session spec published Sep 23, 2026" is stale: the draft-00 header now reads 26 Sep 2026 and expires 30 Mar 2027. The sessions product also predates it (Tempo blog "MPP Sessions", 2 Apr 2026). Reframe why-now as "sessions are moving into a chain-native TIP-20 channel precompile (v2), and the spec was revised this week".
   - (b) Replace "Protocol authors prescribe it / that 'should', shipped" with a direct quote of the payee-protection text: §6.4.5 "allows the payee time to submit any outstanding vouchers before forced closure" plus §14.13 "even during… maintenance windows". §14.12's "Monitor" line is reorg guidance "for high-value channels", followed by "Cease service delivery".
   - (c) Add a `temprano-watchtower` row to the competition table ("broadcasts signed Tempo txs; not channel-aware") and a "DIY: settle on a cron" row.
   - (d) Either add the multichain evidence to the evidence files or cut the chain list to "MPP implementations appearing on other chains (unverified semantics)".
   - (e) Rewrite the "Why won't Tempo build this?" answer. A Tempo-hosted tower would *not* share the payee's ops, so that argument is backwards. The honest answer is that Tempo would more likely ship an in-SDK monitor, which dies with the server, and would avoid operating payee-authority services. And yes, they could; your edge is focus and being the operator.

5. **Make the business and market slides quantitative and cut the script to ≤450 words.**
   - Size in session payees and value-at-risk, not "MPP servers". Price per payee per month with a concrete test price (e.g. "$29/mo, validating with design partners"). Add the impact line: "safe payees means bigger deposits and longer sessions."
   - Merge slides 10 and 11 into one "small, growing, first 10" slide to free about 10 s, and give the demo 35-40 s.
   - Move a one-line founder credential into the first 10 s. Replace the zero-user fallback with a factual line: "I've contacted N session builders; M replied; here's what they said."
   - Recount honestly. The script is about 485-490 words including the placeholder estimates, not 473.

---

## Script length check

| Slide | Stated window | Words (my count) | Implied wpm |
|---|---|---|---|
| 1 | 0:00-0:10 | 24 | 144 |
| 2 | 0:10-0:35 | 64 | 154 |
| 3 | 0:38-0:50 | ~36 (placeholder example) | 180 |
| 4 | 0:50-1:02 | 37 | 185 |
| 5 | 1:02-1:22 | 61 | 183 |
| 6 | 1:22-1:50 | 61 | 131 |
| 7 | 1:50-2:04 | ~36 (incl. placeholder) | 154 |
| 8 | 2:04-2:16 | 38 | 190 |
| 9 | 2:18-2:30 | 32 | 160 |
| 10 | 2:30-2:40 | 39 | **234** |
| 11 | 2:40-2:50 | 37 | **222** |
| 12 | 2:50-3:00 | 23 | 138 |
| **Total** | 3:00 | **~485-490** | ~163 avg |

At about 165 wpm, 490 words is 2:58 with zero slack for pauses, the demo's on-screen beats, or the founder speaking slower than a TTS. It is over the deck's own 480 budget and well over winner-patterns' "aim for 2:45 or less" (about 450 words). There are also unexplained gaps at 0:35-0:38 and 2:16-2:18.

---

## Technical errors / things a Tempo engineer would catch

- **T1 (high): "Only the payee may settle, so a Lightning tower can't be reused; the tower must act with the payee's authority."**
  - This holds for v1 contract channels (§6.5 access table). But the live spec says new servers SHOULD use v2, whose descriptor has an `operator` ("payee-side operator authorized for channel operations", §6.2.1, advertised via `methodDetails.operator`, hashed into the channelId).
  - Even on v1, a payee can pre-sign `settle()` transactions and give them to a keyless tower, the same trust shape as Lightning justice txs. The spec itself references expiring-nonce transactions, and temprano-watchtower already durably broadcasts signed Tempo txs.
  - Presenting this as the "non-obvious part / that's the product" invites the reply "just use the operator slot, or pre-sign".
- **T2 (high): Slide 5 "Mirror (`acceptedCumulative`, `settledOnChain`)" and "`settle()` the highest voucher".**
  - The field that determines what the payee is owed is `spent` (§12.1: `available = acceptedCumulative - spent`), and it is omitted.
  - §13.2: servers "MUST NOT capture more than the amount actually consumed… MUST NOT treat cumulativeAmount alone as the amount to capture"; for v2, `captureAmount = max(spent, settledOnChain)`.
  - `settle()` can only claim a *signed* cumulative amount, so the tower must pick the highest voucher ≤ owed, or use v2 `close` with the correct capture amount.
  - "Settle the highest voucher" over-captures pre-authorized, unconsumed funds, which is exactly what a payer-side reviewer will flag.
- **T3 (medium): §14.12 is mis-framed.** The "Monitor for ChannelClosed or CloseRequested events" line sits under *Chain Reorganization*, scoped to "high-value channels", next to "Cease service delivery if the channel becomes invalid". It is about channel validity, not about settling before a deadline. The payee-protection rationale is §6.4.5 and §14.13. The slide 2 script also merges them: "The spec tells servers to watch for this 'even during maintenance windows'", but §14.13 is rationale, not an instruction to servers.
- **T4 (medium): Wrong failure modes emphasized.** Appendix A leads with reorg handling ("re-submit if the settle is reorged out"), yet §14.12 states ~500 ms finality. The real risks it omits:
  - dropped WebSocket subscriptions or RPC outages (missed `CloseRequested`), which call for a `getChannelsBatch` poll fallback on `closeRequestedAt`;
  - nonce conflicts if the tower and the API server sign with the same payee key;
  - the tower key's fee-token balance;
  - `topUp()` emitting `CloseRequestCancelled` followed by a fresh `requestClose` (state-machine churn).
- **T5 (medium): Voucher replication consistency isn't addressed.** A tower on separate infra only knows the vouchers the server forwarded. If the server crashes after accepting a voucher but before forwarding it, the tower settles a stale amount. The design needs "forward (or durably replicate) before serving", and the deck should say so, because "what's your consistency model?" is the first interview question.
- **T6 (low-medium): The Lightning analogy's threat model differs.** Lightning towers exist to punish *revoked-state (fraud)* broadcasts in bidirectional channels. Tempo's unidirectional cumulative vouchers have no revoked state; the risk here is *liveness/timeout*. "Lightning needed watchtowers. Tempo payees have none" implies the same threat. Say "not fraud protection, liveness insurance".
- **T7 (low-medium): The "Why won't Tempo build this?" logic is backwards.** A Tempo-hosted tower would be independent of the payee's stack, so "sharing ops with the payee's stack" is not the reason. See fix 4e.
- **T8 (low): The grace period is "at least 15 minutes", and implementations "MAY use different grace periods" (§6.4.5, §14.13).** The tower must read `CLOSE_GRACE_PERIOD` / `closeGraceEnd` rather than assume 15:00. The on-slide "15:00" is fine as a visual; the architecture should say so.
- **T9 (feasibility): The demo depends on an unconfirmed pre-build check** (escrow or precompile usable on testnet, and its grace period). If only v1 is deployed on testnet, the whole v2/operator story needs a local deployment. Resolve this before recording anything.

---

## Claims I could not trace to the evidence files (or that are stale)

1. **"Session spec published Sep 23, 2026"** (slide 4). It is traceable to phase2/phase3, but **stale**: my live fetch today shows draft-00 dated **26 September 2026**, expiring 30 March 2027. It is also misleading as a "new" event, since MPP Sessions was announced on the Tempo blog on 2 Apr 2026 (phase3 verdict). Note too that the live draft now leads with v2 / `operator`, which the evidence files don't discuss.
2. **"Community MPP session implementations on Avalanche, XRPL, Hedera, Stellar, Arc"** (slide 10). This is not in any evidence file. The competitor re-check lists different awesome-mpp entries, and the deck's own appendix admits "semantics may differ". It is unverified that these are *session* (channel) implementations.
3. **"Not one [repo] watches channels" / "Nobody settles Tempo channels for you"** (slides 4, 8). The evidence covers repo names and descriptions plus the mpp-rs README only, not SDK source code (e.g. mppx server session handling). It is traceable at repo level and unverified at code level.
4. **"Protocol authors prescribe it" / "tempo-watchtower is that 'should', shipped"** (slide 7). The quote is traceable, but the interpretation is not supported (T3).
5. **"Frequent settles cost fees"** (Appendix C). There are no Tempo fee figures in the evidence, and this is the load-bearing counter-argument (fix 3).
6. **"Total spoken: ~473 words"**. My count is ~485-490.
7. **"Lightning & CKB Fiber watchtowers ✓ act inside their window"**, **"Generic monitoring ✗ pages a human"**, and **"Lightning learned years ago that channels need watchtowers"**. These are general knowledge, acceptable unsourced, but not in the evidence files.

Everything else checked out against the evidence and the live spec:
- 15-minute grace and `withdraw` refunds the unsettled balance (§6.4.5, §6.4.6, §13.3);
- payee-only `settle`/`close` for v1 (§6.5);
- the §12.1 fields;
- the §14.12 and §14.13 quotes (verbatim);
- ~70 repos with 0 watchtowers;
- ~326 servers (Mar 2026, flagged);
- Stripe co-authorship;
- Fiber's watchtower and MPP = multi-path;
- MPP-Inspector;
- the Colosseum guidance quotes and the Tokamai, Zoneless and Cohort V stats.

## What it takes to reach 90

Fixes 1-3 are worth about +15 combined, across Novelty, Insight, Business, Viability, Composability, Hook and Impact. Fixes 4-5 are worth about +8 more (Honesty, Flow, Market, Build-readiness). That still leaves the deck just short of 90 on paper facts alone. Getting over the bar needs one thing only the founder can bring: **a real session builder saying on record that they would run it**, ideally with the watchtower having caught a real forced close on their testnet channel.
