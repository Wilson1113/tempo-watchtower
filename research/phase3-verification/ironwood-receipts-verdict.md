# Ironwood Receipts — Verifier Verdict

Verified: 2026-09-25 (same day as deck). Verifier session independently re-fetched primary sources; did not trust the deck's quotes.

## VERDICT: **AT RISK**

Not dead, not fully solved by anyone — but the deck contains a **verifiably false claim** about ZCG grant #437 ("zero comments since submission"), and independent research surfaced a live, shipped, Rust, Orchard-native verification project (**ZAP1**) that the deck's competitive scan never searched for. Neither finding kills the pitch outright, but both must be fixed before this deck is presentation-ready, and one (the grant applicant's now-public technical design) meaningfully erodes the "novelty/first" framing unless the deck explicitly counters it.

---

## Check 1 findings — does the problem exist, right now?

**ZIP 311 — VERIFIED fresh (re-fetched independently this session).**
`https://zips.z.cash/zip-0311` confirmed: `Status: Draft`, `"TODO: Add support for Orchard."`, `"Reference implementation: TBD."` Matches the deck exactly. Tracking issue `zcash/zips#387` (VERIFIED via GitHub API) is still `open`, created 2020-08-03, and has exactly **1 comment**, from 2024-07-17 (str4d, a tangential note about "Sign Mode Textual"), last updated 2026-03-24. The gap is real and essentially dormant at the spec level.

**Ironwood/NU6.3 — VERIFIED independently** (shieldedlabs.net/ironwood/, fetched fresh, not reused from the deck). Activated as NU6.3 at block height 3,428,143 (~2026-07-28 12:00 UTC), in response to a counterfeiting vulnerability in the Orchard circuit found by Shielded Labs' Taylor Hornby; Orchard pool sealed via turnstile, all new shielded value flows into the new Ironwood pool. Matches the deck's claim.

