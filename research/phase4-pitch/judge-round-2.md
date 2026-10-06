# Judge round 2: tempo-watchtower pitch deck (v2)

Judge persona: Colosseum partner plus a Tempo-track judge who knows payment channels. Date: 2026-09-27.
Deck reviewed: `research/phase4-pitch/tempo-watchtower-pitch-deck.md` (v2, 11 slides).
Calibrated against `judge-round-1.md` (v1 = 60/100), using the same standard.

Placeholder assumption, as instructed: a credible but not famous background; the founder contacted the ~15 affected payees, got 2-3 replies and one usable quote; the demo has a real testnet tx hash.

## What I verified myself (Bash only)

1. **Contract operator permissions: VERIFIED.** I fetched `tempoxyz/tempo` `tips/verify/src/TIP20ChannelReserve.sol` (450 lines).
   - `settle` and `close` revert `NotPayeeOrOperator` unless the sender is the payee or a non-zero `descriptor.operator`.
   - `requestClose`, `withdraw` and `topUp` are payer-only.
   - Payouts go only to `descriptor.payee` and `descriptor.payer`. `CLOSE_GRACE_PERIOD = 15 minutes`.
   - **But:** in `close`, the caller chooses `captureAmount` anywhere in `[settled, cumulativeAmount]`. `close` also has **no `closeRequestedAt` check**, so it stays valid after the grace period ends, until the payer's `withdraw` lands. Both points matter below.
2. **No close-request watcher in the SDKs: VERIFIED.** I shallow-cloned `wevm/mppx` (HEAD dcf1589, 25 Sep) and `tempoxyz/mpp-rs`.
   - mppx server code has no `CloseRequested` subscription, `watchContractEvent` or `watchEvent`. `closeRequestedAt` appears only in reject paths (`Settlement.ts:320`, `ChannelStore.ts`, `RequestState.ts`, `Transports.ts`).
   - `isSettlementDue` returns `false` when no schedule is set (`Settlement.ts:218`).
   - mpp-rs has no listener either.
   - The mppx *client* state machine runs `requestClose` then `withdraw` (`client/Runtime.ts:312-327`), so a unilateral close is a first-class client path.
3. **Live chain.** Mainnet `CLOSE_GRACE_PERIOD()` returns `0x384` (900 s). VERIFIED.
   - Live mainnet `eth_gasPrice` today is **`0x23c34600` = 6.0e8 attodollars/gas**, and block `baseFeePerGas` is 600,000,000, the T7 floor. The evidence file's 5.9125e9 is about 10x higher than what I read today.
   - Gas snapshot (`..._t7.snap`): settle 301,831 and close 80,913. VERIFIED. The attodollar unit (`primitives/src/transaction/mod.rs:47-52`) is VERIFIED.
4. **Forced-close counts, re-run on the cached logs (`/tmp/tw/logs.json`).**
   - I reproduced 343 payer withdrawals, 1 payee close, 15 payees, 68 "used", $129.59 and 73 with an operator.
   - **The method has a material bug:** it counts only a `close()` as a "response".
   - **38 of the 343 forced channels had a `Settled` event inside the grace window**, after the `CloseRequested` and before the `withdraw`.
     - I checked each tx sender over RPC: 25 were sent by the payee itself, 13 by an operator or other key.
     - Median delay was 83 s after the request; the fastest was 1 s.
   - These payees *did* respond: they claimed what they were owed, and the payer then withdrew the remainder.
   - The operator claim fails too. Of the 73 "operator" channels, **38 have operator == payee** (no delegation at all), and **27 had an in-grace settle**.
   - Across all 676 operator channels, 302 are self-assigned (operator == payee).
   - Provable exposure: forced withdrawals on channels with provable usage (at least one `Settled`) and **no** in-grace response come to **29 channels, 5 payees, $36.00 refunded**. That is an upper bound, since refunds include unconsumed deposit.
   - Refund sizes: median **$0.50**, p90 $25, max $25. 275 of 343 had $0 ever settled. Wallet `0xa1024faf…` is simultaneously a top forcing payer (55), an affected payee (37) and an operator, which looks like someone testing both sides.

---

## TOTAL: 72 / 100 (pass = 90). Closer, but not shortlisted yet.

In one sentence: v2 fixes almost every technical error from round 1 and replaces guesses with real code and chain evidence, a big jump in rigor. But its new headline number ("343 forced closes, payees responded once") is contradicted by the deck's own dataset. The "why not settle often?" rebuttal attacks a strawman at a gas price 10x today's. A Tempo judge who reruns the published scripts (the deck invites exactly that) will find both in about 10 minutes.

