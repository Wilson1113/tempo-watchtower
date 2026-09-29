//! `tw-demo` skeleton: `keygen` and `grace` are GIVEN; the rest of the CLI is M1 (yours).
use std::{fs, path::Path};

use alloy::{primitives::B256, signers::local::PrivateKeySigner};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Demo payer / payee / scenario tools for tempo-watchtower (TESTNET ONLY)")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Fill every missing or EMPTY key/token in .env (creating it if needed). Prints addresses only.
    Keygen,
    /// Print CLOSE_GRACE_PERIOD() from the chain at $TW_RPC_URL.
    Grace,
}

/// Private keys: value is secret, `public` (the address) may be printed.
const KEY_VARS: [&str; 3] = ["TW_OPERATOR_KEY", "TW_PAYER_A_KEY", "TW_PAYER_B_KEY"];
const TOKEN_VAR: &str = "TW_INGEST_TOKEN";

struct Generated {
    value: String,  // secret: written to .env only
    public: String, // safe to print
}

fn generate(var: &str) -> Generated {
    if var == TOKEN_VAR {
        Generated {
            value: B256::random().to_string(),
            public: "(generated, hidden)".into(),
        }
    } else {
        let k = PrivateKeySigner::random();
        Generated {
            value: k.to_bytes().to_string(),
            public: format!("address {}", k.address()),
        }
    }
}

/// Unquoted value of a `VAR=value  # comment` line, trimmed; `None` if the line isn't `var=`.
fn value_of<'a>(line: &'a str, var: &str) -> Option<&'a str> {
    let rest = line
        .trim_start()
        .strip_prefix(var)?
        .trim_start()
        .strip_prefix('=')?;
    let rest = rest.split(" #").next().unwrap_or("");
    Some(rest.trim())
}

fn needs_value(var: &str, v: &str) -> bool {
    v.is_empty() || (var == TOKEN_VAR && (v == "change-me" || v.len() < 16))
}

/// Returns the new .env contents and the printable report. Existing non-empty values are kept,
/// empty ones are replaced IN PLACE (dotenvy keeps the first occurrence, so appending a second
/// line would not work), missing ones are appended.
fn fill_env(contents: &str, mut gen_value: impl FnMut(&str) -> Generated) -> (String, Vec<String>) {
    let vars: Vec<&str> = KEY_VARS.iter().copied().chain([TOKEN_VAR]).collect();
    let mut seen = Vec::new();
    let mut report = Vec::new();
    let mut out = String::new();
    for line in contents.lines() {
        let hit = vars
            .iter()
            .find_map(|v| value_of(line, v).map(|val| (*v, val)));
        match hit {
            Some((var, val)) if needs_value(var, val) && !seen.contains(&var) => {
                let g = gen_value(var);
                out.push_str(&format!("{var}={}\n", g.value));
                report.push(format!("{var}: {}", g.public));
                seen.push(var);
            }
            Some((var, _)) => {
                out.push_str(line);
                out.push('\n');
                seen.push(var);
            }
            None => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    for var in vars {
        if !seen.contains(&var) {
            let g = gen_value(var);
            out.push_str(&format!("{var}={}\n", g.value));
            report.push(format!("{var}: {}", g.public));
        }
    }
    (out, report)
}

fn keygen(path: &Path) -> anyhow::Result<()> {
    let before = fs::read_to_string(path).unwrap_or_default();
    let (after, report) = fill_env(&before, generate);
    fs::write(path, after)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    }
    if report.is_empty() {
        println!("{}: all keys already set (nothing changed)", path.display());
    }
    for line in report {
        println!("{line}"); // addresses only; secrets stay in the file
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match Cli::parse().cmd {
        Cmd::Keygen => keygen(Path::new(".env"))?,
        Cmd::Grace => {
            dotenvy::dotenv().ok();
            let url = std::env::var("TW_RPC_URL")
                .unwrap_or_else(|_| "https://rpc.moderato.tempo.xyz".into());
            let p = tw_chain::http_provider(&url)?;
            println!("{}", tw_chain::grace_period(&p).await?);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../.env.example");

    fn dotenv_values(s: &str) -> Vec<(String, String)> {
        dotenvy::from_read_iter(s.as_bytes())
            .map(|r| r.expect("valid .env"))
            .collect()
    }

    #[test]
    fn keygen_fills_the_documented_env_example_and_prints_no_secret() {
        let (out, report) = fill_env(EXAMPLE, generate);
        let vals = dotenv_values(&out); // parse it exactly like the daemon will
        let get = |k: &str| vals.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone());
        for var in KEY_VARS {
            let v = get(var).expect("present");
            assert!(
                v.parse::<PrivateKeySigner>().is_ok(),
                "{var} not a usable key"
            );
            assert!(
                report.iter().all(|line| !line.contains(&v[2..])),
                "{var} leaked in report"
            );
        }
        let token = get(TOKEN_VAR).expect("token");
        assert!(token.len() >= 16 && !report.iter().any(|l| l.contains(&token)));
        assert_eq!(get("TW_CHAIN_ID").as_deref(), Some("42431")); // other lines preserved
        assert_eq!(report.len(), 4);
    }

    #[test]
    fn keygen_keeps_existing_values_and_is_idempotent() {
        let (first, _) = fill_env(EXAMPLE, generate);
        let (second, report) = fill_env(&first, generate);
        assert_eq!(first, second);
        assert!(report.is_empty());
    }

    #[test]
    fn keygen_works_without_any_env_file_and_replaces_the_example_token() {
        let (out, report) = fill_env("TW_INGEST_TOKEN=change-me\n", generate);
        assert_eq!(report.len(), 4);
        assert!(!out.contains("change-me"));
    }
}
