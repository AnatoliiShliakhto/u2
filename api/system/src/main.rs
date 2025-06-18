mod app;
mod repository;
mod service;

use crate::{app::init_state, service::amqp::amqp_consumer};
use ::api_util::{
    Error,
    amqp::AMQPPoolExt,
    log::{self},
    shutdown::wait_for_shutdown,
};
use crate::repository::migration::Migration;

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    println!(include_str!("../../../res/logo/banner.txt"));

    let state = init_state().await?;

    log::amqp_logger(&state.amqp).await;

    state.db.services_init().await?;
    
    state.amqp.set_topic_delegate("system.svc", amqp_consumer).await?;
    state.amqp.set_broadcast_delegate(amqp_consumer).await?;

    wait_for_shutdown().await;

    Ok(())
}
