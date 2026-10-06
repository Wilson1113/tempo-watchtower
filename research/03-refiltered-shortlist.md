# Refiltered shortlist — the 4-point niche test

Prompted by a direct question: "will corewriter-receipts even find a single person who really
wants to pay for it?" That question exposed a real gap in the original 7-metric scoring rubric
(00-brief.md), which weighted evidence/solo-feasibility/demo-ability but had no explicit test
for *willingness to pay* or *permanence of the competitive gap*. The user supplied the fix — a
mandatory 4-point test, all four required:

1. Named and reachable users (ideally in the hackathon's own community)
2. Real cost — money, time, or regulatory risk, not just annoyance
3. STRUCTURAL big-co neglect — not merely "haven't gotten to it yet"
4. Enjoyable work — cryptography, VMs, concurrent ledgers

All 26 candidates sourced across the 5 Phase-1 scout files were re-screened against this test
(no new Phase-1 scouting was run — the existing evidence base was reused; this session's
WebSearch budget was already exhausted, 200/200, before this re-screen began, so the audit used
the already-fetched primary sources plus targeted WebFetch re-checks rather than fresh discovery
searches). Full candidate-by-candidate reasoning is in the conversation; this file records the
outcome.

## Tier 1 — passes all four cleanly

### T1 — HL Clear-Sign (Hyperliquid, cryptography)
Source: cryptography.md candidate C2 (score 31/35 under the old rubric — highest of any idea
in the whole research pass).
- Users: HIP-3 deployers (500k HYPE staked, named), Hyperliquid validators, multisig
  treasuries/trading desks — named, in-track.
- Cost: catastrophic fund theft via blind signing — directly evokes the Bybit Feb-2025 hack
  ($1.5B) this whole research effort's original brief was framed around.
- Structural: hardware wallet vendors (Ledger/Trezor) have no near-term commercial reason to
  build bespoke clear-signing for one L1's msgpack+keccak scheme; SEAL's own published security
  framework covers Safe and Squads but has zero Hyperliquid coverage — a named security body
  admitting the gap, not just this research finding it.
- Domain: cryptography (hash recomputation, EIP-712 decoding).

### T2 — TEMPO-WATCHTOWER (Tempo, concurrent ledgers)
Source: async-infra.md candidate A2 (score 31/35, tied-highest).
- Users: payees running metered/streamed servers on Tempo's MPP session channels — small
  (hundreds of servers) but named, gathered in Tempo's own dev channels, in-track.
- Cost: total, direct loss of unsettled funds if a payee misses the spec's own 15-minute
  forced-close grace period — the spec itself calls "maintenance windows" an expected failure
  mode, i.e. this WILL happen to real operators.
- Structural: a watchtower is *by design* meant to be an independent third party — Tempo Labs
  building this in-house would defeat the point (the same reason Lightning watchtowers are a
  distinct ecosystem role that Lightning Labs itself doesn't ship). This is the cleanest
  "permanent, not circumstantial" neglect story in the whole pool.
- Domain: concurrent-ledger/payment-channel state machines.

### T3 — upgrade-lens (Solana, VM + cryptography)
Source: vm.md candidate VM-05 (score 28/35).
- Users: Squads multisig signers, DAO/protocol governance/security councils approving program
  upgrades — named, Solana's home-turf community (the most-judged track).
- Cost: a signer blindly approving a malicious/buggy program upgrade → direct fund loss — the
  program-level analog of the Bybit blind-signing narrative, but for Solana program bytecode.
- Structural: SEAL's own wallet-security framework admits "Limited tooling is available for
  Solana verification compared to EVM." The real structural point: OtterSec/Zellic/Sec3 already
  do exactly this analysis by hand as *billable audit work* — they are structurally
  disincentivized to ship a free tool that cannibalizes their own revenue.
- Domain: sBPF bytecode diffing (VM) + signer security (cryptography).

### T4 — zkVM Accelerator Conformance & Differential Test Kit (Ethereum L1, VM + cryptography)
Source: vm.md candidate VM-03 (score 28/35).
- Users: 6-8 named zkVM teams (SP1/Succinct, RISC Zero, OpenVM, ZisK, Airbender, Jolt, Pico)
  building in public on GitHub — extremely reachable, technical, in-track.
- Cost: this catches SOUNDNESS bugs — a wrong-but-accepted proof means a fabricated state
  transition gets treated as valid on-chain. Already found 2 real, disclosed bugs in 2026
  (OpenVM's `ecrecover` returning a wrong key while reporting success; SP1's BLS12-381 encoding
  bug) — the highest-stakes "cost" story of any candidate in the entire research pass.
- Structural: an EF-run standards repo issue has sat open 6+ months explicitly asking someone to
  build this; each zkVM vendor tests its own accelerator in isolation (how both bugs shipped),
  and no single vendor has an incentive to build a cross-vendor conformance suite — a textbook
  public-good market failure.
- Domain: RISC-V VM internals + cryptographic-primitive correctness — the deepest technical
  intersection of the four.

## Tier 2 — 3 of 4, kept as reserves
- **zec-ledger** (Zcash tax-export CLI): strong on cost (regulatory/tax risk) and structural
  (horizontal tax software won't do deep shielded-Zcash engineering for a small slice), but
  further from deep VM/crypto research and more applied plumbing.
- **Ironwood Receipts / zkreceipt** (Zcash payment disclosure): good on 3, but the "structural"
  claim is weaker — reads as ECC/ZF being busy with a 2026 crunch (circumstantial) rather than
  permanently uninterested, and Phase 3 verification already found a live competing grant
  application with a real technical design posted.

## Confirmed drops (fail at least one filter outright)
- **corewriter-receipts / HL-COREWRITER-TRACE** (Hyperliquid): fails #2 — real bug, zero
  competitors, but the cost is engineer debugging-hours, DIY-able in an afternoon once
  understood, not money/time/regulatory risk at stake. This was the finding that triggered the
  whole re-screen.
- **noncewarden** (Solana durable-nonce lifecycle): fails #3 — Sec3, txscope-hq/txscope, and
  AaronTan11/solana-nonce-guard all shipped within weeks of the Drift hack; this is actively
  being served, not neglected.
- **corpaction-rs** (Robinhood Chain corporate actions): fails #1 — there is really only one
  current relevant counterparty (Robinhood itself), not a reachable community.
- Everything else in the 26-candidate pool: either fails #2 (annoyance-tier pain: HL-NODE-GUARD,
  R1 scaled-balance's diffuse-adoption-gap framing) or is too early/thin to test (zone-witness,
  qsk-vault, ct-auditor's near-zero current usage) or is now openly crowded (D1, Zcash FROST
  wallets generally).

## Next step
Phase 2 (decks) + Phase 3 (adversarial verification) for the Tier-1 four. Given the session's
WebSearch budget is exhausted, deck-builders and verifiers should lean on WebFetch (specific,
known URLs) rather than WebSearch (broad discovery) — this is a re-verification/deepening pass
on already-sourced candidates, not blind discovery, so it should still work well.
