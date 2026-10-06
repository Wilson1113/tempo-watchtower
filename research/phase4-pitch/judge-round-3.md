# Judge round 3: tempo-watchtower pitch deck (v3)

Judge persona: Colosseum partner plus a Tempo-track judge who knows payment channels. Date: 2026-09-27.
Deck reviewed: `research/phase4-pitch/tempo-watchtower-pitch-deck.md` (v3, 11 slides).
Calibrated against `judge-round-1.md` (v1 = 60) and `judge-round-2.md` (v2 = 72), using the same standard. Round 2 said deck-only fixes cap at about 82 and the last points need founder-only facts. I apply that here.

Placeholder assumption, as instructed:
- Modest solo background.
- The founder contacted the ~14 affected payees, got 2-3 replies and one usable quote.
- The demo has a real testnet tx hash.
- The repo link is a real public repo with a working MVP. This is new versus round 2, which assumed no repo.
- NOT assumed: the dominant payee signing on, or a real caught close on mainnet.

## What I verified myself (Bash only)

1. **304 of 344, 14 payees, 202 dominant, 29 / 5 / $36: REPRODUCED.** I re-ran `agg2.py` against `/tmp/tw/logs.json`; it is byte-identical to the copy in `onchain/`. It gives:
   - 304 forced withdraws with no payee-side response, 39 answered by an in-grace settle, 1 payee close, 64 unresolved.
   - 14 unanswered payees, 202 of them the dominant payee.
   - Provably used and unanswered: 29 channels, 5 payees, $36.00.
   - Median unanswered refund $0.499, max $25, total $1,966.13.
   - Operators: 676 channels, 302 self-assigned, 374 delegated, 37 distinct delegated operators.
2. **83 s median: REPRODUCED.** `delay.py` gives 39 responses, min 1 s, median 83 s, **max 726,599 s (8.4 days)**. By block gap, 38 of the 39 fall inside the 900 s grace. One is an after-grace settle that landed before the payer got round to `withdraw` (the "no grace check on close/settle" case). Counting it as a "response" is defensible in contract terms, but that response was not "watching".
3. **$0.00018 per settle at live gas: VERIFIED.**
   - Live mainnet (chainId `0x1079`) `eth_gasPrice` = `0x23c34600` = 6.0e8, and `baseFeePerGas` = 600,000,000 at block 41,409,262.
   - 301,831 × 6e8 = **$0.000181 per settle**, and $0.26/day at one settle per minute. The cap (12e9) gives $0.0036.
   - "About a fiftieth of a cent" is fair (it is about 1/55 of a cent).
   - `CLOSE_GRACE_PERIOD()` on mainnet returns `0x384` (900 s).
4. **Operator / captureAmount trust statements: VERIFIED.** I fetched `tempoxyz/tempo` `tips/verify/src/TIP20ChannelReserve.sol` (450 lines).
   - `settle` and `close` revert `NotPayeeOrOperator` unless the sender is the payee or a non-zero operator.
   - `close` enforces `settled ≤ captureAmount ≤ cumulativeAmount` and `≤ deposit`, and needs a valid voucher signature only if `captureAmount > settled`. It pays `delta` to the payee and `deposit − captureAmount` to the payer, and has **no `closeRequestedAt` check**.
   - `requestClose`, `withdraw` and `topUp` are payer-only, and `topUp` cancels a pending request.
   - The deck's Appendix C ("chooses the capture amount within [settled, voucher]; under-capture hurts payee, over-capture hurts payer") and Appendix A ("keep retrying past `closeGraceEnd` until `ChannelClosed`") are now correct.
   - The on-slide caption "payouts only to payee or payer" is literally true and no longer claims more than that.
5. **SDK gap: RE-VERIFIED.** `wevm/mppx` HEAD `dcf1589` (25 Sep):
   - `CloseRequested` / `watchContractEvent` appear only in ABI files, client code and a test, never in server code.
   - In server code, `closeRequestedAt` only gates charging and voucher acceptance (`Settlement.ts:320`, `ChannelStore.ts:434/482/849`, `RequestState.ts:146`).
