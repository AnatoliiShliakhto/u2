use api_util::{amqp::{BasicAckOptions, DeliveryResult}, log::{error, info}};

pub async fn amqp_consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    let app_id = delivery.properties.app_id().clone().unwrap_or_default();
    let message_id = delivery.properties.message_id().clone().unwrap_or_default();
    let data = String::from_utf8_lossy(&delivery.data);
    info!("app: {app_id} message_id: {message_id} data: {data}");

    if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
        error!("Delivery ack error: {}", err);
    }
}