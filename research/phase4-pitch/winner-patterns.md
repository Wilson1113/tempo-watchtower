# WINNER-PATTERNS — How past Colosseum winners actually pitched (for tempo-watchtower)

Research date: 2026-09-27. Builds on `phase1-scouting/hackathon-intel.md` §A-B (judging dims, winner lists, cohorts) — not repeated here.

**Method.** WebSearch was not used (budget exhausted). Sources were fetched directly with `curl`:
- The blog sitemap (`blog.colosseum.com/sitemap-posts.xml`, 163 posts). I read every winner announcement, every cohort announcement and the guidance posts.
- `colosseum.com/hackathon` (FAQ) and `colosseum.com/worldsfair`.
- 40 arena project pages. They are SvelteKit pages, but the submission data is embedded in the HTML: description, category, repo, pitch-video link and team roster.
- 12 GitHub READMEs.
- One Google Slides pitch deck (Zircon), exported as text.
- **Pitch-video transcripts for 15 winners**, pulled from Loom's public transcript JSON. These are machine transcripts (Whisper), so some words come out garbled. I quote them as-is.

YouTube-hosted pitches (TAPEDRIVE, MCPay, FluxRPC, Seer, Txtx) returned only title and length. Their captions were blocked, so there are no transcripts for them.

**Labels.** VERIFIED means I fetched it from the primary source in this session. UNVERIFIED means it is inferred or not fetched. INFERENCE marks my own synthesis.

---

## A. Colosseum's official pitch guidance (verbatim)

### A1. "How to Win a Colosseum Hackathon" (Matty Taylor, 20 Feb 2024): VERIFIED
URL: https://blog.colosseum.com/how-to-win-a-colosseum-hackathon/

- "The final presentation is critical since there are hundreds of product submissions… the product presentation slide deck and video are the most important component in rising above the crowd."
- "In the final week of the hackathon, the focus should center on the presentation video (we recommend using Loom over a slide deck). Effective pitches always include:
  - Team background
  - Product description
  - Why you started building the product
  - The potential market opportunity unlocked by your product
  - How you will get initial product usage (or if applicable, the traction and user feedback already received)
  - How the product works (your demo)"
- "Judges have to review hundreds of project submissions, so presentations are required to be under 3 minutes. As you practice your presentation recording, be concise when telling your product story, walking through your demo, and explaining why it is destined to become the next breakout crypto product."
- On prioritisation: "prioritize the features that allow you to create an amazing working demo… Prioritize the features that you believe will give users an 'aha' moment where they understand why your product fulfills their needs and how they will use it on a frequent basis."
- On the final week: "The final week should be spent on testing the demo to ensure functionality and creating the submission presentation pitch."
- On building in public: "create a project Twitter X account and begin sharing your product vision… it enables you to find beta testers who will provide feedback during the engineering sprint."
- On judging intent: "prizes will be awarded to teams who intend to build full-time and develop products with potentially viable business models (although, there will always be an award for the best public good)."
- On ideation: "Don't be afraid to build products that have been attempted before. Timing and execution matter a lot…" and "Build products that solve problems that affect you personally."

### A2. "Perfecting Your Hackathon Submission" (Mike Hale, 8 May 2025; workshop by Nate Levine, Matty Taylor, Clay Robbins): VERIFIED
URL: https://blog.colosseum.com/perfecting-your-hackathon-submission/

- "The pitch video is the most important element of the submission. It is usually the first item judges review and can determine whether a project is shortlisted for deeper evaluation."
- "The video should be no more than three minutes long and should include a concise explanation of the team's background, the problem they are solving, and who the product is for. Teams should also mention any feedback or validation they have received from users, even if informal, and outline the broader vision for the project."
- "**A clear and well-structured narrative is more valuable than professional video editing**, such as a voiceover accompanying a slide deck. Teams are encouraged to **treat their pitch like a brief startup pitch, not a product demo**."
- Technical demo: "While the pitch video is for explaining the why, the technical demo is about the how. Teams should avoid turning this into another pitch. It should be technical, direct, and specific to implementation…" and "walk through the core features they built, explain their tech stack, and outline the decisions made in prioritizing specific components. **Judges are particularly interested in the reasoning behind these decisions**, especially with regard to Solana integration, on-chain logic, and overall architecture." For World's Fair, read "Solana integration" as Tempo integration.
- Validation: "judges look for evidence that a team is solving a real problem for real users. Projects that stand out typically demonstrate early traction, conversations with potential users, via feedback on platforms like Twitter or Telegram."
- "Teams should focus on explaining the market they are targeting, the problem's scale, and why their solution matters. **Even projects building in public goods categories should show evidence of product-market fit through usage, community involvement, or open-source adoption.**"
- **Solo founders:** "Solo founders are allowed. However, many of the top-performing teams are two or three people… **Solo founders should explain their relevant experience and why they're uniquely suited to build the product.**"
- Accelerator: "emphasizes founder potential and adaptability over the hackathon submission itself."
- Post-submission: "Judges may ask about post-hackathon progress during interviews… weekly update videos… can strengthen a submission by demonstrating momentum and iteration."

