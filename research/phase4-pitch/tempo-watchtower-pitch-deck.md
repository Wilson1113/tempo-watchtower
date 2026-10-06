# tempo-watchtower — Pitch Deck (v5)

Colosseum Crypto World's Fair · Tempo track · submission deadline 2026-10-12 23:59 PT

**How to use this file.**
- Slides 1–11 are the deck. Give only the SLIDES section to an AI slide generator. Everything
  after `END OF SLIDES` is for the founder.
- Each slide has **On-slide** text (≤20 words), a **Visual** brief and a **Script** (the Loom
  voiceover). The spoken budget is ≤450 words.
- `[[FOUNDER: …]]` means fill in real facts before recording; never invent them.
  `[[DEMO: …]]` means measured numbers from your own test run.
  `[[INSERT RECORDING]]` means real screen capture.

**Style brief:**
- Dark background.
- Amber = "the clock / at risk"; green = "claimed / safe".
- Monospace for on-chain names.
- One idea per slide.
- A small 15:00 countdown motif in the corner of slides 2–6.

---

# SLIDES

## Slide 1 — Title
**On-slide:**
- tempo-watchtower
- Claims what your API earned before a payer's forced close takes it back.
- [[FOUNDER: name]] · Tempo track · open source

**Visual:** lighthouse mark whose beam is a 15:00 clock face.

**Script (0:00–0:09):** "I'm [[FOUNDER: name — one-line credential]]. tempo-watchtower claims
what your API earned before a payer's forced close takes it back."

---

## Slide 2 — The problem, told by Tempo's biggest session API
**On-slide:**
- Payer: `requestClose()` → 15:00 → `withdraw()` → unclaimed revenue returns
- Busiest payee: 0 of 194 close requests answered (Jun–Aug)

**Visual:** timeline (request → amber 15:00 → withdraw) above a stat card: "Tempo's busiest
session payee (99% of channels), Jun 10 – Aug 22 2026: 0 of 194 close requests answered."

**Script (0:09–0:30):** "On Tempo, AI agents pay APIs through payment channels. The agent can
call requestClose anytime; if the API doesn't claim what it earned within fifteen minutes, the
agent withdraws it all back. Tempo's busiest session API answered zero of its first 194 close
requests."

---

## Slide 3 — Why it keeps happening
**On-slide:**
- No SDK watches for close requests
- Auto-settle is off by default
- Anything served since the last settle is exposed

**Visual:** three icons (crashed server, deploy arrow, idle moon), each pointing at the 15:00
clock hitting zero.

**Script (0:30–0:43):** "Why? Neither official SDK, mppx or mpp-rs, watches the chain for close
requests. Automatic settling is off by default and only runs while requests flow. Anything served
since the last settle is exposed."

---

## Slide 4 — Why me
**On-slide:**
- [[FOUNDER: 2–3 specific proof points, numbers over adjectives]]

**Visual:** founder photo + proof-point chips.

**Script (0:43–0:53):** "[[FOUNDER: two specific, checkable proof points, ~25 words]]"

---

## Slide 5 — How it works: your channel's operator
**On-slide:**
1. Watchtower = channel `operator` (Tempo v2)
2. Mirrors client consumption
3. Close request → `close()` in seconds

**Visual:** [Your API server] —vouchers + consumption→ [tempo-watchtower = operator, separate
region] —watches→ [Tempo v2 channel precompile]; API server down (red), watchtower's `close()`
landing (green). Caption: "pays only payee or payer · trusted on amount only".

**Script (0:53–1:11):** "tempo-watchtower becomes your channel's operator, a role Tempo's v2
channels support natively. It mirrors what each client consumed, watches for close requests, and
closes in seconds, claiming what was consumed. The contract only pays payee or payer, each payee
gets its own key, and your main key stays cold."

---

## Slide 6 — Demo
**On-slide:**
- Tempo testnet · real transactions · nothing faked
- Detected in [[DEMO: N]] s · closed in [[DEMO: M]] s

**Visual:** [[INSERT RECORDING]] split screen.
- Both sides: kill the server, the agent calls `requestClose`, countdown (time-lapse, labeled).
- Left ends with `withdraw`: the earned balance returns to the agent.
- Right: watchtower log "CloseRequested → close(capture = consumed)", explorer link
  [[DEMO: real tx hash]], and an on-screen timer showing detect and close latency.

**Script (1:11–1:39):** "This is Tempo testnet, nothing faked. Left, no watchtower: we kill the
server, the agent requests close, and fifteen minutes later withdraws everything back. Right, same
crash, but the watchtower detects the request in [[DEMO: N]] seconds and lands the close. Here's
the transaction. The API keeps what was consumed."

---

## Slide 7 — Validation: builders are hand-rolling this
**On-slide:**
- 6 of 16 payees now answer close requests — each built it themselves
- 10 still answer none
- [[FOUNDER: N contacted · M replied · ≤12-word quote]]

**Visual:** 16 payee tiles: 6 green ("hand-rolled responder"), 10 red ("no response"); a small
inset timeline of the busiest payee turning from red to green on Aug 22.

