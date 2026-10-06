# HL Clear-Sign — Pitch Deck (Phase 2)

Prepared: 2026-09-25. Track: Hyperliquid ($100k / 10 winners, HyperCore or HyperEVM). Domain: Cryptography.
Status legend: **VERIFIED-fresh** = fetched by this deck-builder agent today, 2026-09-25 (WebFetch/curl, this session). **INHERITED** = from Phase 1 (`research/phase1-scouting/cryptography.md`, candidate C2), not independently re-fetched this session. **UNVERIFIED** = snippet/inference only, fetched by no one.

---

## 1. Title + one-liner

**HL Clear-Sign**
*A pure-Rust crate + CLI that independently decodes every HyperCore multisig action and recomputes the exact hash you're about to sign — from audited primitives alone — so a silently-swapped withdrawal destination gets caught before, not after, your signature.*

Working name kept as given. A sharper alternative considered and rejected: **"Hypersig-Verify"** — rejected because "Hypersig" is already the name of an existing hosted signing-convenience product (see §8), and pairing our name with theirs would blur the exact distinction the pitch needs to make (we verify *content*, existing tools verify *identity*/coordinate *signing*). "HL Clear-Sign" is kept because it names the specific security property (clear-signing, the opposite of blind-signing) this tool restores, and reads correctly as a crate name (`hl-clearsign`) and CLI binary name.

---

## 2. Problem statement

**Who exactly hits this** (named, reachable, in-track):
- **HIP-3 market deployers** — anyone deploying a builder-deployed perpetual market on HyperCore must stake **500,000 HYPE on mainnet** (VERIFIED-fresh, fetched today from `hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-improvement-proposals-hips/hip-3-builder-deployed-perpetuals`: *"The staking requirement for mainnet will be 500k HYPE"*). Deployers commonly manage this stake and the associated dex-deployer permissions via multisig. (Phase 1's USD estimate of this stake, ~$25M, is **INHERITED** and not re-priced this session — HYPE's spot price was not re-checked.)
- **Hyperliquid validators running multisig for validator operations** — a named, real use case: `Pier-Two/hyperliquid-multi-sig-actions`, description re-confirmed fresh today: *"Hyperliquid multi-signature actions with a focus on validator operations"* (VERIFIED-fresh, GitHub API, last pushed 2026-01-08).
- **Small trading-desk / DAO treasuries** running raw HyperCore native multisig or a signing-convenience layer (Hypersig, `open-multisig-hl`) on top of it.

**The mechanism that creates the gap** (re-derived today directly from the maintained reference implementation, `hyperliquid-dex/hyperliquid-python-sdk`, file `hyperliquid/utils/signing.py`, fetched fresh in full — VERIFIED-fresh):
- Every ordinary HyperCore action is hashed as `action_hash = keccak256(msgpack.packb(action) || nonce(8 bytes, big-endian) || vault_flag[+vault_addr] [+ 0x00 + expires_after])`, then wrapped as an EIP-712 `Agent{source, connectionId}` message (`construct_phantom_agent`) signed under domain `{chainId: 1337, name: "Exchange", verifyingContract: 0x0}`.
- For a **multisig** action specifically, `sign_multi_sig_action` computes `multi_sig_action_hash = action_hash(action_without_tag, vault_address, nonce, expires_after)` — i.e. the *real* action, stripped of its `type` tag — and then each individual authorized signer signs a **separate, tiny envelope**: `{"multiSigActionHash": <bytes32>, "nonce": <uint64>}` under EIP-712 type `HyperliquidTransaction:SendMultiSig` (`MULTI_SIG_ENVELOPE_SIGN_TYPES`). **This confirms the exact blind-signing surface precisely: in the standard flow, what an individual multisig participant is asked to approve is a bare `bytes32` hash plus a nonce — not the underlying action at all.** A hardware wallet without bespoke Hyperliquid clear-signing support shows exactly that: an opaque 32-byte value.
- For L1 actions signed through the multisig path, the envelope hashed is `[payload_multi_sig_user, outer_signer, action]` (a 3-element msgpack array), and for user-signed multisig actions (`usdSend`, `withdraw3`/`Withdraw`, `spotSend`, `convertToMultiSigUser`, etc.) the fields `payloadMultiSigUser`/`outerSigner` are spliced directly into the action's EIP-712 type list right after `hyperliquidChain` (`add_multi_sig_types`/`add_multi_sig_fields`). Both paths funnel into the same opaque-hash-only outer signature a participant actually approves.
- Confirmed fresh today, official HyperCore docs (`hyperliquid.gitbook.io/hyperliquid-docs/hypercore/multi-sig`): *"A `MultiSig` action wraps around any normal action and includes a list of signatures from authorized users"* — i.e. every action type in the system (order, transfer, withdrawal, dex-deploy, validator op) is a candidate for this exact opaque-hash-signing pattern. (VERIFIED-fresh.)

