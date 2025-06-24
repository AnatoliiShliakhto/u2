mod amqp;
mod app;

use crate::{
    amqp::init_amqp,
    app::{init_app, init_state},
};
use ::api_util::{Error, console::*, log, panic::*, server, shutdown::*};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    print_banner();

    let shutdown_handle = create_shutdown_handle().await;
    set_panic_hook(Some(shutdown_handle.clone()));

    let state = init_state().await?;
    log::amqp_logger(state.cfg.name, &state.amqp).await;

    print_service_started(state.cfg.name, state.cfg.version);

    init_amqp().await?;

    server::start_server(init_app(), shutdown_handle).await;

    print_service_stopped();
    Ok(())
}