## A. Official contest criteria: 35 / 48

| # | Criterion | Score | Justification |
|---|---|---|---|
| A1 | Functionality | 6/8 | Feasibility is now proven: the precompile is live on testnet and mainnet with a 900 s grace, and the design is specific (`close(capture=max(spent, settledOnChain))`, event plus `getChannelStatesBatch` fallback, per-channel state machine, separate operator key). The split-screen demo with a real tx is right. It still has zero code-quality signal (no repo link, tests or CI), and the demo is a single happy path in time-lapse. |
| A2 | Potential Impact | 5/8 | Real bottom-up numbers replace the stale server count, and an ecosystem argument now exists. But the argument has a logic slip: payee exposure is delivered minus settled, not deposit size, so forced-close risk caps *credit extended between settles*, not "how much agents may deposit". The on-slide loss proxy is also overstated. The honest data (about $4.2k lifetime deposits, non-dominant activity falling since July) makes the market story weak. |
| A3 | Novelty | 6/8 | The gap is now verified at code level (neither SDK watches), temprano is distinguished, and the Lightning threat model is correct ("liveness insurance"). But the operator slot that enables the product also makes it trivially replicable: the real competitor is a 100-line DIY script holding an operator key, and it is missing from the table. The chain also shows payee-side keys already settling inside the grace window within 1-83 s. |
| A4 | UX | 6/8 | "You never hand over your main key" is a genuine protocol-native UX story, and "one-line integration" is the right promise. Missing: the operator is hashed into the channelId, so only *new* channels are protected and switching providers changes channelIds. There is no mention of what the payee sees (alerts are on the roadmap) or of the added latency from forward-before-serve. |
| A5 | Open-source / composability | 7/8 | Concrete and correct: `methodDetails.operator`, precompile events, `getChannelStatesBatch`, mppx/mpp-rs middleware, MIT/Apache, Rust. It loses a point for not addressing payees who already use the single operator slot as their own settlement sender (mppx `assertSettlementSender`), or Tempo access keys as an alternative delegation route. |
| A6 | Business Plan | 5/8 | The operator model unblocks the hosted tier, which was the main round-1 blocker. There is still no revenue math. A "$29/mo per payee" test price sits against a market whose *lifetime* deposits are about $4.2k, and against a threshold-settle alternative that costs a fraction of a cent. No customer-level ROI is shown. |

## B. Colosseum internal dimensions: 18 / 28

| Dimension | Score | Justification |
|---|---|---|
| Founder-market fit | 2/4 | With modest filled facts, the background still didn't drive the idea. The contract, SDK and chain work shows diligence; I credit that under Insight and Execution, not fit. |
| Insight | 3/4 | "Neither SDK watches close requests; auto-settle is opt-in and request-driven; the v2 operator slot enables a keyless tower" is non-obvious and code-verified. Docked for building the headline on a mis-measured "responded once", and for ignoring that amount-threshold settling already bounds exposure to the sub-threshold tail. |
| Product + execution | 3/4 | Feasibility is confirmed on both networks and the design choices are the ones a Tempo engineer would make. Nothing is built yet, and there is no build-progress signal (repo, commits, weekly update). |
| Market size | 2/4 | Honest: 70 payees, 1,718 payers, 99.3% of channels with one payee. But it is small and shrinking outside the dominant payee, and the growth mechanism is inferred, not shown. |
| Founder communication | 4/4 | 11 slides, one idea each, about 434 spoken words, speakable pacing, and a crisp closer. The round-1 deductions (overrun, unspeakable timings) are fixed. |
| Viability | 2/4 | The rebuttal to "just settle often" rests on $2.60/day at 1/min, which is a strawman. mppx already supports `SettlementSchedule.amount`, which caps exposure at $X per channel for about fee/$X of revenue. At today's live gas price that is $0.00018 per settle. The payee's residual risk is small, so willingness to pay $29/mo is unargued. |
| Traction | 2/4 | Targeted outreach to chain-identified payees (15 contacted, 2-3 replies, 1 quote) is smart, but a reply is not a user or design partner. The chain "validation" is weaker than claimed (see Honesty). |

## C. Pitch craft: 12 / 16

