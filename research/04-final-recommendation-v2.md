# Final recommendation v2 — after the 4-point niche re-filter

Supersedes `02-final-recommendation.md`. Triggered by a direct question — "will corewriter-receipts
even find a single person who really wants to pay for it?" — which exposed a gap in the original
scoring rubric, followed by the user's own 4-point test (named+reachable users / real cost, not
annoyance / STRUCTURAL not circumstantial big-co neglect / enjoyable domain work — saved to
memory as `niche-selection-filter.md`). All 26 Phase-1 candidates were re-screened against it
(`03-refiltered-shortlist.md`); 4 passed cleanly and went through a full second Phase-2/3 pass.

## Verdict summary (Phase 3, adversarial)

| Candidate | Track | Verdict | What the adversarial pass found |
|---|---|---|---|
| **TEMPO-WATCHTOWER** | Tempo | **SOUND WITH CAVEATS** (cleanest of the four) | Core evidence reproduced byte-for-byte via raw fetch; zero competitors found on a second independent search. Only caveat: "structural neglect" and the ~326-server TAM are *absence of evidence* (no roadmap page exists), not positive proof — real but not disqualifying. |
| **upgrade-lens** | Solana | SOUND WITH CAVEATS | The critical test — does Solana Foundation's newly-active tooling threaten the structural-neglect argument? — survived: their 3 recently-updated repos stay at CI/hash-authenticity level, no move toward bytecode diffing. One technical-scoping caveat: static CPI-target diffing only catches hardcoded-pubkey CPI changes, not runtime-resolved ones — narrower than the pitch currently states, fixable with wording. |
| **HL Clear-Sign** | Hyperliquid | AT RISK | The deck's central differentiation claim — that competitor `hl-rs` "cannot validate withdrawals/transfers" — is **factually false** on direct code read; that function exists and works. A judge could disprove this in 2 minutes on GitHub. The idea survives (neither competitor does independent hardware-wallet cross-checking or risk-flagging), but the deck needs a real correction, not just a caveat, before it's safe to present. |
| **accel-conform** | Ethereum L1 | SOUND WITH CAVEATS, but with an open question | Confirmed: eth-act's own `ere` repo already ships most of the vector corpus + harness (validates the "wrap ere, build the certification/differential layer it lacks" reframe). Unresolved: whether `ere`'s existing SP1 test already catches the exact bug (sp1#2881) the demo is built around — couldn't be checked with available tools. This must be tested locally *before* committing to build, or the demo's key "gotcha" moment may not fire. A cleaner fallback bug (openvm#3146) was found as backup demo material. |

## Recommendation: build **TEMPO-WATCHTOWER**

Ranking rationale, in order of what matters most:
1. **Cleanest verification result of the four** — no live competitor, no factual errors, no
   unresolved technical question blocking the demo. The only caveat is an absence-of-evidence
   one (can't prove a negative about Tempo's roadmap), which is a fundamentally weaker objection
   than what the other three carry.
2. **Best "would someone actually pay" story of every candidate researched across both rounds**
   (this directly answers the question that triggered the whole re-filter). The cost prevented
   is total, immediate loss of a payee's own already-earned, unsettled funds — not debugging
   time, not a theoretical future risk. A payee facing "pay a small watchtower fee, or risk
   losing 100% of this channel's balance if you miss a 15-minute window" has about as direct and
   legible an incentive to pay as exists in this whole research pass. This mirrors real,
   proven Lightning-watchtower economics, not a hypothetical business model.
3. **Structural neglect is the cleanest of the four, definitionally, not just evidentially**: a
   watchtower is *by design* meant to be a separate, independent party from the payee's own
   server — Tempo Labs building this in-house would partially defeat its own purpose. This isn't
   "they haven't gotten around to it," it's "this role structurally belongs outside the core
   protocol team," same as Lightning.
4. **First-mover, freshest evidence**: the spec (`draft-tempo-session-00`) was published one day
   before Phase 1 research began. No competing repo exists anywhere in Tempo's 69-repo GitHub
   org.
5. **Domain fit**: concurrent-ledger/payment-channel state machines — squarely "VM/async/crypto
   core infra" and the kind of work the builder wants to do.

**Honest limitation to carry into the pitch, not hide**: the addressable market is genuinely
small today (an estimated few hundred MPP servers, and that estimate itself is ~6 months stale
with nothing fresher found). Frame this the way the deck already does — a real, if narrow,
recurring-revenue niche today, with a natural expansion path if Tempo/MPP adoption grows, not a
venture-scale platform story on its own.

Required before presenting: apply the fixes in
`phase3-verification/tempo-watchtower-verdict.md` (soften "structural neglect" to reflect it's
an absence-of-announcement finding, add the TAM-staleness caveat to §2 not just §7, fix a
cosmetic repo-count discrepancy).

## Fallback order if TEMPO-WATCHTOWER falls through
1. **upgrade-lens** (Solana) — apply the two required fixes (scope the CPI-diffing claim to
   hardcoded pubkeys, state TAM as large-but-uncounted with a concentrated near-term buyer list
   rather than implying broad near-term revenue), then build. Structural argument is genuinely
   solid; this is the strongest fallback.
2. **HL Clear-Sign** (Hyperliquid) — fix the false `hl-rs` claim first (do not skip this), then
   reframe the pitch around standalone-CLI + independent hardware-wallet cross-display + risk-
   flagging (all still genuinely absent from both real competitors) rather than "nobody else can
   compute this hash."
3. **accel-conform** (Ethereum L1) — only proceed after locally running `ere`'s existing SP1
   test against the sp1#2881 vector to confirm it does NOT already catch the bug; swap in
   openvm#3146 as backup/replacement demo material for the OpenVM side regardless.

## Process note
This second pass burned the session's WebSearch budget (200/200 used before Phase 2 of round 2
began) — all round-2 re-verification relied on WebFetch against specific known URLs rather than
broad discovery search. This was sufficient because round 2 was deepening/re-verifying
already-sourced candidates, not blind discovery; a genuinely fresh discovery pass (if ever
needed) would require raising `CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`.

Startup methodology remains on hold in all four decks, per your instruction — fill in once a
final candidate is chosen.
