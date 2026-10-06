use std::time::Duration;
use alloy::{network::ReceiptResponse, primitives::{aliases::U96, Address, B256, U256, Bytes}, providers::{Provider, ProviderBuilder}, signers::local::PrivateKeySigner};
use tempo_alloy::{contracts::precompiles::{ITIP20ChannelReserve as R, ITIP20, TIP20_CHANNEL_RESERVE_ADDRESS as A}, TempoNetwork};
const RPC: &str = "https://rpc.moderato.tempo.xyz";
const PATH_USD: Address = alloy::primitives::address!("0x20c0000000000000000000000000000000000000");
async fn fund(a: Address) -> anyhow::Result<()> {
    let p = ProviderBuilder::new_with_network::<TempoNetwork>().connect_http(RPC.parse()?);
    let _: Vec<B256> = p.raw_request("tempo_fundAddress".into(), (a,)).await?;
    for _ in 0..30 { if ITIP20::new(PATH_USD, &p).balanceOf(a).call().await? > U256::ZERO { return Ok(()) } tokio::time::sleep(Duration::from_secs(1)).await; }
    anyhow::bail!("fund")
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let payer = PrivateKeySigner::random(); let op = PrivateKeySigner::random(); let payee = Address::random();
    fund(payer.address()).await?; fund(op.address()).await?;
    let pp = ProviderBuilder::new_with_network::<TempoNetwork>().wallet(payer.clone()).connect_http(RPC.parse()?);
    let r = R::new(A, &pp);
    let rc = r.open(payee, op.address(), PATH_USD, U96::from(500_000u64), B256::random(), Address::ZERO).send().await?.get_receipt().await?;
    let e = rc.decoded_log::<R::ChannelOpened>().unwrap().data;
    let d = R::ChannelDescriptor { payer: e.payer, payee: e.payee, operator: e.operator, token: e.token, salt: e.salt, authorizedSigner: e.authorizedSigner, expiringNonceHash: e.expiringNonceHash };
    let a = r.requestClose(d.clone()).send().await?.get_receipt().await?;
    println!("req1 grace_end {}", a.decoded_log::<R::CloseRequested>().unwrap().data.closeGraceEnd);
    let t = r.topUp(d.clone(), U96::from(1u64)).send().await?.get_receipt().await?;
    println!("topUp status {} cancelled event: {}", t.status(), t.decoded_log::<R::CloseRequestCancelled>().is_some());
    tokio::time::sleep(Duration::from_secs(3)).await;
    let b = r.requestClose(d.clone()).send().await?.get_receipt().await?;
    println!("req2 grace_end {}", b.decoded_log::<R::CloseRequested>().unwrap().data.closeGraceEnd);
    let again = r.requestClose(d.clone()).send().await?.get_receipt().await?;
    println!("req3 (no cancel) emits event: {}", again.decoded_log::<R::CloseRequested>().is_some());
    let opp = ProviderBuilder::new_with_network::<TempoNetwork>().wallet(op.clone()).connect_http(RPC.parse()?);
    let c = R::new(A, &opp).close(d.clone(), U96::ZERO, U96::ZERO, Bytes::new()).send().await?.get_receipt().await?;
    let cl = c.decoded_log::<R::ChannelClosed>().unwrap().data;
    println!("empty-sig close status {} settledToPayee {} refunded {}", c.status(), cl.settledToPayee, cl.refundedToPayer);
    let st = R::new(A, &opp).getChannelStatesBatch(vec![e.channelId]).call().await?;
    println!("post-close state {:?}", (st[0].settled, st[0].deposit, st[0].closeRequestedAt));
    Ok(())
}
