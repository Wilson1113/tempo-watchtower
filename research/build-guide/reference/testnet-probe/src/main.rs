use std::time::{Duration, Instant};

use alloy::{
    primitives::{aliases::U96, Address, B256, U256},
    network::ReceiptResponse,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    signers::{local::PrivateKeySigner, Signer},
    sol,
    sol_types::{eip712_domain, SolEvent, SolStruct},
};
use futures::StreamExt;
use tempo_alloy::{
    contracts::precompiles::{ITIP20ChannelReserve, ITIP20, TIP20_CHANNEL_RESERVE_ADDRESS},
    TempoNetwork,
};

const RPC: &str = "https://rpc.moderato.tempo.xyz";
const WS: &str = "wss://rpc.moderato.tempo.xyz";
const PATH_USD: Address = alloy::primitives::address!("0x20c0000000000000000000000000000000000000");
const CHAIN_ID: u64 = 42431;

sol! {
    struct Voucher { bytes32 channelId; uint96 cumulativeAmount; }
}

async fn fund(addr: Address) -> anyhow::Result<()> {
    let p = ProviderBuilder::new_with_network::<TempoNetwork>().connect_http(RPC.parse()?);
    let hashes: Vec<B256> = p.raw_request("tempo_fundAddress".into(), (addr,)).await?;
    println!("funded {addr}: {} txs", hashes.len());
    let token = ITIP20::new(PATH_USD, &p);
    for _ in 0..30 {
        let b = token.balanceOf(addr).call().await?;
        if b > U256::ZERO { println!("  pathUSD balance {b}"); return Ok(()); }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    anyhow::bail!("not funded")
}

async fn tx_type<P: Provider<TempoNetwork>>(p: &P, h: B256) -> anyhow::Result<String> {
    let v: serde_json::Value = p.raw_request("eth_getTransactionByHash".into(), (h,)).await?;
    Ok(format!("{} feeToken={}", v["type"], v["feeToken"]))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let payer = PrivateKeySigner::random();
    let operator = PrivateKeySigner::random();
    let payee = PrivateKeySigner::random().address(); // payee never signs anything here
    println!("payer {} operator {} payee {payee}", payer.address(), operator.address());
    fund(payer.address()).await?;
    fund(operator.address()).await?;

    // --- WS subscription (the watchtower's view), started before anything happens
    let ws = ProviderBuilder::new_with_network::<TempoNetwork>().connect_ws(WsConnect::new(WS)).await?;
    let filter = Filter::new().address(TIP20_CHANNEL_RESERVE_ADDRESS).event_signature(vec![
        ITIP20ChannelReserve::CloseRequested::SIGNATURE_HASH,
        ITIP20ChannelReserve::ChannelClosed::SIGNATURE_HASH,
    ]);
    let sub = ws.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    // --- payer opens a channel with operator set (plain EIP-1559 tx, no fee_token)
    let payer_p = ProviderBuilder::new_with_network::<TempoNetwork>().wallet(payer.clone()).connect_http(RPC.parse()?);
    let reserve = ITIP20ChannelReserve::new(TIP20_CHANNEL_RESERVE_ADDRESS, &payer_p);
    let grace = reserve.CLOSE_GRACE_PERIOD().call().await?;
    println!("CLOSE_GRACE_PERIOD = {grace}");
    let salt = B256::random();
    let rcpt = reserve
        .open(payee, operator.address(), PATH_USD, U96::from(1_000_000u64), salt, Address::ZERO)
        .send().await?.get_receipt().await?;
    println!("open tx {} status {} type {:?}", rcpt.transaction_hash, rcpt.status(), tx_type(&payer_p, rcpt.transaction_hash).await?);
    let opened = rcpt.decoded_log::<ITIP20ChannelReserve::ChannelOpened>().expect("ChannelOpened");
    let ev = &opened.data;
    let descriptor = ITIP20ChannelReserve::ChannelDescriptor {
        payer: ev.payer, payee: ev.payee, operator: ev.operator, token: ev.token,
        salt: ev.salt, authorizedSigner: ev.authorizedSigner, expiringNonceHash: ev.expiringNonceHash,
    };
    let channel_id = ev.channelId;
    println!("channelId {channel_id} deposit {}", ev.deposit);

    // --- payer signs a voucher for 0.30; server "spent" 0.25
    let domain = eip712_domain! { name: "TIP20 Channel Reserve", version: "1", chain_id: CHAIN_ID, verifying_contract: TIP20_CHANNEL_RESERVE_ADDRESS, };
    let v = Voucher { channelId: channel_id, cumulativeAmount: U96::from(300_000u64) };
    let digest = v.eip712_signing_hash(&domain);
    let onchain_digest = reserve.getVoucherDigest(channel_id, U96::from(300_000u64)).call().await?;
    println!("digest local==onchain: {}", digest == onchain_digest);
    let sig = payer.sign_hash(&digest).await?;
    let sig_bytes = alloy::primitives::Bytes::from(sig.as_bytes().to_vec());

    // --- payer requests close
    let rc = reserve.requestClose(descriptor.clone()).send().await?.get_receipt().await?;
    let t_req = Instant::now();
    println!("requestClose tx {} status {}", rc.transaction_hash, rc.status());

    // --- tower detects via WS
    loop {
        let log = tokio::time::timeout(Duration::from_secs(30), stream.next()).await?.expect("stream");
        if log.topic0() == Some(&ITIP20ChannelReserve::CloseRequested::SIGNATURE_HASH) && log.topics()[1] == channel_id {
            let d = ITIP20ChannelReserve::CloseRequested::decode_log_data(log.data())?;
            println!("WS saw CloseRequested after {:?}, closeGraceEnd {}", t_req.elapsed(), d.closeGraceEnd);
            break;
        }
    }
    let st = reserve.getChannelStatesBatch(vec![channel_id]).call().await?;
    println!("poll state: settled {} deposit {} closeRequestedAt {}", st[0].settled, st[0].deposit, st[0].closeRequestedAt);

    // --- operator closes (plain EIP-1559 tx; fee paid in default fee token)
    let op_p = ProviderBuilder::new_with_network::<TempoNetwork>().wallet(operator.clone()).connect_http(RPC.parse()?);
    let op_reserve = ITIP20ChannelReserve::new(TIP20_CHANNEL_RESERVE_ADDRESS, &op_p);
    let t_close = Instant::now();
    let pending = op_reserve.close(descriptor.clone(), U96::from(300_000u64), U96::from(250_000u64), sig_bytes).send().await?;
    let tx_hash = *pending.tx_hash();
    let cr = pending.get_receipt().await?;
    println!("close tx {tx_hash} status {} type {:?} in {:?} (total since request {:?})", cr.status(), tx_type(&op_p, tx_hash).await?, t_close.elapsed(), t_req.elapsed());
    let closed = cr.decoded_log::<ITIP20ChannelReserve::ChannelClosed>().expect("ChannelClosed");
    println!("ChannelClosed settledToPayee {} refundedToPayer {}", closed.data.settledToPayee, closed.data.refundedToPayer);
    let bal = ITIP20::new(PATH_USD, &op_p).balanceOf(payee).call().await?;
    println!("payee pathUSD balance {bal}");
    Ok(())
}
