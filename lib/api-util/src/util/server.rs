use crate::{env, prometheus, shutdown};
use ::axum::Router;
use ::std::net::{IpAddr, Ipv4Addr, SocketAddr};
use ::tracing::{error, info};

const DEFAULT_SERVER_HOST: &str = "0.0.0.0";
const DEFAULT_SERVER_PORT: &str = "80";

fn print_startup_banner() {
    println!(include_str!("../../../../res/logo/banner.txt"));
}

fn get_server_address() -> SocketAddr {
    let host = env::get_var_or_default("SERVER_HOST", DEFAULT_SERVER_HOST);
    let port = env::get_var_or_default("SERVER_PORT", DEFAULT_SERVER_PORT);

    let ip: IpAddr = host
        .parse()
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    let port_num: u16 = port.parse().unwrap_or(80);

    SocketAddr::new(ip, port_num)
}

pub async fn start_server(router: Router) {
    print_startup_banner();
    info!("service started");
    info!("waiting for shutdown signal...");

    let shutdown_handle = shutdown::create_shutdown_handle().await;
    let server_address = get_server_address();

    tokio::spawn(prometheus::start_metrics_server(Some(
        shutdown_handle.clone(),
    )));

    if let Err(err) = axum_server::bind(server_address)
        .handle(shutdown_handle)
        .serve(router.into_make_service())
        .await
    {
        error!("failed to start HTTP server: {}", err);
        panic!("failed to start server");
    }

    info!("service stopped");
}