### A3. Current submission portal and judging FAQ (colosseum.com/hackathon, fetched 2026-09-27): VERIFIED
- "Hackathon builders should view their product submission as **a pitch to Colosseum's venture fund**, other investors, and an application to our Accelerator."
- Required fields:
  - Name and brief description
  - Chains and tools integrated
  - Teammates "with context on their backgrounds and previous experience"
  - Location, logo and GitHub repo
  - "A two-to-three-minute presentation video. This is one of the first resources judges review, so it should be clear, concise, and high quality."
  - "A product-demo video of no more than three minutes explaining how the product works"
  - "**Go-to-market strategy, demand validation, and plans for developing distribution**"
- "We evaluate more than the product itself. We want to understand founder-market fit, **how the opportunity was uncovered**, how the team prioritizes, and whether the founders are committed to building a venture-scale business."
- Criteria worth quoting for our case:
  - Insight: "Is there a new technology, or a new trend that creates an opportunity for their startup to succeed?"
  - Market Size: "Is it already large, **or small but growing rapidly?**"
  - Viability: "presentations should provide a holistic view of the company."
- GitHub, what they look for: "Did significant work during the hackathon / Were the ones to do this work… / Prioritized feature development strategically."
- GitHub, what they do **not** look for: "Using a particular language or framework / Specific design patterns, best practices, or code-quality checks."
- Weekly updates: "strongly recommend them for anyone serious about competing… a concise, one-minute video highlighting progress and notable challenges."
- Solo: "You may submit a product as a solo founder. However, our hackathons are highly competitive, so we recommend teaming up with builders who can help develop both your product and go-to-market strategy."

### A4. How Colosseum says it actually selects: VERIFIED
- Frontier winners post (26 Jun 2026): the next 25 "demonstrated the necessary **execution speed, insight, founder-market fit, prioritization, and overall talent** to potentially build an enduring crypto startup." (blog.colosseum.com/announcing-the-winners-of-the-solana-frontier-hackathon/)
- Every cohort post, Cohorts 1-5: selected startups "exhibited the **technical talent, speed, vision, and competitive drive** required to build… enduring products."
- Volta Prize post (Matty Taylor, 2026). They run an internal evaluation engine ("Cerebro") that scores "founder-market fit, product velocity, user demand, market timing, **evidence of grit from their past**, and surfaces the spiky green flags." Also: "the signals we act on **look nothing like a pitch deck**. We watch thousands of teams ship real products, under pressure, for five weeks… we are acting on weeks of evidence about **who iterates, who listens to users**." (blog.colosseum.com/the-volta-prize-colosseum-and-hackathons-in-the-age-of-ai/)
- Cohort V Demo Day (Codex, Sep 2026): pitches were **two minutes each**, and "**Nine of the 21 list a traction number** in Colosseum's Demo Day program." (blog.colosseum.com/cohort-v-demo-day-solana-microscope-alpenglow-migration/)

### A5. Accelerator-level framing of infra bets ("2024 Investments and Themes for 2025"): VERIFIED
URL: https://blog.colosseum.com/colosseums-2024-investments-and-themes-for-2025/
- Dev tooling: "As crypto matures, development teams require increasingly advanced tooling to secure, **monitor**, and scale their products. In traditional tech, tools like HashiCorp's Terraform made cloud infrastructure management far simpler… Txtx is poised to offer similarly transformative infrastructure." Also: "For monitoring and rapid bug resolution, **Tokamai has also emerged as a crypto-native equivalent to Sentry**."
- Rakurai: "we also believe there are **sustainable opportunities for monetizing more tightly-scoped** validator improvements even sooner. Rakurai, **a team of experienced engineers from Apple and high-frequency trading**… benchmarked at up to 5x the current block rewards and yield boosts of over 30%."
- Takeaway (INFERENCE): Colosseum explains an infra bet in three moves. First a Web2 incumbent analogue (Terraform, Sentry). Then a tight, monetizable scope. Then either team pedigree or a benchmark number.

---

## B. Per-winner teardown (infra / dev-tool / public-goods / payments-infra)

Team size is the number of members listed on the arena page at submission (VERIFIED). It may undercount people who work off-platform. Pitch length comes from Loom or YouTube metadata.

