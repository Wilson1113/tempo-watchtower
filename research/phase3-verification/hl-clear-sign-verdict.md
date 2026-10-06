# HL Clear-Sign — Phase 3 Adversarial Verification Verdict

Verified: 2026-09-25. Verifier fetched all sources fresh this session (curl against
`raw.githubusercontent.com` / `api.github.com`, WebFetch against `github.com` HTML pages
and `frameworks.securityalliance.org`). `api.github.com` hit unauthenticated rate limits
partway through, so a few checks fell back to the GitHub web UI via WebFetch — noted inline.
Status legend: **VERIFIED** (fetched by this verifier, this session) / **INHERITED**
(carried from the deck, not independently re-fetched by this verifier) / **UNVERIFIED**.

## Verdict: **AT RISK**

The pitch is not fatally undermined, but the deck's own central Check-1 evidence contains a
**falsifiable, false claim** about `kinetiq-research/hl-rs` — one a technical judge could
disprove live on GitHub in under two minutes. The underlying product idea and the
`infinitefield/hypersdk` differentiation both survive scrutiny. This must be corrected before
submission; if submitted as-is, the specific sentence about the missing function is a
credibility risk in Q&A.

---

## Check 1 (critical): Could either competitor easily close the gap?

### `kinetiq-research/hl-rs` — the deck's claim is factually wrong on the load-bearing point

**VERIFIED** by full re-read of `src/actions/multisig_validation.rs` (862 lines, fetched in
full from the `main` branch, current as of this session) plus `src/actions/mod.rs` and
`src/actions/traits.rs`:

- The deck states (§8): *"it explicitly refuses user-signed inner actions —
  `MultisigValidationError::UserSignedInnerNotSupported`, with an error message pointing to a
  `validate_multisig_user_signed_action` function that **does not exist anywhere in the fetched
  repository**"* and concludes it *"cannot validate `withdraw3`, `usdSend`, `spotSend`, or
  `convertToMultiSigUser`."*
- **This is false as of the current `main` branch.** `validate_multisig_user_signed_action`
  is fully implemented (lines ~460–520 of the file), is publicly exported from
  `src/actions/mod.rs` (`pub use multisig_validation::{validate_multisig_l1_action,
  validate_multisig_user_signed_action, ...}`), is generic over any `A: UserSignedAction +
  Action`, and has a passing unit test (`typed_user_signed_round_trip_passes`) that exercises it
  end-to-end against `SpotTransfer` — a real user-signed transfer action, i.e. exactly the
  category the deck claims is unsupported. `Withdraw` and `UsdSend` are both registered
  `UserSignedAction` types in the same crate (confirmed in `mod.rs`'s `impl_action_kind!` list
  under "User-signed actions") and README-documented with builder examples, so the same generic
  function validates them too — there is no code-level reason it wouldn't.
- Commit history on this file (`api.github.com/repos/kinetiq-research/hl-rs/commits?path=...`,
  VERIFIED before rate-limiting) shows the relevant work — `"feat: multisig recovery"` /
  `"fix: multisig recovery"` — landed **2026-07-07 to 2026-07-30**, i.e. this capability has
  existed for ~2 months, well before the deck was written today. This was not a same-day change
  the deck-builder narrowly missed; it was already there when they looked.
- **What the real, narrower gap actually is:** `MultiSigRequest::validate()` — the *convenience*
  entrypoint that takes raw `/exchange`-shaped JSON without the caller already knowing the
  concrete Rust action type — still short-circuits with `UserSignedInnerNotSupported` for
  user-signed inner actions instead of dispatching to the typed function. But the dispatcher
  needed to close this is *also already in the same file*: `inner_action_kind()` /
  `dispatch_action_kind` classifies raw JSON into a typed `ActionKind` (including
  `Withdraw`, `UsdSend`, `SpotTransfer`, `ConvertToMultiSigUser`) for exactly this purpose
  already. Wiring the two together — match on `ActionKind`, call
  `validate_multisig_user_signed_action` with the classified action instead of erroring — is
  glue code, plausibly 20–50 lines, not new cryptography and not a new architecture.
- **Answer to the coordinator's framing question:** the gap is not architectural, and it is
  *more* than "nobody wrote 20 more lines" — the hard part (independent EIP-712/msgpack hash
  recomputation for user-signed actions, tested) is already done and shipping in a public API.
  Only the last convenience wrapper is missing.
