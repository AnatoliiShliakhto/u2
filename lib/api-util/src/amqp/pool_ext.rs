use crate::{
    Error,
    amqp::{AMQPChannelOptions, AMQPPool, ConsumerDelegate, ExchangeKind},
};

pub trait AMQPPoolExt {
    fn set_broadcast_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> impl Future<Output = Result<(), Error>>;

    fn set_direct_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> impl Future<Output = Result<(), Error>>;

    fn set_fanout_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> impl Future<Output = Result<(), Error>>;

    fn set_topic_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        routing_key: &str,
        delegate: D,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl AMQPPoolExt for AMQPPool {
    async fn set_broadcast_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> Result<(), Error> {
        let service_options = AMQPChannelOptions::default()
            .with_exchange(ExchangeKind::Topic)
            .durable();

        self.set_delegate(service_options, delegate).await?;

        Ok(())
    }

    async fn set_direct_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> Result<(), Error> {
        let service_options = AMQPChannelOptions::default()
            .with_exchange(ExchangeKind::Direct)
            .durable();

        self.set_delegate(service_options, delegate).await?;

        Ok(())
    }

    async fn set_fanout_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> Result<(), Error> {
        let service_options = AMQPChannelOptions::default()
            .with_exchange(ExchangeKind::Fanout)
            .durable();

        self.set_delegate(service_options, delegate).await?;

        Ok(())
    }

    async fn set_topic_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        routing_key: &str,
        delegate: D,
    ) -> Result<(), Error> {
        let service_options = AMQPChannelOptions::default()
            .with_exchange(ExchangeKind::Topic)
            .with_routing_key(routing_key)
            .durable();

        self.set_delegate(service_options, delegate).await?;

        Ok(())
    }
}