| Project | Hackathon / prize (→ accel.) | Their one-liner (verbatim) | Problem-framing style | Traction shown | Business framing | Team / "why us" | Source | Status |
|---|---|---|---|---|---|---|---|---|
| **Zoneless** | Frontier 2026, Public Goods ($10k) → **Cohort 5** | "an open-source alternative to Stripe Connect using USDC on Solana… ~$0.002 per payout vs $2+ on Stripe… identical API… migrate in minutes" | **Founder's own pain + Web2 analogue.** "I built Zoneless to solve my own problem," from paying Stripe Connect fees at PromptBase. The transcript garbles the amount ("seven, $9,400 per month"). Names three flaws of the incumbent: fees, days-long payouts, limited countries | Dogfooded in production: "over 1000 payouts… onboarded over 2000 sellers… **over 73% of people will now choose Zoneless over Stripe Connect**" on PromptBase | Savings math: "a thousand payouts per month… $30… saving over $38,000… a year." Open source (Apache 2.0) and self-host with one-line Docker | **Solo.** "master's… co-founding a FinTech… Gradient Ventures… ~11,000 followers… founder of PromptBase… 450,000 users." Pitch 3:45 | arena/zoneless; Loom transcript; github.com/zonelessdev/zoneless | VERIFIED |
| **Tokamai** | Radar 2024, 2nd Infra → **Cohort 2** | "Catch Solana errors before your users do. Real-time monitoring and alerting for on-chain programs. Detect, analyze, fix – faster than ever." | **Status-quo story with a hook.** "It's 2024, and some Solana teams are still waiting for the users to get angry to start looking for errors…" Then walks the pain: Discord → Solscan "walls and walls of transactions" → "weighs a ton of engineering hours… users are losing money… really bad for the brand" | **Logos + first revenue.** "onboarded Metaplex, Sphere, Etherfuse, Helium, and Radiance… yesterday… **Etherfuse became the first paid customer**." Live demo on the real Metaplex Bubblegum dashboard | "full suite Solana program DevOps… get started in seconds." Competition: "no real competition… only competitor… on Ethereum… BlockTorch… acquired" | **Solo.** "This is my third company. I have a 10 years experience selling B2B to large companies," plus a named Solana history. Pitch 3:34 | arena/tokamai; Loom transcript ("[Radar final pitch] Tokamai - Pitch Deck - v4 - Google Slides") | VERIFIED |
| **IDL Space** | Breakout 2025, Public Goods ($10k) | "a Postman-like tool… to seamlessly explore program interfaces, validate, test, and debug Solana programs" | **Newcomer's-eye Web2 gap.** "In Web2, we have powerful API tools. Like Postman, Swagger… But in Solana, the tools are limited." Three named problems: Speed, Collaboration, Composability | None beyond a working product. Demo is "three simple steps. Load IDL, send transaction, check logs" | Public good; 4-step roadmap ending in "an IDL marketplace" | **Solo.** "17 years of development experience… developed a Bitcoin wallet 12 years ago… a YC company… side project [with] more than 100,000 users." Pitch 2:48 | arena/idl-space; Loom transcript; github.com/williamwa/idlman | VERIFIED |
| **Zircon** | Renaissance 2024, Public Goods ($10k) | "Challenges and guided courses for Solana developers." | **Two-slide problem.** "Onboarding new developers is non-linear… Tutorial hell is rampant" / "Evaluating existing developers is time-consuming." | **Web2 comparables as proof of demand.** "CryptoZombies 400k+ Registered Users / LeetCode 5m Registered Users" | "This is a free and open-source public good." GTM slide: "First 10k users / Next 100k users" | **Solo.** "Previous Founder of Stockpile Labs • Built the 1st Quadratic Funding Primitive on Solana" | arena/zircon; **deck text exported from Google Slides** | VERIFIED |
| **TAPEDRIVE** | Breakout 2025, **Grand Champion** ($50k) → **Cohort 3** | "makes it easy to read and write data on Solana. **It's over 1,400x cheaper than using an account.**" | **One quantified multiple against the status quo**, then a short how-it-works: "compressing your data into tiny on-chain proofs… miners…" | Not in the description; the pitch transcript was unavailable | Token economics stated in the description: "TAPE token, capped at 7 million (decaying ~15% per year)" | **Solo** ("Zel: Making solana do weird things"). Pitch 3:00 (YouTube "TAPEDRIVE: Pitch Deck") | arena/tapedrive; YouTube metadata | VERIFIED (desc) / transcript UNVERIFIED |
| **MCPay** | Cypherpunk 2025, 1st Stablecoins ($25k) → **Cohort 4** | "Charge for Model Context Protocol tools, data sources, and specialized agent capabilities using x402." | **Verb-first one-liner** (what you can now do). README: "Why MCPay (in 30 seconds)", with one line each for Clients, Developers and Agents | README links a live registry (mcpay.tech/servers) | Open-source infra (Apache 2.0). A proxy "handles `402` negotiation, on-chain payments, and retries" | **Solo.** Pitch 2:58 (YouTube) | arena/mcpay; github.com/microchipgnu/MCPay | VERIFIED (desc/README) / transcript UNVERIFIED |
| **Corbits** | Cypherpunk 2025, 2nd Infra | "open-source x402 endpoint dashboard for merchants… lets merchants and facilitators make real-time revops and devops decisions" | **Vision-first**, paradigm framing ("x402 represents a paradigm shift"). Problem stated abstractly: "information asymmetry… billing is difficult… uptimes… don't go away" | Vague: "our customers are being receptive." The README lists live dashboards for DFlow, Helius, Nansen, Titan, Triton and Yatori | Monitoring and revenue dashboards (Grafana) as a wedge into agentic commerce | 1 listed member (chief of staff). Credibility line: "Our software lives not just on the earth. But also on Mars." Pitch 3:05 | arena/corbits.dev; Loom transcript; github.com/faremeter/402-dashboard | VERIFIED |
| **Latinum** | Breakout 2025, 1st AI ($25k) | "a payment middleware that enables MCP builders to get paid" | **Concrete user-journey friction.** A coding agent needs Figma, "can't pay, it can't sign up" → developer creates account → card → credits → API key… "It takes the agency out of agent." | "investment from NDRC for 100k… pilot with a local commerce platform in Dublin… significant VC interest" | "We'll charge a small fee… medium term… know-your-agent service akin to KYC." Market claimed as "multi-trillion dollar" | 2 people. "led GTM in Europe for Plaid" + "5 years at Amazon… Alexa." **Memorable closer:** "Agents that can't pay are just fancy search engines." Pitch 3:01 | arena/latinum-agentic-commerce; Loom transcript | VERIFIED |
| **Seer** | Cypherpunk 2025, 1st Infra ($25k) | "a breakthrough in transaction debugging for Solana. **Similar to Tenderly on EVM**… full function and line trace…" | **Cross-ecosystem analogue** (Tenderly), with an honest scope: "The POC… is extendable into a fully framework-agnostic solution to **10x developer debugging speed**." | POC only; README: "proof of concept tracing tool" | Not stated in the description | 3 people. "Former EVM expert. Written code managing $50m+ AUM." Pitch 3:00 (YouTube) | arena/seer; github.com/VasilyGerrans/seer | VERIFIED (desc) / transcript UNVERIFIED |
| **Samui Wallet** | Cypherpunk 2025, Public Goods ($10k) | "an open-source wallet and toolbox for Solana builders… not another consumer wallet" | **Values contradiction.** "Our ecosystem values auditability… However, our most critical tools… are closed source black boxes." Plus a gatekeeper problem | **Partner confirmations won during the hackathon:** Unruggable, Keystone, MoonPay, Jupiter, Titan, Arcium/Umbra | Explicit revenue even for a public good: swap and ramp fees, a validator with Triton, an LST, grants | 2 people. **Explicit ask and commitment:** "We hope to get accepted… We're both ready to work full-time." Pitch 4:34 | arena/samui-wallet; Loom transcript | VERIFIED |
| **One-Time Action Codes** | Breakout 2025, 4th Infra | "make blockchain interactions as simple as pasting a short code… a powerful UX primitive that enhances Solana Pay" | **Familiar-pattern analogue** (one-time codes "used by billions every day": "We are not inventing a new habit") | "It's working today. We are live on DevNet." | "lightweight fee per action… we sponsor all transaction costs"; verified branded prefixes for enterprises | **Solo.** "8+ years… Everywhere I looked, the same issue kept showing up." Pitch 3:06 | arena/one-time-action-codes-1; Loom transcript | VERIFIED |
| **Ionic** | Cypherpunk 2025, 3rd Infra | "the missing data aggregation layer for Solana… milliseconds while traditional tools (Dune, BitQuery) take minutes" | **Concrete query example** ("find top 10 traders on a specific token") plus a named incumbent speed gap | **Revenue:** "$15,000 a month from [three] customers… production server… running for the last two weeks" | Custom builds → generalize into a cloud tool → marketplace | 2 people. Origin: co-founder "realized how difficult and painful it was to do data aggregation for his… strategy." Pitch 3:19 | arena/ionic; Loom transcript | VERIFIED |
| **Sudont** | Frontier 2026, winner ($10k) | "The bare-metal execution firewall and local RPC for Solana. **After running a passive node and seeing thousands of trapped bots fail transactions, I built Sudont.**" | **Pure live demo.** "Nothing on the screen is faked. These are all real RPC calls in a real chain state." Then scripted attack scenarios, each blocked on screen | Demo only | Defensive firewall, plus an "offensive" revenue use (Monte Carlo for quants) | **Solo.** "Ex-Citadel & Custody Architect ($15B+ secured)." Pitch 3:08 | arena/sudont; Loom transcript | VERIFIED |
| **Torque** | Renaissance 2024, 2nd Infra → **Cohort 1** | "an onchain offers protocol for builders to deploy marketing strategies, at scale" | **Customer discovery as proof.** "We've interviewed hundreds of builders" (description) / "dozens of teams" (pitch) → "distribution in crypto is broken" | "For Colosseum we built an SDK and the offers launchpad." Prior win: "grand prize for Metaplex C-Hack" | Vision: "Solana's onchain attention economy" | 3 people. "building on Solana for the last two and a half years." Pitch 2:27 | arena/855; Loom transcript | VERIFIED |
| **Autonom** | Cypherpunk 2025, 1st RWA ($25k) | "a specialized oracle for RWAs, factoring in for corporate action adjustments" | **Founder hit the wall himself** (co-founder "couldn't get past the oracle issue when building a perp") plus a dated real event (Netflix earnings) | Pull oracle and risk module on testnet. A partner proposal accepted (transcript garbled) | **Named ICP** (Jupiter, Drift, Flash Trade, Adrena, GMX), named competitors (Pyth, Stork, RedStone), and top-down sizing ("$1 trillion… Solana… $60 billion") | 2 people. "ran Techstars Web3" + "developer of API3's core OEV product." Pitch 3:21 | arena/autonom…; Loom transcript | VERIFIED |
| **Credible** | Cypherpunk 2025, 2nd Stablecoins → **Cohort 4** | "The first USD–INR remittance rail… guaranteed minimum FX rate that's 2% better than Wise, XE, or Remitly." | **Mechanism vs incumbent latency** ("ACH… 24 to 48 hours") | **Hard numbers, on-chain verifiable:** "close to $50 million in transactions… $5.8 million this month… 175K in revenue… 1.6%… integrated on Dune" | Take rate stated | 2 people; payments and lending background. Pitch 3:02 | arena/credible-finance-1; Loom transcript | VERIFIED |
| **Txtx** | Radar 2024, 1st Infra ($30k) → **Cohort 2** | "turns the stress, pain and complexity of Smart Contract Infrastructure management into a secure, reproducible… developer experience" | **Web2 analogue + loss number.** README: "Txtx is to Web3 what Hashicorp Terraform is to cloud infrastructure"; "Every year, between $500M and $1B are lost due to compromised private keys" | README: "The 1st runbook ever executed on Mainnet moved $2.5M" | Beta; infra-as-code | 2 people; "Previously… Hiro & ConsenSys (Ganache)." Pitch 6:27 (YouTube) | arena/txtx; github.com/txtx/txtx | VERIFIED (desc/README) / transcript UNVERIFIED |
| **Rakurai (High TPS Solana Client)** | Renaissance 2024, 1st Infra ($30k) → **Cohort 1** | "boosts TPS with proprietary scheduling & pipeline optimizations… **up to 30% higher staking yield**" | **Benchmark-led** | Benchmarks (see A5) | "available through a liquid staking pool" | 3 people. "ex-Apple engineer with 20+ years." | arena/high-tps-solana-client | VERIFIED |
| **FluxRPC** | Breakout 2025, 1st Infra ($25k) | "**the first RPC on Solana that fully separates from the validator layer**… no credit games, just simple bandwidth-based pricing" | **Category-first claim** plus a pricing-model contrast | Not in the description | Bandwidth-based pricing vs credits | 2 people; "over a decade in web2." Pitch 2:46 | arena/flux-rpc-1 | VERIFIED (desc) |
| **Attest Protocol** | Radar 2024, Public Goods ($10k) | "**We're building https on the blockchain** 🔐" | **Single Web2 metaphor** as the entire description | — | — | Solo | arena/attest-protocol | VERIFIED |
| **Unruggable** | Cypherpunk 2025, **Grand Champion** → **Cohort 4** | "the first Solana-native hardware wallet… written entirely in Rust" | **Incumbent failure with a date.** "Solana support always come second, even Ledger delayed SPL token support until 2025." | Integration list (Jito, Squads, Jupiter…) and "sub 30 second set up" | Hardware plus app | 3 people | arena/unruggable-3 | VERIFIED (desc) |