**Script (1:39–1:57):** "The validation: builders are hand-rolling this. Of sixteen payees that
received close requests, six now answer them, each with code they wrote themselves, including
the busiest, since August. Ten still answer none. [[FOUNDER: 'I contacted N; M replied. One said:
…']]"

---

## Slide 8 — Why not DIY or just settle often?
**On-slide:**
- Settle often: cheap, off by default, leaves a tail
- DIY: 6 payees each rebuilt it
- Best: threshold + watchtower

**Visual:** three stacked cards; the last one green.

**Script (1:57–2:17):** "Why not settle often? It's cheap, but it's off by default and still
leaves a tail. Why not build your own? You can; six payees each wrote their own. That's
the pattern a shared, open tool replaces. The right setup: a settle threshold as
a backstop, plus a watchtower."

---

## Slide 9 — Business model
**On-slide:**
- Open source; self-host free
- Free hosted operator for the first 10 payees
- Then priced on protected volume

**Visual:** three steps: open source → free design partners → volume-based paid tier.

**Script (2:17–2:30):** "The core is open source. The hosted operator is free for the first ten
payees. After that, we price on the volume we protect, so what a payee pays tracks what we save
them."

---

## Slide 10 — Market (honest) + first customer
**On-slide:**
- 70 payees · 1,718 payers · 268k channels since June
- Early; one payee holds 99%
- First: 10 payees answering none

**Visual:** small stat row; concentric circles: "10 payees answering none → 6 hand-rolled
responders → every new MPP session server".

**Script (2:30–2:46):** "Honestly, the market is early: seventy payees, and one runs 99% of
channels. Six payees proved the need by building their own. My first customers are the ten
answering none, then the six maintaining their own, then every new session server."

---

## Slide 11 — Roadmap + close
**On-slide:**
- Open repo: [[FOUNDER: link]] · weekly updates
- Next: one-line mppx / mpp-rs integration → hosted operator
- **tempo-watchtower — keep what you earned.**

**Visual:** three-step arrow ending on the logo.

**Script (2:46–2:55):** "I'm building this full-time, in the open. Next: a one-line mppx and
mpp-rs integration. tempo-watchtower: keep what you earned."

# END OF SLIDES

---

## Founder notes (not for the generator)
- **Spoken budget:** 372 fixed words plus ~45 in placeholders, roughly 417 total, 2:55 at ≤163 wpm per slide.
- **Honesty caveats you MUST know (judge round 4, verified):**
  - All 44 of the busiest payee's "$25 refunded, $0 settled" channels came from **one payer
    wallet (`0xa1024faf…`), which is itself a payee**. Two such wallets account for 99% of the
    busiest payee's unanswered refunds. Much of this is **builders testing each other**, not real
    customers. That's why the $1,100 figure was removed from the hook; never present it as lost
    revenue.
  - Among payees that answer none, provable exposure is tiny (≤ ~$6 each).
- **Slide 2/7 facts:** reproducible via `onchain/dom.py` (`dominant_payee_output.txt`) and
  `agg2.py`. The 6 / 10 split is in `onchain/payee_split_output.txt`.
  - 0 of 194 answered before 2026-08-22; 11 of 19 answered since.
  - The 8 unanswered since then had ~$0 to claim (refunds of $0.00–$0.49), so its watcher works.
    **Never say it "misses"**; that would be false.
  - 44 channels × $25 refunded with $0 settled, all before Aug 22.
  - **Do not say those $1,100 were "lost":** only the payee knows whether that service was
    consumed.
- **The single most valuable outreach:**
  - Ask the busiest payee: were those 44 $25 channels used, and what did building your watcher
    cost you? A quote like "it took us X weeks" is gold for slide 7.
  - Ask the 10 payees answering none (and the 6 with hand-rolled responders) whether they'd run
    the watchtower on new channels. One yes is founder fact (a).
- **Why me (slides 1 and 4):** dense, checkable specifics only. Solo winners always did this.
- **Validation (slide 7):** the outreach line must be literal. If nobody replies, say so and drop
  the quote.
- **Demo numbers (slide 6):** measure detect-to-close latency on your testnet run. Target
  well under the leader's DIY median of 83 s.
- **Weekly one-minute update videos** until 2026-10-12.

## Appendix A — Technical demo video (≤3:00, the "how")
1. **0:00–0:30 Architecture.**
   - The payee server advertises `methodDetails.operator = <per-payee watchtower key>`, and v2
     channels bind it into their channelId (spec §6.2.1).
   - Vouchers are forwarded synchronously. Consumption (`spent`) is streamed with a bounded lag
     of *L* s, and exposure while the tower is up is at most *L* s of service.
   - Capture is `max(spent, settledOnChain)` from the mirror (spec §13.2), never the voucher
     amount alone.
2. **0:30–1:30 Live run on testnet (chain 42431).** Open a channel, stream paid requests, kill
   the server, have the payer call `requestClose`, then show the tower log, the `close()`
   transaction and the measured latency.
3. **1:30–2:20 Decisions and failure paths.**
   - **Detection:** a WebSocket subscription to `CloseRequested`, `CloseRequestCancelled` and
     `ChannelClosed`, plus a `getChannelStatesBatch` poll as fallback.
     - Test shown: kill the WebSocket mid-grace; the poll recovers it.
     - Test shown: restart the tower mid-grace; it resumes from its ledger.
   - **Deadline:**
     - Aim for seconds.
     - Keep retrying past `closeGraceEnd` until `ChannelClosed`. `close` has no grace check, so
       the real deadline is the payer's `withdraw` being mined.
     - A fresh `requestClose` after a `topUp` cancellation starts a fresh deadline.
   - **Metrics:** separate "request with a claimable balance" (spent > settled) from "nothing to
     claim". Only the first counts as a save.
   - **Keys and health:**
     - A per-payee operator key, never shared.
     - A low fee-balance alert, and a heartbeat alert on the tower itself.
     - The payee keeps its SDK's amount-threshold settlement on as a backstop. The tower never
       calls `settle`; it only closes on a close request.
   - **Rust:** a static always-on binary with no GC pauses near a deadline, running a typed
     per-channel state machine.
4. **2:20–3:00 Constraints and deprioritized work.**
   - There is one operator slot per channel. A payee already using it as its own settlement key
     would move that role to the tower; Tempo access keys are an alternative route to evaluate.
   - New channels only: the operator is fixed at open.
   - v1 channels are unsupported.
   - Deprioritized: the hosted multi-tenant tier and an alert UI.

## Appendix B — Portal fields
- **Brief description:** tempo-watchtower is an open-source operator for Tempo v2 payment
  channels. It detects payer close requests and closes the channel in seconds, claiming what each
  client consumed, so a crash, deploy or quiet period doesn't hand earned revenue back.
- **Chains/tools:** Tempo mainnet/testnet, TIP-20 channel precompile (v2 sessions), mppx,
  mpp-rs, Rust/tokio.
- **Demand validation:** on-chain close-request analysis (`evidence-round2.md` corrections,
  `onchain/`) plus the outreach log.
- **Pre-existing code:** disclose any.

## Appendix C — Interview prep (short; full drill in the study guide)
- **"Why not settle often?"** It's cheap (~$0.00018/settle at the mainnet floor gas price), so we
  don't argue fees. It's off by default, leaves the tail since the last settle, and only runs
  while requests flow. Recommended setup: threshold backstop plus the tower.
- **"Payees already build their own. Why would anyone use yours?"** Six of 16 did, which shows
  the need. Each rewrote the same thing, and ten still have nothing. A shared, open, independent
  tool replaces six private copies and covers the ten. We sell reliability and not having to
  build it, not raw speed.
- **"Isn't most of this activity just builders testing?"** Largely, yes. Two payer wallets that
  are themselves payees drive 99% of the busiest payee's forced refunds. We say so. The
  behavior, not the dollars, is the signal.
- **"What can a rogue operator do?"** It can only pay the payee or refund the payer. But it
  chooses the capture amount, so it could under- or over-capture. Mitigations: a per-payee key,
  capture from the forwarded `spent`, custody, and an alert on every close.
- **"Existing channels?"** No; the operator is fixed at open, so only new channels are covered.
- **"Were those $1,100 really lost?"** Very likely not: all 44 came from one tester wallet that is itself a payee. Unknown in any case. $0 was claimed, but only the payee knows if the
  service was used. That's the question I'm asking them.
- **"The market is tiny."** Yes, and we say so. The leader's own investment shows the problem
  matters to the one operator with real volume.

## Appendix D — Claim → source
| Claim | Source | Status |
|---|---|---|
| requestClose → 15-min grace → withdraw refunds `deposit − settled` | spec §6.4.5–6; contract `withdraw` | VERIFIED |
| Operator may settle/close; pays only payee/payer; chooses captureAmount | spec §6.2.1; `TIP20ChannelReserve.sol` | VERIFIED |
| Capture = max(spent, settledOnChain) | spec §13.2 | VERIFIED |
| Busiest payee: 99.3% of channels; 0 of 194 answered before 2026-08-22; 11 of 19 answered after, and the 8 unanswered after had ~$0 to claim | `onchain/dom.py`, `forced_close_rows.json` | COMPUTED (reproducible) |
| 44 × $25 channels refunded with $0 settled, all before Aug 22, all from one payer wallet that is itself a payee (removed from the slides) | `forced_close_rows.json`; `payee_split_output.txt` | COMPUTED |
| 6 of 16 payees answer close requests; 10 answer none | `payee_split_output.txt` | COMPUTED |
| 304 of 344 resolved requests unanswered chain-wide; responders' median 83 s | `agg2.py`, `delay.py` | COMPUTED |
| 70 payees, 1,718 payers, 268,596 channels since 2026-06-10 | `agg.py` | COMPUTED |
| No SDK close-request watcher; auto-settle opt-in and request-driven | SDK source grep | VERIFIED |
| Settle ≈ $0.00018 at the mainnet floor gas price | gas snapshot × live mainnet gas price 6e8 | COMPUTED |
| Colosseum guidance / winner patterns | `winner-patterns.md` | VERIFIED |
