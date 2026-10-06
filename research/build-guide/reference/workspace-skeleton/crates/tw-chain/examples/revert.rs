//! Shows that precompile reverts decode into typed errors (M7).
use alloy::primitives::{Address, B256, Bytes, aliases::U96};
use tw_chain::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let p = http_provider("https://rpc.moderato.tempo.xyz")?;
    let d = Reserve::ChannelDescriptor {
        payer: Address::repeat_byte(1),
        payee: Address::repeat_byte(2),
        operator: Address::repeat_byte(3),
        token: Address::ZERO,
        salt: B256::ZERO,
        authorizedSigner: Address::ZERO,
        expiringNonceHash: B256::ZERO,
    };
    let reserve = Reserve::new(RESERVE, &p);
    let call = reserve
        .close(d, U96::from(1), U96::from(1), Bytes::new())
        .from(Address::repeat_byte(3));
    match call.call().await {
        Ok(_) => anyhow::bail!("expected a revert"),
        Err(e) => println!("classified: {:?}", classify(&e)),
    }
    Ok(())
}