**`orchard` crate — VERIFIED independently.** crates.io API confirms current stable **0.15.5**, published by ebfull (Sean Bowe), size 42,354 lines. docs.rs for `orchard::note_encryption` (fetched fresh) confirms `IronwoodDomain` and `IronwoodNoteEncryption` exist and are shipped (as type aliases, not structs — a harmless imprecision if the deck said "struct," but the deck itself didn't claim a specific Rust item kind, so this is not a real error). `zcash-receipt`-style crate names are unclaimed; `ironwood-receipt` does not exist on crates.io (checked directly) — no squatting risk.

**ZCG grant #437 — the important correction.** Re-fetched via GitHub API directly (`api.github.com/repos/ZcashCommunityGrants/zcashcommunitygrants/issues/437`):
- State: still `open`, labels `Ready For ZCG Review` — matches deck.
- **Comment count: 6, not 0.** The deck states twice (problem statement §2 and self-critique §8, point 1) that there have been "zero comments" / "no comments or activity beyond the original 2026-09-21 submission." This is **factually wrong** as of the same day the deck was written. The actual comment thread (all VERIFIED-fresh, fetched this session):
  - 2026-09-21: FPF/bot onboarding comment.
  - 2026-09-21: applicant (`inkedinlove`, real name per application: Joshua Kassabian) confirms budget breakdown, reports being unable to post to the forum.
  - 2026-09-21: `alexgbo` raises the applicant's forum trust level.
  - 2026-09-21: applicant posts the forum link (`forum.zcashcommunity.com/t/.../57729`).
  - **2026-09-22: `hhanh00`** (the real-world maintainer of **Ywallet**, one of the wallets the deck's own §8 lists as "doesn't do this") comments: *"the technical approach section doesn't give enough details about how this would work."*
  - **2026-09-22: applicant posts a long, technically fluent reply** laying out a complete design: sender-side derivation of a per-output "outgoing cipher key" (`ock`) via `Domain::derive_ock` (not the reusable OVK itself), a verifier that calls `zcash_note_encryption::try_output_recovery_with_ock` against chain-fetched ciphertext, explicit separation of "output evidence" vs. "chain confirmation" vs. "invoice binding," negative-test cases (wrong ock, tampered ciphertext, duplicate credit), and citations to ZIP 244 and Zebra RPC methods.
- The Zcash forum thread (`forum.zcashcommunity.com/t/grant-application-zcash-selected-payment-receipts-sdk/57729`, confirmed via Discourse JSON API) has 2 posts, last posted **2026-09-22**, matching the GitHub timeline.
- **No code has appeared.** I checked the applicant's own GitHub account (`inkedinlove`, real name "modestminer," 30 public repos) directly — none relate to Zcash, payment disclosure, or receipts; his prior grant-funded work was on Stellar/Avalanche. Searches for `CopperSeventhLLC` and `Kassabian` on GitHub returned nothing relevant. So the deck's "no code exists yet" conclusion is still correct — only its "no comments/no technical progress" framing is wrong.
- **A genuine silver lining for this pitch that the deck under-uses**: the applicant's detailed Sept-22 design is written entirely in terms of **Orchard**, not Ironwood — he explicitly frames Ironwood as a future add-on ("An Ironwood profile would use its corresponding domain... not be silently treated as Orchard"). Since Orchard is now sealed (turnstile-only, no new deposits post-2026-07-28), a Milestone-1 delivery scoped to "Orchard" as literally described would not, on its face, cover any *current* shielded payment. This is a real, defensible differentiation Ironwood Receipts should lead with explicitly — the deck currently only mentions this obliquely (§2, "ZIP 311's Sapling-only text is now stale... a strictly worse gap") rather than pointing directly at the rival grant's own Orchard-vs-Ironwood scoping gap.

**Net effect on Check 1**: the underlying protocol gap is real and unchanged. But the deck's evidentiary claim about grant #437 being static is wrong — the applicant has moved from "vague proposal" to "publicly posted, technically specific competing design" three days before the deck was written using this exact issue as its own top cited risk. That is a bigger, not smaller, threat to the "novelty" story than the deck represents, even though no code exists.

---

## Check 2 findings — does a competitor already exist?

All items independently checked this session (GitHub API, crates.io API, or WebFetch of the primary source), not reused from the deck.

| Candidate | Link | Date/activity (VERIFIED) | Solves the problem? |
|---|---|---|---|
| **ZAP1 Attestation Protocol** | github.com/Frontier-Compute/zap1 | **VERIFIED**: repo created 2026-03-27, **pushed 2026-09-23** (2 days before the deck was written). Rust, MIT. Companion crates independently confirmed live on crates.io: `zap1-verify` 0.2.1 (created 2026-03-30, 519 lines Rust) and `zcash-memo-decode` 0.1.1 (606 lines Rust, 524 downloads) | **No, but closest miss found.** Anchors application-chosen event commitments (BLAKE2b Merkle roots) into Orchard shielded memos so third parties can verify "this event happened" without keys. Different mechanism/claim type than Ironwood Receipts (proactive event attestation vs. retroactive proof of one already-mined payment's recipient/amount/memo). Does **not** do OVK-based note recovery or per-payment disclosure. But it is a real, live, actively-maintained, Rust, Orchard-native "verify without keys" tool that the deck's GitHub search terms (`zip311`, `orchard+receipt+proof+payment`, etc.) never surfaced — a genuine gap in the deck's search methodology, not a genuine gap in the market. |
| **Laminar / "Iron Core"** | github.com/darklight-labs/Laminar | **VERIFIED**: repo created 2026-01-31, **last pushed 2026-02-14** (dormant ~7 months), Rust, 0 stars. Zcash forum RFC (id 54302) confirms it explicitly names **ZIP-311 as a planned "Mode C (Future)"** feature | **No.** Treasury/admin "Receipt Bundle" tool linking a payment *plan* to its execution via hashing; its "Selective Disclosure" mode requires documented consent between parties, not independent cryptographic verification without key material. Dormant, no evidence of continued work; not a live threat, but confirms Ironwood Receipts is not the only builder who has looked at "receipts" + ZIP 311 in the same sentence. |
| **ZCG #437 applicant's design** | (see Check 1) | 2026-09-22 | **No — still no code.** But see above: the *design* is now public and detailed. |
| **Aweb Zcash "Private Agent Receipts"** | forum thread 55886 | **VERIFIED**: ZCG-**declined** 2026-05-29 ("outside current funding scope") | Not relevant — AI-agent execution-audit receipts, no Zcash payment proving at all. Dead end. |
| **GitHub repo-search, fresh, multiple query variants** (`zip311`, `zip-311`, `zcash+payment+disclosure`, `zcash+receipt+shielded`, `orchard+receipt+proof+payment`, `ironwood+receipt`, `zcash+selected-payment`, `orchard+disclosure`, `zcash+proof+of+payment`) | GitHub Search API | **VERIFIED fresh**, this session | 0 relevant hits except the same 2018 `Fmstrat/zcashd` Sprout-only Docker image the deck already found. Confirms deck's whitespace claim for these exact query terms — but Check 2's ZAP1 finding shows the query list itself was too narrow (missing "attestation," "verification layer," "anchor"). |
| **crates.io search** (`ironwood`, `zcash-receipt`, `ironwood-receipt`) | crates.io API | **VERIFIED fresh** | No relevant crate exists under these names; `ironwood` itself is an unrelated MCTS/Python-bindings crate. No naming collision. |
| **Zallet, Ywallet, Zingolib, Zashi/Zodl** | READMEs | Not re-fetched this session (deck's own VERIFIED-fresh checks accepted as sufficient given time budget — low risk of a README changing in a way that adds a disclosure feature in 24 hours) | Deck's conclusion (no disclosure feature) accepted, with light residual risk. |
| **Zcash forum broader search** (`payment disclosure`, `receipt`) via Discourse JSON API | forum.zcashcommunity.com/search.json | **VERIFIED fresh** | Surfaced ZAP1's grant thread, Laminar's RFC thread, and other tangential threads ("Order out of Chaos" = the actual title behind Laminar's RFC; "Zonp Wallet-native Commerce," "Shielding the Context of Commerce" — skimmed via search results only, not fetched in full; **UNVERIFIED** whether these two contain anything closer — flagged as an incomplete lead, not a confirmed dead end). |
| **hhanh00 (Ywallet maintainer)** | — | Confirmed via the grant-issue comment that he is actively watching Zcash grant/wallet proposals in this exact space as of 2026-09-22 | Not a competitor himself, but a signal that experienced wallet maintainers are already paying attention to this problem space — increases the chance a maintainer ships a quick feature if the idea gets more visible. |

**Two threads skimmed only via search snippet, not fetched** ("Zonp Wallet-native Commerce App," "Shielding the Context of Commerce: Closing the Metadata Gap") — marked **UNVERIFIED**, should be checked before final submission if time allows; titles suggest commerce/metadata adjacency but not confirmed to overlap with per-payment disclosure.

**Zallet/Ywallet/Zingolib/Zashi READMEs**: relying on deck's same-day fetch rather than re-fetching independently — the one place this verification is not fully independent, flagged honestly.

---

## TAM / business sanity check

The claimed user population — "low-thousands of active operators" (shielded merchants, exchanges/OTC desks doing shielded withdrawals, NGOs/donors needing audit proof) — is a plausible order of magnitude for a privacy-coin-specific niche in 2026, and is honestly **not** inflated; if anything it may be slightly generous, since almost none of it is independently sized (it's Phase 1's own inference, correctly flagged UNVERIFIED in the deck's evidence appendix, item 13). The three named sub-populations (merchants, exchanges, NGOs) are the right shape for the brief's "niche, big-co-neglected" strategy, and the existence of *two* separate ZCG-adjacent grant threads in this same conceptual space in 2026 (this grant, plus ZAP1's and Laminar's independent attempts at Zcash-native verification/receipt tooling) is itself evidence that a small but real paying/building community believes this problem is worth money — which supports rather than undermines the business-model section. The $50–500/mo hosted-API pricing is explicitly self-labeled UNVERIFIED/estimate in the deck and should stay labeled that way; it is a reasonable guess for infra-as-a-service to a low-thousands B2B market, not a researched number.

---

## Required fixes to the deck before it is presentation-ready

1. **Correct the false claim about grant #437.** Replace "zero comments" / "no comments or activity beyond the original 2026-09-21 submission" (§2 and §8 self-critique point 1) with the accurate picture: 6 comments, including a detailed technical design posted 2026-09-22 by the applicant in response to a challenge from Ywallet maintainer hhanh00. Do not let a judge who clicks the deck's own linked issue find a contradiction.
2. **Reframe the #437 risk explicitly around the Orchard-vs-Ironwood scoping gap.** The rival design is written for Orchard, which is sealed/turnstile-only post-2026-07-28; explicitly state that Ironwood Receipts is Ironwood-native from day one while the funded proposal treats Ironwood as a "future profile." This is the strongest, most concrete moat available and is currently under-emphasized.
3. **Add ZAP1 (github.com/Frontier-Compute/zap1, crates.io `zap1-verify`/`zcash-memo-decode`) to the competitive scan table**, with an explicit one-line "why it doesn't fully solve this" (proactive Merkle-anchored event attestation, not retroactive per-payment recipient/amount/memo disclosure) — omitting a live, actively-maintained, Rust, Orchard-native project from the competitive scan is a credibility risk if a judge or fellow entrant knows about it.
4. **Broaden and re-run the competitive keyword list** before final submission to include "attestation," "verification layer," "receipt bundle," and check the two unresolved forum leads ("Zonp Wallet-native Commerce," "Shielding the Context of Commerce") — currently UNVERIFIED, not confirmed dead ends.
5. Optionally note Laminar (dormant since Feb 2026, explicitly named ZIP-311 as future scope) as a minor "prior interest" data point strengthening the "this problem is real and others have circled it" narrative, without overstating it as competition.

None of these fixes require changing the core technical mechanism, the demo script, or the business model — they are evidentiary corrections and competitive-scan additions.
