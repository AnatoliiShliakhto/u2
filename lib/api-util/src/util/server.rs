use crate::{env, prometheus};
use ::axum::Router;
use ::axum_server::Handle;
use ::std::net::{IpAddr, Ipv4Addr, SocketAddr};
use ::tracing::error;

const DEFAULT_SERVER_HOST: &str = "0.0.0.0";
const DEFAULT_SERVER_PORT: &str = "80";

fn get_server_address() -> SocketAddr {
    let host = env::get_var_or_default("SERVER_HOST", DEFAULT_SERVER_HOST);
    let port = env::get_var_or_default("SERVER_PORT", DEFAULT_SERVER_PORT);

    let ip: IpAddr = host
        .parse()
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    let port_num: u16 = port.parse().unwrap_or(80);

    SocketAddr::new(ip, port_num)
}

pub async fn start_server(router: Router, shutdown_handle: Handle) {
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
}
