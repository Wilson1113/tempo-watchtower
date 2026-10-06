# Judge round 4: tempo-watchtower pitch deck (v4)

Judge persona: Colosseum partner plus a Tempo-track judge who knows payment channels. Date: 2026-09-27.
Deck reviewed: `research/phase4-pitch/tempo-watchtower-pitch-deck.md` (v4, 11 slides). The main change is that slides 2, 7, 8 and 10 now tell the story of the dominant payee building its own close-request responder on 2026-08-22.
Calibrated against rounds 1-3 (60, 72, 78) with the same standard. Rounds 2 and 3 said deck-only fixes cap in the low 80s and that 90+ needs founder-only facts. I apply that here.

Placeholder assumption (same as round 3):
- Modest solo background.
- About 14 payees contacted, 2-3 replies, one usable quote.
- A real testnet tx hash, with the `[[DEMO: N/M]]` latency numbers filled from a real run.
- The repo link is a real public repo with a working MVP.
- NOT assumed: any payee agreeing to run the tower, or a caught mainnet close.

## What I verified myself (Bash only)

All scripts were re-run against the cached `/tmp/tw/logs.json` (471 MB). `onchain/agg2.py`, `delay.py`, `dom.py` and `forced_close_rows.json` are byte-identical to the `/tmp/tw` copies. My own checks are in `/tmp/tw/j4.py`, `j5.py` and `j6.py`.

1. **The dominant-payee timeline: REPRODUCED.** `dom.py` gives 213 resolved requests.
   - Before 2026-08-22 14:14:58 UTC, **0 of 194** were answered.
   - From that date on, **11 of 19** were answered.
   - The 8 unanswered since then all had a prior settle. Their refunds were $0.000, 0.000, 0.000, 0.001, 0.001, 0.002, 0.006 and 0.489. So "~$0 to claim, $0.00-$0.49" is accurate.
2. **44 × $25 refunded with $0 settled, all before Aug 22: REPRODUCED, but with a finding the deck omits.**
   - The 44 channels were close-requested between 2026-06-12 and 2026-07-11, and none had a prior settle.
   - **All 44 were opened by a single payer, `0xa1024faf…`.** That is the wallet round 2 flagged as self-testing. It is also a payee (123 channels), an operator, the top forcing payer, and a responder to its own channels (13 answered since 2026-07-26).
   - Of the dominant payee's $1,641.57 in unanswered refunds, **$1,626 (99%) came from two payers that are themselves MPP payees**: `0xa1024faf…` ($1,136; 47 requests) and `0x516a449f…` ($490; 50 requests).
   - Before Aug 22, only 3 of the 194 unanswered channels had any prior settle, carrying $0.05 of refunds. Meanwhile the payee settled 166,654 times over the same period.
   - The likely reading: channels that got used were already being settled, and the unanswered ones were mostly never-used test channels.
3. **What the leader's responder actually does: COMPUTED.**
   - Delays from request to settle, in seconds: 12, 71, 118, 208, 250, 252, 272, 277, 288, 298, 298. The **median is 252 s** and the maximum is under 300 s. That pattern looks like a ~5-minute poll or sweep; an event watcher would respond faster.
   - All 11 answered channels had zero prior settles.
   - Settled amounts across the 11 add up to about **$0.12**.
   - No operator is set on any of its channels; the payee key itself responds.
4. **"The 13 other affected payees haven't built one" (Appendix C): FALSE.**
   - `0xa1024faf…` has answered close requests since 07-26 (13 answered).
   - `0xfbb2b310…` has **answered since 2026-08-19 20:03 UTC, 2.7 days before the leader**: 13 answered, and its last unanswered request was 3 hours before its first answer.
   - `0x516a449f…` answered its one request after its last unanswered one.
   - The remaining 10 payees have **$20.35 of total unanswered refunds, and $6.49 provably used** ($5.99 for `0xd7724ae1…`, $0.50 for `0x59fed89d…`). Only those two were active in September.