**Independent framing of the risk, re-fetched today** (`hyperliquidguide.com/guides/getting-started/best-hardware-wallet-for-hyperliquid`, published 2026-07-25, updated 2026-09-23 — VERIFIED-fresh):
- *"Ledger has shipped clear-signing support for specific EIP-712 schemas, but the coverage is incomplete, and Hyperliquid's schemas are not guaranteed to be among them."*
- *"Blind signing is exactly what it sounds like: approving something you cannot read."*
- *"Blind signing plus a phishing site is the combination that empties accounts."*

**Structural neglect, re-verified fresh today**: `frameworks.securityalliance.org` (SEAL's own published security-practices site) has a dedicated "Signing and Verification" section with step-by-step pages for **Safe** (`secure-multisig-safe-verification`) and **Squads** (`secure-multisig-squads-verification`) — full sitemap fetched today, confirming **no Hyperliquid page exists anywhere on the site** (VERIFIED-fresh). SEAL's own Safe-verification page states: *"Safe verification requires matching transaction hashes and decoded intent across independent channels before any signature"* — precisely the property this pitch proposes for HyperCore, which SEAL has not built. The general signing-verification page adds: *"Never sign blindly. Prefer the hardware wallet screen and independently verified calldata as ground truth over any web UI claim"* (VERIFIED-fresh).

**Official tooling gap, re-confirmed fresh today**: the maintained `hyperliquid-dex/hyperliquid-rust-sdk` — re-fetched via GitHub API today — has **zero** `multisig`/`multi_sig` occurrences anywhere in `src/exchange/exchange_client.rs`, no `multisig.rs` file in its `src/exchange/` directory listing, and its last push is **2025-10-21** (11 months stale as of today, VERIFIED-fresh) — the official Rust SDK still has no multisig support at all.

**Background context (INHERITED, not re-verified this session):** this blind-signing pattern is the failure mode implicated in the February 2025 Bybit hack (a Safe multisig transaction whose displayed content did not match its signed content). I attempted to re-fetch a primary source for this specific claim this session and could not (one guessed URL returned DNS failure, a second returned 404); it is retained here as inherited background framing from the research brief, not as a freshly-verified citation, and should not be quoted as freshly sourced.

---

## 3. What we are

**Ships as:** a Rust crate (proposed name `hl-clearsign`) + CLI binary of the same name, optionally fronted by a tiny local `axum` UI (still a Rust binary, no server-side component, no telemetry).

**Exact technical mechanism** (decode → recompute → compare → flag):
1. **Decode.** Parse an incoming action (JSON as received from any coordinator — Hypersig, `open-multisig-hl`, a raw `/exchange` payload, a QR code) into HyperCore's full action-type table (`order`, `withdraw3`/`Withdraw`, `usdSend`, `spotSend`, `usdClassTransfer`, `sendAsset`, `convertToMultiSigUser`, `perpDeploy::*` (10 sub-variants confirmed today via a third-party Rust SDK's action tree, e.g. `setOracle`, `setMarginTable`, `toggleTrading`, `registerAsset`, etc.), `spotDeploy::*`, `tokenDelegate`, `approveAgent`, `approveBuilderFee`, and the multisig envelope itself), rendering each into a human-readable summary (e.g. `withdraw3: 50,000 USDC → 0xBBBB…`).
2. **Recompute.** Independently rebuild `action_hash`/`connectionId` (via `rmp-serde` for msgpack — matching the reference implementation's `msgpack.packb` byte-for-byte — and `sha3`/`tiny-keccak` for keccak256) for L1 actions, and the EIP-712 typed-data hash (via `alloy`/`k256`) for user-signed actions and the outer `SendMultiSig` envelope — using *only* audited, widely-used Rust primitives, never a bespoke reimplementation of msgpack or keccak.
3. **Compare.** Print the recomputed `multiSigActionHash` (and, for the outer envelope, the exact `HyperliquidTransaction:SendMultiSig` hash) in the same encoding a hardware wallet would show, so a signer can character-compare it against their Ledger/Trezor screen — the two-device cross-check pattern SEAL recommends for Safe (§2) but that has no Hyperliquid-specific tooling today.
4. **Flag.** Layer a lightweight anomaly/risk pass on top of the decode (not present in any competitor found, see §8): flag destinations/addresses not seen in the wallet's own prior action history, flag amount/permission changes vs. a locally-cached "last approved" baseline, and flag any msgpack-key-ordering discrepancy between the wire bytes and the canonical action struct (the exact class of bug `kinetiq-research/hl-rs`'s validator had to special-case — see §8 — because `serde_json` key order isn't guaranteed to match what was actually hashed).

**Why pure Rust specifically:** the entire trust chain here rests on msgpack and keccak256 being bit-for-bit correct — any language mismatch or reimplementation is exactly the kind of subtle bug this tool exists to prevent others from having. `rmp-serde`, `sha3`, and `alloy`/`k256` are the same class of audited, widely-depended-on crates the reference Python SDK's `msgpack`/`eth_utils.keccak` occupy in Python — using them from Rust means zero new cryptography, only independent re-derivation in a memory-safe, statically-typed binary that a signer can build and run offline, air-gapped from whatever machine assembled the original proposal.

**Who's building this** (minimal, per instructions): solo builder, submitting individually to Colosseum's Crypto World's Fair. No founder-fit narrative is claimed for this pitch.

---

## 4. Tech demo (3-minute video, minute-by-minute)

- **0:00–0:20 — Set the scene.** Screen recording of a Telegram/Discord group chat: three multisig signers discuss and verbally approve a `withdraw3` of 50,000 USDC to an address ending `…AAA1`, screenshotted and pasted in chat for "everyone to check."
- **0:20–0:45 — The swap.** Cut to the coordinator's terminal: they assemble the actual action JSON sent to signers, but the destination is now `…BBB2` — one flipped address, no other visible change. Show the raw JSON payload on screen exactly as it would arrive at a signer's machine.
- **0:45–1:25 — What existing tooling shows.** Split screen: (a) a bare hardware-wallet-style prompt showing only `multiSigActionHash: 0x7f3a9c…`, `nonce: 1758…` — an opaque bytes32, matching the reference SDK's actual envelope (§2) — with no way to tell `…AAA1` from `…BBB2`; (b) for contrast, note on screen (text overlay, not narrated as a formal claim) that the two closest Rust prior-art projects found in research — a signature-authorization validator and a P2P-coordination CLI's own debug-print — would each pass this transaction too, because the signature really is valid and, in the CLI's case, its own decode trusts its own P2P channel rather than independently cross-checking a hash.
- **1:25–2:15 — `hl-clearsign verify`.** Run the CLI against the same JSON. Terminal output: `withdraw3: 50,000 USDC → 0xBBB2… [⚠ UNSEEN ADDRESS — not in this wallet's prior withdrawal history]`, followed by the independently recomputed `multiSigActionHash` and outer `SendMultiSig` hash, printed for direct comparison against the hardware wallet screen.
- **2:15–2:45 — Reject and recover.** Signer, now informed, rejects; cut back to the group chat to show the originally agreed `…AAA1` — the mismatch is unambiguous and happened *before* any signature was produced, not after funds moved.
- **2:45–3:00 — Close.** One line on screen: *"Existing tools check who signed. HL Clear-Sign checks what you're signing."* Repo/crate link.

---

## 5. Business model

**Applying the 4-point niche filter explicitly (per `03-refiltered-shortlist.md`):** this is the load-bearing test for this idea, so it is answered directly rather than folded into a generic SaaS pitch.

- **Named, reachable users:** HIP-3 deployers (500k HYPE staked, gathered around Hyperliquid's own Discord/dev channels), validators running multisig ops (the `Pier-Two` use case), trading-desk/DAO treasuries on Hypersig or native multisig — all named and in-track (§2).
- **Real cost, not annoyance — and specifically CATASTROPHIC, not incremental:** the failure mode this tool targets is a successfully-executed, fully-authorized-looking, irreversible transfer of the entire multisig's controlled funds — for a HIP-3 deployer, that includes the 500k-HYPE stake and associated dex-deployer control; for a trading desk, the full treasury. This is categorically different from the debugging-hours cost that got `corewriter-receipts` rejected in the Phase 1 re-screen (`03-refiltered-shortlist.md`, "Confirmed drops"): a debugging tool saves an engineer an afternoon; this tool is the difference between a treasury existing tomorrow or not. A team with real stake/treasury at risk pays for loss-prevention on a completely different willingness-to-pay curve than a team paying for a convenience feature — the same reason organizations pay for a Safe/Squads verification workflow (SEAL's own documented best practice, §2) rather than shrugging off blind-signing as a minor UX papercut.
- **Structural, not circumstantial, neglect:** confirmed fresh this session (§2, §8) — SEAL's own framework covers Safe and Squads by name and has zero Hyperliquid coverage; the official `hyperliquid-rust-sdk` has had no multisig support for 11 months; hardware-wallet vendors have no near-term commercial reason to build bespoke EIP-712/msgpack clear-signing for one L1's bespoke action-hash scheme when broader EVM-schema coverage serves more customers per engineering-hour. This has not changed since Phase 1 and re-checking it fresh did not surface any sign it is about to change.
- **Enjoyable work:** msgpack/keccak/EIP-712 decode-and-recompute engineering, squarely in the cryptography domain.

**Who pays, how, realistic pricing:**
- **Free, open-source core** (MIT/Apache-2.0): the `hl-clearsign` crate and CLI, decode + recompute + compare for every action type. Free is required both for judging criterion (e) Open-source and because a verification tool nobody can audit undermines its own trust claim — this mirrors every comparable project surfaced in this research pass (Ironwood Receipts, `zec-ledger`, SEAL's own tooling recommendations).
- **Paid: hosted/self-hosted multi-signer coordination with audit logging and anomaly alerts** (Slack/Telegram/email on any decoded action that fails a saved baseline — new destination, changed threshold, changed dex-deployer permission) — priced per multisig wallet/seat, positioned near what named comparable convenience tooling in this exact space (Hypersig, a hosted signer-UI product) already charges HIP-3 deployers and trading desks for signing convenience; this pitch does not have a verified Hypersig price point to cite, so no specific dollar figure is asserted here (flagged UNVERIFIED rather than invented) — the go-to-market claim is comparability to that pricing tier, not a fabricated number.
- **Paid: custom-decoder support** for bespoke HIP-3 market action patterns (each HIP-3 deployer can define non-standard `perpDeploy` sub-actions) — integration consulting, billed per engagement.

---

## 6. Startup methodology

On hold — to be filled in later.

---

## 7. Track fit & judging-criteria mapping

- **(a) Functionality:** the demo (§4) shows a real, live decode-recompute-compare-flag cycle against an actual tampered action, not a mocked screen — the hash math is independently checkable by any judge who runs the CLI against the reference SDK's own test vectors.
- **(b) Potential Impact:** every HyperCore multisig wallet — HIP-3 deployers, validators, treasuries — is a candidate user; the addressable set grows with HyperCore's own growth, and the tool composes with (does not compete against) every existing signing-convenience product in the ecosystem.
- **(c) Novelty:** the specific combination of (i) full HyperCore action-type coverage, (ii) independent hash recomputation from raw wire bytes, and (iii) semantic risk-flagging is not currently shipped together anywhere found in this research pass (§8) — though two of the three pieces now exist separately in other projects, which is the most important honest caveat in this deck (see §8).
- **(d) UX:** turns an opaque `bytes32` a signer cannot evaluate into a human-readable, risk-flagged summary paired with the exact hash to cross-check on a hardware wallet — a direct, concrete UX improvement over the status quo described in §2.
- **(e) Open-source:** MIT/Apache-2.0 core, composing with the existing HyperCore ecosystem (Hypersig, `open-multisig-hl`, any coordinator) rather than replacing it — it reads the same action JSON any of them already produce.
- **(f) Business Plan:** §5 names payers, a real (not invented) cost-avoided story, and a pricing tier positioned against existing comparable convenience tooling rather than an unverified number.

---

## 8. Competitive scan (for the verifier) — be the harshest critic

This section is where fresh research **materially weakens** Phase 1's whitespace claim, and that should be stated plainly rather than softened.

**Phase 1's finding, verbatim (`cryptography.md`, C2):** *"None independently recompute and cross-check the action hash against a decoded, human-readable action."* Re-checking this today, across a wider set of repos than Phase 1 searched, **this claim no longer holds cleanly.** Two real, currently-maintained Rust projects were found this session that were not in Phase 1's 4-repo GitHub search:

- **`kinetiq-research/hl-rs`** (8 stars, created 2025-11-02, last pushed 2026-08-07, README: *"Beta Software - Not Production Ready"* — VERIFIED-fresh). Ships `src/actions/multisig_validation.rs`, whose own doc-comment states its purpose almost word-for-word what this pitch proposes: *"Given a request in exactly the shape Hyperliquid's `/exchange` endpoint receives, we recompute the hashes each party signed and recover their addresses, so a leader (or an off-chain policy engine) can decide whether the transaction is authorized before it is broadcast."* Reading the full source (VERIFIED-fresh, fetched in full today) confirms it really does: reconstructs the canonical wire action (handling key-order drift between `serde_json` and the original msgpack hash — a subtlety Phase 1 did not surface), recomputes the inner L1 `connectionId` and the outer `SendMultiSig` envelope hash, recovers each signature's address via `alloy_signer`, and checks membership against an authorized-signer set plus a numeric threshold. **This is the single closest piece of prior art to our core mechanism found in the entire research pass, and it must be named directly in the pitch, not glossed over.**
  - **Why it still doesn't solve this problem**, confirmed by direct source inspection: (1) it explicitly refuses user-signed inner actions — `MultisigValidationError::UserSignedInnerNotSupported`, with an error message pointing to a `validate_multisig_user_signed_action` function that **does not exist anywhere in the fetched repository** (confirmed via a full recursive tree fetch and a full read of the sibling `multisig.rs`, which contains only signing-side helpers) — meaning it cannot validate `withdraw3`, `usdSend`, `spotSend`, or `convertToMultiSigUser`, i.e. every actual fund-movement or permission-change action type, the exact highest-stakes category this pitch targets; only lower-stakes L1 actions (`order`, `perpDeploy`, etc.) are covered.
  - (2) It checks **who** signed (authorization + threshold), not **what** was signed semantically. A coordinator who successfully tricks every authorized signer into blind-approving a swapped destination produces a report this validator would call fully valid — every signature really did come from a real authorized key over the real (tampered) bytes. It does not claim, and structurally cannot, catch the Bybit-style failure mode this pitch targets.
  - (3) No CLI, no human-readable decode, no risk-flagging anywhere in the repository (confirmed via full recursive file-tree scan) — it is a library function requiring a caller to write Rust code invoking it inside their own backend/policy engine; it is not a tool an individual signer runs before clicking "approve."
  - (4) Low adoption (8 stars) and effectively stalled (no push in ~7 weeks as of today).

- **`infinitefield/hypersdk`** (219 stars, pushed 2026-09-23 — actively maintained, VERIFIED-fresh). Ships `hypecli`, a CLI with real peer-to-peer multisig coordination (`hypecli/src/multisig.rs`) *and* Trezor hardware-wallet integration (`hypecli/src/trezor.rs`, including `scan_hw_signers`). Reading the actual `sign()` function (VERIFIED-fresh, fetched in full) shows a receiving signer's flow: `validate_proposal(...)` runs, then `println!("{:#?}", action)` prints a Rust `Debug`-derived dump of the decoded action struct, then prompts `"Accept (y/n)?"` before signing.
  - **Why it still doesn't fully solve this problem:** the printed decode is the CLI's own interpretation of whatever it received over its own P2P channel — there is no independent recomputation of the signed hash cross-checked against what a hardware wallet will separately show; if `hypecli` itself (or the P2P transport) is compromised or has a decode bug, there is no second, independently-derived check, which is the exact "independent" property this pitch is built around. It also only works inside `hypecli`'s own coordination flow — it cannot be pointed at an arbitrary action JSON produced by Hypersig, a raw `/exchange` payload, or any other coordinator, which is a meaningful use-case restriction our standalone CLI does not have. No semantic risk-flagging (unseen address, changed permission, etc.) was found anywhere in the repo. What `validate_proposal()` itself actually checks was not read this session and is an open question — noted honestly rather than assumed favorable or unfavorable.

**Verdict for the verifier:** the honest positioning after this session's research is **not** "nobody does this" — it is "two real, Rust, currently-not-abandoned projects each do *part* of this, and neither does the *whole* thing (standalone + full action-type coverage including withdrawals/transfers + independent hardware-wallet-hash cross-check + semantic risk-flagging), under any name." The pitch should name both competitors explicitly and make the differentiation argument on its merits, because a judge who searches GitHub will find them in minutes.

**Other competitors, re-checked fresh today (all previously found in Phase 1, all confirmed unchanged in substance):**
- `tradingstrategy-ai/open-multisig-hl` (Svelte signer UI, wallet-connect based; re-confirmed pushed 2026-06-24, 0 stars) — relies on MetaMask/Rabby's own (per §2, incomplete) EIP-712 decoding; no independent recomputation.
- `tholos-inc/hyperliquid-multisig-cli` (Python, re-confirmed last pushed 2025-03-09, 1 star) — stale, no verification claim.
- `Pier-Two/hyperliquid-multi-sig-actions` (Python, re-confirmed pushed 2026-01-08, 0 stars, "Use at your own risk") — validator-ops signing convenience, no independent verification.
- **`hyperliquid-dex/hyperliquid-rust-sdk`** (official, 473 stars, re-confirmed pushed 2025-10-21) — zero multisig support of any kind.

**Broader unofficial-Rust-SDK landscape (new this session, via fresh GitHub API search "hyperliquid rust sdk", 27 total repos returned):** the two competitors above are the only ones with any multisig-adjacent verification code found; spot-checking the next-largest (`ControlCplusControlV/ferrofluid`, 130 stars) found no multisig files anywhere in its file tree at all (VERIFIED-fresh). The remainder are plain trading/market-data SDKs.

**Naming/whitespace check (new this session):** GitHub repository search for `hyperliquid clearsign`, `hyperliquid clear-sign`, `hyperliquid action hash`, `hyperliquid verify signing`, and `hyperliquid msgpack` each returned **0 results** today (VERIFIED-fresh) — no project has staked out this specific framing or name.

**Rejected-idea consistency check:** this reconfirms Phase 1's own note that the *signing-convenience* layer (Hypersig, `open-multisig-hl`) is well-served and correctly out of scope — the new finding is that the *validation/verification* layer, believed wide open in Phase 1, now has two partial, genuine occupants.

---

## 9. Evidence appendix

| Claim | Status | Source |
|---|---|---|
| `action_hash` = keccak256(msgpack(action) \|\| nonce \|\| vault_flag[+addr][+expires_after]); `construct_phantom_agent`; `Agent{source,connectionId}` EIP-712 domain | VERIFIED-fresh | `raw.githubusercontent.com/hyperliquid-dex/hyperliquid-python-sdk/master/hyperliquid/utils/signing.py`, fetched in full today |
| Multisig outer envelope = `{multiSigActionHash: bytes32, nonce: uint64}` signed under `HyperliquidTransaction:SendMultiSig`; L1 multisig envelope = `[payload_multi_sig_user, outer_signer, action]`; user-signed multisig splices `payloadMultiSigUser`/`outerSigner` into the type list | VERIFIED-fresh | same file, `sign_multi_sig_action`, `add_multi_sig_types`, `add_multi_sig_fields`, `sign_multi_sig_l1_action_payload` |
| Official HyperCore docs: "A `MultiSig` action wraps around any normal action and includes a list of signatures from authorized users" | VERIFIED-fresh | `hyperliquid.gitbook.io/hyperliquid-docs/hypercore/multi-sig`, fetched today |
| HIP-3 mainnet staking requirement: 500,000 HYPE | VERIFIED-fresh | `hyperliquid.gitbook.io/hyperliquid-docs/hyperliquid-improvement-proposals-hips/hip-3-builder-deployed-perpetuals`, fetched today |
| HIP-3 stake ≈ $25M USD estimate | INHERITED | Phase 1, `cryptography.md` C2 — not re-priced this session |
| `hyperliquidguide.com` quotes on blind signing / incomplete Ledger EIP-712 coverage | VERIFIED-fresh | `hyperliquidguide.com/guides/getting-started/best-hardware-wallet-for-hyperliquid`, fetched today, page shows "Published July 25, 2026 \| Updated September 23, 2026" |
| SEAL frameworks site: dedicated Safe + Squads signing-verification pages, zero Hyperliquid coverage anywhere in the sitemap | VERIFIED-fresh | `frameworks.securityalliance.org` sitemap + `.../wallet-security/signing-and-verification/signing-verification` + `.../secure-multisig-safe-verification`, all fetched today |
| Official `hyperliquid-rust-sdk`: no multisig code, last pushed 2025-10-21 | VERIFIED-fresh | GitHub API `repos/hyperliquid-dex/hyperliquid-rust-sdk` + raw grep of `exchange_client.rs` + contents listing of `src/exchange/`, all fetched today |
| `Pier-Two/hyperliquid-multi-sig-actions` — validator-ops focus, pushed 2026-01-08, 0 stars | VERIFIED-fresh | GitHub API, fetched today |
| `tradingstrategy-ai/open-multisig-hl` — Svelte, pushed 2026-06-24, 0 stars | VERIFIED-fresh | GitHub API, fetched today |
| `tholos-inc/hyperliquid-multisig-cli` — Python, pushed 2025-03-09, 1 star | VERIFIED-fresh | GitHub API, fetched today |
| `kinetiq-research/hl-rs` `multisig_validation.rs` — full mechanism, and its gaps (no user-signed-action support, function it references doesn't exist in repo, no CLI/decode/risk-flagging, 8 stars, stalled ~7 weeks) | VERIFIED-fresh | full raw source of `multisig_validation.rs` and `multisig.rs`, full recursive repo tree, repo metadata — all fetched today |
| `infinitefield/hypersdk` `hypecli` — P2P multisig sign flow with Trezor support and a `Debug`-print review step before accept; 219 stars, pushed 2026-09-23 | VERIFIED-fresh | full raw source of `hypecli/src/multisig.rs` and `hypecli/src/trezor.rs`, full recursive repo tree, repo metadata — all fetched today |
| `ferrofluid` (130 stars) has no multisig files | VERIFIED-fresh | full recursive repo tree, fetched today |
| GitHub search "hyperliquid clearsign" / "clear-sign" / "action hash" / "verify signing" / "msgpack" — 0 results each | VERIFIED-fresh | GitHub search API, fetched today |
| 27 total "hyperliquid rust sdk"-matching repos exist | VERIFIED-fresh | GitHub search API, fetched today |
| Bybit February 2025 hack as background framing for the blind-signing failure mode | INHERITED (background framing from research brief; not independently re-sourced this session — two fetch attempts this session failed) | not re-verified |

**Net effect on the Phase 1 finding:** the core evidence (mechanism, structural neglect by SEAL/official SDK, named user population) held up and in places got sharper (exact envelope structure, official docs quote, HIP-3 figure). The whitespace/competitor claim did **not** hold up unchanged — two genuine partial competitors were found that Phase 1's narrower search missed. This should be treated as the single most important finding for Phase 3 to stress-test, and the pitch itself should name both competitors rather than assert an empty field.
