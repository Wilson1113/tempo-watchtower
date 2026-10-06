//! `tempo-watchtower` M0 shell: config, logging, one chain call, `/healthz`.
use axum::{Router, routing::get};
use clap::Parser;
use tracing_subscriber::EnvFilter;

use tokio_util::sync::CancellationToken;
use tw_daemon::{
    config, ledger,
    supervise::{Tasks, supervise},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok(); // loads .env if present; never print the environment
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let cfg = config::Config {
        args: config::Args::parse(),
        operator: config::Secret::from_env("TW_OPERATOR_KEY")?,
        ingest_token: config::IngestToken::from_env("TW_INGEST_TOKEN")?,
    };
    tracing::info!(?cfg, "starting"); // safe: Debug is redacted (see config.rs tests)
    let p = tw_chain::http_provider(&cfg.args.rpc_url)?;
    tracing::info!(grace = tw_chain::grace_period(&p).await?, operator = %cfg.operator.address(), "connected");
    let _ledger = ledger::Ledger::open(&cfg.args.ledger)?;
    let app = Router::new().route("/healthz", get(|| async { "ok" }));
    let listener = tokio::net::TcpListener::bind(&cfg.args.listen).await?;

    let shutdown = CancellationToken::new();
    let mut tasks = Tasks::new();
    let token = shutdown.clone();
    tasks.spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(token.cancelled_owned())
            .await?;
        Ok(())
    });
    // M4: tasks.spawn(brain.run(..)); tasks.spawn(poller(..)); tasks.spawn(closer(..));
    // Inside the brain: if `close_tx.send(job)` returns Err, the closer is gone. Return
    // Err(..) from the brain (never `let _ =` it) so `supervise` stops the process.
    let ctrl_c = shutdown.clone();
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        ctrl_c.cancel();
    });
    supervise(tasks, shutdown).await // Err -> logged, non-zero exit
}

#[cfg(test)]
mod tests {
    use axum::{
        Json, Router,
        body::Body,
        extract::Path,
        http::{Request, StatusCode},
        routing::post,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[derive(serde::Deserialize)]
    struct SpentUpdate {
        spent: String,
    }

    // Stand-in handler showing the test pattern; the real one sends a Cmd to the brain.
    async fn post_spent(
        Path(id): Path<String>,
        Json(u): Json<SpentUpdate>,
    ) -> (StatusCode, String) {
        if u.spent.parse::<u128>().is_err() {
            return (
                StatusCode::BAD_REQUEST,
                "spent must be a base-10 integer string".into(),
            );
        }
        if id != "0xknown" {
            return (
                StatusCode::NOT_FOUND,
                "unknown channel; register it first".into(),
            );
        }
        (StatusCode::NO_CONTENT, String::new())
    }

    fn app() -> Router {
        Router::new().route("/v1/channels/{id}/spent", post(post_spent))
    }

    async fn send(uri: &str, body: &'static str) -> (StatusCode, String) {
        let req = Request::post(uri)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .expect("request");
        let res = app().oneshot(req).await.expect("response");
        let status = res.status();
        let bytes = res.into_body().collect().await.expect("body").to_bytes();
        (status, String::from_utf8_lossy(&bytes).into_owned())
    }

    #[tokio::test]
    async fn rejects_non_numeric_spent_and_unknown_ids() {
        let (s, b) = send("/v1/channels/0xknown/spent", r#"{"spent":"x"}"#).await;
        assert_eq!(s, StatusCode::BAD_REQUEST);
        assert!(b.contains("integer"));
        assert_eq!(
            send("/v1/channels/0xother/spent", r#"{"spent":"1"}"#)
                .await
                .0,
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            send("/v1/channels/0xknown/spent", r#"{"spent":"1"}"#)
                .await
                .0,
            StatusCode::NO_CONTENT
        );
    }

    #[tokio::test(start_paused = true)]
    async fn paused_time_makes_a_15_minute_wait_instant() {
        let t = tokio::time::Instant::now();
        tokio::time::sleep(std::time::Duration::from_secs(900)).await;
        assert!(t.elapsed() >= std::time::Duration::from_secs(900));
    }
}