- **What genuinely still differentiates HL Clear-Sign from hl-rs, confirmed by inspection:**
  (1) hl-rs has **no `[[bin]]` target** anywhere in `Cargo.toml` (VERIFIED) — it is a pure
  library; there is no CLI a signer runs directly, only Rust functions a backend/policy-engine
  author must call. (2) No semantic risk/anomaly-flagging (unseen destination, changed
  permission, baseline diff) exists anywhere in the repo — confirmed by the full file read; this
  is orthogonal cryptographic-vs-heuristic functionality hl-rs was never trying to provide.
  (3) It is stalled: 8 stars, `pushed_at` 2026-08-07, no push in ~7 weeks as of today (VERIFIED,
  `api.github.com/repos/kinetiq-research/hl-rs` metadata) — real but note this cuts against the
  pitch too, since it means the *maintainer* could trivially ship the missing dispatch glue on
  any given weekend and remove even this differentiator.

**Net effect:** the deck's "cannot validate withdrawals/transfers" claim must be deleted or
rewritten. The honest differentiation left standing is packaging (standalone CLI vs. library),
UX (human-readable + hardware-wallet-comparable output, out of the box, no integration), and
risk-flagging — not "the crypto for this doesn't exist elsewhere in Rust."

### `infinitefield/hypersdk` — momentum confirmed high; "self-trusting decode" claim confirmed correct

**VERIFIED**, repo metadata + commit list (via WebFetch on the GitHub web UI after
`api.github.com` rate-limited this session): 219 stars, actively and rapidly maintained —
12 commits visible from **2026-09-21 to 2026-09-23** (2–4 days before this verification),
including a version release (`0.2.16`) and commits specifically in the multisig/Trezor code
path: *"fix: complete multisig signing and unify CLI action options,"* *"feat(hypecli): support
multisig Earn and Send with Trezor signing,"* *"feat(hypecli): discover Trezor addresses locally
with xpubs."* This is a fast-moving, well-resourced project actively investing engineering time
in exactly this feature area — real momentum risk, not a stalled side project.

**VERIFIED** by full read of `hypecli/src/multisig.rs` (826 lines): the receiving-signer flow
in `sign()` (line ~299) calls `validate_proposal(&action, cmd.multi_sig_addr,
&multisig_config.authorized_users)` and its body (line ~595) does exactly two checks:
(1) `action.multi_sig_user == expected_account`, (2) `authorized_users.contains(&outer_signer)`.
**No hash recomputation, no content/semantic check of any kind.** Immediately after, it runs
`println!("{:#?}", action)` — a Rust `Debug` dump of the same struct the CLI itself just
deserialized off its own P2P (`iroh`) channel — then prompts `Accept (y/n)?` before signing.

