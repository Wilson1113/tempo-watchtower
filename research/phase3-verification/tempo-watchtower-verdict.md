# tempo-watchtower — Phase 3 Adversarial Verification Verdict

Verification date: 2026-09-25. Independent re-fetch of primary sources (raw `curl`+`grep` of the
spec, direct GitHub/crates.io API + WebFetch), not a re-read of the deck's own citations. WebSearch
was not used (budget exhausted per instructions); all checks below are WebFetch/curl against known
or logically-derived URLs.

## VERDICT: SOUND WITH CAVEATS

The core technical claim (Check 1) is fully re-verified and rock-solid — this is the strongest part
of the pitch. The competitive-whitespace claim (Check 2) also holds after independent re-checking,
with one small, immaterial numeric discrepancy (repo count) worth noting but not fixing anything
load-bearing. The structural-neglect argument (Check 3) still holds on the available evidence, but
"still holds" here means "no counter-evidence found," which is a weaker claim than the deck's
confident phrasing implies — Tempo Labs could ship this and there is no public commitment either
way. The TAM figure is stale (6 months old) and this session could not find a fresher number either
— the deck already flags this, but the verdict below strengthens that caveat into a required fix.

---

## Check 1 — Does the problem exist exactly as described?

**VERIFIED, independently, byte-exact.** Fetched `https://paymentauth.org/draft-tempo-session-00.txt`
directly with `curl` (bypassing any AI summarizer), got a 3013-line document, dated 23 September
2026, expires 27 March 2027, header authors L. Horne / G. Konstantopoulos / D. Robinson / B. Ryan /
J. Moxey, Tempo Labs — matches the deck's citation exactly.

Re-confirmed by direct `grep` of the raw text (line numbers from my own fetch, cited for
independent traceability):
- Line 684–685: "User requests channel closure, starting a grace period of at least 15 minutes" (§6.4.5 `requestClose`).
- Lines 1999–2015 (§13.3 "Forced Close"): the exact 5-step sequence quoted in the deck, including
  "5. Client receives all remaining (unsettled) funds" and the "SHOULD wait at least 16 minutes"
  buffer recommendation.
- Lines 1753+ (§12.1 "Accounting State"): `acceptedCumulative`, `spent`, `settledOnChain`, `deposit`
  fields confirmed present as described.
- Line 2323 (§14.12): "Monitor for ChannelClosed or CloseRequested events" — confirmed present,
  verbatim.
- Lines 2343–2345 (§14.13 "Grace Period Rationale"): "Provides time to detect close requests and
  submit final settlements, even during network congestion or maintenance windows" — confirmed
  present, verbatim. This independently reproduces the deck's own risk-transparency finding that an
  earlier AI-summarized WebFetch pass falsely reported this phrase as absent; my direct `grep`
  confirms the deck's correction was right, not the false negative.
- `grep -ic "watchtower"` and `grep -ic "keeper"` on the raw text both return `0` — confirmed no
  first-party watchtower/keeper concept exists anywhere in the spec's own text.

**Newer draft check:** `paymentauth.org/draft-tempo-session-01.txt` returns HTTP 404 (checked
directly, 2026-09-25) — no draft-01 has been published. Also checked
`datatracker.ietf.org/doc/draft-tempo-session/` — 404 as well, meaning this is a self-hosted
informal draft (uses IETF boilerplate formatting but is not tracked on the actual IETF datatracker),
so there is no IETF process trail to check for a pending revision either. **Conclusion: mechanism,
grace period, and "maintenance windows" framing are all current as of today and unchanged from what
the deck cites. No newer draft exists anywhere I could find.**

---

## Check 2 — Does a competitor exist?

**No competitor found — re-confirmed independently, with one immaterial discrepancy noted.**

- **tempoxyz GitHub org, fresh fetch via `api.github.com/orgs/tempoxyz/repos?per_page=100`
  (2026-09-25):** returned **69 repos**, not the deck's claimed 72 (no pagination `Link` header was
  present, so 69 is the complete current count, not a partial page). This is a 3-repo discrepancy
  against the deck. Given this org visibly churns rapidly — near-duplicate repos like `reth`/`reth-1`
  through `reth-4` and `foundry`/`foundry-1`/`foundry-2`, and multiple repos with `pushed_at`
  timestamps within hours of my fetch — a handful of repos appearing/disappearing between the deck's
  fetch and mine (both same day) is plausible and not suspicious. **Materially, the conclusion is
  identical**: scanning all 69 names and descriptions, none is named or described as a
  watchtower/keeper/settlement-monitor/channel-liveness daemon. `mpp-tools`, `gh-actions`, and `docs`
  are all present and recent (pushed 2026-09-25, 2026-09-25, 2026-09-24 respectively), matching the
  deck's "three new-in-09" claim.
