mod app;
mod repository;
mod service;

use crate::{
    app::{init_app, init_state},
    service::amqp::amqp_consumer,
};
use ::api_util::{Error, amqp::AMQPPoolExt, log, server};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let state = &init_state().await?;

    log::amqp_logger(&state.amqp).await;

    state.amqp.set_topic_delegate("auth.svc", amqp_consumer).await?;
    state.amqp.set_broadcast_delegate(amqp_consumer).await?;

    server::start_server(init_app()).await;

    Ok(())
}