This directly answers the coordinator's question: **if the coordinator or the P2P transport is
compromised, `hypecli`'s own display protects nobody** — the printed review is the same
process's own interpretation of the same bytes it received, with no second, independently-sourced
value to cross-check against (no recomputed hash printed for comparison against a hardware
wallet's own screen). It is the same single point of trust the pitch targets, just relocated
from "web UI" to "this CLI." (The Trezor integration sends typed data to the device for its own
firmware to display/sign, which is a genuinely separate device — but per the deck's own §2
evidence, Ledger/Trezor's built-in EIP-712 clear-signing coverage is explicitly "incomplete" and
"not guaranteed" to include Hyperliquid's schemas, so this doesn't fully close the gap either.)
This confirms and strengthens the deck's hedged claim here — the deck said this was "an open
question... not read this session"; it is now closed in the pitch's favor.

**Risk to flag:** given the maintainer's current velocity and focus on exactly this code path,
adding a "recompute + print for hardware-wallet comparison" feature to `hypecli` is plausible on
a similar timescale to this hackathon. This is a directional risk, not a present gap.

---

## Check 2: Core evidence re-verification — all confirmed clean

- **`hyperliquid-python-sdk/hyperliquid/utils/signing.py` multisig envelope** — VERIFIED, refetched
  in full. Confirms `multiSigActionHash: bytes32` type (line 151), `sign_multi_sig_action`
  (line 315) building `{"multiSigActionHash": ..., "nonce": ...}` under
  `HyperliquidTransaction:SendMultiSig` (line 327), `sign_multi_sig_l1_action_payload` (line 301)
  building the `[payload_multi_sig_user, outer_signer, action]` envelope (line 304), and
  `add_multi_sig_types`/`add_multi_sig_fields` splicing `payloadMultiSigUser`/`outerSigner` into
  the type list (lines 256, 280). Matches the deck's description exactly.
- **SEAL `frameworks.securityalliance.org` sitemap** — VERIFIED, refetched `sitemap.xml` today:
  zero URLs contain "hyperliquid" anywhere in the ~300+-URL sitemap. Deck's "zero Hyperliquid
  coverage" claim holds.
- **`hyperliquid-dex/hyperliquid-rust-sdk` last commit** — VERIFIED via GitHub web UI (commits
  page): most recent commit **2025-10-21** ("Fix: accept null premium/impact_pxs in
  AssetContext"), no multisig-related commit visible in the recent history shown. Matches the
  deck's "2025-10-21, 11 months stale" claim exactly.

## Check 3: Other competitors — none found

- **`hyperliquid-dex` org repos** — VERIFIED (GitHub org repo listing): 11 repos total (`node`,
  `order_book_server`, both SDKs, `hyper-evm-sync`, `block-importer`, `ts-examples`,
  `hyperliquid-stats(-web)`, `contracts`, `historical_data`). None relate to multisig, signing
  verification, or clear-signing. No new competitor.
- **`Pier-Two/hyperliquid-multi-sig-actions`** — VERIFIED: still validator-ops signing
  convenience only ("Use at your own risk," 0 stars); no independent-verification, hash-recompute,
  or decoded-action-review claim anywhere in the repo.
- **Hypersig (hypersig.xyz)** — VERIFIED: product page confirms it is a signing-coordination /
  multisig-administration UI (thresholds, agent wallets, sub-accounts, signer rotation) whose
  security model is stated as relying on "protocol-level" enforcement by HyperCore itself — no
  independent transaction-content verification claim. No pricing published on the page (deck
  correctly declined to invent a number).

---

## TAM / business sanity check

The niche-lean strategy (00-brief.md) explicitly wants a small, reachable user group big players
won't serve — this idea satisfies that by design, so a "small TAM" is not itself disqualifying.
That said, judged against criterion (b) directly: the named user set (HIP-3 deployers, HL
validators running multisig, a handful of trading-desk/DAO treasuries) is realistically dozens to
low hundreds of wallets, not thousands — the deck never states a deployer count, only the
per-deployer stake requirement, which is a gap worth flagging (a rough current HIP-3
market/deployer count would strengthen §5 materially). The paid tiers (hosted coordination +
anomaly alerts; custom-decoder consulting) are plausible but honestly flagged as unpriced/
comparable-only rather than fabricated, which is the right call given no verified Hypersig price
point was found by this verifier either. Business viability rests more on "avoided catastrophic
loss" willingness-to-pay than on volume — reasonable for this user segment, but a judge scoring
pure TAM size will likely mark it modest, consistent with the niche strategy's known trade-off.

---

## Recommendation

1. **Fix immediately, before any further deck work:** delete or rewrite the §8 claim that
   `validate_multisig_user_signed_action` "does not exist" and that hl-rs "cannot validate
   withdraw3/usdSend/spotSend/convertToMultiSigUser." It exists, is tested, and is exported. Do
   not let this reach a judge unchanged.
2. **Reframe the differentiation** around what actually still holds: (a) a standalone,
   integration-free CLI a signer runs directly against arbitrary wire JSON from any coordinator
   (neither competitor offers this — hl-rs is backend-integration-only, hypersdk only works
   inside its own P2P flow); (b) semantic risk/anomaly-flagging (unseen address, changed
   permission/threshold) — present in neither competitor; (c) printing an independently
   recomputed hash specifically for hardware-wallet cross-comparison — confirmed absent from
   hypersdk's `validate_proposal`/debug-dump flow, and while hl-rs's library *could* produce this
   value, nothing today packages it for a human signer to compare against a Ledger/Trezor screen.
3. **Do not drop the idea.** Both competitor findings from Phase 2 are real, but neither is fatal:
   hypersdk's gap is confirmed genuine (self-trusting decode) though momentum is a real risk;
   hl-rs's gap is much smaller than claimed but the *packaging* gap (no CLI, no risk-flagging) is
   real and independently verified. Ship the demo emphasizing standalone UX + risk-flagging, not
   "nobody can compute this hash but us."
4. Keep Check 2/3 findings as-is in the final deck — they were re-verified clean and need no
   changes.