### B2. How Colosseum itself rewrote infra winners' one-liners (winner post → cohort post): VERIFIED

| Project | Winner-post line | Cohort / themes line |
|---|---|---|
| Tokamai | "a developer tool to catch errors and for real-time monitoring" (Radar) | "Tokamai **catches development errors before your users do** through real-time monitoring and alerting for Solana programs" (C2); "a **crypto-native equivalent to Sentry**" (2024 themes) |
| Txtx | "a developer platform for engineering teams to leverage runbooks" | "takes away the **stress and complexity** of onchain infrastructure management with secure, robust, and reproducible blocks of code called Runbooks" (C2); Terraform analogue (themes) |
| Rakurai | "a new client that utilizes efficient scheduling and pipeline optimizations to improve transaction capacity and increase block reward yields" | "A new Solana client boosting TPS with proprietary scheduling & pipeline optimizations" (C1); "team… from Apple and high-frequency trading… up to 5x… over 30%" (themes) |
| Zoneless | Public Good Award, no line | "An **open-source alternative to Stripe Connect** using USDC. The platform lets **marketplaces pay their sellers globally, at near-zero cost**." (C5) |
| TAPEDRIVE | "a scalable decentralized storage network that rewards you for storing data" | "enables **low-cost, fully onchain data storage** on Solana using compressed proofs and a miner network…" (C3) |

