mod pool;

use crate::pool::Pool;
use ::api_util::{
    amqp::*,
    env,
    logger::{self, error, info},
};
use ::std::sync::LazyLock;

const AMQP_TX_QUEUES: [&str; 2] = ["broadcast", "logger"];
const AMQP_RX_QUEUES: [&str; 2] = ["broadcast", "logger"];

static POOL: LazyLock<Pool> =
    LazyLock::new(|| Pool::new(env::get_var_or_default("LOG_DIR", "/logs")));

pub async fn amqp_consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    let app_id = delivery
        .properties
        .app_id()
        .clone()
        .unwrap_or_default()
        .to_string();
    let message_id = delivery
        .properties
        .message_id()
        .clone()
        .unwrap_or_default()
        .to_string();

    match message_id.as_str() {
        "log" => {
            POOL.write(&app_id, &delivery.data).await.ok();
        }
        "shutdown" => (),
        _ => (),
    };

    if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
        error!("Delivery ack error: {}", err);
    }
}

#[tokio::main]
async fn main() {
    println!(include_str!("../../../res/logo/banner.txt"));
    env::init();
    
    let _logger_guard = logger::file_logger(
        &env::get_var_or_default("LOG_DIR", "/logs"),
        env!("CARGO_PKG_NAME"),
    );

    info!("service started...");

    let amqp = AmqpPool::init(env!("CARGO_PKG_NAME"), &AMQP_TX_QUEUES).await.unwrap();
    amqp.set_consumer(&AMQP_RX_QUEUES,amqp_consumer)
        .await
        .unwrap();

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // remove this later 
    amqp.broadcast("notify", b"I'm alive!").await.unwrap();
    
    tokio::select! {
        _ = ctrl_c => (),
        _ = terminate => (),
    }

    info!("gracefully stopped");
}