5. **Chain-wide figures still hold (`agg2.py`):** 304 / 39 / 1 / 64; 14 payees, 202 of them the dominant one; 29 channels / 5 payees / $36.00; median $0.499; operators 676 / 302 / 374 / 37. `delay.py` still gives 39 responses with a median of 83 s.
6. **SDK gap and gas: RE-VERIFIED.**
   - mppx HEAD is still `dcf1589` (25 Sep). The only server-side hit is the `previousCloseRequestedAt` field in `CredentialVerification.ts:1005`, which is state bookkeeping, not an event subscription.
   - Live `eth_gasPrice` = `0x23c34600` (6e8).
   - An open question I could not settle: mppx shipped `c004d2c` "support hosted fee payer for session settlement" on **2026-08-18**. That is one day before `0xfbb2b310…` began answering and four days before the leader. The dominant payee settled 166k times before Aug 22, so fees were not blocking it. Still, two independent onsets three days apart suggest a shared cause, and the founder should rule this out before the interview.

---

## TOTAL: 79 / 100 (pass = 90). Plateaued; not shortlisted on the deck alone.

In one sentence: v4 fixes round 3's economics error and the absolutes, and the "leader built its own" story is the sharpest framing so far. But the hook's dollar figure comes entirely from one self-testing wallet. The "13 other payees" customer list includes three that already answer close requests. The deck states "built its own watcher" and "needed 10 weeks" as facts when both are inferences. So the Honesty point that round 3 put within reach was spent on new selective framing.

## A. Official contest criteria: 38 / 48

| # | Criterion | Score | Justification |
|---|---|---|---|
| A1 | Functionality | 7/8 | On-screen detect and close latency (assumed real), plus a planned failure-path demo (WebSocket killed and tower restarted mid-grace), address round 3's gaps on paper. There is still no CI or test signal, and the failure tests are a plan, not evidence. The reproducible caught-close run log is founder fact (b). |
| A2 | Potential Impact | 5/8 | Honest but small, and the new data makes it smaller: the leader's own responder has recovered about $0.12, and the non-leader pool has $6.49 of provable exposure. "Every payee that grows will need this" is the right growth argument, but it sits only in Appendix C. |
| A3 | Novelty | 6/8 | The positioning "sell the leader's solution to everyone who isn't the leader" beats round 3's one-liner. But the chain shows at least three payees built responders independently within days (Aug 19, Aug 22, plus the self-tester), and the leader's is a ~5-minute poll. So DIY is the norm, and it is cheap. |
| A4 | UX | 7/8 | "Trusted on amount only · one key per payee" on slide 5 is good. There is still no view of what the payee sees when the tower fires. |
| A5 | Open-source / composability | 8/8 | This round's one real gain. Appendix A now handles the single operator slot (the settlement-key role moves to the tower) and names Tempo access keys as the alternative delegation route, next to the already-correct `methodDetails.operator`, events, `getChannelStatesBatch` and SDK middleware. That was round 3's only deduction. |
| A6 | Business Plan | 5/8 | "Pay a flat fee instead of settling" is gone, and "open source → free for 10 → priced on protected volume" is coherent. That was round 3's recommended fallback. But there is still no revenue math or customer-level ROI, and the named first customers are partly wrong (see verification 4). The real remaining pool is 10 payees with $20 of lifetime unanswered refunds. Removing an error is not the same as adding a plan. |

## B. Colosseum internal dimensions: 20 / 28

| Dimension | Score | Justification |
|---|---|---|
| Founder-market fit | 2/4 | Unchanged under modest facts. |
| Insight | 4/4 | The SDK and operator insight stands. Spotting the leader's behaviour change from logs is a genuinely non-obvious read, even though its wording overreaches (see Honesty). |
| Product + execution | 4/4 | Credited under the working-repo assumption. |
| Market size | 2/4 | 70 payees, one with 99.3% of channels, and that one now self-served. The deck says this honestly. |
| Founder communication | 4/4 | Still crisp. Minor issue: the leader story appears on four slides (2, 7, 8, 10). Slide 7 runs about 177 wpm once the quote is filled. |
| Viability | 2/4 | The fee strawman is gone, but willingness to pay is still unargued, which remains this score's core reason. The data now shows the only payee with volume solved it itself, for about $0.12 of recovered value. |
| Traction | 2/4 | Replies, not users (as assumed). |

## C. Pitch craft: 13 / 16

