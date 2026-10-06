//! GIVEN (M0): end-to-end tracer bullet on Tempo testnet with throwaway keys.
//! fund -> open(operator = tower) -> voucher -> requestClose -> topUp (cancels) -> pre-check sees
//! the cancel -> requestClose again -> operator close(capture = spent) -> ChannelClosed.
//! Run: cargo run -p tw-chain --example probe
use std::time::{Duration, Instant};

use alloy::signers::local::PrivateKeySigner;
use tw_chain::*;

const RPC: &str = "https://rpc.moderato.tempo.xyz";
const CHAIN_ID: u64 = 42431;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let payer = PrivateKeySigner::random();
    let operator = PrivateKeySigner::random();
    let payee = PrivateKeySigner::random().address(); // payee never signs in this probe
    println!(
        "payer {} operator {} payee {payee}",
        payer.address(),
        operator.address()
    );

    let p = http_provider(RPC)?;
    fund(&p, payer.address()).await?;
    fund(&p, operator.address()).await?;
    println!("CLOSE_GRACE_PERIOD = {}", grace_period(&p).await?);

    let payer_p = signing_provider(RPC, payer.clone())?;
    let (id, d, open_tx) = open_channel(&payer_p, payee, operator.address(), 1_000_000).await?;
    println!("open {open_tx}\nchannel {id}\ndescriptor {d:?}");
    println!(
        "channel_id recomputed locally matches: {}",
        tw_core::channel_id(&d, CHAIN_ID) == id
    );

    // payer authorizes 0.30; the payee's `spent` is 0.25
    let sig = sign_voucher(&payer, CHAIN_ID, id, 300_000).await?;
    // Day 1 uses GIVEN code only: the local EIP-712 digest must equal the precompile's.
    let reserve = Reserve::new(RESERVE, &p);
    let onchain = reserve
        .getVoucherDigest(id, alloy::primitives::aliases::U96::from(300_000u64))
        .call()
        .await?;
    let local = tw_core::voucher_digest(CHAIN_ID, id, 300_000);
    println!(
        "voucher sig {sig}\nvoucher digest local == on-chain: {}",
        local == onchain
    );

    println!(
        "requestClose {}",
        payer_call(&payer_p, &d, PayerCall::RequestClose).await?
    );
    println!(
        "topUp(1) {}",
        payer_call(&payer_p, &d, PayerCall::TopUp(1)).await?
    );
    let s = states(&p, &[id]).await?;
    println!(
        "pre-check after topUp: {:?} -> {:?} (closer must NOT send)",
        s.first(),
        s.first().and_then(precheck)
    );

    println!(
        "requestClose again {}",
        payer_call(&payer_p, &d, PayerCall::RequestClose).await?
    );
    let t_req = Instant::now();
    let s = states(&p, &[id]).await?;
    println!(
        "pre-check: {:?} -> {:?} (None = send)",
        s.first(),
        s.first().and_then(precheck)
    );

    // Day 1: a hand-written plan (capture = spent). From M2 on, `tw_core::plan_close` makes it.
    let plan = tw_core::ClosePlan {
        cumulative: 300_000,
        capture: 250_000,
        signature: sig.to_vec(),
        claimable: 250_000,
        clamped: false,
    };
    let op_p = signing_provider(RPC, operator.clone())?;
    let tx = broadcast_close(&op_p, operator.address(), &d, &plan)
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;
    println!(
        "close sent {tx} (plan capture {} cumulative {})",
        plan.capture, plan.cumulative
    );
    let st = wait_receipt(&p, id, tx, Duration::from_secs(30)).await?;
    println!(
        "receipt: {st:?}  ({:?} after the second request)",
        t_req.elapsed()
    );
    println!("after: {:?}", states(&p, &[id]).await?.first());

    // Error path (N1): sending again must return an error, never panic. On day 1 the
    // `classify_revert` stub maps it to Transient; after M7 it becomes AlreadyClosed.
    let again = broadcast_close(&op_p, operator.address(), &d, &plan).await;
    println!("second close on a closed channel -> {again:?}");
    Ok(())
}
