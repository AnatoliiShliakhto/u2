mod router;
mod state;
mod service;

use crate::{state::AppState, service::amqp::amqp_consumer};
use ::api_util::{logger, server, amqp::AmqpPool};
use ::std::sync::Arc;

const AMQP_TX_QUEUES: [&str; 4] = ["broadcast", "logger", "access", "auth"];
const AMQP_RX_QUEUES: [&str; 2] = ["broadcast", "auth"];

#[tokio::main]
async fn main() {
    let amqp = Arc::new(AmqpPool::init(env!("CARGO_PKG_NAME"), &AMQP_TX_QUEUES).await.unwrap());
    amqp.set_consumer(&AMQP_RX_QUEUES, amqp_consumer)
        .await
        .unwrap();
    logger::amqp_logger(&amqp).await; // Logger queue "logger" must be initialized before this

    let router = router::init_app(&Arc::new(AppState::init(&amqp))).await.unwrap();

    // remove this later 
    amqp.broadcast("notify", b"I'm alive!").await.unwrap();
    
    server::start_server(router).await
}