| Item | Score | Justification |
|---|---|---|
| Winner patterns | 3/4 | There is still no dollar contrast between loss and price (C6): the price is "free, then volume", and the loss is unconfirmed. The user is anonymized (C7). |
| Hook | 3/4 | It is better: there is a protagonist ("Tempo's busiest session API") and a dollar figure by about 0:28. But the dollar figure is one self-testing wallet's channels, and the protagonist has since fixed its problem. A judge who groups by payer deflates the hook in one line. |
| Narrative flow | 4/4 | Problem → why → me → how → demo → validation → alternatives → business → market → close still earns each step. |
| Honesty | 3/4 | Round 3's fix 3 was done: "never lose" and "exactly" are removed, slide 10 is reframed, and the trust assumption is on-slide. But new selective framing replaced them. The 44 × $25 figure leaves out that it is a single tester payer. "Aug 22: it built its own watcher" is an inference stated as fact. "The leader needed 10 weeks" conflates time-to-care with time-to-build. "13 other payees haven't built one" is false. The numbers are all true; the framing is not. |

## D. Build-readiness for an AI deck generator: 8 / 8

| Item | Score | Justification |
|---|---|---|
| Per-slide completeness | 4/4 | Every slide has a title, on-slide text, a visual and a script. Slides 5 and 8 are about 23-24 words, a marginal overrun of the 20-word rule. |
| Self-contained | 4/4 | Clean SLIDES boundary. The round 3 dangling word-count line is gone. |

---

## Delta vs round 3

| Item | R3 | R4 | Δ | Driver |
|---|---|---|---|---|
| A1 Functionality | 7 | 7 | 0 | Latency on-slide and failure tests planned; no CI; run log is fact (b) |
| A2 Potential Impact | 5 | 5 | 0 | Still small; leader's recovered value is about $0.12 |
| A3 Novelty | 6 | 6 | 0 | Better rebuttal; chain shows DIY is common and cheap |
| A4 UX | 7 | 7 | 0 | Trust line added; no payee-facing view |
| A5 Composability | 7 | 8 | +1 | Operator-slot conflict and access keys addressed |
| A6 Business Plan | 5 | 5 | 0 | Economics error removed; customer list partly wrong; no ROI |
| Founder-market fit | 2 | 2 | 0 | — |
| Insight | 4 | 4 | 0 | — |
| Product + execution | 4 | 4 | 0 | — |
| Market size | 2 | 2 | 0 | — |
| Founder communication | 4 | 4 | 0 | Repetition; slide 7 pace |
| Viability | 2 | 2 | 0 | Willingness to pay still unargued |
| Traction | 2 | 2 | 0 | — |
| Winner patterns | 3 | 3 | 0 | No C6 contrast; anonymized user |
| Hook | 3 | 3 | 0 | Protagonist added; dollar figure is test-wallet traffic |
| Narrative flow | 4 | 4 | 0 | — |
| Honesty | 3 | 3 | 0 | Old absolutes out, new selective framing in |
| Per-slide completeness | 4 | 4 | 0 | — |
| Self-contained | 4 | 4 | 0 | — |
| **Total** | **78** | **79** | **+1** | |

A note on judge consistency: round 3 recommended the 44 × $25 framing without checking payer concentration. That miss is mine as much as the deck's. The standard is still what a Tempo judge re-running the published data would find, and the payer grouping takes one line.

---

## Top 5 remaining fixes

1. **Fix the hook's provenance.** All 44 $25 channels came from one payer (`0xa1024faf…`) that is itself a payee, an operator and a self-tester. 99% of the leader's unanswered refunds came from two payer wallets that are themselves payees.
   - Either disclose it ("mostly from two developer wallets") or drop the dollar figure from slide 2.
   - Lead with the behavioural fact instead: "194 close requests unanswered for 10 weeks, then it started answering."
2. **Correct the customer list and the DIY story.**
   - Replace "the 13 other payees" with the truth: in August, at least three payees began answering close requests on their own (`0xfbb2b310…` on Aug 19, the leader on Aug 22, `0x516a449f…`), plus the self-tester.
   - Ten payees still don't answer. Their total unanswered refunds are $20 and only two were active in September.
   - Frame DIY as validation: payees who care all end up building one, and a ~5-minute poll is what they build.
   - Name first customers as *new* session servers plus the two active non-responders.
   - Delete Appendix C's "haven't built one".
3. **State the inferences as inferences, and sell reliability, not seconds.**
   - "Aug 22: began answering close requests (median 252 s)" rather than "built its own watcher".
   - Drop "the leader needed 10 weeks".
   - Fix the founder note: 83 s is the chain-wide median; the leader's median is 252 s.
   - Against a 15-minute grace (and `close` has no grace check), a 4-minute poll is already fast enough. The tower's edge is independence from the payee's crashing stack and not owning the on-call, not latency.
