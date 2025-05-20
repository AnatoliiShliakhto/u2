use crate::POOL;
use ::api_util::{
    amqp::{BasicAckOptions, DeliveryResult, ShortString},
    log::error,
};

pub async fn amqp_consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    let app_id = delivery
        .properties
        .app_id()
        .clone()
        .unwrap_or(ShortString::from("unknown"))
        .to_string();

    POOL.write(&app_id, &delivery.data).await.ok();

    if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
        error!("Delivery ack error: {}", err);
    }
}
