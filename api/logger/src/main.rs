mod app;

use crate::app::{amqp::amqp_consumer, pool::Pool};
use ::api_util::{
    Error,
    amqp::{AMQPChannelOptions, ExchangeKind},
    amqp_init,
    console::*,
    env, log,
    panic::*,
    shutdown::*,
};
use ::std::sync::LazyLock;

static POOL: LazyLock<Pool> =
    LazyLock::new(|| Pool::new(env::get_var_or_default("LOGS_DIR", "/logs")));

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    print_banner();
    set_panic_hook(None);

    let _logger_guard = log::file_logger(
        env::get_var_or_default("LOGS_DIR", "/logs"),
        env!("CARGO_PKG_NAME"),
    );

    print_service_started(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    amqp_init!()
        .set_delegate(
            "logger.log",
            AMQPChannelOptions::default()
                .with_exchange(ExchangeKind::Topic)
                .with_routing_key("log.write")
                .with_durable(),
            amqp_consumer,
        )
        .await?;

    wait_for_shutdown_signals().await;

    print_service_stopped();
    Ok(())
}