4. **Say out loud why anyone pays.**
   - Admit the recovered dollars are cents today (the leader's responder has claimed about $0.12).
   - Make the argument operational: correctness, on-call and the tail during crashes.
   - Keep the free tier, and let founder fact (a) supply the first real dollar figure. Without it, Viability and A6 cannot move.
5. **Housekeeping.**
   - Trim slide 7 so it stays at or under 160 wpm with the quote.
   - Add a script for the 44 × $25 figure and the per-payer breakdown to `onchain/`. Appendix D cites a "verification transcript".
   - Strike evidence §2's "worst a rogue operator can do is close a channel early". This is carried over from round 3 and still unfixed.
   - Check whether mppx `c004d2c` (Aug 18) explains the Aug 19 and Aug 22 onsets.

## Overstated or untraceable claims

1. **Slide 2 "44 × $25 refunded, $0 claimed"** beside the on-slide line "unclaimed revenue returns". The count is true, but all 44 are from one self-testing payer. Calling it "revenue" is unsupported (only 3 of the 194 had any settle).
2. **Slide 7 "Aug 22: it built its own watcher"** and slide 10's "proved the need by building its own". These are inferences. The data shows it began settling close-requested zero-settle channels 12-298 s after the request. An event watcher and a ~5-minute unsettled-balance sweep cannot be told apart on-chain.
3. **Slide 8 "DIY watcher: works — the leader needed 10 weeks"** and Appendix C "it took ten weeks … to get there". Ten weeks is the time before it started answering, not how long it took to build. 185 of the 194 requests came before July 15.
4. **Slide 7 script "answers whenever there's money to claim".** This is mostly supported (the 8 misses had ≤$0.49 refundable, 7 of them ≤$0.006), but it is still an inference.
5. **Slide 10 "First customers: the 13 other payees left unanswered"** is misleading, and **Appendix C "The 13 other affected payees haven't built one"** is FALSE: at least 3 of the 13 answer close requests.
6. **Founder notes "the leader's DIY median of 83 s"** is wrong. 83 s is the median across all responders; the leader's is 252 s.
7. **Appendix D, 44 × $25 "`rows.json` check (verification transcript)"**: there is no script in `onchain/`. It is reproducible, but not as published. `dom.py` also reads `/tmp/tw/` paths, not `onchain/`.
8. **Evidence §2 "worst a rogue operator can do is close a channel early"**: still not struck through.

These traced correctly:
- 0/194 before Aug 22, 11/19 after, and the 8 misses at ~$0.
- 44 × $25 at $0 settled, all before Aug 22.
- 99% of channels; 304/344; 14 payees; 202; ≤$36.
- 83 s median chain-wide.
- 70 / 1,718 / 268k.
- No SDK watcher; auto-settle off by default.
- $0.00018 per settle at 6e8.
- Operator semantics.

## Technical errors / things a Tempo engineer would catch

- **T1 (medium) Latency is not the differentiator.** The grace is 900 s, and `close` stays valid until the payer's `withdraw` is mined. The leader's under-5-minute DIY responder already meets it with 3x margin. "`close()` in seconds" (slide 5) is a demo nicety, not a moat. The moat is uptime independent of the payee's stack.
- **T2 (medium) Watcher vs sweep.** The leader's response distribution (12-298 s, clustered just under 300 s) is consistent with a periodic poll or a "settle unsettled balances" sweep. If it is a sweep, the leader's fix is the "settle often" alternative that slide 8 calls off by default and tail-leaving. The deck's story depends on which it is, and only the payee knows.
- **T3 (medium) The analytics don't filter test traffic.** Payers that are also payees or operators (`0xa1024faf…`, `0x516a449f…`) dominate the dollar figures. The deck's metrics, and the product's own "saves" KPI, should exclude self-dealing and test wallets alongside the claimable/not-claimable split in Appendix A.
- **T4 (low) Possible common cause.** Payees started answering on Aug 19 and Aug 22, days after mppx `c004d2c` (Aug 18, hosted fee payer for session settlement). This is unverified, but if a shared tool change explains it, "no SDK does this" needs re-checking in whatever SDK version those payees run.
- **T5 (low) Access keys may be the retrofit path.** If a Tempo access key can be scoped to the precompile's `settle`/`close` on the payee account, it could protect *existing* channels, which the operator slot cannot. "To evaluate" leaves the best UX answer unexplored.

Round 3's T3 (the settle-rarely risk) is fixed by "threshold backstop + watchtower". T5 (`topUp` then a fresh request) is fixed in Appendix A.

## Script length check

| Slide | Window | Words (fixed + est. placeholder) | wpm |
|---|---|---|---|
| 1 | 9 s | 16 + ~8 | ~160 |
| 2 | 21 s | 49 | 140 |
| 3 | 13 s | 33 | 152 |
| 4 | 10 s | ~25 | ~150 |
| 5 | 18 s | 49 | 163 |
| 6 | 28 s | 48 + ~2 | 107 |
| 7 | 18 s | 34 + ~19 | **~177** |
| 8 | 20 s | 51 | 153 |
| 9 | 13 s | 34 | 157 |
| 10 | 16 s | 40 | 150 |
| 11 | 9 s | 19 | 127 |
| **Total** | 2:55 | **~374 + ~54 ≈ 428** | ~147 |

It fits, but slide 7 is too fast. Take 3 s from slide 6.

---

## Projection WITH founder facts (a) + (b)

- **(a)** One affected payee confirms on record that a forced close cost real earned revenue, and agrees to run the tower as operator on new channels.
- **(b)** A repo run log showing a caught `CloseRequested` and a landed `close`.

This assumes both are integrated (named user in the hook, confirmed dollars on slides 2, 7 and 9, run-log numbers on slide 6) and v4's wording is otherwise unchanged.

| Item | R4 now | With (a)+(b) | Why |
|---|---|---|---|
| A1 Functionality | 7 | 8 | A reproducible caught close on top of the MVP |
| A2 Potential Impact | 5 | 6 | A confirmed loss replaces an upper bound (only 5 if it is ≤$6) |
| A3 Novelty | 6 | 7 | A payee choosing the tower over DIY |
| A4 UX | 7 | 7 | — |
| A5 Composability | 8 | 8 | — |
| A6 Business Plan | 5 | 6 | A committed design partner executes step 2 of the plan; 7 needs a price or paid pilot |
| Founder-market fit | 2 | 3 | Winning a payee from chain forensics |
| Insight | 4 | 4 | — |
| Product + execution | 4 | 4 | — |
| Market size | 2 | 2 | — |
| Founder communication | 4 | 4 | — |
| Viability | 2 | 3 | First willingness-to-pay evidence |
| Traction | 2 | 3 | A real user on new channels; 4 only if it is the dominant payee, live on mainnet |
| Winner patterns | 3 | 4 | Named user + confirmed dollars (C6/C7); it backfires if the loss is a few dollars |
| Hook | 3 | 4 | A named payee and real dollars replace the test-wallet figure |
| Narrative flow | 4 | 4 | — |
| Honesty | 3 | 3 | Founder facts don't fix the framing in fixes 1-3 |
| Per-slide completeness | 4 | 4 | — |
| Self-contained | 4 | 4 | — |
| **Total** | **79** | **88** | |

Reaching 90 needs two more points:
- **+1 Honesty (deck-only, fixes 1-3).** That gives 89.
- **Then either:**
  - +1 Traction (the confirming payee is the dominant one, live on mainnet), or
  - +1 Business Plan (a price or paid pilot).

**Realism warning (stronger than round 3's).**
- The dominant payee now runs its own responder. Its post-Aug 22 misses had ≤$0.49 to claim, and its "$1,100" is one tester wallet's channels. A confirmation of a large loss from it is structurally unlikely.
- Every payee that doesn't answer has ≤$6 of provable exposure.
- The likely founder-fact outcome is a small confirmed loss, which lands at **about 85-88**, not 90.
- The strongest available (a) is now **a non-responding payee agreeing to run the tower** (adoption, not loss), or a **responder like `0xfbb2b310…` agreeing to replace its DIY poll**. The second is the direct proof against DIY.

## Can deck-only edits still raise the score?

**Barely. Deck-only work has plateaued.** At most **+2, to about 81**:
- +1 Honesty from fixes 1-3.
- A possible +1 on A6 if the customer list is corrected *and* a credible new-server acquisition path is shown.

Every other open point (A1, A2, A3, Viability, Traction, Hook, Winner patterns, founder-market fit) is now blocked on founder facts, not wording. Further rewording of the dominant-payee story adds risk, not points: each reframe so far has surfaced a new data caveat.
