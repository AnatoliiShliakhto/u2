use crate::{env, prometheus, shutdown};
use ::axum::Router;
use ::std::net::{IpAddr, SocketAddr};
use ::tracing::{error, info};

pub async fn start_server(router: Router) {
    println!(include_str!("../../../../res/logo/banner.txt"));
    
    env::init();

    /*
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Crypto Provider initialization failed.");

    let tls_cfg =
        axum_server::tls_rustls::RustlsConfig::from_pem_file("/etc/cert/cert.pem", "/etc/cert/key.pem")
            .await.unwrap();


     */

    let host = env::get_var_or_default("HOST", "0.0.0.0:80");
    let addr = host.parse().unwrap_or(SocketAddr::new(
        IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
        80,
    ));
    let metrics_host = env::get_var_or_default("METRICS_HOST", "0.0.0.0:3001");

    let shutdown_handle = shutdown::handle_init().await;
    tokio::spawn(prometheus::start_metrics_server(
        metrics_host,
        Some(shutdown_handle.clone()),
    ));

    info!("starting...");

    info!("listening on {addr:?}");
    axum_server::bind(addr)
        .handle(shutdown_handle)
        .serve(router.into_make_service())
        .await
        .map_err(|err| {
            error!("error starting server: {}", err);
        })
        .unwrap();

    info!("gracefully stopped");
}
