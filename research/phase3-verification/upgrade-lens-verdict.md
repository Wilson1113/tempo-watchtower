# upgrade-lens — Verifier Verdict

Verifier pass on `research/phase2-decks/upgrade-lens.md`, run 2026-09-25. This session's WebSearch budget was exhausted (per task constraint), so all checks below use direct `curl` against GitHub's REST API / raw content and crates.io's API, plus WebFetch against specific known URLs (docs.rs, raw.githubusercontent.com, github.com pages). WebFetch results that came back as an AI-summarized page (rather than raw JSON/text I read myself) are marked accordingly — they are a step less directly verified than a raw `curl` fetch, but still first-party sourced, not memory or invention.

## Verdict: **SOUND WITH CAVEATS**

The deck's central structural-neglect claim (filter #3) survives adversarial re-testing: no evidence found, across Solana Foundation's and Anza's active repos or the two adjacent OSS tools, of any roadmap movement toward bytecode/CPI-level upgrade diffing. But this pass surfaced one new technical caveat the deck did not fully grapple with (how much of real-world CPI is runtime-resolved rather than a static bytecode constant — see Check 3), plus the deck's own already-flagged TAM/monetization softness is confirmed genuinely soft, not resolved. Neither is fatal; both should shape how the pitch is scoped and worded.

---

## Check 1 findings (the critical one) — is Solana Foundation's active tooling a threat to the differentiator?

**Repo metadata, VERIFIED via `api.github.com` (raw JSON, read directly):**

| repo | pushed_at | open issues | stars | forks |
|---|---|---|---|---|
| `solana-foundation/squads-program-action` | 2026-09-16T18:23:04Z | 13 | 3 | 2 |
| `solana-foundation/github-actions` | 2026-09-16T18:12:14Z | 4 | 12 | 10 |
| `solana-foundation/github-workflows` | 2026-08-04T17:00:02Z | 1 | 26 | 10 |

Dates match the deck's citations exactly.

**What the issue/PR history actually shows (VERIFIED, full issue lists read via API):** Every open and closed issue across all three repos is either a Dependabot version bump (npm/rollup/actions-version bumps — the large majority), or a CI/build-plumbing feature: PDA-first remote verify, base image/features pass-through, cargo/npm publish actions, IDL-upload fixes, cache fixes, buffer-size fixes, SHA-pinning. **Zero** issues, open or closed, reference content diffing, CPI-target analysis, or account-check inspection. A GitHub code-search for the keyword "diff" across all three repos' issues/PRs returned 29 hits — every one is a dependency-bump PR title or an unrelated feature PR; none is about program-content diffing.

**README fetch of `squads-program-action` (WebFetch, AI-summarized):** confirms directly — "does **not** perform bytecode diffing or analyze program changes... does not perform content or bytecode diffing between program versions... does not analyze CPI call targets or instruction handler changes... does not check account-ownership logic." Its actual scope is buffer/IDL orchestration + an "Optional PDA verification instruction" (hash/authenticity), exactly as the deck's §2 evidence #4 and §8 state.

**Org-level scan for anything broader (WebFetch, AI-summarized, so treat as directionally reliable rather than exhaustive):**
- `github.com/orgs/solana-foundation/repositories` (sorted by stars, top 25 of 104 total shown): the only program-verification-adjacent tool listed is `solana-verifiable-build` (deterministic build + hash match against on-chain program — the same authenticity-only category already covered in the deck via `otter-verify`/`solana-verify`). No diffing/CPI/analysis tool appears.
- `github.com/orgs/anza-xyz/repositories` (sorted by recently-pushed): `agave`, `agave-block-production`, `sbpf`, `solana-sdk`, `kit`, `security-audits`, `tpu-tools`, `mollusk` are the active repos shown. `security-audits` is published audit *reports*, not a diffing tool. Nothing in the list targets upgrade-diffing, CPI analysis, or account-check comparison.

