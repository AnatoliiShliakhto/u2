use ::axum_server::Handle;
use ::tracing::info;

const SERVICE_STARTED_MSG: &str = "service started";
const WAITING_FOR_SHUTDOWN_MSG: &str = "waiting for shutdown signal...";
const SERVICE_STOPPED_MSG: &str = "service stopped";

async fn wait_for_shutdown_signals() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => (),
        _ = terminate => (),
    }
}

async fn handle_graceful_shutdown(handle: Handle) {
    wait_for_shutdown_signals().await;
    handle.shutdown();
}

pub async fn create_shutdown_handle() -> Handle {
    let handle = Handle::new();
    tokio::spawn(handle_graceful_shutdown(handle.clone()));
    handle
}

pub async fn wait_for_shutdown() {
    info!("{SERVICE_STARTED_MSG}");
    info!("{WAITING_FOR_SHUTDOWN_MSG}");
    wait_for_shutdown_signals().await;
    info!("{SERVICE_STOPPED_MSG}");
}
