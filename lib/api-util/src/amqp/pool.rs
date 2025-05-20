use super::*;
use crate::Error;
use ::deadpool_lapin::{
    Config, Pool, Runtime,
    lapin::{Channel, Queue},
};

pub use deadpool_lapin::lapin::{
    BasicProperties, ConsumerDelegate, ExchangeKind, message::*, options::*, types::*,
};

pub struct AMQPPool {
    name: ShortString,
    pool: Pool,
    channel: Channel,
}

impl AMQPPool {
    pub async fn init(name: impl ToString, url: impl ToString) -> Result<Self, Error> {
        let config = Config {
            url: Some(url.to_string()),
            ..Default::default()
        };

        let pool = config
            .create_pool(Some(Runtime::Tokio1))
            .map_err(|err| Error::Amqp(err.to_string()))?;

        let channel = create_channel(&pool).await?;

        Ok(Self {
            name: name.to_string().into(),
            pool,
            channel,
        })
    }

    pub async fn set_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        options: AMQPChannelOptions,
        delegate: D,
    ) -> Result<(), Error> {
        let channel = self.init_exchange_channel(&options).await?;
        self.bind_channel_queue(&channel, &options).await?;
        channel
            .basic_consume(
                self.name.as_str(),
                &format!("{}.consumer", self.name.as_str()),
                BasicConsumeOptions {
                    no_local: false,
                    no_ack: false,
                    exclusive: options.exclusive,
                    nowait: options.nowait,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?
            .set_delegate(delegate.clone());

        Ok(())
    }

    async fn init_exchange_channel(&self, options: &AMQPChannelOptions) -> Result<Channel, Error> {
        let channel = create_channel(&self.pool).await?;

        channel
            .exchange_declare(
                exchange_kind_to_str(&options.exchange),
                options.exchange.clone(),
                ExchangeDeclareOptions {
                    passive: options.passive,
                    durable: options.durable,
                    auto_delete: options.auto_delete,
                    internal: options.internal,
                    nowait: options.nowait,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        Ok(channel)
    }

    async fn bind_channel_queue(
        &self,
        channel: &Channel,
        options: &AMQPChannelOptions,
    ) -> Result<Queue, Error> {
        let queue = channel
            .queue_declare(
                self.name.as_str(),
                QueueDeclareOptions {
                    passive: options.passive,
                    durable: options.durable,
                    exclusive: options.exclusive,
                    auto_delete: options.auto_delete,
                    nowait: options.nowait,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        channel
            .queue_bind(
                self.name.as_str(),
                exchange_kind_to_str(&options.exchange),
                &options.routing_key,
                QueueBindOptions {
                    nowait: options.nowait,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        Ok(queue)
    }

    pub async fn send(
        &self,
        exchange: ExchangeKind,
        routing_key: &str,
        options: AMQPMessageOptions,
        payload: &[u8],
    ) -> Result<(), Error> {
        let mut properties = options.properties;
        if options.app_id {
            properties = properties.with_app_id(self.name.clone());
        }

        self.channel
            .basic_publish(
                exchange_kind_to_str(&exchange),
                routing_key,
                BasicPublishOptions {
                    mandatory: options.mandatory,
                    immediate: options.immediate,
                },
                payload,
                properties,
            )
            .await
            .map_err(|err| Error::Amqp(err.to_string()))?;

        Ok(())
    }
}

async fn create_channel(pool: &Pool) -> Result<Channel, Error> {
    let connection = pool
        .get()
        .await
        .map_err(|err| Error::Amqp(err.to_string()))?;

    let channel = connection
        .create_channel()
        .await
        .map_err(|err| Error::Amqp(err.to_string()))?;

    Ok(channel)
}

fn exchange_kind_to_str(kind: &ExchangeKind) -> &str {
    match kind {
        ExchangeKind::Custom(name) => name.as_str(),
        ExchangeKind::Direct => "amq.direct",
        ExchangeKind::Fanout => "amq.fanout",
        ExchangeKind::Headers => "amq.headers",
        ExchangeKind::Topic => "amq.topic",
    }
}