**Conclusion on Check 1:** The deck's own §8 finding holds up under a harder look this session — Solana Foundation and Anza are demonstrably active (real feature velocity: PDA verification, IDL upload, PMP account handling all shipped in the last ~6 weeks) but that activity is entirely in CI-orchestration and buffer/hash-authenticity, one layer below the content-diffing layer this product targets. I found **no roadmap signal, issue, or discussion anywhere in the three named repos or either org's broader repo list** suggesting movement toward bytecode/CPI-level diffing. The "audit firms are structurally disincentivized, foundation is building adjacent-not-this" argument is **not falsified** by this check. The one thing worth flagging plainly for judges/investors (which the deck's own §7 "honest risk" paragraph already anticipates, and this check corroborates rather than discovers fresh): the Foundation has real infrastructure and momentum *right next to* this gap — if it or Anza ever chose to extend scope, it would not be starting cold. That is a live watch-item, not evidence of an imminent build.

---

## Check 2 findings — could `ratchet` or `solana-upgrade-guard` extend into bytecode diffing in the hackathon window?

**`saicharanpogul/ratchet`** (WebFetch, AI-summarized page read): 1 star, 0 forks, 0 watchers, 0 open issues. The only forward-looking roadmap text found is about deeper **Quasar IDL schema** integration ("stable Quasar `__QUASAR_SCHEMA` binary reader," "deeper compiler-pass integration when Quasar exposes a plugin surface") — still IDL/schema-level, not bytecode/CPI. No mention anywhere of adding CPI-target, bytecode, or account-ownership-check diffing. Direct `api.github.com` calls for this repo's commit/contributor history hit GitHub's unauthenticated rate limit mid-session and could not be re-confirmed by raw JSON; the WebFetch-summarized README read is the basis for this finding — flagged as a shade below the Check 1 raw-JSON confidence level, but consistent with the deck's own characterization (1 star, last pushed 2026-07-30).

**`FannBe/solana-upgrade-guard`** (WebFetch, AI-summarized page read): 0 stars, 0 forks, 0 watchers, described in the README as covering account-layout diffing, upgrade-authority verification, deploy mechanics, and rollback — **no roadmap or future-work section exists at all**. Reads as a single-day, abandoned hackathon-style artifact, matching the deck's characterization exactly.

**Conclusion on Check 2:** Neither project shows momentum, contributor growth, or a stated roadmap toward the bytecode/CPI layer. Neither is a live 2-3-week threat. This confirms rather than revises the deck's own assessment.

---

## Check 3 findings — re-verify the core technical claim (`solana-sbpf`)

**VERIFIED via crates.io API (raw JSON):**
- `solana-sbpf`: max/newest version **0.25.0**, `updated_at` **2026-09-22T09:17:06Z** (3 days before this check), repository `github.com/anza-xyz/sbpf`, description "Virtual machine and JIT compiler for eBPF programs."
- `solana_rbpf`: max/newest version **0.8.5**, `updated_at` **2024-08-08T22:11:31Z** — over two years stale, confirming it is the legacy/predecessor name.

This exactly matches the deck's §3 correction. **VERIFIED, not just inherited.**

**docs.rs (WebFetch, AI-summarized) confirms the crate's public modules:** `ebpf`, `elf`/`elf_parser`, `disassembler`, `assembler`, `interpreter`, `jit`, `verifier`, `vm`, `x86`. The `ebpf::Insn` struct exposes `ptr` (instruction address), `opc` (opcode), `dst`/`src` (registers), `off` (offset), `imm` (immediate) — sufficient low-level primitives for a consumer to build custom CFG extraction and call-site identification on top, but **the crate itself does not ship CFG extraction** — that confirms the deck is being accurate (not overclaiming what the dependency provides for free) when it describes CFG extraction as something `upgrade-lens` builds itself in step 2 of its pipeline.

**New technical caveat surfaced this session, not previously flagged in the deck:** sBPF `call`-instruction immediates typically encode **intra-program, function-relative call targets** (calls to other functions within the same compiled ELF, resolved via relocation), not the invoked program's pubkey. Actual Solana CPI happens through the `sol_invoke_signed` syscall family, where the invoked program ID is a **runtime value** — usually a `Pubkey` read from the transaction's account list / an `AccountInfo` passed in by the caller, not a bytecode-embedded constant. The deck's own step 4 already partially acknowledges this ("whether a compile-time constant or a runtime-resolved account reference"), but the practical split matters a lot for the pitch's core claim: the "hardcoded pubkey loaded via `lddw` from `.rodata` before the invoke call" case is real and detectable via straightforward data-flow tracing, but a large share of real-world CPIs (e.g., a `token::transfer` where the SPL Token program ID is simply whatever account the caller supplied) resolve their target from **caller-supplied account metadata**, not from anything visible in the callee's own bytecode. For that (probably common) case, static bytecode diffing alone cannot recover "the CPI target changed" — it would require also tracking which account slot is wired to which instruction-account position, a materially harder problem than plain CFG + syscall-callsite extraction. This is a genuine scoping/credibility risk for the MVP and demo script, not a market-viability kill — the deck's own demo plan (a staged, deliberately-reintroduced bug on a forked copy) can be written to specifically showcase the hardcoded-constant case, which is honest and still demoable, but the pitch copy should not imply "catches any malicious CPI redirection" without that qualifier.

---

## TAM / business sanity check

The deck itself is honest that there is "no hard population count" for Squads v4 signers / DAO governance councils (§2) — this session found no additional data to firm that number up (consistent with the WebSearch-budget constraint; a hard count would likely require Squads' own usage dashboards, which are not publicly enumerable via the tools available this session). Two things worth flagging plainly for judging criterion (b) Potential Impact and (f) Business Plan:
1. **End-user population is plausibly large but genuinely uncounted** — this is a real gap, not resolved by this verification pass, and should be named as such rather than implied to be known.
2. **The B2B monetization path is concentrated on a small number of platform buyers** — realistically Squads and Realms are close to the entire near-term buyer list for the "embed this in the approval UI" tier (§5). That doesn't undercut the *problem*'s scale (many signers/treasuries are exposed to the risk), but it does mean the paid-tier revenue ceiling in the near term rests on convincing one or two platform teams, not a broad multi-buyer market — a concentration risk the deck doesn't explicitly name in its business-model section.

---

## Recommendation

**The pitch still clears the 4-point filter, including #3, on the evidence available this session.** Filter #3 requires structural (not circumstantial) big-co neglect — this check specifically stress-tested that against the most current, most active adjacent tooling (Solana Foundation's three actively-maintained repos, Anza's org, and the two nearest OSS competitors) and found no counter-evidence: no roadmap, issue, or repo anywhere in scope points toward bytecode/CPI-level diffing. The economic argument for why OtterSec/Zellic/Sec3 won't automate away their own billable judgment-layer audit work remains logically intact and unrefuted by anything found here.

That said, two things should change how the pitch is built and pitched, not whether it's built:
1. **Scope the demo and pitch copy around statically-resolvable CPI targets and account-check removal**, not "any malicious CPI redirection" — the Check 3 caveat is a real gap between the pitch's implied generality and what static bytecode diffing can actually recover for caller-supplied (account-metadata-resolved) CPI targets. This affects functionality/novelty credibility if a technical judge probes it live.
2. **Name the TAM honestly as "large but uncounted" and the monetization path as "concentrated on a small number of platform buyers in the near term"** rather than letting the large aggregate end-user population imply a large near-term revenue market — this is a business-plan-criterion honesty issue, not a kill switch.

Neither caveat is fatal. Both are pre-existing soft spots the deck already gestured at (§7's "thin moat" admission, §2's "no hard population count") that this adversarial pass confirms are real rather than overclaimed, plus one genuinely new technical nuance (the CPI-target resolution split) worth building the MVP and demo script around rather than discovering during a judge Q&A.

---

## Evidence appendix (this session only)

| # | Claim | Status | Source |
|---|---|---|---|
| 1 | `squads-program-action`/`github-actions`/`github-workflows` pushed_at dates, issue/star/fork counts | VERIFIED | api.github.com (raw JSON) |
| 2 | All issues/PRs across the 3 repos are CI/dependency-bump/orchestration work; zero reference content-diffing, CPI, or account-check analysis | VERIFIED | api.github.com issues listings + code search for "diff" (raw JSON) |
| 3 | `squads-program-action` README explicitly disclaims bytecode diffing, CPI analysis, account-ownership checks | VERIFIED (WebFetch, AI-summarized) | raw.githubusercontent.com/solana-foundation/squads-program-action/main/README.md |
| 4 | solana-foundation org's broader repo list (top 25 by stars) has no diffing/CPI-analysis tool beyond `solana-verifiable-build` (authenticity only) | VERIFIED (WebFetch, AI-summarized, partial listing — 25 of 104 repos) | github.com/orgs/solana-foundation/repositories |
| 5 | anza-xyz org's most-recently-pushed repos have no diffing/CPI-analysis tool | VERIFIED (WebFetch, AI-summarized, partial listing) | github.com/orgs/anza-xyz/repositories |
| 6 | `ratchet`: 1 star, 0 open issues, roadmap limited to Quasar IDL-schema integration, no CPI/bytecode plan | VERIFIED (WebFetch, AI-summarized; raw-JSON re-check blocked by GitHub API rate limit this session) | github.com/saicharanpogul/ratchet |
| 7 | `solana-upgrade-guard`: 0 stars, no roadmap section, appears abandoned | VERIFIED (WebFetch, AI-summarized) | github.com/FannBe/solana-upgrade-guard |
| 8 | `solana-sbpf` v0.25.0, updated 2026-09-22; `solana_rbpf` v0.8.5, updated 2024-08-08 | VERIFIED | crates.io API (raw JSON) |
| 9 | `solana-sbpf` public modules (ebpf, elf, disassembler, assembler, interpreter, jit, verifier, vm, x86); `Insn` struct fields (ptr/opc/dst/src/off/imm); no built-in CFG extraction | VERIFIED (WebFetch, AI-summarized) | docs.rs/solana-sbpf/latest |
| 10 | sBPF `call`-immediate targets are typically intra-program/relocation-resolved, not CPI pubkeys; real CPI targets are frequently runtime/account-metadata-resolved | Domain-knowledge-based technical analysis by this verifier, not sourced from a fetched document — flagged as reasoning, not a citation | n/a |
| 11 | Deck's own admission of "no hard population count" for TAM; this session found no additional data to firm it up | VERIFIED (absence confirmed — no new source found) | this session's search scope |
