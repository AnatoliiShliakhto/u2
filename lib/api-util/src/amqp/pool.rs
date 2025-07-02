use super::*;
use crate::Error;
use ::deadpool_lapin::{
    Config, Pool, Runtime,
    lapin::{Channel, Queue},
};
pub use deadpool_lapin::lapin::{
    BasicProperties, ConsumerDelegate, ExchangeKind, message::*, options::*, types::*,
};

const DEFAULT_CONSUMER_TAG_SUFFIX: &str = "consumer";

pub struct AMQPPool {
    pool: Pool,
    channel: Channel,
}

impl AMQPPool {
    pub async fn new(url: impl Into<String>) -> Result<Self, Error> {
        let config = Config {
            url: Some(url.into()),
            ..Default::default()
        };
        let pool = config
            .create_pool(Some(Runtime::Tokio1))
            .map_err(map_amqp_err)?;
        let channel = create_channel(&pool).await?;
        Ok(Self { pool, channel })
    }

    pub async fn set_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        queue: &str,
        options: AMQPChannelOptions<'_>,
        delegate: D,
    ) -> Result<(), Error> {
        let channel = self.prepare_exchange(&options).await?;
        let _queue = self.prepare_queue(&channel, queue, &options).await?;
        self.start_consumer(&channel, queue, &options, delegate)
            .await?;
        Ok(())
    }

    async fn prepare_exchange(&self, options: &AMQPChannelOptions<'_>) -> Result<Channel, Error> {
        let channel = create_channel(&self.pool).await?;
        let exchange_name = exchange_kind_to_str(&options.exchange);
        let declare_options = OptionsBuilder::exchange_declare_options(options);

        channel
            .exchange_declare(
                exchange_name,
                options.exchange.clone(),
                declare_options,
                FieldTable::default(),
            )
            .await
            .map_err(map_amqp_err)?;
        Ok(channel)
    }

    async fn prepare_queue(
        &self,
        channel: &Channel,
        name: &str,
        options: &AMQPChannelOptions<'_>,
    ) -> Result<Queue, Error> {
        let declare_options = OptionsBuilder::queue_declare_options(options);
        let queue = channel
            .queue_declare(name, declare_options, FieldTable::default())
            .await
            .map_err(map_amqp_err)?;

        let bind_options = OptionsBuilder::queue_bind_options(options);
        channel
            .queue_bind(
                name,
                exchange_kind_to_str(&options.exchange),
                &options.routing_key,
                bind_options,
                FieldTable::default(),
            )
            .await
            .map_err(map_amqp_err)?;
        Ok(queue)
    }

    async fn start_consumer<D: ConsumerDelegate + Clone + 'static>(
        &self,
        channel: &Channel,
        name: &str,
        options: &AMQPChannelOptions<'_>,
        delegate: D,
    ) -> Result<(), Error> {
        let consumer_tag = format!("{name}.{DEFAULT_CONSUMER_TAG_SUFFIX}");
        let consume_options = OptionsBuilder::consume_options(options);

        channel
            .basic_consume(name, &consumer_tag, consume_options, FieldTable::default())
            .await
            .map_err(map_amqp_err)?
            .set_delegate(delegate.clone());
        Ok(())
    }

    pub async fn send(
        &self,
        exchange: ExchangeKind,
        routing_key: &str,
        options: AMQPMessageOptions,
        payload: &[u8],
    ) -> Result<(), Error> {
        let publish_options = OptionsBuilder::publish_options(&options);
        self.channel
            .basic_publish(
                exchange_kind_to_str(&exchange),
                routing_key,
                publish_options,
                payload,
                options.properties,
            )
            .await
            .map_err(map_amqp_err)?;
        Ok(())
    }
}

struct OptionsBuilder;

impl OptionsBuilder {
    fn consume_options(channel_options: &AMQPChannelOptions) -> BasicConsumeOptions {
        BasicConsumeOptions {
            no_local: false,
            no_ack: false,
            exclusive: channel_options.exclusive,
            nowait: channel_options.nowait,
        }
    }

    fn exchange_declare_options(channel_options: &AMQPChannelOptions) -> ExchangeDeclareOptions {
        ExchangeDeclareOptions {
            passive: channel_options.passive,
            durable: channel_options.durable,
            auto_delete: channel_options.auto_delete,
            internal: channel_options.internal,
            nowait: channel_options.nowait,
        }
    }

    fn queue_declare_options(channel_options: &AMQPChannelOptions) -> QueueDeclareOptions {
        QueueDeclareOptions {
            passive: channel_options.passive,
            durable: channel_options.durable,
            exclusive: channel_options.exclusive,
            auto_delete: channel_options.auto_delete,
            nowait: channel_options.nowait,
        }
    }

    fn queue_bind_options(channel_options: &AMQPChannelOptions) -> QueueBindOptions {
        QueueBindOptions {
            nowait: channel_options.nowait,
        }
    }

    fn publish_options(message_options: &AMQPMessageOptions) -> BasicPublishOptions {
        BasicPublishOptions {
            mandatory: message_options.mandatory,
            immediate: message_options.immediate,
        }
    }
}

fn map_amqp_err<E: ToString>(err: E) -> Error {
    Error::Amqp(err.to_string())
}

async fn create_channel(pool: &Pool) -> Result<Channel, Error> {
    let connection = pool.get().await.map_err(map_amqp_err)?;
    let channel = connection.create_channel().await.map_err(map_amqp_err)?;
    Ok(channel)
}

fn exchange_kind_to_str(kind: &ExchangeKind) -> &str {
    match kind {
        ExchangeKind::Custom(name) => name,
        ExchangeKind::Direct => "amq.direct",
        ExchangeKind::Fanout => "amq.fanout",
        ExchangeKind::Headers => "amq.headers",
        ExchangeKind::Topic => "amq.topic",
    }
}
