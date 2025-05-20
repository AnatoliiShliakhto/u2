use ::axum_server::Handle;
use ::tracing::info;

pub async fn handle_init() -> Handle {
    let handle = Handle::new();
    tokio::spawn(shutdown_handler(handle.clone()));
    handle
}

async fn shutdown_handler(handle: Handle) {
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
        _ = ctrl_c => handle.shutdown(),
        _ = terminate => handle.shutdown(),
    }
}

pub async fn pending_shutdown_signal() {
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

    info!("service started");
    info!("waiting for shutdown signal...");   
    
    tokio::select! {
        _ = ctrl_c => (),
        _ = terminate => (),
    }

    info!("service stopped");    
}