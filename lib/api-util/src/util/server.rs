use crate::{prometheus, shutdown};
use ::axum::Router;
use ::std::net::{IpAddr, SocketAddr};
use ::tracing::{error, info};

pub async fn start_server(router: Router) {
    println!(include_str!("../../../../res/logo/banner.txt"));

    info!("service started");
    info!("waiting for shutdown signal...");

    let shutdown_handle = shutdown::handle_init().await;
    tokio::spawn(prometheus::start_metrics_server(Some(shutdown_handle.clone())));
    
    axum_server::bind(SocketAddr::new(
        IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
        80,
    ))
        .handle(shutdown_handle)
        .serve(router.into_make_service())
        .await
        .map_err(|err| {
            error!("error starting HTTP server: {}", err);
        })
        .expect("failed to start server");

    info!("service stopped");
}
