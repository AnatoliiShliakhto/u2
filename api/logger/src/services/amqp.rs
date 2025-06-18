use crate::POOL;
use ::api_util::amqp::{DeliveryExt, DeliveryResult};

pub async fn amqp_consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    let app_id = delivery.app_id();

    POOL.write(&app_id, &delivery.data).await.ok();

    delivery.ack_async()
}
