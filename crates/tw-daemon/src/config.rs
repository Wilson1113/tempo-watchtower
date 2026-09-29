//! GIVEN (M0). Keys never go through clap: a flag lands in shell history and `ps`, and clap's
//! `env = ...` prints the VALUE in `--help` (`[env: TW_OPERATOR_KEY=0x...]`) unless
//! `hide_env_values = true`. Secrets are read with `std::env::var` into types whose `Debug`
//! never prints them.
use std::fmt;

use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Watchtower for Tempo v2 payment channels")]
pub struct Args {
    #[arg(
        long,
        env = "TW_RPC_URL",
        default_value = "https://rpc.moderato.tempo.xyz"
    )]
    pub rpc_url: String,
    #[arg(
        long,
        env = "TW_WS_URL",
        default_value = "wss://rpc.moderato.tempo.xyz"
    )]
    pub ws_url: String,
    #[arg(long, env = "TW_CHAIN_ID", default_value_t = 42431)]
    pub chain_id: u64,
    #[arg(long, env = "TW_LISTEN", default_value = "127.0.0.1:8787")]
    pub listen: String,
    #[arg(long, env = "TW_LEDGER", default_value = "tower.redb")]
    pub ledger: std::path::PathBuf,
    #[arg(long, env = "TW_POLL_MS", default_value_t = 1000)]
    pub poll_ms: u64,
    /// Poll-only mode (no WebSocket).
    #[arg(long)]
    pub no_ws: bool,
}

/// A private key whose `Debug` shows only the address.
pub struct Secret(PrivateKeySigner);

impl Secret {
    pub fn from_env(var: &str) -> anyhow::Result<Self> {
        let raw = std::env::var(var)
            .map_err(|_| anyhow::anyhow!("{var} is not set (see .env.example)"))?;
        let key: PrivateKeySigner = raw
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("{var} is not a valid private key"))?;
        Ok(Self(key))
    }
    pub fn from_signer(s: PrivateKeySigner) -> Self {
        Self(s)
    }
    pub fn address(&self) -> Address {
        self.0.address()
    }
    pub fn signer(&self) -> &PrivateKeySigner {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Secret(address = {})", self.0.address())
    }
}

/// Shared secret between the payee server and the tower's ingest API.
pub struct IngestToken(String);

impl IngestToken {
    pub fn from_env(var: &str) -> anyhow::Result<Self> {
        let t = std::env::var(var).map_err(|_| anyhow::anyhow!("{var} is not set"))?;
        if t == "change-me" || t.len() < 16 {
            anyhow::bail!(
                "{var} must be a random string of at least 16 chars (not the example value)"
            );
        }
        Ok(Self(t))
    }
    pub fn matches(&self, presented: &str) -> bool {
        // not constant-time; fine for a localhost demo, use `subtle` before exposing it
        self.0 == presented
    }
}

impl fmt::Debug for IngestToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IngestToken(<redacted>)")
    }
}

#[derive(Debug)]
pub struct Config {
    pub args: Args,
    pub operator: Secret,
    pub ingest_token: IngestToken,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn debug_never_prints_the_key_or_token() {
        let key = PrivateKeySigner::random();
        let hex = key.to_bytes().to_string(); // 0x + 64 hex
        let cfg = Config {
            args: Args::parse_from(["tempo-watchtower"]),
            operator: Secret::from_signer(key),
            ingest_token: IngestToken("a-long-random-ingest-token".into()),
        };
        let shown = format!("{cfg:?}");
        assert!(!shown.contains(&hex[2..]), "key leaked: {shown}");
        assert!(!shown.contains("a-long-random-ingest-token"));
        assert!(shown.contains(&cfg.operator.address().to_string()));
    }

    #[test]
    fn help_has_no_key_arguments() {
        let help = Args::command().render_long_help().to_string();
        assert!(!help.contains("KEY"), "{help}");
    }
}
