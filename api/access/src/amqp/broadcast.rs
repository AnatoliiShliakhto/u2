use ::api_util::amqp::{DeliveryExt, DeliveryResult};

pub async fn consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    delivery.confirm()
}
