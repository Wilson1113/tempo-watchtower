//! GIVEN (M4). Run the core tasks (brain, poller, closer, api, ws) in one `JoinSet`. If any of
//! them returns or panics while we are NOT shutting down, log it, cancel the rest and return an
//! error, so `main` exits non-zero. A crash-and-restart is safe by design (ledger + `Restarted` +
//! startup poll); a zombie tower whose closer silently died is not.
use std::time::Duration;

use anyhow::anyhow;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

pub type Tasks = JoinSet<anyhow::Result<()>>;

pub async fn supervise(mut tasks: Tasks, shutdown: CancellationToken) -> anyhow::Result<()> {
    let first = tokio::select! {
        r = tasks.join_next() => r,
        _ = shutdown.cancelled() => None,
    };
    let requested = shutdown.is_cancelled();
    shutdown.cancel(); // stop everything else
    let result = match first {
        None => Ok(()),
        Some(Ok(Ok(()))) if requested => Ok(()),
        Some(Ok(Ok(()))) => Err(anyhow!("a core task exited unexpectedly")),
        Some(Ok(Err(e))) => Err(e.context("a core task failed")),
        Some(Err(join)) => Err(anyhow!("a core task panicked: {join}")),
    };
    if let Err(e) = &result {
        tracing::error!(error = %format!("{e:#}"), "stopping the tower; restart it (state is persisted)");
    }
    // Give the other tasks a bounded time to finish their shutdown.
    while let Ok(Some(_)) = tokio::time::timeout(Duration::from_secs(5), tasks.join_next()).await {}
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn waits_for(token: &CancellationToken) -> impl Future<Output = anyhow::Result<()>> + use<> {
        let t = token.clone();
        async move {
            t.cancelled().await;
            Ok(())
        }
    }

    #[tokio::test]
    async fn a_panicking_task_stops_the_tower_with_an_error() {
        let shutdown = CancellationToken::new();
        let mut tasks = Tasks::new();
        tasks.spawn(waits_for(&shutdown)); // e.g. the brain
        tasks.spawn(async { panic!("closer bug") });
        let err = supervise(tasks, shutdown.clone())
            .await
            .expect_err("must fail");
        assert!(format!("{err:#}").contains("panicked"), "{err:#}");
        assert!(shutdown.is_cancelled(), "other tasks were told to stop");
    }

    #[tokio::test]
    async fn a_task_that_returns_early_is_an_error_too() {
        let shutdown = CancellationToken::new();
        let mut tasks = Tasks::new();
        tasks.spawn(waits_for(&shutdown));
        tasks.spawn(async { Err(anyhow!("closer channel closed")) });
        let err = supervise(tasks, shutdown).await.expect_err("must fail");
        assert!(
            format!("{err:#}").contains("closer channel closed"),
            "{err:#}"
        );
    }

    #[tokio::test]
    async fn ctrl_c_style_shutdown_is_a_clean_exit() {
        let shutdown = CancellationToken::new();
        let mut tasks = Tasks::new();
        tasks.spawn(waits_for(&shutdown));
        tasks.spawn(waits_for(&shutdown));
        shutdown.cancel();
        assert!(supervise(tasks, shutdown).await.is_ok());
    }
}
