use ::api_util::amqp::{DeliveryExt, DeliveryResult};

pub async fn consumer(delivery: DeliveryResult) {
    let Ok(Some(delivery)) = delivery else { return };

    match delivery.routing_key.as_str() {
        "entity.created" => {}
        "entity.updated" => {}
        "entity.deleted" => {}
        _ => (),
    }

    delivery.confirm()
}