6. **New finding from the same logs, which the deck does not use** (my script `/tmp/tw/j3.py`):
   - **All 39 in-grace responses were on channels with zero settles before the request.** The payees who answer use *settle-on-close-request* (lazy settlement plus a watcher). That is exactly the "settle rarely, answer close requests" posture v3 sells.
   - The responders are concentrated: `0xa1024faf…` (13, the self-testing wallet), `0xfbb2b310…` (13 of 16 requests on 27 channels) and the **dominant payee (11)**.
   - **The dominant payee began answering close requests on 2026-08-22** (block 35,972,499). **194 of its 202 unanswered requests predate that.** The 8 after it are all channels with a prior settle.
   - Only 11 of its 202 unanswered channels were provably used, carrying **$0.55** of refunds.
   - Its unanswered refunds do include **44 × $25 channels with $0 ever settled** (about $1,100), all before it started answering.
   - Read plainly: the biggest payee appears to have **built its own close-request responder in late August**. That is both the best demand signal in the dataset and the DIY competitor the deck has to beat. Slide 10 presents this payee as "left 202 unanswered, so it's my first design partner", which misses both points.

---

## TOTAL: 78 / 100 (pass = 90). Improved, still not shortlisted.

In one sentence: v3 fixes every falsifiable headline from round 2. The on-chain stat, the gas figure and the operator trust model now reproduce exactly, and the assumed working repo adds real execution credit. But the deck still has no named user or dollar contrast, and its business slide argues "pay a flat fee instead of settling constantly" right after telling the judge a settle costs a fiftieth of a cent. Its chosen first customer already appears to run its own watcher. These are the questions that decide a Colosseum interview, and deck edits alone cannot answer them.

## A. Official contest criteria: 37 / 48

| # | Criterion | Score | Justification |
|---|---|---|---|
| A1 | Functionality | 7/8 | A public repo with a working MVP plus a real testnet `close()` tx is the credible "it works" signal round 2 lacked. Appendix A is the design a Tempo engineer would write: synchronous voucher forwarding, bounded `spent` lag *L*, `max(spent, settledOnChain)`, retrying past `closeGraceEnd`, per-payee keys and poll fallback. Short of 8: there is still no on-screen number for detect-to-close latency or *L*, no failure-path proof (tower restart mid-grace, missed WS event), and no test/CI signal on any slide. |
| A2 | Potential Impact | 5/8 | The impact logic is now mechanically right ("settle constantly or limit the credit they extend"). But the deck's own honest numbers make impact small: ≤$36 provably lost chain-wide in 3.5 months, about $4.2k lifetime deposits, and falling non-dominant activity. At $0.00018 per settle, the "trade-off" it removes costs payees cents. Growth is still asserted, not shown. |
| A3 | Novelty | 6/8 | The DIY watcher is now named (slide 8), as round 2 asked, and the Lightning distinction is right. But the rebuttal is one line ("you own a 15-minute on-call"). The chain shows the top payee and `0xfbb2b310…` already run settle-on-close responders, so DIY is not hypothetical: it is what the market leader chose in August. The deck doesn't say why a payee that has already built one would switch. |
| A4 | UX | 7/8 | "Operator slot, main key stays cold" is a real protocol-native UX win. The adoption constraints round 2 flagged are now handled: new channels only and a provider switch means new channelIds (Appendix C), and forward-before-serve latency is handled by async `spent` with bounded lag (Appendix A). It still doesn't show what the payee sees when the tower fires; the alerting UI is deprioritized. |
| A5 | Open-source / composability | 7/8 | Concrete and correct integration points: `methodDetails.operator`, precompile events, `getChannelStatesBatch`, and mppx/mpp-rs middleware. Still unaddressed from round 2: there is one operator slot per channel. A payee already using the operator as a separate hot settlement key (supported by mppx `assertSettlementSender`) must choose, and Tempo access keys as an alternative delegation route are not mentioned. |
| A6 | Business Plan | 5/8 | It names a first design-partner target and a price placeholder, but there is still no revenue math or customer-level ROI. Slide 9's "Pay a flat fee instead of settling constantly" is economically backwards: by the deck's own number, constant settling costs cents per month, and the dominant payee's ~218k lifetime settles cost about $40 in total. The chosen first customer appears to have already solved it DIY on 2026-08-22. |

## B. Colosseum internal dimensions: 20 / 28

