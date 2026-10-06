# Verification verdict: accel-conform (zkVM Accelerator Conformance & Differential Test Kit)

**Verifier pass date:** 2026-09-25
**Tooling constraints this session:** WebSearch budget exhausted (not used). WebFetch only, on specific known URLs. No `gh` CLI available in this environment (checked: not found). Unauthenticated GitHub REST API calls (`api.github.com/repos/...`) returned HTTP 403 — could not be used as a workaround. This materially limited one part of this verification (see Check 2 and the zkevm-standards#16 sub-finding below) — flagged explicitly rather than papered over.

---

## Verdict: **SOUND WITH CAVEATS**

The deck's own harshest-critic pass (section 8) already did most of the adversarial work correctly and did not overclaim. Independent re-fetching in this session **confirms the deck's central finding** (`eth-act/ere` PR #413 is a real, merged precursor) and **adds one new, unresolved, high-priority risk** the deck did not surface: whether `ere`'s *existing* test suite already exercises the SP1 bug right now. That single open question is the fulcrum of whether the demo's headline bug survives. Neither outcome kills the pitch — but the team must resolve it before finalizing the demo script, and I could not resolve it with the tools available to me.

---

## Check 1 (critical): How much of the product does `ere` PR #413 actually cover?

Independently re-fetched (not just re-reading the deck's citations): the PR itself, `ere`'s README, the fixtures directory, the harness source file, the SP1 guest test program, `ere`'s commit history, and CI workflow listing.

**Confirmed, matching the deck:**
- PR #413, merged **2026-08-25** (VERIFIED, independent re-fetch matches deck's date). Adds:
  - A guest/host test harness at `crates/util/test/src/program/zkvm_interface.rs`: an `Accelerator` enum enumerating essentially the full accelerator surface (confirmed `Bls12381Fp2ToG2`, and by cross-reference the fixture list below — secp256k1, BN254, hashes, modexp, KZG, secp256r1 all present), a `Vector`/`Vectors` type carrying `inputs` + a recorded `Outcome{status, output}`, and a `check()` function whose own docstring text (as rendered) is "runs every test vector and panics on the first one that does not match" — i.e. checks against **one fixed recorded reference value per vector**, confirmed **no live cross-vendor differential comparison logic** anywhere in the visible harness. This directly confirms the deck's claim that `ere` covers "check against known answer" but not "n-way differential."
  - No skip/TODO/exclude comment for any accelerator or known bug is visible in the harness source (checked explicitly).
  - Fixture JSON files at `tests/fixtures/zkvm_interface/` covering: `bls12_381_fp_to_g1`, `bls12_381_fp2_to_g2`, `bls12_381_g1_add`, `bls12_381_g1_msm`, `bls12_381_g2_add`, `bls12_381_g2_msm`, `bls12_381_pairing_check`, `secp256k1_ecrecover`, `blake2_compress`, `sha256`, `modexp`, `ripemd160`, `bn254_*`, `secp256r1_verify_signature`, `verify_kzg_proof` — this is essentially the full accelerator surface, not a partial slice. VERIFIED.
  - Guest test programs exist for **all three** covered zkVMs — `tests/openvm/zkvm_interface/src/main.rs`, `tests/sp1/zkvm_interface/src/main.rs`, `tests/zisk/zkvm_interface/src/main.rs` — confirmed via the PR's file-change list. (One earlier single-pass summary of the bare PR page mistakenly read this as "OpenVM-only"; the files-changed listing corrects this and matches the deck's OpenVM+SP1+ZisK claim.) VERIFIED.
  - `ere`'s README confirms it supports exactly **OpenVM, SP1, ZisK** (versions listed: OpenVM 2.1.0-preview, SP1 6.6.0, ZisK 1.3.0-alpha) — RISC Zero, Airbender, Jolt, Pico are absent. VERIFIED, matches deck exactly.
  - `ere` is **actively maintained**: commit history through **2026-09-23** (two days before this verification), multiple commits/week, releases through v0.18.1, including SP1/ZisK version bumps as recently as this week. This is a live, healthy dependency, not abandonware — this materially *strengthens* the "wrap/extend ere" framing as a live option, not a dead-end.

- **Reframing validated:** the pitch should shift from "build a conformance kit" to "build the differential/certification layer `ere` doesn't provide, on top of `ere`" — confirmed as the correct framing. `ere`'s harness is a Rust trait-based host/guest split, MIT/Apache-licensed, actively maintained — realistically wrappable/extendable rather than something to duplicate. The deck already reaches this conclusion in section 8; this session's independent re-fetch supports it rather than contradicting it.

**New finding this session, not in the deck — the single most important open risk:**
- The `bls12_381_fp2_to_g2.json` fixture encodes Fp2 elements as **`c0||c1`** with an **all-zero infinity encoding** — i.e., its recorded reference outcomes were computed against the **EIP-2537-correct convention that SP1 (`sp1#2881`) violates** (SP1 encodes `c1||c0` with a `0x40` infinity flag byte). If `ere`'s harness actually runs this exact fixture against a live SP1 accelerator call, and the fixture corpus contains a vector that actually exercises the divergent encoding path (e.g., an infinity-point case), **SP1's run of this test should currently be failing** given the bug is still open/unfixed.
- However, the most recent CI activity I could observe showed a **"Test and clippy SP1" workflow passing** (on a recent PR branch touching unrelated code). This creates a genuine, unresolved tension I could not settle with available tools: either (a) the corpus — extracted from *real* `execution-specs@tests-zkevm@v0.8.2` block-execution traces rather than deliberately crafted edge cases — simply doesn't happen to contain an actual infinity-point/divergent-encoding call (plausible: raw on-chain BLS12-381 calls rarely hit that exact edge case), so the mechanism exists but doesn't yet exercise this specific bug; or (b) I'm misreading CI scope/timing from a summarized fetch.
- **I was not able to resolve this conclusively.** No `gh` CLI, GitHub API blocked (403 unauthenticated), WebSearch exhausted, and WebFetch's GitHub HTML rendering did not reliably surface deep CI/comment content in this session (see Check 2). This is UNVERIFIED, flagged rather than guessed at.
- **This is the single highest-priority pre-build validation task for the team**, higher priority than anything else in this report: clone `eth-act/ere`, run its existing SP1 `zkvm_interface` test locally against the pinned SP1 version, and observe pass/fail.
  - If it currently **fails** on the BLS12-381 vector: `ere` already catches `sp1#2881` today. The deck's SP1 demo half is dead as "novel" material — but this is not fatal to the whole pitch; it *actually strengthens* the "wrap/extend, add certification layer" framing (proof the mechanism works end-to-end) and the demo should pivot entirely to (a) OpenVM material (see Check 2) and/or (b) a freshly-injected synthetic bug in an uncovered zkVM (RISC Zero/Jolt/Pico) to prove the coverage-extension value-add live.
  - If it currently **passes** despite the open bug: this is exactly the differentiator the pitch needs, and it should be *stated explicitly and proven live in the demo* — "here is `ere`'s own existing test, passing today, on a vector that should catch this bug in principle; here is why it doesn't (real-chain-derived corpus lacks the adversarial edge case); here is our spec-derived vector that does catch it." That is a stronger, more specific demo than the deck's current script and should replace it.

---

## Check 2: Is the OpenVM bug dead as demo material?

**Limitation to disclose up front:** I attempted to re-fetch `openvm-org/openvm#3147`'s comment thread four separate ways (direct issue URL, issue URL with an explicit "transcribe the whole page" prompt, a comment-anchor URL, and the unauthenticated GitHub API). All four either returned only the issue body/metadata with no comments visible, or (for the API) a 403. **I could not independently confirm or refute the deck's central Check-2 claim** — that the most recent (2026-09-04) comment attributes the failure to a possible Rosetta/BMI2 emulation artifact. That specific claim remains **INHERITED from the deck's own research, not independently re-verified by this verifier pass**, due to a tooling limitation, not contradicting evidence.

**What I did independently confirm:**
- The issue body itself (VERIFIED, re-fetched): reporter `oxlipefe`, opened 2026-09-03, title "Accelerated `zkvm_secp256k1_ecrecover` returns a wrong public key with `ZKVM_EOK` (v2.1.0-preview)." The report is methodologically careful — it explicitly rules out FFI/calling-convention causes (control tests via `zkvm_sha256`/`zkvm_ripemd160` on identical machinery pass), and claims the accelerated path's logic matches the native reference line-for-line yet diverges in output. This is not a sloppy report; it raises the prior that something real is going on, independent of whether the Rosetta theory later holds.
- **New finding, not in the deck:** `openvm-org/openvm#3146` — filed by the same reporter (`oxlipefe`), still open, a **separate, independently well-diagnosed critical bug**: a big-integer-division miscompilation in OpenVM v2.1.0-preview's accelerator codegen, deterministic **200/200 failures** when the divisor's top bit is set vs. **0/200** when clear, reproducible, opt-level-dependent (fails at `opt-level=2-3`, not `0-1`), with a working (if costly) workaround identified. This is exactly the kind of rigorous, hard-to-dismiss-as-a-test-rig-artifact bug report the deck's own harshest-critic framing would want, and it sits in the *same* subsystem (secp256k1 field arithmetic) as #3147.

**Assessment:** #3147 on its own is genuinely ambiguous, and the deck's "present it honestly as contested" framing is the right call *if #3147 is used at all*. But #3147 is not the only available OpenVM material — **#3146 is a cleaner, more rigorous, currently-unambiguous critical bug in the same accelerator subsystem** that the deck did not consider as an alternative. Recommendation: either drop #3147 from the demo entirely and substitute #3146 (stronger, less hedging needed, same narrative beat — "OpenVM's own accelerator path has open, unresolved correctness bugs in September 2026"), or keep #3147 with the deck's existing contested-framing *and* add #3146 as corroborating evidence that the accelerator layer generally has live, serious problems right now regardless of #3147's specific resolution. This is a strict improvement over the deck's current plan, discovered independently in this pass.

---

## Check 3: Any other competitor?

- GitHub repo searches for **"zkvm conformance"**, **"zkvm differential"**, **"accelerator test vector zkvm"** surfaced no material new competitor:
  - "zkvm conformance" → one irrelevant hit (`jiayaoqijia/eth2030`, an EVM execution-client project, not a zkVM accelerator tester).
  - "zkvm differential" → `bshastry/zkvm-witness`, checked directly: a narrow, low-activity "semantic-drift" checker comparing **only** RV32 `DIVU`/`REMU` instruction execution between OpenVM v2.0.1 and RISC Zero v3.0.6 against a Lean4-derived reference model. Zero coverage of secp256k1/BLS12-381/modexp/KZG or any `zkvm_accelerators.h` primitive; reads as a one-shot snapshot (single commit visible, dated 2026-08-02), not an actively maintained project. Does not compete with accel-conform's scope.
  - "accelerator test vector zkvm" → zero results.
- `zkevm-test-monitor`'s live dashboard: re-fetched; still shows only RISC-V ISA/ACT4 compliance content, no accelerator/precompile material (consistent with the deck's re-confirmation). I could **not** independently confirm or refute whether the maintainer discussion the deck cites (jsign/han0110, 2026-09-22–24, debating whether to fold accelerator-conformance results into this repo) actually took place — the same GitHub-comment-thread access limitation from Check 2 applied here too when I tried to re-fetch `zkevm-standards#16`'s comments (repeated attempts surfaced only the issue body/metadata, zero comments, for an issue the deck says has an active September thread). This is flagged as **UNVERIFIED-BY-VERIFIER (tool limitation)** — I found no evidence contradicting the deck's claim, but I also could not independently corroborate it. Treat as INHERITED-FROM-DECK for this specific sub-claim only; the load-bearing claim (PR #413's existence and contents) is independently confirmed regardless.
- **Conclusion: no new competitor found.** The deck's competitive scan (section 8: `ere`, `zkvmBlast`, `zkevm-test-monitor`, Arguzz) remains the complete known field as of this session.

---

## TAM / business sanity check

- Buyer base for the paid certification layer is genuinely tiny: ~6-8 zkVM teams, at a stated $500–2,000/mo badge fee, caps the paid-layer ceiling at roughly $3k–16k/mo even at 100% penetration — this is a small, narrow business, not a VC-scale one, and the deck does not pretend otherwise (it explicitly frames this as "priced like a compliance-badge SaaS," not enterprise security tooling).
- This is defensible under the hackathon's own judging criteria: criterion (b) rewards *impact concentration* ("each is the trust root for potentially billions in value settled on their proofs"), not headcount-scale TAM, and the OSS layer alone remains eligible for the Public Goods Award ($5k) independent of whether the paid layer ever closes a sale. As a hackathon Business Plan answer ("is there a viable business"), narrow-but-real is an acceptable answer; it would be a weak answer for an actual seed-round TAM slide, but that is not what is being judged here.
- No red flags found that change this assessment from the deck's own framing.

---

## Recommendation

**Keep this candidate.** Do not downgrade below SOUND WITH CAVEATS. The deck already did the hard, honest work of finding and disclosing its own two weaknesses (contested OpenVM bug, `ere` precursor) rather than having a verifier find them cold — that itself is a strong signal about the deck's reliability elsewhere. Independent re-verification in this session:
1. **Confirms** the `ere` PR #413 finding in full (mechanism, scope, license, maintenance health) — no walk-back needed, and it actually *strengthens* feasibility (proven, live pattern to extend) even as it narrows novelty (already priced in by the deck).
2. **Adds one unresolved, high-priority pre-build task**: determine whether `ere`'s existing SP1 test already fails on the BLS12-381 vector today. This determines the demo script's headline bug, not the viability of the underlying product. Do this first, before writing any demo code.
3. **Improves Check-2 demo material**: substitute or supplement the contested `#3147` with the cleaner, currently-unambiguous `openvm-org/openvm#3146` (same subsystem, same reporter, rigorous, reproducible, unresolved).
4. **Finds no new competitor** — whitespace claim (narrowed, per the deck's own honest downgrade) holds up.

If, after the team's own local test run, `ere` is found to already catch the SP1 bug outright, the pitch does **not** need to be dropped — it needs the demo re-centered on (a) proving that live in the demo as the differentiator, plus (b) `#3146` and/or a fresh synthetic-bug injection into an `ere`-uncovered zkVM, exactly as this report lays out above.

---

## Evidence log (this session, verifier's own fetches)

| # | Claim | Status | Source |
|---|---|---|---|
| V1 | PR #413 merged 2026-08-25; adds guest harness `zkvm_interface.rs` + fixtures + guest mains for OpenVM/SP1/ZisK | VERIFIED (independent re-fetch) | https://github.com/eth-act/ere/pull/413 , /pull/413/files |
| V2 | `ere` supports only OpenVM 2.1.0-preview, SP1 6.6.0, ZisK 1.3.0-alpha | VERIFIED (independent re-fetch) | https://github.com/eth-act/ere/blob/master/README.md |
| V3 | Fixture list at `tests/fixtures/zkvm_interface/` covers full accelerator surface incl. all BLS12-381 ops, secp256k1_ecrecover, KZG, modexp, BN254, hashes | VERIFIED (independent re-fetch) | https://github.com/eth-act/ere/tree/master/tests/fixtures/zkvm_interface |
| V4 | Harness checks against one fixed recorded outcome per vector; no live n-way differential logic; no skip/TODO for known bugs | VERIFIED (independent re-fetch) | https://github.com/eth-act/ere/blob/master/crates/util/test/src/program/zkvm_interface.rs |
| V5 | `bls12_381_fp2_to_g2.json` fixture encodes Fp2 as c0\|\|c1 with all-zero infinity (EIP-2537-correct convention, the one SP1 violates per sp1#2881) | VERIFIED (independent re-fetch) | https://github.com/eth-act/ere/blob/master/tests/fixtures/zkvm_interface/bls12_381_fp2_to_g2.json |
| V6 | `ere` actively maintained, commits through 2026-09-23, releases through v0.18.1 | VERIFIED (independent re-fetch) | https://github.com/eth-act/ere/commits/master |
| V7 | Whether `ere`'s current SP1 CI run passes or fails specifically on the BLS12-381 fixture given the open sp1#2881 bug | **UNVERIFIED** — could not resolve with available tools; flagged as top pre-build validation task | n/a |
| V8 | openvm#3147 issue body: reporter ruled out FFI/calling-convention causes via control tests; claims line-for-line logic match yet divergent output | VERIFIED (independent re-fetch, body only) | https://github.com/openvm-org/openvm/issues/3147 |
| V9 | openvm#3147's cited 2026-09-04 "Rosetta/BMI2 artifact" maintainer comment | **UNVERIFIED-BY-VERIFIER** (tool limitation: comments section not retrievable in this session across 4 attempts) — treat as INHERITED from deck only | https://github.com/openvm-org/openvm/issues/3147 |
| V10 | openvm#3146: independent, rigorous, reproducible, still-open critical big-integer-division miscompilation bug in same subsystem, same reporter | VERIFIED (new finding, independent fetch) | https://github.com/openvm-org/openvm/issues/3146 |
| V11 | zkevm-standards#16 2026-09-22/24 maintainer discussion (jsign/han0110) re: `ere`/zkevm-test-monitor | **UNVERIFIED-BY-VERIFIER** (tool limitation, comments not retrievable) — treat as INHERITED from deck only | https://github.com/eth-act/zkevm-standards/issues/16 |
| V12 | No new competitor found via repo search for "zkvm conformance" / "zkvm differential" / "accelerator test vector zkvm" | VERIFIED (searches performed, results checked individually) | GitHub repo search, and https://github.com/bshastry/zkvm-witness |
| V13 | `bshastry/zkvm-witness` is a narrow, low-activity RV32 DIVU/REMU-only semantic-drift checker (OpenVM vs RISC Zero), not accelerator conformance | VERIFIED (independent re-fetch) | https://github.com/bshastry/zkvm-witness |
| V14 | `zkevm-test-monitor` live dashboard still shows only ACT4 RISC-V ISA content | VERIFIED (re-fetch consistent with deck; page was mid-load but source content confirmed via repo) | https://eth-act.github.io/zkevm-test-monitor/ |
