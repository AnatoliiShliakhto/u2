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
        self.set_delegate_with_exchange(ExchangeKind::Topic, None, delegate)
            .await
    }

    async fn set_direct_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> Result<(), Error> {
        self.set_delegate_with_exchange(ExchangeKind::Direct, None, delegate)
            .await
    }

    async fn set_fanout_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        delegate: D,
    ) -> Result<(), Error> {
        self.set_delegate_with_exchange(ExchangeKind::Fanout, None, delegate)
            .await
    }

    async fn set_topic_delegate<D: ConsumerDelegate + Clone + 'static>(
        &self,
        routing_key: &str,
        delegate: D,
    ) -> Result<(), Error> {
        self.set_delegate_with_exchange(ExchangeKind::Topic, Some(routing_key), delegate)
            .await
    }
}

impl AMQPPool {
    async fn set_delegate_with_exchange<D: ConsumerDelegate + Clone + 'static>(
        &self,
        exchange_kind: ExchangeKind,
        routing_key: Option<&str>,
        delegate: D,
    ) -> Result<(), Error> {
        let mut service_options = AMQPChannelOptions::default()
            .with_exchange(exchange_kind)
            .with_durable();

        if let Some(key) = routing_key {
            service_options = service_options.with_routing_key(key);
        }

        self.set_delegate(service_options, delegate).await
    }
}