| Dimension | Score | Justification |
|---|---|---|
| Founder-market fit | 2/4 | Unchanged. With modest facts, the background still didn't drive the idea. Diligence is credited under Insight and Execution. |
| Insight | 4/4 | "No SDK watches close requests; auto-settle is opt-in and request-driven; the v2 operator slot enables a keyless tower; 304 of 344 requests went unanswered while watching payees answered in a median 83 s" is non-obvious, code-verified and now correctly measured. Both round-2 deductions (mis-measured headline, ignored threshold settling) are fixed. There is a sharper insight left in the data (the settle-on-close pattern, and the leader building its own), but what is on the slides is true and non-obvious. |
| Product + execution | 4/4 | Under the stated assumption (a public repo with a working MVP plus a real testnet close), there is now a build signal. Combined with the design quality in Appendix A, this earns the point round 2 withheld for "nothing is built yet". |
| Market size | 2/4 | Honest: 70 payees, 1,718 payers and 268k channels, 99.3% of them with one payee. Small and not growing outside that payee. The concentric-circle "every MPP session server" is unshown, and v1 legacy servers are explicitly out of scope. |
| Founder communication | 4/4 | 11 slides, one idea each, about 425 spoken words, all slides at 107-167 wpm, and a crisp closer. |
| Viability | 2/4 | The strawman is gone: the fee is correct at live gas, and threshold settling and DIY are named. But correctly priced, the alternative costs cents. The only residual risk is the sub-threshold tail plus a crash, and chain-wide provable losses are ≤$36. Willingness to pay $29/month is still unargued, which was round 2's core reason for this score, and slide 9 makes it worse by framing the price as a replacement for fees. Round 2 projected +1 here, but the fix landed on slide 8 and was undone on slide 9. |
| Traction | 2/4 | Same assumption as round 2: 14 contacted, 2-3 replies, one quote. A reply is not a user, and no payee has agreed to run the tower. |

## C. Pitch craft: 13 / 16

