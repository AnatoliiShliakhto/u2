use crate::{Error, env};
use ::deadpool_lapin::{
    Pool, Runtime,
    lapin::{
        BasicProperties, Channel, ConsumerDelegate, ExchangeKind,
        options::{
            BasicConsumeOptions, BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions,
            QueueDeclareOptions,
        },
        types::FieldTable,
    },
};
use ::std::{collections::HashMap, sync::Arc};
use ::tokio::sync::Mutex;

// Re-export dependencies
pub use ::deadpool_lapin::lapin::{
    message::DeliveryResult, options::BasicAckOptions, types::ShortString,
};

pub struct AmqpPool {
    app_id: ShortString,
    pool: Arc<Pool>,
    channels: Mutex<HashMap<String, Arc<Channel>>>,
}

impl AmqpPool {
    pub async fn init(app_id: &str, queues: &[&str]) -> Result<Self, Error> {
        env::init();
        let amqp_url = env::get_var_or_default("AMQP_URL", "amqp://root:root@rabbitmq:5672/%2f");
        let cfg = deadpool_lapin::Config {
            url: Some(amqp_url),
            ..Default::default()
        };

        let pool = Arc::new(
            cfg.create_pool(Some(Runtime::Tokio1))
                .map_err(|err| Error::Amqp(err.to_string()))?,
        );

        let amqp_pool = Self {
            app_id: app_id.into(),
            pool,
            channels: Mutex::new(HashMap::new()),
        };

        amqp_pool.create_broadcast_channel(app_id).await?;

        for queue in queues {
            amqp_pool.create_channel(queue).await?;
        }

        Ok(amqp_pool)
    }

    async fn create_broadcast_channel(&self, app_id: &str) -> Result<Arc<Channel>, Error> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;
        let channel = Arc::new(
            connection
                .create_channel()
                .await
                .map_err(|err| Error::Amqp(err.to_string()))?,
        );
        channel
            .exchange_declare(
                "amqp.broadcast",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    passive: false,
                    durable: false,
                    auto_delete: true,
                    internal: false,
                    nowait: false,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;
        channel
            .queue_declare(
                &format!("{app_id}.broadcast"),
                QueueDeclareOptions {
                    passive: false,
                    durable: false,
                    exclusive: false,
                    auto_delete: true,
                    nowait: false,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        channel
            .queue_bind(
                &format!("{app_id}.broadcast"),
                "amqp.broadcast",
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        self.channels
            .lock()
            .await
            .insert("broadcast".to_string(), channel.clone());

        Ok(channel)
    }

    async fn create_channel(&self, queue: &str) -> Result<Arc<Channel>, Error> {
        if let Some(channel) = { self.channels.lock().await.get(queue).cloned() } {
            return Ok(channel);
        }

        let connection = self
            .pool
            .get()
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;
        let channel = Arc::new(
            connection
                .create_channel()
                .await
                .map_err(|err| Error::Amqp(err.to_string()))?,
        );
        channel
            .queue_declare(
                queue,
                QueueDeclareOptions {
                    passive: false,
                    durable: true,
                    exclusive: false,
                    auto_delete: true,
                    nowait: false,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        self.channels
            .lock()
            .await
            .insert(queue.to_string(), channel.clone());

        Ok(channel)
    }

    async fn get_channel(&self, queue: &str) -> Result<Arc<Channel>, Error> {
        self.channels
            .lock()
            .await
            .get(queue)
            .cloned()
            .ok_or(Error::Amqp(format!("AMQP channel not found: {queue}")))
    }

    pub async fn set_consumer<D: ConsumerDelegate + Clone + 'static>(
        &self,
        queues: &[&str],
        delegate: D,
    ) -> Result<(), Error> {
        for queue in queues {
            let channel = if let Ok(channel) = self.get_channel(queue).await {
                channel
            } else {
                self.create_channel(queue).await?
            };

            let queue = if queue == &"broadcast" {
                format!("{}.broadcast", self.app_id.as_str())
            } else {
                queue.to_string()
            };

            channel
                .basic_consume(
                    &queue,
                    &format!("{}.consumer", self.app_id.as_str()),
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
                .map_err(|err| Error::Amqp(err.to_string()))?
                .set_delegate(delegate.clone());
        }

        Ok(())
    }

    pub async fn send(&self, queue: &str, message_id: &str, payload: &[u8]) -> Result<(), Error> {
        let channel = self.get_channel(queue).await?;

        channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default()
                    .with_app_id(self.app_id.clone())
                    .with_message_id(message_id.into()),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        Ok(())
    }

    pub async fn broadcast(&self, message_id: &str, payload: &[u8]) -> Result<(), Error> {
        let channel = self.get_channel("broadcast").await?;

        channel
            .basic_publish(
                "amqp.broadcast",
                "",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default()
                    .with_app_id(self.app_id.clone())
                    .with_message_id(message_id.into()),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        Ok(())
    }
}

impl Drop for AmqpPool {
    fn drop(&mut self) {
        self.pool.close()
    }
}
