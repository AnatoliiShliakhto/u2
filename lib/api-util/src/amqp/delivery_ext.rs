use ::deadpool_lapin::lapin::{message::Delivery, options::BasicAckOptions};
use ::serde::de::DeserializeOwned;
use ::serde_json::Error;
use ::std::borrow::Cow;
use ::tracing::error;

pub trait DeliveryExt {
    fn app_id(&self) -> &str;
    fn message_id(&self) -> &str;
    fn reply_to(&self) -> &str;
    fn confirm(self);
    fn extract_string(&self) -> String;
    fn extract_str(&self) -> Cow<str>;
    fn extract_json<T: DeserializeOwned>(&self) -> Result<T, Error>;
}

impl DeliveryExt for Delivery {
    fn app_id(&self) -> &str {
        self.properties.app_id().as_ref().map_or("", |s| s.as_str())
    }

    fn message_id(&self) -> &str {
        self.properties
            .message_id()
            .as_ref()
            .map_or("", |s| s.as_str())
    }

    fn reply_to(&self) -> &str {
        self.properties
            .reply_to()
            .as_ref()
            .map_or("", |s| s.as_str())
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
            "failed to acknowledge AMQP delivery"
        );
    }
}