| Item | Score | Justification |
|---|---|---|
| Winner patterns | 3/4 | The credential comes in the first 10 s, and the deck has a "nothing faked" demo, an honest market slide, a named competitor row, a roadmap, full-time and a restated one-liner. Still missing C6: no loss-vs-price dollar contrast anywhere on screen (the price is a placeholder, and no loss figure is shown). Also C7: no named user. |
| Hook | 3/4 | The stat is now correct and arrives by 0:31. But round 2's fix 4 (a named user and a dollar amount in the first 20 s) was not done: the opening is still "an AI agent pays an API", with no name and no dollars. |
| Narrative flow | 4/4 | Problem, why it happens, why me, how, demo, validation, alternatives, business, market, close. The demo gets 30 s. Each slide earns the next. |
| Honesty | 3/4 | The three false round-2 headlines are gone and Appendix D matches reproducible scripts. The remaining overstatements are smaller but real: "settle rarely, never lose" (slide 8), "claiming exactly what was earned" (slide 5, which conflicts with Appendix A's lag *L*), and slide 10's "It left 202 close requests unanswered" (a true count, but it omits that 194 predate the payee answering them itself, and that only $0.55 of it is provably used). Slide 10's "payees either settle constantly or limit credit" is inference stated as fact. |

## D. Build-readiness for an AI deck generator: 8 / 8

| Item | Score | Justification |
|---|---|---|
| Per-slide completeness | 4/4 | Every slide has a title, on-slide text, a visual and a script. On-slide text is now about 15-22 content words per slide, near the ≤20 rule (slide 3 is the worst at about 22). The round-2 overruns (slides 5 and 8) are fixed. |
| Self-contained | 4/4 | Clean `SLIDES` / `END OF SLIDES` split, a usable style brief, and `[[INSERT RECORDING]]` marked. One nit: the line "**Spoken word count:** see the founder notes for the per-slide count" sits *inside* SLIDES (a generator may render it) and points to a count that doesn't exist in the founder notes. Delete it. |

---

## Delta vs. round 2

| Item | R2 | R3 | Δ | Driver |
|---|---|---|---|---|
| A1 Functionality | 6 | 7 | +1 | Working public repo (assumed) plus a real testnet close; strong design appendix |
| A2 Potential Impact | 5 | 5 | 0 | Logic fixed; honest numbers still tiny; no growth shown |
| A3 Novelty | 6 | 6 | 0 | DIY row added, but chain shows the leader already DIY'd it; one-line rebuttal |
| A4 UX | 6 | 7 | +1 | New-channels-only and latency handled |
| A5 Composability | 7 | 7 | 0 | Single operator slot conflict and access keys still unaddressed |
| A6 Business Plan | 5 | 5 | 0 | Target named; no ROI; slide 9 fee framing backwards |
| Founder-market fit | 2 | 2 | 0 | Unchanged |
| Insight | 3 | 4 | +1 | Headline correctly measured; threshold settling acknowledged |
| Product + execution | 3 | 4 | +1 | Build signal (assumed repo MVP) |
| Market size | 2 | 2 | 0 | Honest but tiny |
| Founder communication | 4 | 4 | 0 | Still crisp |
| Viability | 2 | 2 | 0 | Strawman fixed, but WTP unargued and slide 9 re-uses the fee argument |
| Traction | 2 | 2 | 0 | Replies, not users |
| Winner patterns | 3 | 3 | 0 | Still no $ contrast (C6), no named user (C7) |
| Hook | 3 | 3 | 0 | No named user or $ in the first 20 s |
| Narrative flow | 4 | 4 | 0 | Holds |
| Honesty | 2 | 3 | +1 | False headlines removed; residual absolutes and a selective slide 10 |
| Per-slide completeness | 3 | 4 | +1 | Word limits now roughly met |
| Self-contained | 4 | 4 | 0 | Dangling word-count line |
| **Total** | **72** | **78** | **+6** | |

Against round 2's forecast: it projected about 78 after fixes 1-3 and about 82 after fixes 4-5.
- v3 executed fixes 1-3 well and fix 5 partially.
- It skipped fix 4 (named user, dollar contrast), which is why Hook and Winner patterns did not move.
- Viability did not take its projected +1, because slide 9 reintroduces the fee argument that fix 2 said to drop.
- The repo assumption adds 2 points (A1, Product + execution) that round 2 did not assume.
- Net: 78.

---

## Top 5 remaining fixes (ordered)

1. **Rewrite slide 10 around what the dominant payee actually did. This is also the route to founder fact (a).**
   - From the same logs: it left 194 close requests unanswered from June to 2026-08-22, then started answering them itself (11 settle-on-close responses, and only 8 unanswered after that).
   - Among them are 44 channels of $25 each (~$1,100) that closed with $0 ever settled, all before it began answering.
   - Honest slide: "The biggest payee on Tempo built its own close-request watcher in August. Everyone else is still exposed."
   - That turns the leader into validation of the need, and makes the pitch to it "replace your in-house watcher with an independent operator" rather than "you left 202 unanswered".
   - Then ask that payee the one question only it can answer: were those $25 channels used?
2. **Fix the business slide's economics and state the ROI.**
   - Delete "Pay a flat fee instead of settling constantly": the deck itself says settles cost a fiftieth of a cent.
   - Sell what fees can't buy: the tail since the last settle, crash-to-restart gaps, and not owning a 15-minute on-call.
   - Give one customer-level line: "payee X: $Y exposed per incident × Z incidents vs $P/month". If no payee's numbers justify $29, then price per protected volume, or make it free for the first payees and say so.
3. **Remove the remaining absolutes and state the one trust assumption on-slide.**
   - "settle rarely, never lose" → "settle rarely; exposure bounded to *L* seconds of service while the tower is up."
   - "claiming exactly what was earned" → "claiming what each client consumed."
   - Add on slide 5: "trusted on the amount only; one key per payee."
   - Replace slide 10's "payees either settle constantly or limit credit" with the observed pattern (settle-on-close by the three responders).
4. **Put a named user and a dollar contrast in the first 20 s and on slide 9 (C6, C7).**
   - Use the confirmed payee's story if you get it. Otherwise use the demo channel ("a $25 channel; the server crashes; the agent takes back $X it already consumed").
   - Pair it with "vs $P/month" on slide 9.
   - This was round 2's fix 4 and it was not done.
5. **Turn the assumed repo into on-screen proof.**
   - Put measured numbers on the demo slide: "CloseRequested detected in N s; close landed in M s; *L* = K s".
   - Add a CI/test badge and one failure-path test: tower restart mid-grace, and a missed WS event recovered by the `getChannelStatesBatch` poll.
   - In the tech demo, address the single-operator-slot conflict and Tempo access keys (A5).

---

## Overstated or untraceable claims

1. **"tempo-watchtower: settle rarely, never lose"** (slide 8, on-slide and spoken). This is an absolute the design itself contradicts. Appendix A bounds exposure to *L* seconds of lag, and the tower can be down or run out of fee-token balance.
2. **"closes within seconds, claiming exactly what was earned"** (slide 5). "Exactly" conflicts with Appendix A's bounded `spent` lag. "Within seconds" has no measured number yet.
3. **"Pay a flat fee instead of settling constantly"** (slide 9, on-slide). This contradicts slide 8 and Appendix C (settle ≈ $0.00018; the fee is "not the argument").
4. **"It left 202 close requests unanswered, so it's my first design-partner target"** (slide 10, spoken). The count is TRUE (reproduced), but it is selective:
   - 194 of the 202 predate 2026-08-22, when this payee began answering them itself (11 responses).
   - Only 11 of the 202 were provably used, with $0.55 refunded.
   - Omitting that the target already runs a responder will look evasive to a judge who re-runs `agg2.py`, which the deck invites.
5. **"Today, payees either settle constantly or limit the credit they extend"** (slide 10). INFERENCE, not in the evidence files. The data shows a third behaviour, settle-on-close (all 39 responses had zero prior settles).
6. **"40 payee responses; median ~83 s"** (slide 7). Traceable, with two caveats. The 83 s median is over 39 settles only; the one close was 24 s. One of the 39 "responses" arrived 8.4 days later, after grace, before a late `withdraw`. "Watching works" rests mostly on three payees, one of them a self-testing wallet.
7. **Slide 2's juxtaposition**, "withdraws all of it back … 304 of 344 got no response". The count is correct, and the wording is the one round 2 recommended. But 275 of the 304 channels had $0 ever settled, so "no response" often plausibly means "nothing to claim". The ≤$36 provable figure is only in Appendix C. This is acceptable phrasing, but keep it off any line that implies 304 losses.
8. **"one payee holds 99%"** (slide 10 on-slide). It is 99.3% *of channels*. Say "of channels".
9. **"→ every MPP session server"** (slide 10 visual). v1 legacy servers are out of scope (Appendix A), so this means v2 session servers only.
10. **Evidence file §2** still says "The worst a rogue operator can do is close a channel early". The corrections block supersedes it, but a judge reading the evidence top-down sees it after the corrections. Strike it through.
11. **"Spoken word count: see the founder notes for the per-slide count"**. That count does not exist in the founder notes.

These traced correctly (reproduced or fetched):
- 304 / 344 / 39 + 1 / 64, 14 payees, 202 dominant.
- 29 / 5 / $36; median $0.50; 676 / 302 / 374 / 37 operators.
- 83 s median and 1 s minimum.
- 900 s grace on mainnet.
- Live gas 6e8 and $0.00018 per settle.
- Operator may settle and close, only pays payee or payer, and chooses `captureAmount ∈ [settled, voucher]`.
- `close` has no grace check.
- No server-side watcher in mppx.
- 70 payees / 1,718 payers / 268k channels (from round 2's reproduction).

## Technical errors / things a Tempo engineer would catch

- **T1 (medium) The unanswered metric doesn't separate "nothing to claim".**
  - For a channel whose `spent == settled`, the correct response to `requestClose` is to do nothing.
  - Both the deck's KPI and the product's own reporting should split "unanswered with a claimable balance" from "no claimable balance". Off-chain, only the payee (or the tower's mirror) knows the difference.
  - Otherwise the tower's own dashboard will report false "saves".
- **T2 (medium) Single operator slot.** Carried from round 2's T6 and only half-fixed. New-channels-only is now stated. The conflict with payees using the operator slot as their own hot settlement key (mppx `assertSettlementSender`), and Tempo access keys as a competing delegation path, are still absent.
- **T3 (low) The "settle rarely" advice concentrates risk on the tower.**
  - The less the payee settles, the more a correlated failure costs (tower and payee down together, or a fee-token balance of 0).
  - The deck should give a recommended combination, e.g. an amount threshold as a backstop plus the tower, not "rarely" alone.
  - The low-balance alert in Appendix A is good; a heartbeat and alert on the tower itself is missing.
- **T4 (low) The deadline target is unquantified.** Appendix A says "target seconds after the request" but gives no SLO. Existing DIY responders hit a median of 83 s. The tower should publish its number and beat it by an order of magnitude to justify "vs DIY".
- **T5 (low) `topUp` cancels a pending close.** This is handled (`CloseRequestCancelled` is subscribed). The tower must also treat a later `requestClose` on the same channel as a fresh deadline; the contract keeps only one `closeRequestedAt`. It is fine if the state machine re-enters "watching".

All of round 2's T1-T5, T7 and T8-as-data are fixed. T8 as strategy (explain the dominant payee) is still open; see fix 1.

## Script length check

| Slide | Window | Words (fixed + est. placeholder) | wpm |
|---|---|---|---|
| 1 | 0:00-0:09 (9 s) | 16 + ~8 | ~160 |
| 2 | 0:09-0:31 (22 s) | 56 | 153 |
| 3 | 0:31-0:47 (16 s) | 39 | 146 |
| 4 | 0:47-0:57 (10 s) | ~25 | ~150 |
| 5 | 0:57-1:15 (18 s) | 46 | 153 |
| 6 | 1:15-1:45 (30 s) | 54 | 108 |
| 7 | 1:45-2:01 (16 s) | 23 + ~20 | ~161 |
| 8 | 2:01-2:19 (18 s) | 50 | 167 |
| 9 | 2:19-2:29 (10 s) | 25 | 150 |
| 10 | 2:29-2:44 (15 s) | 41 | 164 |
| 11 | 2:44-2:53 (9 s) | 19 | 127 |
| **Total** | 2:53 | **~369 fixed + ~53 = ~422** | ~146 |

It fits, under 450 words and under 3:00. Slide 8 is the fastest; if fix 1 adds words to slide 10, take them from slide 6's slack.

---

## Projection IF the founder adds (a) and (b)

- **(a)** The dominant payee, or another affected payee, confirms on record that a forced close cost real earned revenue, and agrees to run the tower as operator on new channels.
- **(b)** A repo run log showing the tower caught a real `CloseRequested` and landed the `close`.

The deck also has to integrate both facts: a named user in the hook, the confirmed dollars on slide 7 and slide 9, and the run-log numbers on slide 6. Deck wording is otherwise left as in v3.

| Item | R3 now | With (a)+(b) | Why |
|---|---|---|---|
| A1 Functionality | 7 | 8 | A reproducible run log of a caught close, on top of a working MVP, is the full "works" proof. |
| A2 Potential Impact | 5 | 6 | A confirmed loss converts the ≤$36 upper bound into a real figure; the market is still small. |
| A3 Novelty | 6 | 7 | A payee choosing the tower over DIY is the missing proof against the DIY alternative. If it is the dominant payee, which already built a responder, the proof is direct. |
| A4 UX | 7 | 7 | No change. |
| A5 Composability | 7 | 7 | No change (single operator slot, access keys). |
| A6 Business Plan | 5 | 6 | A committed design partner. Without a price or paid pilot, revenue math is still absent. |
| Founder-market fit | 2 | 3 | Finding and winning a payee from chain forensics is a credible grit and execution signal. This is the softest +1. |
| Insight | 4 | 4 | Already maxed. |
| Product + execution | 4 | 4 | Already maxed. |
| Market size | 2 | 2 | One partner doesn't change the size of the market. |
| Founder communication | 4 | 4 | Already maxed. |
| Viability | 2 | 3 | A payee on record that the loss was real is the first willingness-to-pay evidence. There is still no price. |
| Traction | 2 | 3 | A real user running the tower on new channels. This is 4 only if it is the dominant payee and live on mainnet. |
| Winner patterns | 3 | 4 | C6 (confirmed $ vs price) and C7 (named user) become possible. It backfires if the confirmed loss is under the monthly price. |
| Hook | 3 | 4 | Named payee plus dollars in the first 20 s. |
| Narrative flow | 4 | 4 | No change. |
| Honesty | 3 | 3 | Founder facts don't remove "never lose" / "exactly" / slide 10's selective framing. It is 4 only if fix 3 is also done. |
| Per-slide completeness | 4 | 4 | No change. |
| Self-contained | 4 | 4 | No change. |
| **Total** | **78** | **87** | |

**Does that route reach 90? Not by itself: 87.** Three more points are available, and each has a specific condition:
- **+1 Honesty.** Deck-only: do fix 3 (remove "never lose" and "exactly", and reframe slide 10 truthfully).
- **+1 Traction (to 4).** The confirming payee must be **the dominant payee** (99.3% of channels), with the tower live on its mainnet channels, not a second-tier payee or the self-testing `0xa1024faf…`.
- **+1 Business Plan (to 7).** That payee agrees to a price or a paid pilot, so slide 9 shows real revenue math instead of a placeholder.

With all three, the total is **90**. Adding the confirmed dollars as a real C6 contrast still leaves Market at 2, so 90-91 is the realistic ceiling.

**Warning.** Every affected payee except the dominant one has at most about $6 of provable used-and-unanswered refunds. The exception is `0xa1024faf…` at $24.55, which is the self-tester. A second-tier payee confirming "$6 lost" next to "$29/month" hurts Viability and Winner patterns rather than helping. The only confirmation that clears 90 is the dominant payee's, most plausibly about its pre-2026-08-22 $25 zero-settle channels (about $1,100) and its decision to replace its in-house responder.
