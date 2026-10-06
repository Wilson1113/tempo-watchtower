# Evidence, round 2 (2026-09-27): spec v2, contract code, SDK code, fees, on-chain data

> **CORRECTIONS (round 3, after judge round 2). These supersede §2, §5 and §6 below where they
> conflict.** The originals are kept for the audit trail.
>
> 1. **Response counting bug (§6).** `forced.py` counted only a `close()` as a payee response.
>    The corrected script is `onchain/agg2.py`.
>    - A payee-side `settle()` after the request and before the payer's `withdraw` is also a
>      response. The contract only lets the payee or operator call `settle`, so any `Settled`
>      event proves the payee side acted.
>    - **Corrected counts:**
>      - 408 close requests. 344 are resolved; 64 are unresolved (no close yet).
>      - Of the 344 resolved: **304 forced withdraws had NO payee-side response** (14 payees).
>        **39 had a payee-side settle** before the withdraw, and 1 was closed by the payee.
>      - Response delay: median **83 s**, minimum 1 s (`onchain/delay_output.txt`).
>    - **Provably used and unanswered:** 29 withdraws, 5 payees, **≤ $36.00** refunded.
>      "Provably used" means at least one settle before the request. This is an upper bound;
>      refunds include unconsumed deposit.
>    - The median refund per unanswered withdraw is **$0.50** (max $25, total $1,966).
>    - The dominant payee accounts for 202 of the 304.
>    - **Operators:** of 676 operator channels, 302 are self-assigned (operator == payee) and
>      374 are delegated, across 37 distinct delegated operators. The old "73 operator channels
>      never acted" claim was wrong and is withdrawn.
> 2. **Gas price (§5).** 5.9125e9 was read from the **testnet** RPC by mistake. Live **mainnet**
>    `eth_gasPrice` and `baseFeePerGas` are **6.0e8**, the T7 floor.
>    - Settle ≈ **$0.00018** (floor) to $0.0036 (cap). Close ≈ $0.00005 (floor) to $0.00097
>      (cap).
>    - Settling one channel every minute ≈ **$0.26/day** at the floor ($5.22 at the cap).
>    - The dominant payee's ~218k lifetime settles cost ≈ $40 in total at the floor.
>    - **Implication:** fees alone are not a strong argument against frequent settling.
> 3. **Operator trust (§2).** The operator can only pay the payee or refund the payer, but it
>    **chooses `captureAmount` within [settled, voucher]** and can close at any time.
>    - It is therefore trusted by *both* sides on the amount. A compromised operator could
>      under-capture (hurting the payee) or over-capture above `spent` (hurting the payer).
>    - `close` has no `closeRequestedAt` check, so the true deadline is the payer's `withdraw`
>      being mined, not `closeGraceEnd`. One mainnet channel was settled ~8 days after its
>      request because the payer hadn't withdrawn yet.
>    - The operator is bound into the channelId at open. So it protects **new channels only**,
>      switching provider changes channelIds, and a channel has one operator slot.

This round answers the round-1 judge's technical flags. Every item was fetched or computed
directly. The scripts and outputs are in `onchain/`; to reproduce, run
`python3 onchain/scan.py`, then `agg.py`, `forced.py` and `detail.py`. They need ~465 MB of logs
and about 2 minutes against `https://rpc.tempo.xyz`.

## 1. The spec was revised in place; v2 (TIP-20 channel precompile) is the recommended path
- `paymentauth.org/draft-tempo-session-00.txt` now carries a **26 Sep 2026** header (expires
  30 Mar 2027). It still says "draft-00", so earlier research fetched a 23 Sep version of the
  same file. VERIFIED (curl).
- §4 defines v1 as the "legacy TempoStreamChannel contract" and v2 as the "TIP-20 channel
  escrow precompile". It says: "New servers SHOULD emit `sessionProtocol: v2`". VERIFIED.
- §6.2.1 v2 descriptor: `operator` is an "Optional payee-side operator authorized for channel
  operations; zero address if unset". It is hashed into channelId, and servers advertise it via
  `methodDetails.operator`. VERIFIED.
- §13.2: servers "MUST NOT capture more than the amount actually consumed". For v2,
  `captureAmount = max(spent, settledOnChain)`. VERIFIED.
- §11.3 step 5: servers MUST reject vouchers while a close request is pending. VERIFIED.

## 2. Contract code: the operator can settle and close; only the payer can requestClose/withdraw
Source: `tempoxyz/tempo`, `tips/verify/src/TIP20ChannelReserve.sol`, the reference
implementation of the precompile. VERIFIED (curl).
- `settle` and `close` revert with `NotPayeeOrOperator` unless `msg.sender` is the payee or a
  non-zero `descriptor.operator`.
- `requestClose` and `withdraw` revert with `NotPayer` unless the caller is the payer.
- `close(descriptor, cumulativeAmount, captureAmount, signature)` pays the payee
  `captureAmount - settled` and refunds the payer `deposit - captureAmount`. The operator can
  **never redirect funds**: payouts go only to `descriptor.payee` and `descriptor.payer`.
- `withdraw` refunds `deposit - settled` to the payer. Both paths emit `ChannelClosed`.
- `CLOSE_GRACE_PERIOD = 15 minutes` (constant).
- **Consequence:** a watchtower should be the channel's `operator`. That is protocol-native
  delegation: the tower never holds the payee's main key and cannot steal. The worst a rogue
  operator can do is close a channel early.

