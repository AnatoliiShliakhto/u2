use crate::amqp::BasicAckOptions;
use ::deadpool_lapin::lapin::message::Delivery;
use ::tracing::error;

pub trait DeliveryExt {
    fn app_id(&self) -> String;
    fn message_id(&self) -> String;
    fn ack_async(self);
}

impl DeliveryExt for Delivery {
    fn app_id(&self) -> String {
        self.properties
            .app_id()
            .clone()
            .unwrap_or_default()
            .to_string()
    }

    fn message_id(&self) -> String {
        self.properties
            .message_id()
            .clone()
            .unwrap_or_default()
            .to_string()
    }

    fn ack_async(self) {
        tokio::spawn(async move {
            handle_delivery_ack(self).await;
        });
    }
}

async fn handle_delivery_ack(delivery: Delivery) {
    if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
        error!(
            error = %err,
            delivery_tag = delivery.delivery_tag,
            "Failed to acknowledge AMQP delivery"
        );
    }
}
