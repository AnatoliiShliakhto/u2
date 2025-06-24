mod app;

use crate::app::init_app;
use ::api_util::Error;
use ::api_util::{
    console::*,
    env,
    log::{error, stdout_logger},
    panic::set_panic_hook,
    prometheus,
    shutdown::create_shutdown_handle,
};
use ::std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    print_banner();
    stdout_logger();

    let shutdown_handle = create_shutdown_handle().await;
    set_panic_hook(Some(shutdown_handle.clone()));

    print_service_started(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    use axum_server::tls_rustls::RustlsConfig;
    let config = RustlsConfig::from_pem_file(
        env::get_var_or_default("SSL_CRT", "/etc/ssl/private/cert.pem"),
        env::get_var_or_default("SSL_KEY", "/etc/ssl/private/key.pem"),
    )
    .await
    .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 443));

    tokio::spawn(prometheus::start_metrics_server(Some(
        shutdown_handle.clone(),
    )));

    if let Err(err) = axum_server::bind_rustls(addr, config)
        .handle(shutdown_handle)
        .serve(init_app().into_make_service())
        .await
    {
        error!("failed to start HTTP server: {}", err);
        panic!("failed to start server");
    }

    Ok(())
}
