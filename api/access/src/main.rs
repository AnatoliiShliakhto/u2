mod app;
mod handlers;
mod services;

use crate::{app::*, services::init_logger};
use ::prometheus_metrics::start_metrics_server;
use ::std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

fn main() -> Result<(), Box<Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(10 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(start_access_server())
}

async fn start_access_server() -> Result<(), Box<Error>> {
    init_logger();
    dotenv::dotenv().ok();
    tokio::spawn(start_metrics_server());

    let state = Arc::new(AppState::init());

    let app = router::init_app(&state).await?.into_make_service();

    let handle = axum_server::Handle::new();

    let addr = SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)), 80);

    println!("HTTP server listening on {addr:?}");

    axum_server::bind(addr)
        .handle(handle)
        .serve(app)
        .await
        .map_err(|_| Box::new(Error::CustomError("Error starting HTTP server")))?;

    Ok(())
}