- **`mpp-rs` (crate `mpp`, crates.io):** confirmed via crates.io API — `max_version`/`newest_version`
  both `0.13.0`, `updated_at` 2026-09-21. Confirmed via direct `grep` of the raw README
  (`raw.githubusercontent.com/tempoxyz/mpp-rs/main/README.md`, 197 lines): zero occurrences of
  "watchtower," "keeper," "monitor," "settle," or "close" — matches the deck's claim of a
  protocol-primitives-only SDK. Confirmed the quickstart RPC endpoint `rpc.moderato.tempo.xyz`
  verbatim via `grep`. Confirmed via GitHub file-tree fetch that `crates/` contains **exactly one**
  workspace crate, `alloy-transport-mpp` — matches the deck's "no autonomous monitoring, one extra
  transport-wrapper crate" claim exactly.
- **crates.io name-squat check (re-run independently with all four query strings):** `tempo-watchtower`
  → 0 results. `tempo-keeper` → 0 results. `mpp-watchtower` → 0 results. `tempo-channel` → 0 results.
  A generic query for `watchtower` returns only `matthewjberger/watchtower` (a WIP, unrelated generic
  crate, 1443 downloads) and Solana-validator tools (`solana-watchtower`/`agave-watchtower`) — no
  Tempo/MPP hit. **Confirmed: the name and the problem space are both open.**
- **`danhper/temprano-watchtower` name-collision, independently re-verified:** fetched the raw README
  directly (`raw.githubusercontent.com/danhper/temprano-watchtower/main/README.md`) — confirms it is
  "a Rust service that accepts signed Tempo transactions, stores them durably, and broadcasts them
  throughout their validity window until mined, expired, invalid, or canceled," grouped by nonce key,
  hosted at `watchtower.temprano.io`. This is a general-purpose transaction-delivery/nonce-management
  service with **no mention anywhere in the README of channels, vouchers, `ChannelClosed`,
  `CloseRequested`, or MPP** — confirms the deck's characterization exactly: unrelated to session-channel
  forced-close monitoring. Also independently confirmed via WebFetch of the GitHub page itself: **0
  stars**, description "Rust service for durable ingest and guaranteed broadcast of scheduled Tempo
  transactions" (I could not get an exact last-push timestamp — GitHub API calls were rate-limited
  unauthenticated during this session, and the HTML page doesn't surface it plainly; the deck's
  "last pushed 2026-02-06" claim is **UNVERIFIED by me**, could not confirm or refute it directly,
  though nothing found contradicts it).
- **First-party announcement check:** checked `tempo.xyz/blog` directly — the only two MPP-related
  posts are "MPP Credits: fund your agent with a card" (17 Jun 2026) and "MPP Sessions: Web-Scale
  Payments for AI Agents" (2 Apr 2026); neither mentions a watchtower/keeper/monitor. Checked
  `github.com/tempoxyz/mpp-specs` issues for "watchtower" — no results. **No first-party
  announcement found from Tempo Labs or Stripe.**

---

## Check 3 — Does the "structural neglect" argument really hold? TAM sanity check.

**Structural argument: holds, but only as an absence-of-evidence finding, not a positive
commitment either way.** I looked for a public roadmap to check directly for a planned first-party
watchtower:
- `docs.tempo.xyz/roadmap` redirects (308) to `tempo.xyz/developers/roadmap`, which returns **HTTP
  404** — there is currently no public roadmap page at that URL. I could not find an alternate
  roadmap page within this session's tool constraints (no WebSearch to discover one at a different
  path).
- `github.com/tempoxyz/mpp-specs` issues and `github.com/tempoxyz/tempo-support` issues: no
  "watchtower"/"keeper"/"monitor" hits in either.
- **Side finding, not a red flag but worth noting:** `tempoxyz/tempo-support` — the repo the deck's
  own evidence trail implicitly treats as "Tempo's community support channel" — shows a banner:
  "This repository was archived by the owner on Sep 9, 2026. It is now read-only," with no
  redirect/replacement pointed to. This doesn't change the watchtower finding (issue search on an
  archived repo still returns real historical issues, and none matched), but it does mean the
  primary place a community feature request would have been filed is no longer active as of
  2026-09-09 — support may have moved to Discord or elsewhere, which this session's tools can't
  check. **UNVERIFIED where support moved to.**
