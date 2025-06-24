mod amqp;
mod app;
mod repository;

use crate::{amqp::init_amqp, app::init_state, repository::migration::Migration};
use ::api_util::{Error, console::*, log, panic::*, shutdown::*};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    print_banner();
    set_panic_hook(None);

    let state = init_state().await?;
    log::amqp_logger(env!("CARGO_PKG_NAME"), &state.amqp).await;

    print_service_started(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    state.db.services_init().await?;

    init_amqp().await?;

    wait_for_shutdown_signals().await;

    print_service_stopped();
    Ok(())
}
