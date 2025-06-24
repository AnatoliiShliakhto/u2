use ::api_util::amqp::{DeliveryExt, DeliveryResult};

pub async fn consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };
    
    // TODO: setup broadcast consumer delegate

    delivery.confirm()
}