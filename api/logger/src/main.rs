mod router;
mod state;

use crate::state::AppState;
use ::api_util::{
    amqp::*,
    logger::{self, info},
    server,
};
use ::std::sync::Arc;

pub async fn ampq_callback(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    let data = String::from_utf8_lossy(&delivery.data);
    let app_id = delivery.properties.app_id().clone().unwrap_or_default();
    let message_id = delivery.properties.message_id().clone().unwrap_or_default();
    info!("app: {app_id} message_id: {message_id} data: {data}");
    
    if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
        dbg!("Delivery ack error: {}", err);
    }
}

#[tokio::main]
async fn main() {
    logger::stdout_logger();

    let ampq = AmpqPool::init(env!("CARGO_PKG_NAME")).await.unwrap();
    ampq.create_channel("logger.service").await.unwrap();
    ampq.set_delegate("logger.service", "logger.service.consumer", ampq_callback)
        .await
        .unwrap();

    let state = Arc::new(AppState::init(ampq));
    let router = router::init_app(&state).await.unwrap();

    server::start_server(router).await;
}
