# Building in Public on X: tempo-watchtower (personal account = build log + resume)

**Goal:** turn the next 15 days of building into two things at once:
1. proof judges can see ("who iterates, who listens to users");
2. a public portfolio that outlives the hackathon.

**Time budget:** 30–45 minutes a day, total. Coding comes first; posting is the by-product of the
devlog you already keep (`docs/devlog.md`).

**Source note.** Handles below were checked on the projects' own websites (2026-09-29). The
advice on what the X algorithm favours is general best practice, not an official X statement;
treat it as a starting point and adjust to what your own analytics show.

---

## 1. Why this matters for the hackathon (not just vanity)
- Colosseum's own guide says to "create a project Twitter X account and begin sharing your
  product vision… it enables you to find beta testers who will provide feedback during the
  engineering sprint."
- Judges also say they act on "weeks of evidence about who iterates, who listens to users", and
  they strongly recommend weekly one-minute update videos.
- Sources: `research/phase4-pitch/winner-patterns.md` §A1, §A3, §A4.

**Your X feed becomes evidence for three judging criteria:**
- Traction: replies from real MPP builders.
- Product + execution: daily visible progress.
- Founder communication: clear, honest updates.

**Personal account vs. a project account.** Colosseum suggests a project account. For you, use
your **personal account** as the main voice, because it doubles as your resume. Optionally reserve
a project handle (e.g. `@tempowatchtower`) now, so nobody squats it, and keep it light: releases
and a quote-post of your main threads.

---

## 2. Profile setup (do this on day 1, 15 minutes)

### Name field
`Wilson Teo · building tempo-watchtower`
Keep your real name first; the project fades from the name field after the hackathon.

### Bio: three versions, changing over time

**Now → Oct 12 (building):**
> Building tempo-watchtower: an open-source Rust watchtower for @tempo payment channels.
> Learning Rust in public for @colosseum World's Fair. Daily devlog ↓

**Oct 13 → Dec 5 (submitted, waiting for results):**
> Built tempo-watchtower (Rust, open source): keeps @tempo payment channels from losing earned
> revenue on forced close. @colosseum World's Fair entrant. Fintech backend dev.

