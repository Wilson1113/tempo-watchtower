//! GIVEN (M6): WebSocket subscriptions with tempo-alloy: payee-filtered logs + newHeads heartbeat.
use alloy::{
    consensus::BlockHeader,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use futures::StreamExt;
use tempo_alloy::TempoNetwork;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ws = ProviderBuilder::new_with_network::<TempoNetwork>()
        .connect_ws(WsConnect::new("wss://rpc.moderato.tempo.xyz"))
        .await?;
    let payee: Address = std::env::var("PAYEE")
        .unwrap_or_else(|_| Address::ZERO.to_string())
        .parse()?;
    let _logs = ws
        .subscribe_logs(&tw_chain::payee_filter(payee))
        .await?
        .into_stream();
    let mut heads = ws.subscribe_blocks().await?.into_stream();
    for _ in 0..3 {
        let Some(h) = heads.next().await else {
            anyhow::bail!("head stream ended")
        };
        println!(
            "head {} timestamp_millis {}",
            h.number(),
            h.timestamp_millis
        );
    }
    Ok(())
}
