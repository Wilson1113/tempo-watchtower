# Phase 1 → Phase 2 shortlist: synthesis across all 5 scouts

Selection method: cross-scout convergence weighted above any single scout's score. When 2-3
independently-briefed scouts (different domains, different search paths) land on the *same*
underlying problem, that's a stronger signal the problem is real and reachable than any one
scout's self-assessed score. Track diversity was a secondary tiebreaker (avoid an all-eggs-in-
one-track shortlist for a solo entrant who submits once).

## Convergent problems (found independently by 2+ scouts)

1. **HyperEVM ↔ HyperCore CoreWriter silent-failure gap** — found by VM scout (VM-02,
   "corewriter-verify", 30/35), async scout (A4, "HL-COREWRITER-TRACE", 24/35), and user-first
   scout (H1, "corewriter-receipts", 31/35). Three scouts, three independent GitHub searches,
   all confirmed **zero** competing crates/repos. Track: Hyperliquid ($100k/10).
2. **Zcash ZIP 311 payment-disclosure gap, now Ironwood-relevant** — found by cryptography scout
   (C1, "Ironwood Receipts", 30/35) and user-first scout (Z1, "zkreceipt", 29/35). Both
   independently found ZIP 311 has said "Reference implementation: TBD" since 2020, and that a
   competing $42k grant (ZCG #437, filed 2026-09-21) won't deliver until after our deadline.
   Track: Zcash ($100k/10) — the track hackathon-intel scout flags as having the best odds
   (Colosseum's own co-founder predicts a flood of shallow "ZEC the asset" submissions).
3. **Solana SIMD-0500 / SBPFv3 forced-migration crunch** — found by VM scout (VM-01, "Solana
   Runtime-Change Impact Simulator", 30/35) and hackathon-intel scout (C3, "sbpf-fleet", 27/35).
   Both independently found the SIMD text itself ("17,279 programs using SBPFv0"), the `call -1`/
   `CallDepthExceeded` footgun, and near-zero existing tooling (2 toy repos, 0 for "SIMD-0500").
   Track: Solana ($100k/10) — most crowded track overall, but this specific sub-niche is thin.

## Shortlist for Phase 2 (pitch decks)

| # | Working name | Track | Domain | Scores (by scout) | Why this one of the converging pair |
|---|---|---|---|---|---|
| 1 | **corewriter-receipts** | Hyperliquid | async/cross-VM | 31, 30, 24 | Best-evidenced single problem in the whole research pass (3-way convergence); pick H1's framing (crate + daemon) as the base, folding in VM-02's static footgun/pre-flight ideas |
| 2 | **Ironwood Receipts** | Zcash | cryptography | 30, 29 | Higher-scoring of the pair (cryptography C1); composes with already-shipped audited `orchard` v0.15.0 primitives, cleanest "fill a spec gap first" story |
| 3 | **simd-diff** (Runtime-Change Impact Simulator) | Solana | VM/execution | 30, 27 | Higher-scoring of the pair (VM-01); more technically sophisticated (replay+diff+static-analysis vs. a simpler fleet scanner), quantified affected population (17,279 programs) is a strong "Potential Impact" judging-criterion hook |

## Reserves (strong single-scout ideas, not carried to Phase 2 unless a finalist fails verification)
- **HL Clear-Sign** (cryptography C2, 31/35 — highest single score of any idea across all scouts) — independent verifier for HyperCore multisig blind-signing, directly evokes the Bybit-hack framing from the original brief. Not carried forward only to avoid 2 of 3 finalists sharing the Hyperliquid track.
- **TEMPO-WATCHTOWER** (async A2, 31/35) — settlement watchtower for Tempo's brand-new (1-day-old at research time) payment-channel spec. Not carried forward because it's single-scout, and the addressable ecosystem is explicitly small today ("hundreds of servers").

## What's explicitly rejected (do not resurrect without new evidence)
See each scout file's own "Rejected ideas" section for full reasoning. Headline avoid-list from
hackathon-intel: consumer/social trading apps, prediction-market aggregators, generic AI-agent
trading, stablecoin remittance dApps, hardware wallets (Unruggable dominates), generic RPC
(FluxRPC already won), generic HIP-3 oracle services (Pyth/RedStone already there), FROST
shielded-wallet products (5+ competing entries just shipped in ZecHub Hackathon 3.0, Aug 2026).
