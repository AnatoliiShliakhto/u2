mod amqp;
mod app;
mod controller;
mod middleware;
mod model;
mod repository;

use crate::{
    amqp::init_amqp,
    app::{get_state, init_app, init_state},
    repository::TokenRepository,
};
use ::api_util::{Error, console::*, log, panic::*, server, shutdown::*};
use ::tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    print_banner();

    let shutdown_handle = create_shutdown_handle().await;
    set_panic_hook(Some(shutdown_handle.clone()));

    let state = init_state().await?;
    log::amqp_logger(state.cfg.name, &state.amqp).await;

    print_service_started(state.cfg.name, state.cfg.version);

    init_amqp().await?;

    state.update_permissions_map().await?;

    tokio::spawn(async {
        let state = get_state();
        let timeout = Duration::from_secs(state.cfg.security.delete_expired_tokens_interval);
        loop {
            if state.db.delete_expired_refresh_tokens().await.is_ok() {
                log::info!("expired refresh tokens deleted successfully");
            };
            sleep(timeout).await;
        }
    });

    server::start_server(init_app(), shutdown_handle).await;

    print_service_stopped();
    Ok(())
}