## 3. Live chain checks (JSON-RPC)
- Testnet `rpc.moderato.tempo.xyz`: chainId 0xa5bf (42431). The precompile at
  `0x4d50500000000000000000000000000000000000` has code `0xef`, and `CLOSE_GRACE_PERIOD()`
  returns `0x384` (**900 s**). VERIFIED.
- Mainnet `rpc.tempo.xyz`: chainId 0x1079 (4217). The same precompile address returns
  `CLOSE_GRACE_PERIOD()` = **900 s**. VERIFIED. The demo can run on testnet, and the product
  works on mainnet today.
- The precompile address comes from `wevm/mppx` `src/tempo/session/precompile/Protocol.ts:527`
  (`tip20ChannelEscrow`). VERIFIED.

## 4. SDK code: nothing watches for close requests; scheduled settlement is opt-in and request-driven
Both SDKs were shallow-cloned and grepped (2026-09-27). VERIFIED.
- **`wevm/mppx`** (TypeScript server SDK):
  - Server code checks `closeRequestedAt !== 0` only to **reject** vouchers and charges
    (`ChannelStore.ts`, `CredentialVerification.ts`, `Settlement.ts:320` throws
    `ChannelClosedError('pending close request')`).
  - There is no `CloseRequested` event subscription and no `watchContractEvent` in server code.
  - `SettlementSchedule` (units / amount / intervalMs) is **optional**. With no schedule,
    `isSettlementDue` returns `false`, so nothing settles automatically by default.
  - When a schedule is set, `maybeSettleScheduled` runs from the request path
    (`Session.ts:342`), not as a background job, so it stops when requests stop.
  - The SDK supports an operator or Tempo access-key account as the settlement sender
    (`assertSettlementSender`).
- **`tempoxyz/mpp-rs`** (Rust SDK):
  - Server code checks `close_requested_at != 0` only to reject vouchers.
  - It has no settlement schedule, no background settle task and no close-request listener.
  - It exposes `settle_session_call` as an encoding helper.

## 5. Settlement cost (from Tempo source + live gas price)
- Gas, from Tempo's own snapshot
  (`crates/node/tests/it/gas/snapshots/...tip20_channel_reserve_gas_snapshot_t7.snap`):
  - `settle` (existing payee balance): **301,831**
  - `close`: **80,913**
  - `request_close`: 30,323
- Units (`crates/primitives/src/transaction/mod.rs`): gas price is in **attodollars**
  (10⁻¹⁸ USD).
- Base-fee range (`crates/hardfork/src/constants.rs`): cap 12e9, floor 6e8 (T7).
- Live mainnet `eth_gasPrice`, 2026-09-27: 5,912,500,000 attodollars/gas. That puts a
  **`settle` at about $0.0018** and a **`close` at about $0.0005**. The range across floor and
  cap is roughly $0.0002-$0.0036 per settle. These are COMPUTED figures and exclude priority
  fees.
- **Implication:** settling one channel every minute costs about $2.6/day at today's gas price.
  Cost scales as channels × frequency, and it still needs an always-on process. A watchtower
  pays only when a close request arrives.

## 6. On-chain reality: Tempo mainnet, v2 channel precompile, 2026-06-10 → 2026-09-26
Full log scan: 493,160 events. VERIFIED/COMPUTED; see `onchain/*_output.txt`.
- **Channels:** 268,596 v2 channels opened.
  - 70 unique payees and 1,718 unique payers.
  - 99.3% of channels (266,750) went to one payee, `0xca4e…79fe`.
- **Operators:** 676 channels set a non-zero operator, across 39 unique operators. Payees
  already use the delegation slot.
- **Value:** lifetime deposits were ~$4,234 in the main USD token. Lifetime settled was ~$352.
  **Absolute value at stake today is tiny.**
- **Close requests:** 408. Of the 344 channels later closed, **343 ended in a payer `withdraw`**
  (forced close), and **only 1 was closed by the payee**, 24 s after the request.
  - 275 of the forced withdraws had $0 ever settled to the payee.
  - $2,059.71 was refunded to payers through forced withdraws. That is an upper bound on value
    at risk: refunds include never-consumed deposit, and on-chain data cannot show `spent`.
  - 68 forced withdraws were on channels with at least one prior `settle` (service provably
    used), with **$129.59** refunded. That is an upper bound on provable lost revenue.
  - 15 distinct payees were hit, including the dominant payee (213 forced withdraws).
  - 73 forced withdraws were on channels **with an operator set**, so delegation alone didn't
    protect them. Someone still has to watch and act.
- **Trend (honest):** excluding the dominant payee, channels opened per ~1.2M-block bucket ran
  12, 19, 384, 401, 162, 221, 262, 68, 112, 71, 28, 28, 16, 62. Activity peaked early and is
  **lower recently. It is not a clean growth curve.**
- **Interpretation (INFERENCE):** payees on Tempo today are not watching for close requests.
  Forced withdraw is the normal ending, not an edge case. The dollar amounts are small because
  the market is early. The behavior is what the product fixes.

## 7. What round 1 got wrong that this fixes
- "Only the payee can settle, so a Lightning tower can't be reused" is **v1-only**. v2 has the
  operator slot, which *enables* a third-party tower. Deck v2 pivots to "the tower is your
  channel's operator".
- "Settle the highest voucher" was wrong. The tower must close with
  `captureAmount = max(spent, settledOnChain)`, so the payee must replicate `spent`, not just
  vouchers.
- The "Sep 23" date is stale. Use "revised 26 Sep; v2 precompile live on mainnet since June".
- The "~300 MPP servers" figure is superseded by real on-chain v2 numbers: 70 payees, 1,718
  payers, 268k channels.
- The multichain session claim was cut from the deck because it isn't in the evidence files.
