# simd-diff — Verifier Verdict

Verifier pass on `research/phase2-decks/simd-diff.md`, run 2026-09-25. All facts below marked VERIFIED were re-fetched live in this session via direct `curl` against GitHub's raw content / REST API and crates.io's API (not the deck's own citations, not memory). WebSearch was unavailable this session (budget exhausted), so a small number of items remain UNVERIFIED where GitHub/crates.io APIs couldn't cover them — flagged explicitly below.

## Verdict: **AT RISK**

UpgradeGuard is real, working, and functionally overlaps on the pitch's two headline mechanisms (static ELF/sBPF linter + LiteSVM dual-binary replay/diff). But it has **zero moat** — it is a same-hackathon-window rival's ~12.5-hour, single-contributor, zero-star repo, not an incumbent or funded competitor — and it has two confirmed, current, checkable gaps that map directly onto a legitimate, ownable, demoable narrower angle. The deck's own §8 self-assessment is accurate; my independent re-verification confirms it rather than softening or hardening it. See Recommendation for the specific angle this builder must commit to.

---

## Check 1 findings — does the problem exist, right now?

All VERIFIED fresh this session via direct fetch (not WebFetch/AI-summarized — raw file content and REST JSON read directly):

- **SIMD-0500 raw file** (`raw.githubusercontent.com/solana-foundation/solana-improvement-documents/main/proposals/0500-disable-deployment-of-sbpf-v0-v1-v2.md`): frontmatter reads `status: Idea`, `feature: TBD`, `created: 2026-03-17`. Matches the deck's claim exactly — no status advancement.
- **Program counts, confirmed from the table in the file itself**: "Loader-v3 (Upgradable) | SBPFv0 17279 | SBPFv1 12 | SBPFv2 41" = **17,332 total**, sourced in-document to Blueshift's `program-sync` tool. Deck's headline number is exact, not rounded or stale.
- **Agave release status** (`api.github.com/repos/anza-xyz/agave/releases`): `v4.3.0` published **2026-09-18T12:15:17Z** (stable, `prerelease: false`). `v4.4.0-alpha.5` published the same day, **2026-09-18T15:32:23Z**, `prerelease: true`. No stable v4.4 tag exists as of today (2026-09-25). Confirms the deck's claim precisely — the SBPFv3-enforcement release is still pre-release only.
- **Official first-party migration tooling from Anza/Solana Foundation**: none found via GitHub API search across UpgradeGuard's own competitive table, `feature-activation-verifier` (still 1 star, last pushed 2026-05-26, generic gate matrix — not per-program), and the Anza blog quote already in the deck (recommends third-party Mollusk/LiteSVM/Surfpool/Anchor, doesn't ship an end-to-end tool). **UNVERIFIED / partial**: I could not run a fresh general web search this session (budget exhausted) to check for a brand-new announcement in the last few days outside GitHub; nothing in the GitHub-side surface (Agave release notes, SIMD repo, Anza's linked blog) suggests one exists.

Conclusion: Check 1 fully holds. Nothing has changed since the deck was written; the deck's own numbers and dates check out exactly.

---

## Check 2 findings — UpgradeGuard deep-dive (the critical section)

**Repo vitals** (VERIFIED, `api.github.com/repos/VictorGSoutoXP/UpgradeGuard`):
- Created **2026-09-24T03:07:24Z**, last pushed **2026-09-24T15:40:24Z** — the entire repository's life span is **~12.5 hours, all in a single calendar day**, the day before this deck was written.
- **0 stars, 0 forks, 0 watchers, 0 open issues.** All 20 "issues" visible via the API are actually closed/merged pull-request records tied to the 54 commits — there is no separate open-issue backlog.
- **Single contributor**: `VictorGSoutoXP`, 54/54 contributions. No co-contributors.
- Branch names (`claude/project-thread-*`, `claude/readme-and-research-*`, `claude/friendly-carson-*`) plus `CLAUDE.md`/`AGENTS.md` in the repo root confirm this is an AI-agent-assisted (Claude Code) build — very plausibly another solo entrant's Colosseum submission built the same way this builder is building theirs. `research.md` (VERIFIED, fetched fresh, 861 lines) explicitly scopes itself to **the identical contest window, 14/09–12/10/2026**, and independently cites the same 17,332 figure via the same Blueshift program-sync source.
- **License: Apache-2.0** (VERIFIED via repo metadata) — permissive, no legal blocker to building something adjacent, but it is not published to crates.io (checked: `crates.io/api/v1/crates?q=upgradeguard` returns zero crates), so there is no realistic "depend on it as a library" composition path either. It's a rival's finished hackathon entry, not infrastructure to build on.

**Functional reality of `inspect` and `test-diff`** (VERIFIED by reading the full 333-line raw README plus the actual repo file tree and source of the relevant crates, not just the deck's summary):
- `test-diff` is real: `crates/upgradeguard-harness/src/{fixture.rs,run.rs,diff.rs}` load a baseline and candidate `.so` into two LiteSVM instances and diff status/return-data/normalized-logs/account-state/write-set/CU, exactly as the deck describes.
- `inspect` is real: `crates/upgradeguard-verify/src/{elf64.rs,sbpf.rs,stack.rs}` check ELF `e_flags`/layout, unresolved `call -1`, static-syscall registration.

**Confirmed differentiator (a) — no real-tx auto-fetch:** I grepped the full raw README myself (`grep -in "signature|getSignatures|real transaction|recent tx|mainnet traffic"`) — **zero matches**. I also read the full repo file tree: the only on-chain-reading code is in `crates/upgradeguard-scanner` (`rpc.rs`, `census.rs`, `scan.rs`), which does `getAccountInfo`-based scanning/census of *which* programs are affected — it never fetches a program's transaction history. `test-diff`'s only input path is a hand-authored `--fixtures fixtures.json` file (confirmed schema in the README: named accounts, cases, instructions with literal hex data). **The deck's claim is exactly right and still true as of this fresh check.**

**Confirmed differentiator (b) — no SIMD-0459/0460 coverage:** grepped the same README for "0459"/"0460" — zero matches. I went further than the deck and read `crates/upgradeguard-verify/src/stack.rs` in full: it is a **regex-style text parser over `cargo build-sbf` log output**, matching lines like `"Stack offset of N exceeded max offset of 4096 by M bytes"`. This is a compile-time log-scrape for the existing 4KiB static-stack-size warning the toolchain already prints — it is not a runtime semantic check of SIMD-0459/0460's stack-address-layout-dependence class of bug (code that computes/stores an absolute stack address and breaks when frame gaps are removed, which survives compilation cleanly and only surfaces at runtime). **The deck's claim is exactly right**, and is in fact under-stated if anything — the gap is not just "no mention of 0459/0460," it's "the nearest-sounding check is a different, narrower, already-solved problem."

**Pricing convergence** (VERIFIED, `research.md`): identical `$99–199/mo Pro`, `$1,000–5,000+/mo Enterprise` tiers, explicitly labeled by UpgradeGuard's own authors as "hipótese, não resultado de pesquisa de willingness-to-pay" (hypothesis, not a validated result) — matches the deck's framing that this is a directional signal only, not proof anyone will pay.

**Is it an ongoing live threat?** Commit activity stopped as of the last fetch (2026-09-24T15:40:24Z, i.e., before "today"); no evidence in this session of continued development past that point. It could resume at any time before the 2026-10-12 deadline — that risk is real and should be named to the builder, but as of right now it is a snapshot of a one-day sprint, not a moving target actively closing the two identified gaps.

---

## Other competitors found

- **SolanaRepro** — unchanged since 2026-08-26 (VERIFIED, commit history), MVP scope explicitly excludes complex CPI/ALT/Token-2022/DeFi — the exact programs most at risk from SIMD-0459/0460.
- **Surfpool `profile` PR #786** — still open/unmerged (VERIFIED), no binary-override capability, unresolved historical-feature-state correctness bug.
- **`feature-activation-verifier`** (Solana Foundation) — 1 star, stale since 2026-05-26, generic gate matrix, not per-program.
- **Mollusk / Trident** — libraries/harnesses requiring manual composition, not combined fetch+replay+diff+lint tools.
- **`solana-verify`** — build-authenticity, not runtime-behavior diffing.
- **New find this session, `sondir`** (`github.com/rifuki/sondir`, crates.io, MIT license, created 2026-07-04 — predates the hackathon, so an independent real tool, not a hackathon rival): a legitimate small crate (57 downloads) doing Anchor/toolchain **dependency-version pre-flight checks** (`sondir doctor`, `sondir resolve`, `sondir fix`) — it lists SIMD-0500 only as a live feature-gate status line item and has no LiteSVM dual-binary replay, no ELF/sBPF static linting, no SIMD-0459/0460 coverage. Adjacent problem space (toolchain fragility around the same migration), zero mechanism overlap with simd-diff.
- GitHub repository searches for `sbpfv3 solana`, `simd-0500`, `solana program replay diff`, `solana upgrade diff`, `litesvm replay`, `solana sbpf migration`, `solana program diff tool` returned **0 results** for the ones that completed before hitting the GitHub API's unauthenticated rate limit (10/hr for search, exhausted mid-session). No other direct competitor surfaced.

---

## TAM/business sanity check

The 17,332 figure is solid — it's SIMD-0500's own on-chain census, independently corroborated by UpgradeGuard's separately-sourced Blueshift program-sync count, so two independent parties converge on the identical number via different framing. But it is a ceiling, not a market size: both this deck and UpgradeGuard's own research honestly concede the "economically relevant" subset (actively maintained, source-available, team present, expecting future upgrades) is materially smaller and neither has quantified it. The business model — free OSS CLI for the long tail, paid CI-gate SaaS for teams, audit-firm channel — is a believable devtool shape that satisfies judging criterion (f) ("is there a viable business that *could* be built"), but the cited pricing ($99–199/mo, $1–5k/mo) is a competitor's own unvalidated hypothesis, not market evidence, and should be presented to judges with that caveat intact rather than as a forecast.

---

## Recommendation

**Do not try to out-build UpgradeGuard's breadth.** It already ships census/scanner/sources/dashboard/GitHub-Action/migrate-PR machinery that a solo builder cannot match in the remaining ~17 days on top of everything else. Racing on feature count against a tool that already has five build "rounds" shipped is not winnable and is the wrong fight.

**Commit explicitly and loudly to the two confirmed, currently-real gaps, and build the entire demo around proving them on camera:**
1. **Real-mainnet-tx auto-fetch, zero config.** `simd-diff fetch <program_id> --recent N` needs no hand-authored fixture — this is the single feature that matters most for the exact long-tail, abandoned-or-thin-team programs the 17,332 population is made of, since nobody is left to author a `fixtures.json` for them. Demo this directly against UpgradeGuard's fixture requirement, named on screen.
2. **SIMD-0459/0460 runtime stack-semantics detection**, not a compile-log grep. Build (or clearly demo) a check that catches a program relying on absolute stack-address layout — the class of bug that survives compilation and UpgradeGuard's `stack.rs` structurally cannot see. Even a narrow, honest version of this (e.g., flagging specific stack-address-dependent instruction patterns via static analysis, as the deck's §3.5 already plans) is a real, checkable, novel differentiator as long as it's demonstrably different from a 4KiB-overflow log parser.

This is still a strong-enough pitch to build in the remaining time: the differentiators are narrow, technically real, directly verifiable by a judge who checks both repos (which this deck should assume happens, since it already happened once — twice, counting this pass), and they map cleanly onto the Novelty and Functionality judging criteria specifically because they are the two things that require real protocol-semantics understanding rather than orchestration of existing tools. The deck should update its pitch framing to lead with this narrower claim rather than the original "auto-fetch + binary-diff + static-lint, nobody else combines all three" framing, which is no longer accurate now that two of those three are also present (if narrower) in a competitor.

**If this narrower angle is not something the builder wants to commit to explicitly**, this is not yet bad enough to justify a full REJECT and fallback to **HL Clear-Sign** or **Tempo Watchtower** (both in `research/01-shortlist.md`) — the whitespace has narrowed, not vanished. But it is close enough to the line that the builder should make this call deliberately and early, not discover it after 10 days of build time when it's too late to pivot to a reserve idea.
