mod app;
mod service;

use crate::{
    app::{AppState, init_app},
    service::amqp::amqp_consumer,
};
use ::api_util::{Error, amqp::AMQPPoolExt, env, log, server};
use ::std::sync::LazyLock;

pub static APP: LazyLock<AppState> = LazyLock::new(AppState::default);

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    env::init();
    APP.init().await?;

    let amqp = APP.amqp();

    log::amqp_logger(amqp).await;

    amqp.set_topic_delegate("audit.svc", amqp_consumer).await?;
//    amqp.set_broadcast_delegate(amqp_consumer).await?;

    server::start_server(init_app()).await;

    Ok(())
}
