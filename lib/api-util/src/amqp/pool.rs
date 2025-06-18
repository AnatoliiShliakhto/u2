use super::*;
use crate::Error;
use ::deadpool_lapin::{
    Config, Pool, Runtime,
    lapin::{Channel, Queue},
};

pub use deadpool_lapin::lapin::{
    BasicProperties, ConsumerDelegate, ExchangeKind, message::*, options::*, types::*,
};

const DEFAULT_CONSUMER_TAG_SUFFIX: &str = ".consumer";

pub struct AMQPPool {
    name: ShortString,
    pool: Pool,
    channel: Channel,
}

impl AMQPPool {
    pub async fn new(name: impl ToString, url: impl ToString) -> Result<Self, Error> {
        let config = Config {
            url: Some(url.to_string()),
            ..Default::default()
        };
        let pool = config
            .create_pool(Some(Runtime::Tokio1))
            .map_err(map_amqp_err)?;
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
        self.setup_consumer(&channel, &options, delegate).await?;
        Ok(())
    }

    async fn setup_consumer<D: ConsumerDelegate + Clone + 'static>(
        &self,
        channel: &Channel,
        options: &AMQPChannelOptions,
        delegate: D,
    ) -> Result<(), Error> {
        let queue_name = self.name.as_str();
        let consumer_tag = format!("{queue_name}{DEFAULT_CONSUMER_TAG_SUFFIX}");
        let consume_options = self.create_consume_options(options);

        channel
            .basic_consume(
                queue_name,
                &consumer_tag,
                consume_options,
                FieldTable::default(),
            )
            .await
            .map_err(map_amqp_err)?
            .set_delegate(delegate.clone());
        Ok(())
    }

    fn create_consume_options(&self, options: &AMQPChannelOptions) -> BasicConsumeOptions {
        BasicConsumeOptions {
            no_local: false,
            no_ack: false,
            exclusive: options.exclusive,
            nowait: options.nowait,
        }
    }

    async fn init_exchange_channel(&self, options: &AMQPChannelOptions) -> Result<Channel, Error> {
        let channel = create_channel(&self.pool).await?;
        let exchange_name = exchange_kind_to_str(&options.exchange);
        let declare_options = self.create_exchange_declare_options(options);

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

    fn create_exchange_declare_options(
        &self,
        options: &AMQPChannelOptions,
    ) -> ExchangeDeclareOptions {
        ExchangeDeclareOptions {
            passive: options.passive,
            durable: options.durable,
            auto_delete: options.auto_delete,
            internal: options.internal,
            nowait: options.nowait,
        }
    }

    async fn bind_channel_queue(
        &self,
        channel: &Channel,
        options: &AMQPChannelOptions,
    ) -> Result<Queue, Error> {
        let queue_name = self.name.as_str();
        let declare_options = self.create_queue_declare_options(options);

        let queue = channel
            .queue_declare(queue_name, declare_options, FieldTable::default())
            .await
            .map_err(map_amqp_err)?;

        let bind_options = QueueBindOptions {
            nowait: options.nowait,
        };

        channel
            .queue_bind(
                queue_name,
                exchange_kind_to_str(&options.exchange),
                &options.routing_key,
                bind_options,
                FieldTable::default(),
            )
            .await
            .map_err(map_amqp_err)?;
        Ok(queue)
    }

    fn create_queue_declare_options(&self, options: &AMQPChannelOptions) -> QueueDeclareOptions {
        QueueDeclareOptions {
            passive: options.passive,
            durable: options.durable,
            exclusive: options.exclusive,
            auto_delete: options.auto_delete,
            nowait: options.nowait,
        }
    }

    pub async fn send(
        &self,
        exchange: ExchangeKind,
        routing_key: &str,
        options: AMQPMessageOptions,
        payload: &[u8],
    ) -> Result<(), Error> {
        let properties = if options.app_id {
            options.properties.with_app_id(self.name.clone())
        } else {
            options.properties
        };

        let publish_options = BasicPublishOptions {
            mandatory: options.mandatory,
            immediate: options.immediate,
        };

        self.channel
            .basic_publish(
                exchange_kind_to_str(&exchange),
                routing_key,
                publish_options,
                payload,
                properties,
            )
            .await
            .map_err(map_amqp_err)?;
        Ok(())
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
        ExchangeKind::Custom(name) => name.as_str(),
        ExchangeKind::Direct => "amq.direct",
        ExchangeKind::Fanout => "amq.fanout",
        ExchangeKind::Headers => "amq.headers",
        ExchangeKind::Topic => "amq.topic",
    }
}
