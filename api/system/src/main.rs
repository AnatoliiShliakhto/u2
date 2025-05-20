mod app;
mod repository;
mod service;

use crate::{app::AppState, service::amqp::amqp_consumer};
use ::api_util::{
    Error,
    amqp::AMQPPoolExt,
    env,
    log::{self},
    shutdown::pending_shutdown_signal,
};
use ::std::sync::LazyLock;

pub static APP: LazyLock<AppState> = LazyLock::new(AppState::default);

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    env::init();
    APP.init().await?;
    println!(include_str!("../../../res/logo/banner.txt"));

    let amqp = APP.amqp();

    log::amqp_logger(&amqp).await;

    amqp.set_topic_delegate("system.svc", amqp_consumer).await?;
    amqp.set_broadcast_delegate(amqp_consumer).await?;

    pending_shutdown_signal().await;

    Ok(())
}
