# Competitor re-check — user-supplied "Who else is working on this?" (2026-09-27)

The user pasted an AI-generated-looking paragraph naming three groups "building watchtower
architecture specifically for machine payments." Each claim was checked directly (GitHub API,
raw READMEs, arXiv API, the Tempo spec). WebSearch was unavailable (session budget exhausted), so
absence findings are limited to what direct fetches could reach.

## Verdict per claim

| Claim | Status | Finding |
|---|---|---|
| CKB Fiber Network (FNN) has watchtower support | **TRUE (VERIFIED)** | `nervosnetwork/fiber` (Rust, 59★, pushed 2026-09-26) ships `crates/fiber-lib/src/watchtower/` (actor, store, RPC) + e2e force-close tests. README: "Watchtower support, make it easier for node operators." |
| …"optimized alongside MPP flows" | **MISLEADING (VERIFIED)** | In Fiber, "MPP" = **Multi-Path Payment** (README: "[x] Basic Multi-Path Payment (MPP)"), a Lightning routing feature — *not* Tempo's Machine Payments Protocol. Classic acronym collision. |
| …"porting battle-tested Rust watchtowers to the Tempo/Stripe ecosystem" | **NO EVIDENCE (VERIFIED absence in repo)** | Fiber's full source tree has zero paths mentioning tempo/stripe/evm/ethereum. Fiber is a CKB-native Lightning-style network. GitHub repo search "mpp watchtower" / "machine payments watchtower" → 0 results. |
| SilenTower "collateral-free decentralized watchtowers" research | **UNVERIFIED** | arXiv API returns no paper by that name; the only GitHub "SilenTower" is an unrelated 2023 personal repo. May exist at another venue. Even if real, it's Lightning/payment-channel-network research, not a Tempo MPP product. Related real literature exists (e.g. arXiv 2003.06127 "Fail-safe Watchtowers and Short-lived Assertions for Payment Channels", 2020). |
| Sei Labs warning that mass AI-agent channel closures will choke throughput | **UNVERIFIED** | No hits on blog.sei.io / blog.sei.io/research for "payment channel", "state channel", "watchtower", "MPP"; no sei-protocol repos about channels. If it exists, it's commentary that *supports* the thesis, not a product. |

## What we genuinely missed before (now corrected)
1. **Generic watchtower prior art was never framed.** Earlier scans searched Tempo/MPP-specific
   tooling only. Lightning watchtowers (LND's built-in tower, The Eye of Satoshi/`rust-teos`) and
   Fiber's watchtower are real, mature prior art for the *concept*. None targets Tempo MPP session
   channels — and, crucially, they can't be dropped in (see design finding below).
2. **MPP ecosystem directories were never checked.** Now checked:
   - `mbeato/awesome-mpp` (371 lines): no watchtower/keeper/forced-close entry. Settlement-adjacent
     entries (dexter-mpp "managed settlement", mpp-settlement-engine, settlegrid, sardis-guard-mpp,
     x402-proxy) are payer-side clients or billing/settlement layers; READMEs that could be fetched
     contain none of requestClose/grace/watchtower/forced/CloseRequested/ChannelClosed.
   - `mpp.dev/extensions`: no watchtower/keeper/forced-close entry.
   - New adjacent tool found: `amgb20/MPP-Inspector` (TypeScript, 3★, "Postman for HTTP 402") —
     has a `session` command (Preview) to *test* a payment-channel lifecycle manually. A dev/debug
     tool, not an always-on protector. Worth naming in the deck as adjacent.

## Design finding that matters more than any of the above (VERIFIED, spec §6.4.4 + §6.5)
The spec's access-control table: `settle` — **Payee only**; `close` — **Payee only**
("Only callable by the payee"). Consequences:
- A Lightning-style *third-party* tower (which holds pre-signed justice txs and needs no key)
  **does not port directly**: on Tempo, whoever submits `settle()`/`close()` must be the payee.
  This is exactly why an off-the-shelf Lightning/Fiber watchtower can't be reused — a real
  technical moat/insight for the pitch.
- MVP model: payee-operated tower on **independent infrastructure** holding a payee key — valid,
  and matches the "maintenance window" failure mode.
- Managed/third-party tier requires **scoped delegation**: e.g. payee = smart-contract account
  whose policy lets the tower's key call only `escrow.settle/close` for its channels (standard EVM
  pattern; Tempo-specific account/access-key support NOT yet verified), or a future spec change
  making `settle()` permissionless (safe-looking since funds only flow to the payee — a proposal,
  not a fact). This is the product's cryptography/design core and belongs on the roadmap slide.
- The deck's earlier "a watchtower is by design a third party, like Lightning" argument must be
  reworded: on Tempo the independence is *infrastructure/on-call independence*, and true
  third-party operation is the hard, unsolved design problem this product would own.
