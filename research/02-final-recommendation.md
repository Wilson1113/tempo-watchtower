# Final recommendation — Colosseum Crypto World's Fair (deadline 2026-10-12)

Three-phase research complete: 5 Phase-1 scouts → 3 Phase-2 decks → 3 Phase-3 adversarial
verifications. All files under `research/`. This document is the top-line synthesis.

## Verdict summary

| Candidate | Track | Verdict | What changed under adversarial re-check |
|---|---|---|---|
| **corewriter-receipts** | Hyperliquid | **SOUND WITH CAVEATS** | Strengthened — verifier found a 3rd independent open GitHub issue (#174) the deck missed. Zero competitors found across GitHub, crates.io, npm, web search. Only cosmetic edits needed. |
| **Ironwood Receipts** | Zcash | **AT RISK** (wounded, not dead) | Deck's claim that the competing $42k grant application has "zero comments" is **false** — verifier found 6 comments including a real technical design the applicant posted 3 days before the deck was written. No code exists yet, so "ship first" is still possible, but the deck must be corrected before use. |
| **simd-diff** | Solana | **AT RISK** (survivable if narrowed) | A same-hackathon-window competitor (UpgradeGuard) shipped overlapping features ~12.5 hours before research. It's a single-contributor, zero-star repo with two confirmed real gaps this pitch can own — but only if the pitch is explicitly narrowed to those gaps rather than competing on breadth. |

## Recommendation: build **corewriter-receipts**

It's the only one of the three that came back from an adversarial pass *stronger* than it went
in. Rationale:
- **Evidence**: three independent, still-open, unanswered GitHub issues (#171→downgraded,
  #172, #174) all showing the same HyperEVM/HyperCore silent-failure pattern across different
  CoreWriter actions — found by three different Phase-1 scouts *and* the Phase-3 verifier,
  independently, using different search strategies. This is the best-corroborated finding in
  the entire research pass.
- **Whitespace**: exhaustive search (GitHub, crates.io, npm, web) across two separate research
  passes found zero competing outcome-correlation tools. No grant, no rival repo, no announced
  fix from Hyperliquid Labs.
- **Architecture, not a bug**: CoreWriter's fire-and-forget behavior is a deliberate
  cross-VM design choice (async execution between HyperEVM and HyperCore), not a bug Hyperliquid
  Labs is likely to "just patch" out from under this pitch before the deadline.
- **Known weakness, stated honestly**: the addressable population is genuinely small today
  (dozens of active CoreWriter integrators) — this will be the pitch's weakest leg on the
  Potential Impact/TAM judging criterion. The deck says so; the pitch video should frame this as
  "the standard debugging tool for a fast-growing category" rather than overstate current scale.

Required before presenting: apply the Phase-3 fixes in
`phase3-verification/corewriter-receipts-verdict.md` (add issue #174, soften the "no team
response" claim to scope it to GitHub only, note two unreachable search venues).

## If corewriter-receipts falls through: fallback order

1. **Ironwood Receipts** (Zcash) — fix the deck's grant-status claim first (cite the applicant's
   actual Orchard-only design and use "Ironwood-native, not just Orchard" as the explicit
   differentiator), then re-decide. Real risk, but survivable and still the best Zcash-track
   option researched.
2. **simd-diff** (Solana) — only pursue if committed to the narrow angle the verifier specified:
   auto-fetched real-mainnet-tx replay + genuine SIMD-0459/0460 runtime-stack-semantics checking,
   explicitly *not* competing with UpgradeGuard on breadth. Riskiest of the three since a live
   rival repo exists in the same window and could keep shipping.
3. **HL Clear-Sign** or **TEMPO-WATCHTOWER** (reserves, described in `01-shortlist.md`) — both
   scored 31/35 in Phase 1 but were never taken through Phase 2/3, so treat as unverified
   relative to the three above; would need a fresh deck + verification pass before building.

## Process note
Startup methodology (lean, start-small-target-users) was deliberately left as a placeholder in
all three decks per your instruction — fill that in once a final candidate is chosen.
