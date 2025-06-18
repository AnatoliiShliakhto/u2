mod services;

use crate::services::{amqp::amqp_consumer, pool::Pool};
use ::api_util::{
    Error, amqp::AMQPPoolExt, amqp_init, env, log, shutdown::wait_for_shutdown,
};
use ::std::sync::LazyLock;

static POOL: LazyLock<Pool> =
    LazyLock::new(|| Pool::new(env::get_var_or_default("LOGS_DIR", "/logs")));

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    println!(include_str!("../../../res/logo/banner.txt"));

    let _logger_guard = log::file_logger(
        &env::get_var_or_default("LOGS_DIR", "/logs"),
        env!("CARGO_PKG_NAME"),
    );

    let amqp = amqp_init!();

    amqp.set_topic_delegate("log.write", amqp_consumer)
        .await?;

    wait_for_shutdown().await;

    Ok(())
}