| Item | Score | Justification |
|---|---|---|
| Winner patterns | 3/4 | The credential is in the first 10 s, the demo says "nothing faked", there is a honest-market slide, full-time and roadmap, and a closer that restates the one-liner. Still missing C6: no dollar contrast of loss vs. price on screen ("Pay a few dollars, not your unsettled revenue" is unquantified). |
| Hook | 3/4 | A concrete on-chain stat arrives by 0:25, which is much better than v1's slogan. But there is still no named user and no dollar amount, and the stat itself is wrong as phrased. |
| Narrative flow | 4/4 | Problem, why it happens, why me, how, demo, validation, alternatives, business, market, close: each slide earns the next. The demo gets 32 s. Slide 7 is the only tight spot (see script check). |
| Honesty | 2/4 | Placeholders are clean and the outreach instructions are admirably literal. But the three headline data claims are overstated or false against the deck's own reproducible dataset: "responded once", "lost channels" and "operator who never acted". The fee figure is a point estimate at 10x today's live gas price. Appendix D marks these "COMPUTED" as if settled. |

## D. Build-readiness for an AI deck generator: 7 / 8

| Item | Score | Justification |
|---|---|---|
| Per-slide completeness | 3/4 | Every slide has a title, on-slide text, a visual and a script. Slide 8 (a 4-row table plus footnote, about 45 words) and slide 5 (about 30 words) break the deck's own ≤20-word on-slide rule. |
| Self-contained | 4/4 | There is a clean `SLIDES` / `END OF SLIDES` split, guidance is moved out, `[[INSERT RECORDING]]` is marked, and the style brief is usable as is. |

---

## Delta vs. round 1

| Item | R1 | R2 | Δ | Driver |
|---|---|---|---|---|
| A1 Functionality | 5 | 6 | +1 | Feasibility verified; still no code signal |
| A2 Potential Impact | 4 | 5 | +1 | Real numbers plus an ecosystem argument; logic slip; shrinking data |
| A3 Novelty | 5 | 6 | +1 | Code-level gap verified; DIY-operator-script competitor omitted |
| A4 UX | 5 | 6 | +1 | No-main-key story; new-channels-only and latency unaddressed |
| A5 Composability | 5 | 7 | +2 | Concrete integration points and license |
| A6 Business Plan | 4 | 5 | +1 | Hosted tier unblocked; no revenue math or ROI |
| Founder-market fit | 2 | 2 | 0 | Unchanged |
| Insight | 2 | 3 | +1 | SDK and operator finding is real |
| Product + execution | 2 | 3 | +1 | Feasibility confirmed |
| Market size | 2 | 2 | 0 | Honest but tiny and shrinking |
| Founder communication | 3 | 4 | +1 | Within budget, speakable |
| Viability | 2 | 2 | 0 | Strawman rebuttal at inflated gas price |
| Traction | 2 | 2 | 0 | Replies, not users |
| Winner patterns | 3 | 3 | 0 | Still no $ contrast (C6) |
| Hook | 2 | 3 | +1 | Concrete stat early; no named user or $ |
| Narrative flow | 3 | 4 | +1 | Pacing fixed |
| Honesty | 3 | 2 | −1 | Headline stat contradicted by own data |
| Per-slide completeness | 3 | 3 | 0 | Slide 8 and 5 over word limit |
| Self-contained | 3 | 4 | +1 | Clean generator boundary |
| **Total** | **60** | **72** | **+12** | |

---

## Top 5 highest-leverage fixes (ordered)

1. **Recompute and re-word the on-chain headline. It is the single biggest liability.**
   - Count *any* payee or operator `settle`/`close` between `CloseRequested` and `withdraw` as a response.
   - Split out operator == payee.
   - Honest numbers from the same logs:
     - 408 close requests.
     - Of 344 resolved, **305 got no payee-side on-chain response** (1 close plus 38 in-grace settles did respond).
     - On provably-used channels with no response: **29 withdrawals, 5 payees, ≤ $36 refunded**.
   - Replace "Payees responded once" with something like "305 of 344 close requests: the payee never answered". Replace "15 payees lost channels" with "left close requests unanswered".
   - Delete the "73 operator … never acted" line; it is false for 27 channels, and 38 aren't delegated at all.
   - Then use the outreach replies to turn the upper bound into a real figure ("payee X confirms $Y of earned revenue went back").