**After results (resume mode), with the outcome added only if true:**
> Rust + payments infra. Built tempo-watchtower [→ World's Fair <result>]. Backend engineer
> (fintech). Open to <roles you want>.

### Bio rules
- **One concrete thing you built beats five adjectives.** "Rust watchtower for Tempo" says more
  than "passionate web3 builder".
- **No "CEO/founder of" unless it's true.** Judges and recruiters click through.
- **Only mention your employer if their policy allows it.** The hackathon rules (§3c) also make
  you responsible for having your employer's permission to participate. Check that quietly.

### Other profile fields
- **Link:** your GitHub repo (not your profile) during the hackathon. Afterwards, a simple
  portfolio page or your GitHub profile.
- **Pinned post:** the day-1 kickoff thread. Replace it with the demo-video post on about day 12.
- **Header image:** a clean screenshot of the architecture diagram, or the terminal showing
  "CloseRequested → close() landed in 1.9 s". Make it text-light and readable on mobile.
- **Handle consistency:** use the same username on GitHub and X if possible. It helps
  recruiters and judges connect the two.

---

## 3. Hashtags: use almost none
- On X today, hashtags are a weak discovery signal and several look spammy. Mentions, replies
  and communities do far more.
- **Rule: 0–1 hashtag per post, and only when it's a real, active conversation.**
  - Reasonable: `#rustlang` on Rust-learning posts (a long-standing, active Rust tag), and
    `#buildinpublic` on your kickoff thread and weekly recaps.
  - **Don't:** stack 5 hashtags, use invented tags, or tag every post with #web3 #crypto #AI.
  - I found **no official World's Fair hashtag** on colosseum.com/worldsfair. If Colosseum
    announces one in Discord or on @colosseum, use that on weekly recaps only.

### Mentions do the real work, sparingly
| Account | When to mention |
|---|---|
| @colosseum | Kickoff thread, weekly recaps, submission post |
| @tempo | Posts about Tempo findings or the demo (the payments chain itself) |
| @mpp | Posts specifically about MPP sessions / payment channels |
| @stripe, @paradigm | Rarely. Only if a post is genuinely about MPP/Tempo's design, never for reach |
| Judges (listed on colosseum.com/worldsfair) | **Don't tag them in your posts.** Reply thoughtfully to *their* posts when you have something useful to add. Unsolicited tagging of judges reads as lobbying |

**Rule of thumb:** at most **one or two mentions per post**, and only where the account would
actually care.

---

## 4. How much to post
| Type | Frequency | Time |
|---|---|---|
| Build log post (text + screenshot/clip/tx link) | **1 per day** | 10 min |
| Thoughtful replies to others (MPP/Tempo builders, Rust learners, Colosseum posts) | **5–10 per day** | 15–20 min |
| Weekly recap thread + the 1-minute update video | **2 total** (≈10/4 and 10/11) | 30 min each |
| Big moments (first save, demo video, submission) | When they happen | – |

- **Consistency beats volume.** One honest post every day for 15 days outperforms ten posts on
  day 1 and silence after.
- **Replies are where a new account grows.** Your posts start with little reach. Genuine replies
  in the right conversations (MPP builders, Tempo devs, Rust beginners) are how people find you,
  and replies to MPP builders are also your **customer outreach**.
- **Best posting time:** when your audience is awake. Much of the crypto-dev audience is US-based;
  from Malaysia (UTC+8) that's roughly **21:00–01:00 MYT**. Check your own analytics after a week
  and adjust.

---

## 5. Content pillars (rotate these)
1. **Build log:** what you shipped today, with proof.
   - A tx link on the Tempo explorer, a terminal clip, a green CI badge.
   - Example: "Day 4: my watchtower closed its first channel automatically. 1.9 s from request
     to close. [clip] [tx]"
2. **Learning Rust in public:** relatable and highly shareable.
   - "Today the borrow checker taught me…", "Why I used an enum for the channel state machine."
   - Beginners and experienced Rust devs both engage with honest learning posts.
3. **Insight/data:** the on-chain findings, stated carefully (see §7).
   - "Of 344 resolved close requests on Tempo mainnet, 304 got no payee response. Here's how I
     measured it (open-source script)."
4. **Demo moments:** 10–30 s screen clips; videos get more attention than text.
5. **Asking for feedback:** your outreach channel.
   - "Running an MPP session server? What happens if a client force-closes while you're
     deploying? I'm building a fix; want to try it on testnet?"

A good mix per week: 3 build log, 2 learning, 1 insight, 1 ask. Demo clips go wherever there's
something visual.

---

## 6. Ready-to-post drafts (edit into your own voice)

**Day 1: kickoff thread (pin it)**
> 1/ For the next 2 weeks I'm building tempo-watchtower in public: an open-source Rust service
> that stops @tempo payment-channel servers from losing revenue they already earned. First Rust
> project. Here's the problem 🧵 #buildinpublic
>
> 2/ On Tempo, AI agents pay APIs through payment channels (@mpp sessions). The agent can call
> requestClose anytime. If the server doesn't claim what it's owed within 15 minutes, the agent
> withdraws everything unclaimed.
>
> 3/ I scanned every v2 channel event on mainnet: of 344 resolved close requests, 304 got no
> response from the payee. The payees who do respond take ~83 s, and each built that themselves.
>
> 4/ tempo-watchtower acts as the channel's "operator" (a role Tempo already supports), watches
> for close requests, and closes the channel in seconds, claiming only what was consumed.
>
> 5/ Building it for @colosseum World's Fair. Repo: <link>. I'll post progress daily. Rust folks:
> advice welcome, I'm new here.

**Day 2:**
> Day 2: from Rust I opened a payment channel on Tempo testnet, force-closed it as the payer, and
> closed it as the operator. It's all real transactions. <tx link>. Tomorrow: the state machine
> (and fighting the borrow checker).

**Day 3 (learning):**
> Rust TIL: modelling the channel lifecycle as an enum (Watching → Closing → Done / Cancelled)
> with `match` means the compiler refuses to let me forget a state. 22 tests, all green. #rustlang

**Day 4: first automatic save**
> 🎉 Day 4: my watchtower caught a close request on Tempo testnet and closed the channel by itself
> in <N> s. The payee kept what it earned. <10 s clip> <tx>

**Day 7: weekly recap + 1-minute video**
> Week 1 of building tempo-watchtower for @colosseum World's Fair:
> ✅ first automatic save (<N> s)
> ✅ survives a kill -9 restart
> ✅ WebSocket + polling fallback
> Next: split-screen demo. 60-s update ↓ <video>

**Day 12: demo video (new pinned post)**
> Here's tempo-watchtower in 3 minutes: the same crash, two outcomes. Without it, the payee's
> earnings go back to the payer. With it, the channel closes in <N> s. Open source, Rust, built in
> 2 weeks as my first Rust project. <video> <repo>

**Day 15: submission**
> Submitted tempo-watchtower to @colosseum World's Fair (Tempo track). 15 days, first Rust project,
> <N> commits, <N> testnet saves. Thank you to everyone who replied with feedback. <repo>

### Outreach message (DM or reply) to MPP session builders
> Hi, I saw you're running an MPP session server. I'm building an open-source watchtower that
> auto-closes channels when a client force-closes, so you don't lose unsettled revenue during a
> deploy or crash. Would you try it on testnet? Free. Any feedback helps.

Keep it short, one message, no follow-up spam. Log every reply in your devlog; it feeds slide 7
of the pitch.

---

## 7. Hard rules (protect yourself and your submission)
1. **Never show secrets.**
   - No private keys, no `.env` contents, and no terminal scrollback that ever printed a key.
   - Crop screenshots and use testnet-only keys.
   - Follow the build guide's no-secrets-on-screen checklist (§8) before every clip.
2. **Stay honest with the numbers, exactly as in the deck.**
   - Say "304 of 344 unanswered". Never say "payees lost $X". Provable losses are tiny, and much
     of the activity is builders testing.
   - Say "first Rust project" and "AI-assisted plumbing, disclosed" if asked. Never claim you
     wrote something you didn't.
3. **Don't name and shame payee addresses.**
   - Never post "0xabc… lost money". Those are real builders and possible customers.
   - Share aggregate data and the open-source script only.
4. **Don't disparage.** Official rules §12(b)(iii) prohibit content that disparages Colosseum or
   anyone affiliated with the contest. Frame SDK gaps as "not built yet", never "Tempo's SDK is
   broken". The Tempo team is also your best possible distribution partner.
5. **Don't spam judges or sponsors.** No tagging judges, and no mass DMs. Reply when you have
   something useful.
6. **Keep your employer happy.** Check their side-project and social-media policy before you post
   about work experience.
7. **Mute, don't argue.** Ignore trolls and "wen token" replies. You have no token; say so once if
   asked.

---

## 8. Turning it into a resume after Oct 12
- **Write a case-study thread** (and a longer blog/README version). Cover:
  - the problem;
  - the data (with the honest caveats);
  - the design decisions (operator role, capture rule, restart safety);
  - what you learned in Rust;
  - results and measured latency.
  Recruiters read these.
- **Pin the demo video.** Put the repo, the case study and the result in your bio (§2).
- **LinkedIn (optional):** post the same story in plain language, aimed at hiring managers.
  Link the repo and the video.
- **Keep the repo alive.** Tag releases, fix issues, and label "good first issue" (build guide §6).
  A maintained repo is worth more than a finished hackathon entry.
- **What recruiters will see in your timeline:**
  - a first Rust project shipped in 15 days;
  - real testnet transactions;
  - CI/CD;
  - honest writing about a real protocol gap.
  That's a strong signal for Rust, backend and payments-infra roles.

---

## 9. Daily 30-minute routine
1. **After coding (10 min):** turn today's devlog lines into one post with a screenshot, clip or tx
   link.
2. **Evening, 21:00–01:00 MYT (15–20 min):** reply to 5–10 posts from MPP/Tempo builders, Rust
   learners and @colosseum.
3. **Once a week (30 min):** a recap thread plus the 1-minute update video. Upload the same video
   to the Colosseum portal's weekly updates.

**Track only what matters:**
- replies or DMs from real MPP/Tempo builders (your outreach pipeline);
- testers who try the watchtower;
- GitHub stars and forks.

Follower count is the least important number here.
