use ::deadpool_lapin::lapin::{message::Delivery, options::BasicAckOptions};
use ::serde::de::DeserializeOwned;
use ::serde_json::Error;
use ::std::borrow::Cow;
use ::tracing::error;

pub trait DeliveryExt {
    fn app_id(&self) -> String;
    fn message_id(&self) -> String;
    fn reply_to(&self) -> String;
    fn confirm(self);
    fn extract_string(&self) -> String;
    fn extract_str(&self) -> Cow<str>;
    fn extract_json<T: DeserializeOwned>(&self) -> Result<T, Error>;
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

    fn reply_to(&self) -> String {
        self.properties
            .reply_to()
            .clone()
            .unwrap_or_default()
            .to_string()
    }

    fn confirm(self) {
        tokio::spawn(async move {
            handle_delivery_ack(self).await;
        });
    }

    fn extract_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }

    fn extract_str(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.data)
    }

    fn extract_json<T: DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_slice(&self.data)
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
