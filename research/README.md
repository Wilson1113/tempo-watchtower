# World's Fair research: index and status (2026-09-27)

**Decision:** build **tempo-watchtower**. It's an open-source Rust operator for Tempo v2
payment channels. It detects a payer's `requestClose` and closes the channel in seconds,
claiming what each client actually consumed.

**Deadline:** 2026-10-12 23:59 PT (Colosseum Crypto World's Fair, Tempo track).

## Start here
| You want to… | Open |
|---|---|
| Understand Tempo, MPP, payment channels, the problem and the product from zero, and drill judge questions | `study-guide/tempo-watchtower-study-guide.md` |
| Build the slides and record the pitch video | `phase4-pitch/tempo-watchtower-pitch-deck.md` (**v5, current**). Feed only the SLIDES section to your AI deck tool; the founder notes, tech-demo script, portal answers and interview prep come after `END OF SLIDES` |
| Check any number in the deck | `phase4-pitch/evidence-round2.md` (read the CORRECTIONS block first) and `phase4-pitch/onchain/` (scripts and outputs) |
| See how past Colosseum winners pitched | `phase4-pitch/winner-patterns.md` |
| Build the tech demo in Rust (day-by-day, CI/CD, open-source rules) | `build-guide/tempo-watchtower-build-guide.md` (+ `reference/` spoilers, tech-lead reviews) |
| Understand each file of the workspace skeleton | `build-guide/skeleton-tour.md` |
| Build in public on X (bio, posting cadence, drafts, rules) | `social/x-build-in-public-guide.md` |

## Judge history (independent judge agents, same rubric)
| Round | Score | What changed |
|---|---|---|
| 1 (v1) | 60 | The spec was revised (v2 operator), no dollar figure, "why not settle often?" unanswered |
| 2 (v2) | 72 | Operator pivot, SDK and contract proof. The on-chain count and the gas price were wrong (my errors) |
| 3 (v3) | 78 | Numbers corrected and reproducible. Business slide argued fees, which are negligible |
| 4 (v4) | 79 | **Plateau confirmed:** deck-only edits cap at about 81. Found tester-driven data and DIY responders |
| v5 | not re-judged | Applies round 4's honesty fixes (tester caveat, 6-of-16 DIY framing). Expected about 80-81 |

**How to go past 90 (every judge agreed): real-world facts, not wording.**
1. A public repo in which the watchtower caught a real `CloseRequested` on testnet and landed
   `close()`, with measured latency.
2. At least one affected payee on record agreeing to run it on new channels, ideally with a
   paid-pilot commitment.
Judges project about 85-88 with both.

## Honest risks to weigh (read before committing the remaining two weeks)
- **Today's money is tiny.**
  - Lifetime deposits are about $4.2k.
  - Provable unanswered exposure is ≤ $36.
  - Much of the activity is builders testing each other: two payer wallets that are themselves
    payees drive 99% of the busiest payee's forced refunds.
- **DIY is common.** 6 of the 16 payees that received close requests already hand-rolled a
  responder. The pitch is "one shared, open, reliable tool" rather than "nobody solves this".
- **The market is early and lumpy.** Outside the busiest payee, activity has fallen since July.
- **What the pitch leans on:** protocol behavior (default SDKs don't watch) plus MPP growth
  backed by Stripe and Tempo. This fits the Public Goods / open-source angle well; a big
  near-term revenue story is weak.

## Folder map
- `00-brief.md`, `hackathon-official-rules.txt`: the rules and judging criteria.
- `phase1-scouting/`: five scout reports (26 candidate ideas).
- `01-shortlist.md`, `02-final-recommendation.md`: first-round shortlist (superseded).
- `03-refiltered-shortlist.md`: re-screen using the 4-point niche filter.
- `04-final-recommendation-v2.md`: why tempo-watchtower was chosen, plus fallbacks.
- `phase2-decks/`, `phase3-verification/`: decks and adversarial verdicts for all candidates.
- `phase4-pitch/`:
  - the final deck, the judge rubric and rounds 1-4;
  - the evidence files;
  - the competitor re-check;
  - `onchain/`: analysis scripts. Re-running `scan.py` downloads about 465 MB of logs from
    `rpc.tempo.xyz` in about 2 minutes.
- `study-guide/`: the complete learning guide.

## Your to-do list (by priority)
1. **Build the MVP** and run the testnet demo. This fills `[[DEMO: …]]` and the repo link.
2. **Do the outreach.** Contact the 10 non-answering payees and the 6 hand-rolled responders.
   Addresses are in `phase4-pitch/onchain/forced_close_rows.json`.
3. **Fill the `[[FOUNDER: …]]` placeholders** with true, specific credentials and literal outreach
   results.
4. **Record the pitch (≤3:00) and the tech demo (≤3:00).** Post weekly one-minute updates until
   Oct 12, and fill every portal field.