2. **Rebuild slide 8 around the real alternatives, with honest fee math.**
   - Use the gas range (floor 6e8, which is today's live value: $0.00018/settle, about $0.26/day at 1/min; cap 12e9: $0.0036/settle).
   - Name the real alternatives: mppx `SettlementSchedule.amount` (exposure ≤ threshold, cost ≈ fee/threshold of revenue) and a **DIY operator-key script**.
   - Win on what those can't do: the default config has no schedule, and the tail below the threshold is exposed. A crash between accepting and settling loses it. A script on the same box dies with it. Nobody wants to own on-call for a 15-minute deadline.
   - Also say that the tower lets payees settle *less* often, which saves fees.
3. **Fix the operator trust claim.**
   - "Can't redirect funds" is literally true but misleading. The operator picks `captureAmount` within `[settled, voucher]` and can close at any time. A compromised or colluding operator can hand all unsettled revenue back to the payer, or over-capture against the payer.
   - State the mitigations: per-payee operator keys, capture bounded by forwarded `spent`, key custody, and payee-side alerts.
   - Also state the constraints: the operator is bound at channel open, so it protects new channels only and a provider switch changes channelIds.
4. **Put a named user and a dollar contrast in the first 20 s, and fix the impact logic.**
   - Use the demo channel as the example, e.g. "a $25 channel, a region outage, $X earned goes back to the agent" (the max refund on mainnet is exactly $25).
   - Pair it with "vs. $Y/month" on slide 9 (C6).
   - Change "cap how much agents may deposit" to "force payees to settle constantly or cap the credit they extend".
5. **Give the business a customer-level ROI and a build signal.**
   - The only payee where risk is material is the dominant one (266,750 channels, 213 forced withdrawals, about $1,643 refunded). Name it (anonymized) as the design-partner target.
   - Show "N payees × $Y" and why a payee pays $29 rather than setting a threshold.
   - Add a public repo link, a CI/test badge and a testnet run log with a caught close, plus weekly update videos.

---

## Untraceable or overstated claims

1. **"Payees responded once"** (slide 2 on-slide, visual caption "1 payee response", and script). **FALSE as phrased.** 38 in-grace `Settled` events on forced channels (25 sent by the payee, 13 by operator/other; median 83 s after the request) plus 1 close gives at least 39 responses. The evidence script (`forced.py`) only classifies the closing tx.
2. **"73 of those channels had an operator — who never acted"** (slide 7) and evidence §6 "delegation alone didn't protect them". **FALSE.** 38 of the 73 have operator == payee, and 27 had an in-grace settle.
3. **"15 payees already lost channels to forced withdrawals"** (slide 7, spoken). **OVERSTATED.** On-chain data cannot show `spent`. 275 of 343 had $0 ever settled, the median refund is $0.50, and much of it is plausibly unused deposit correctly returned. Provable used-and-unanswered: 29 withdrawals, 5 payees, ≤ $36. At least one "payee" (`0xa1024faf…`) is also a heavy forcing payer and operator (self-testing).
4. **"~$0.0018 per settle at today's Tempo gas price" / "$2.60/day per channel"** (slide 8, spoken). The arithmetic is right for 5.9125e9 (301,831 × 5.9125e9 = $0.00178; × 1440 = $2.57). But **today's live mainnet gas price is 6.0e8**, which gives $0.00018 and $0.26/day. Present it as a range, not "today's".
5. **"Auto-settle … runs only while requests flow → go quiet, and the clock wins"** (slide 3). The fact is verified, but the implication is overstated. With an amount-based schedule, unsettled balance only grows on the request path, so going quiet leaves only the sub-threshold tail exposed.
6. **"Operators can't redirect funds"** (slide 5) and **"worst a rogue operator can do is close a channel early"** (Appendix C, evidence §2). **INCOMPLETE:** the operator also chooses the capture amount.
7. **"Payees who can't trust a forced close cap how much agents may deposit"** (slide 10). This is inference, not in the evidence, and mechanically mis-stated (see A2).
8. **"Payees already use the delegation slot"** (evidence §6, 39 operators). 302 of 676 operator channels are self-assigned.
9. **"Forced withdraw is the normal ending"** (evidence §6 interpretation). It is the normal ending only *after a close request*: 343 of 1,157 closed channels, and 408 requests across 268,596 channels.
10. **Tagline "Settles your Tempo payment channels"** vs. mechanism "close, don't settle". A minor inconsistency, but a Tempo judge notices it.

These traced correctly (spot-checked or reproduced):
- 900 s grace on both chains;
- the operator can settle/close; payer-only requestClose/withdraw; payouts only to payee/payer;
- no SDK watcher; opt-in, request-driven schedule;
- gas snapshot figures and attodollar units;
- 343/1/15/68/$129.59/73 as *counts*;
- 268,596 channels, 70 payees, 1,718 payers, about $4.2k deposits;
- first event 2026-06-10;
- `getChannelStatesBatch` exists in the contract;
- temprano and Fiber positioning.

## Technical errors / things a Tempo engineer would catch

- **T1 (high) Response-detection bug in the analysis.** A payee's valid answer to `requestClose` is `settle` (claim earned, let the payer withdraw the rest), not only `close`. That is exactly what 38 channels show. The product's own detection and metrics must treat it that way too.
- **T2 (high) Operator trust model is overstated.** `close` lets the operator choose `captureAmount ∈ [settled, cumulativeAmount]`. The tower is trusted *by both sides* on amount: under-capture harms the payee, and over-capture above `spent` harms the payer. A multi-tenant hosted operator key is therefore a high-value griefing and collusion target. Per-payee keys are required.
- **T3 (medium) Deadline semantics.** `close` has no `closeRequestedAt` check. The real deadline is "before the payer's `withdraw` is mined", not `closeGraceEnd`. On mainnet the median withdraw lands 216 s after grace end. The tower should keep racing after `closeGraceEnd` until `ChannelClosed`. The deck's "read `closeGraceEnd`, never assume 15:00" is necessary but not sufficient.
- **T4 (medium) Wrong alternative in the killer objection.** The comparison should be against `SettlementSchedule.amount` / `units` (bounded exposure, fee proportional to revenue), not per-minute settling per channel, and at the live gas price.
- **T5 (medium) Forward-before-serve cost.** A synchronous cross-region write per metered unit adds an RTT to every SSE chunk or request. The real design is "forward vouchers synchronously, stream `spent` with bounded lag", and it should state the exposure bound that lag creates. If the tower captures from a lagging `spent`, it under-captures. If it captures from the voucher amount, it violates §13.2.
- **T6 (medium) Adoption constraints of the operator slot.** It is bound into the channelId at open, so there is no retrofit for live channels and a provider change means new channelIds. There is one operator per channel, so payees who use the operator as their own hot settlement key (supported by mppx `assertSettlementSender`) must choose. Tempo access keys are an alternative delegation path the deck ignores.
- **T7 (low) Impact mechanism mis-stated.** Exposure is delivered minus settled, independent of deposit (see A2).
- **T8 (low) The 1-in-99% dominance.** 99.3% of channels and 213 of 343 forced withdrawals belong to one payee, which *does* settle heavily (218k `Settled` events) and settled 11 forced channels inside the grace window. The "payees aren't watching" story needs to explain this payee specifically; it is also the obvious first customer.

Everything round 1 flagged as T1-T9 is otherwise fixed: v2 operator, `spent`/capture, §14.12 framing, failure modes, consistency model, the Lightning threat model, the "why won't Tempo" logic, the grace constant and testnet feasibility.

## Script length check

| Slide | Window | Words (fixed + est. placeholder) | wpm |
|---|---|---|---|
| 1 | 0:00-0:09 (9 s) | 15 + ~10 | ~167 |
| 2 | 0:09-0:29 (20 s) | 51 | 153 |
| 3 | 0:29-0:44 (15 s) | 39 | 156 |
| 4 | 0:44-0:54 (10 s) | ~25 | 150 |
| 5 | 0:54-1:12 (18 s) | 47 | 157 |
| 6 | 1:12-1:44 (32 s) | 62 | 116 |
| 7 | 1:44-2:00 (16 s) | 23 + ~25 | **~180** |
| 8 | 2:00-2:18 (18 s) | 47 | 157 |
| 9 | 2:18-2:28 (10 s) | 27 | 162 |
| 10 | 2:28-2:45 (17 s) | 44 | 155 |
| 11 | 2:45-2:53 (8 s) | 19 | 142 |
| **Total** | 2:53 | **~374 fixed + ~60 = ~434** | ~150 |

The script fits: under 450 words, under 3:00, with 7 s of slack. Only slide 7 is too fast once the outreach line and quote are spoken. Keep the quote to 12 words or less, or take 3 s from slide 6's generous 116 wpm.

## What would get it to 90+

- **Fixes 1-3** restore Honesty (+2), Viability (+1), Insight (+1), Novelty (+1) and Business (+1), which gets to about 78.
- **Fixes 4-5** add Hook, Winner patterns, Impact and Product (about +4), which gets to about 82.
- The last 8 points need founder-only facts the placeholders don't assume:
  - **One affected payee, ideally the dominant one, confirming on record that a forced close cost them real earned revenue and agreeing to run the tower as operator on new channels.**
  - **A public repo in which the tower caught a real `CloseRequested` on testnet (or mainnet) and landed the `close`.**
- With those, Traction, Founder-market fit, Viability and Functionality each move up a notch and the deck clears 90.
