use api_util::{amqp::{DeliveryExt, DeliveryResult}, log::info};

pub async fn amqp_consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    let app_id = delivery.app_id();
    let message_id = delivery.message_id();
    let data = String::from_utf8_lossy(&delivery.data);
    info!("app: {app_id} message_id: {message_id} data: {data}");

    delivery.ack_async()
}