INFERENCE: when a project is promoted to the cohort, Colosseum's rewrite moves toward **user + outcome + (mechanism)**, and toward a **Web2 incumbent analogue** for dev tools.

### B3. Solo-founder evidence (counters the "average winning team size >3" headwind): VERIFIED from arena rosters
Solo-listed winners in infra, dev-tool, public-goods or payments-infra:
- **Zoneless** (→ C5)
- **Tokamai** (→ C2)
- **TAPEDRIVE** (Grand Champion → C3)
- **MCPay** (→ C4)
- **Blackpool/Darklake** (→ C2)
- IDL Space, Zircon, Attest Protocol, One-Time Action Codes, CONYR, Mercantill, Sudont, Corbits (1 listed)

That is **five solo-listed projects later funded into the accelerator**. Every one of these solo pitches that has a transcript opens or closes with dense, specific credentials:
- Zoneless: PromptBase with 450k users
- Tokamai: "third company… 10 years… B2B"
- IDL Space: 17 years, a Bitcoin wallet 12 years ago
- Sudont: ex-Citadel, $15B+ secured
- Zircon: prior founder, "1st Quadratic Funding Primitive on Solana"

### B4. Directly analogous precedents for tempo-watchtower
- **Tokamai** (solo; monitoring and alerting for on-chain programs → Cohort 2) is the closest precedent in shape: a watcher that alerts before users lose money.
- **Corbits** (x402 endpoint monitoring dashboards → 2nd Infra) and **Flovia** (analytics for x402/MPP-paid APIs → Cohort 5) show that Colosseum funds **observability for machine-payment protocols**.