- **Net assessment:** the deck's "Lightning-watchtower-precedent" argument (a watchtower is
  structurally supposed to be third-party, so Tempo Labs building one in-house would defeat its own
  purpose) is a sound *logical* argument independent of any roadmap evidence, and I found nothing
  that contradicts it. But the deck's framing ("this is the cleanest permanent, not circumstantial
  neglect story in the whole pool") should be read as "no public evidence of Tempo Labs building
  this," not "Tempo Labs has ruled this out" — no such statement exists anywhere I could find.

**TAM sanity check: the ~326-server / ~671-agent figure is now ~6 months stale, and I independently
could not find a fresher number either.** Checked `mpp.dev` (fetched directly — no stats/metrics of
any kind on the page), `mpp.dev/stats` (404), and `tempo.xyz/mpp` (fetched directly — no adoption
numbers). **This is not a refutation of the deck's number, but it is a real gap**: MPP is a
fast-moving, actively-promoted flagship product (two blog posts in the last 6 months, 69-repo org
with daily commit activity) — the true current server/agent count could plausibly be meaningfully
higher (or, less likely given the promotion, lower) than the March 2026 snapshot, and neither the
deck nor this verification pass has a way to confirm which. The deck already flags this as INHERITED
and as "the weakest point to defend honestly," which is the right call — but see required fix below.

---

## Required deck fixes

1. **TAM staleness — strengthen the caveat, don't just flag it.** The deck's business-plan section
   (§7) mentions the staleness once; the problem-statement section (§2) states the 326-server figure
   more matter-of-factly. Recommend adding an explicit "as of [date], ~6 months old, most likely
   understates current adoption given two 2026 growth-oriented blog posts since" caveat in §2 itself,
   not just §7, and — if time allows before submission — one more targeted attempt to find a live
   MPP explorer/dashboard (not found in this session; may exist under a URL neither pass tried).
2. **Repo-count discrepancy (69 vs. 72) — cosmetic fix.** Update the "72 repos" figure to whatever
   the count is at time of final submission, or soften to "~70 repos" — the exact number is
   volatile day-to-day in this org and isn't a claim worth being precise about. Does not affect any
   substantive finding.
3. **"Structural neglect" framing — soften the confidence slightly.** Change phrasing like "the
   cleanest permanent, not circumstantial neglect story in the whole pool" to something that
   distinguishes "no public roadmap or announcement found" (what was actually verified) from "Tempo
   Labs has structurally ruled this out" (not verified, and not verifiable from public sources). The
   underlying Lightning-watchtower analogy argument is fine to keep — it's a logical argument, not
   an evidence claim, and doesn't need a source citation.
4. **`temprano-watchtower` last-push date — mark UNVERIFIED, not re-asserted.** This verification
   pass could not confirm the deck's "last pushed 2026-02-06" claim (GitHub API rate-limited
   unauthenticated during this session); nothing contradicts it, but if it resurfaces in the final
   pitch appendix it should carry an UNVERIFIED tag until someone confirms it with an authenticated
   API call or a fresh page load showing the commit date plainly.
5. **No changes needed to Check 1's technical claims** — every quoted section, line, and phrase was
   independently reproduced byte-for-byte from a fresh `curl` fetch. This is the part of the deck
   that needs zero rework.

## Sources fetched directly this session (for traceability)
- `https://paymentauth.org/draft-tempo-session-00.txt` (raw curl, 3013 lines)
- `https://paymentauth.org/draft-tempo-session-01.txt` (404)
- `https://datatracker.ietf.org/doc/draft-tempo-session/` (404)
- `https://api.github.com/orgs/tempoxyz/repos?per_page=100` (69 repos, no pagination header)
- `https://raw.githubusercontent.com/tempoxyz/mpp-rs/main/README.md` (197 lines, grepped)
- `https://github.com/tempoxyz/mpp-rs` and `.../tree/main/crates` (WebFetch, file listing)
- `https://crates.io/api/v1/crates/mpp` and `?q=` searches for tempo-watchtower/tempo-keeper/mpp-watchtower/tempo-channel/watchtower
- `https://raw.githubusercontent.com/danhper/temprano-watchtower/main/README.md`
- `https://github.com/danhper/temprano-watchtower` (WebFetch)
- `https://mpp.dev`, `https://mpp.dev/stats` (404), `https://tempo.xyz/mpp`
- `https://tempo.xyz/blog`, `https://docs.tempo.xyz/roadmap` (redirect) → `https://tempo.xyz/developers/roadmap` (404)
- `https://github.com/tempoxyz/mpp-specs/issues?q=watchtower`, `https://github.com/tempoxyz/tempo-support/issues?q=...`, `https://github.com/tempoxyz/tempo-support` (archived banner)
