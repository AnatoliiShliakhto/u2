mod app;
mod handlers;

use crate::app::*;
use ::prometheus_metrics::start_metrics_server;
use ::std::{
    sync::Arc,
};

fn main() -> Result<(), Box<Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(10 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(start_auth_server())
}

async fn start_auth_server() -> Result<(), Box<Error>> {
    dotenv::dotenv().ok();
    tokio::spawn(start_metrics_server());

    let addr = "0.0.0.0:80".parse().unwrap();

    println!("HTTP server listening on {addr:?}");

    let state = Arc::new(AppState::init());

    let app = router::init_app(&state).await?.into_make_service();

    let handle = axum_server::Handle::new();

    axum_server::bind(addr)
        .handle(handle)
        .serve(app)
        .await
        .map_err(|_| Box::new(Error::CustomError("Error starting HTTP server")))?;

    Ok(())
}