**Risk to address (VERIFIED):** the Solana Foundation open-sourced **Microscope**, "a free, self-hosted monitoring and alerting tool for Solana programs" (Codex, Sep 2026). A foundation can commoditize generic monitoring. tempo-watchtower's defence must be **independence as a structural requirement**: a watchtower is by design a party separate from the payee, as in Lightning. It is not "we monitor too."

---

## C. Distilled rules for a winning Colosseum infra pitch (evidence-backed)

1. **Put the one-liner first, and make it "[Name] is the [Web2 or other-ecosystem incumbent] for [ecosystem/user]", with the outcome in the same breath.** Evidence: Zoneless (Stripe Connect), IDL Space (Postman), Seer (Tenderly), Tokamai (Sentry, per Colosseum), Txtx (Terraform), Attest ("https on the blockchain"), Zircon (CryptoZombies/LeetCode). For us, the Lightning watchtower is the analogue, and it is a real, proven economic model.
2. **Name the user and the loss, not the technology.** Colosseum's rewrites (B2) turn "a developer tool to catch errors" into "catches development errors **before your users do**." Tokamai's pitch spends about 40% of its time on the loss path: angry users, engineering hours, "users are losing money", brand damage.
3. **Open with a dated, concrete hook or a status-quo walkthrough, not a market stat.** Tokamai: "It's 2024, and some Solana teams are still waiting for the users to get angry…" Latinum walks through a specific agent-needs-Figma flow. Autonom uses the Netflix earnings event. Sudont: "After running a passive node and seeing thousands of trapped bots fail…"
4. **Answer "why me" with dense, specific proof. For solo builders it is mandatory.** A2 says verbatim: "Solo founders should explain their relevant experience and why they're uniquely suited." Every solo winner transcript does this (B3). The Cerebro engine scores "evidence of grit from their past."
5. **Say how you found the opportunity: "I hit this myself" or "I read the spec".** The portal asks "how the opportunity was uncovered" (A3). Examples: Zoneless ("solve my own problem"), Autonom (co-founder "couldn't get past the oracle issue"), Ionic (co-founder's own pain), Sudont (ran a node). For us, the discovery is the spec's own forced-close text (§13.3 and §14.12 "SHOULD… Monitor for… CloseRequested events"). Quote the spec on screen.
6. **Give one quantified contrast against the status quo.** Examples:
   - Zoneless: "~$0.002 per payout vs $2+"
   - TAPEDRIVE: "over 1,400x cheaper"
   - Rakurai: "up to 30% higher staking yield"
   - Credible: "2% better than Wise"
   - Ionic: "milliseconds… vs minutes"
   - Seer: "10x developer debugging speed"

   For us, the contrast is "a missed 15-minute window = 100% of that channel's unsettled balance lost, vs. $X/month for a watchtower."
7. **Show traction as named people, not adjectives. Even hackathon-stage infra winners bring logos, pilots or first dollars.**
   - Tokamai: 5 named logos and "first paid customer… yesterday"
   - Ionic: $15k/month
   - Latinum: $100k pre-seed and a pilot
   - Samui: partner confirmations won during the hackathon
   - Zoneless: dogfooding numbers

   Colosseum: "mention any feedback or validation… even if informal." Public goods too: "evidence of… open-source adoption." Nine of 21 Cohort V pitches carried a traction number.
8. **Demo real state, and say so.** Sudont: "Nothing on the screen is faked. These are all real RPC calls in a real chain state." Tokamai demoed on the real Metaplex Bubblegum program. OTAC: "live on DevNet." Colosseum asks for the "aha moment" (A1). For us, the aha is a channel under a `requestClose`, the watchtower detecting it, and `settle()` landing inside the grace window, shown on Tempo testnet.
9. **Name the competition honestly and give the structural reason you win.** Tokamai named BlockTorch (acquired, on another chain). Autonom named Pyth, Stork and RedStone. Seer framed its work as the Tenderly gap on Solana. Our structural reason: independence from the payee, the same as Lightning watchtowers. Also address Microscope-style free tooling up front.
10. **State a business model even for open-source or public-good infra.** Examples:
    - Samui (a Public Goods winner): swap fees, validator, LST, grants
    - OTAC: a fee per action
    - Latinum: small fee now, KYA later
    - Zoneless: OSS plus savings math
    - Zircon: "free and open-source public good", backed by a GTM slide

    Colosseum: "intend to build full-time… potentially viable business models."
11. **Size the market honestly. "Small but growing rapidly" is an explicit, acceptable answer.** The FAQ criterion text says so verbatim (A3). Pair a small bottom-up count with the growth driver (MPP adoption), rather than a "multi-trillion" top-down claim (Latinum did that, but it is the weakest part of its transcript).
12. **Include a GTM slide with named first users and channels.** The portal requires "Go-to-market strategy, demand validation, and plans for developing distribution." Zircon's deck has a "First 10k / Next 100k" GTM slide. Autonom lists its ICP by name.
13. **Say you are going full-time, give a roadmap, and treat the submission as a beginning.** Samui made an explicit ask. The workshop's list of mistakes includes "Treating the hackathon as a finished endpoint." IDL Space and Ionic both close on a staged roadmap.
14. **Close by restating the one-liner or a quotable line.** Zoneless ends on "a Stripe Connect alternative." Latinum: "Agents that can't pay are just fancy search engines."
15. **Keep the pitch "why" and the tech demo "how", and keep Rust out of the pitch as a virtue in itself.** Colosseum is "not looking for… a particular language or framework" (A3). Put Rust in the tech demo only as the reason behind a property: a single static binary, no GC pauses near a deadline, correctness of the voucher state machine. Judges want "the reasoning behind these decisions."

---

## D. Anti-patterns judges penalize

**Verbatim, from the official "Mistakes to avoid" and "Common Mistakes" lists (A2): VERIFIED**
- "Exceeding the 3 minute time limit"
- "Using overly flashy visuals with little substance."
- "Over-relying on buzzwords." / "Relying on buzzwords instead of articulating a clear product hypothesis"
- "Vague or overly technical descriptions"
- "Omitting team background information"
- "Failing to clearly explain the core idea and its impact."
- "Submitting incomplete or unpolished pitch videos"
- "Failing to explain Solana integration clearly." For us, read this as Tempo integration.
- "Ignoring optional fields that could provide important context"
- "Treating the hackathon as a finished endpoint rather than a starting point"
- "Forgetting to grant judges access to google docs, pitch videos, github repos, etc."
- Turning the technical demo "into another pitch" (A2)

**From the FAQ / rules (A3): VERIFIED**
- Misrepresenting development history, or failing to disclose pre-existing code. Consequences: disqualification, a ban, or revoked prizes.
- Private repo without access granted to hackathon@colosseum.com.
- Submitting more than one product.

**Observed in winner data. These are INFERENCE: they are risks, not proven penalties.**
- **Length overruns are common among winners, but don't copy them.** Zoneless ran 3:45, Tokamai 3:34, Samui 4:34 and Txtx 6:27. For World's Fair the portal states "two-to-three-minute," and overrun is the first item on the official mistakes list. Aim for 2:45 or less. Zoneless spoke at about 167 wpm (628 words in 225 s), so a 3:00 budget is roughly **450-480 spoken words**.
- **Vision-first, abstract problem statements are weaker.** Corbits ("paradigm shift", "information asymmetry", "customers are being receptive") still placed 2nd, but it had real merchant dashboards in the repo. The pitch carried less proof than the product did.
- **Unsupported top-down TAM** (Latinum's "multi-trillion dollar market") invites the Potential Market Size question in the 15-minute interview. Prefer bottom-up.
- **"First / only" claims can be falsified in two minutes.** Phase 3 already caught a false competitor claim in the HL Clear-Sign deck. Every "no competitor" line must be sourced (for us: the list of Tempo's 69-repo org).

---

## E. Recommended slide order for tempo-watchtower (12 slides; 3:00 pitch video + separate ≤3:00 tech demo)

The time budget assumes about 165 wpm, which gives a ceiling of roughly 480 words. The deck is the visual track for a Loom voiceover, which is Colosseum's stated preference (A1, A2).

| # | Slide | Time | Content | Justified by |
|---|---|---|---|---|
| 1 | **Title + one-liner** | 0:00-0:12 | "tempo-watchtower: the Lightning-style watchtower for Tempo MPP payment channels, so payees never lose money they already earned." Name and one-line credential spoken. | C1, C2, C14; Zoneless and Tokamai name the founder in the first 10 s |
| 2 | **The loss, as a timeline** | 0:12-0:40 | Status-quo walkthrough: payer calls `requestClose` → 15-min grace (§6.4.5, §13.3) → payee server down, redeploying or reorged → `withdraw` → payee's unsettled vouchers are gone. One number on screen. Quote the spec's own "SHOULD… Monitor for… CloseRequested events" (§14.12). | C3, C5, C6; Tokamai and Latinum friction walkthroughs; "who the product is for" (A2) |
| 3 | **Why me** | 0:40-0:55 | Dense, specific proof of fit (async/state-machine/Rust payments work, prior shipped systems). Why solo is enough for this scope. | A2 solo rule verbatim; B3 solo winners; Cerebro "evidence of grit" |
| 4 | **Why now (insight)** | 0:55-1:08 | Spec `draft-tempo-session-00` dated 2026-09-23; MPP live; Lightning's watchtower precedent; zero watchtower in Tempo's org. Frame it as a Copilot-style "full gap." | "Is there a new technology, or a new trend…" (A3); C5 |
| 5 | **How it works** | 1:08-1:25 | One diagram in three steps: mirror §12.1 accounting (`acceptedCumulative`, `settledOnChain`, `closeRequestedAt`) → watch events → `settle()` inside the window. Independent operator by design. | IDL Space and OTAC "three simple steps"; C8 |
| 6 | **Demo: the aha moment** | 1:25-1:55 | Real Tempo testnet: force-close a channel with the payee offline, then show the watchtower detect it and land `settle()` with the tx hash on screen. Say "nothing on screen is faked." | C8; Sudont, Tokamai; A1 "aha moment" |
| 7 | **Validation / traction** | 1:55-2:10 | Named MPP server operators spoken to, quotes, testnet channels watched, repo activity, any design partner or LOI. Informal counts, but say it out loud. | C7; A2 validation; Cohort V "traction number" |
| 8 | **Competition + structural moat** | 2:10-2:20 | Nothing in the Tempo org; how generic monitors (e.g. Microscope-type) differ; the independence argument; Lightning precedent. | C9; Tokamai and Autonom named competitors; B4 risk |
| 9 | **Business model** | 2:20-2:30 | Open-source core daemon plus a hosted, independent watchtower service with a per-channel or per-month fee. The pitch is "pay a small fee, or risk 100% of an unsettled channel." | C10; Zoneless, Samui, OTAC; A1 viability |
| 10 | **Market: honest and growing** | 2:30-2:40 | Bottom-up count of MPP servers (flag it as stale/estimated), what grows it (MPP adoption, agent payments), and expansion to other channel protocols. | C11; A3 "small but growing rapidly" |
| 11 | **GTM: first 10 users** | 2:40-2:50 | Named segments and channels: MPP server operators (LLM/API sellers), Tempo Discord, mpp-rs users, Tempo partners; the path from first 10 to the next 100. | C12; portal GTM field; Zircon's GTM slide |
| 12 | **Roadmap + commitment + closer** | 2:50-3:00 | Next 90 days, a full-time commitment, then restate the one-liner. | C13, C14; Samui ask; A1 "build full-time" |

**Companion assets (all VERIFIED as asked for or rewarded):**
- **Tech demo video (≤3:00, the "how").**
  - Architecture and Tempo integration points: the event subscription, the settle path, reorg handling (§14.12).
  - Why Rust, framed as properties.
  - What was deprioritized and why. Judges want "the reasoning behind these decisions."
- **Weekly one-minute update videos** for the remaining ~2 weeks. They count as "evidence of… who iterates" (A3, A4).
- **Fill every optional portal field**, including GTM and demand validation, and disclose any pre-existing code (A3, D).
- **Build in public on X** between now and 2026-10-12. Every conversation with an MPP operator becomes slide-7 material (A1).

---

### Source index (all fetched 2026-09-27)
- **Guidance:** blog.colosseum.com/how-to-win-a-colosseum-hackathon/ · /perfecting-your-hackathon-submission/ · colosseum.com/hackathon (FAQ) · colosseum.com/worldsfair
- **Winners:** /announcing-the-winners-of-the-solana-{renaissance,radar,breakout,cypherpunk,frontier}-hackathon/
- **Cohorts:** /introducing-colosseum-accelerator-cohort-{1,2,3}/ · /announcing-colosseums-accelerator-cohort-{4,5}/
- **Framing:** /colosseums-2024-investments-and-themes-for-2025/ · /the-volta-prize-colosseum-and-hackathons-in-the-age-of-ai/ · /cohort-v-demo-day-solana-microscope-alpenglow-migration/
- **Arena:** arena.colosseum.org/projects/explore/{zoneless, tokamai, txtx, idl-space, attest-protocol, zircon, tapedrive, flux-rpc-1, mcpay, seer, corbits.dev, high-tps-solana-client, latinum-agentic-commerce, samui-wallet, one-time-action-codes-1, ionic, sudont, 855, autonom-unleashing-rwas-in-solana, credible-finance-1, unruggable-3, flovia, …}
- **Pitch transcripts:** Loom public transcript JSON for the share links listed on those arena pages.
- **Zircon deck:** docs.google.com/presentation/d/1SjV8B1LzSOK8TMl8R4_XTROZPABTAy1QwWCHZkAI_xc (exported as text)
- **READMEs:** github.com/{zonelessdev/zoneless, txtx/txtx, microchipgnu/MCPay, williamwa/idlman, VasilyGerrans/seer, tapedrive-io/tape, daccred/attest.so, flovia/poc, faremeter/402-dashboard}

**Gaps (UNVERIFIED / not obtained):**
- No transcripts for the YouTube-hosted pitches (TAPEDRIVE, MCPay, FluxRPC, Seer, Txtx, Unruggable); captions were blocked.
- Cloak's transcript was empty and Flovia's Loom had none.
- Arena pages for Zircon and Rakurai load only by slug; the numeric IDs in the old blog links return no data.
