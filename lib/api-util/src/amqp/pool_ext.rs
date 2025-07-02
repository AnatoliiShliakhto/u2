use crate::amqp::AMQPMessageOptions;
use crate::{
    Error,
    amqp::{AMQPPool, ExchangeKind},
};
use ::serde::Serialize;

pub trait AMQPPoolExt {
    fn send_message(
        &self,
        routing_key: &str,
        options: AMQPMessageOptions,
        payload: &[u8],
    ) -> impl Future<Output = Result<(), Error>>;
    fn send_json<S: Serialize>(
        &self,
        routing_key: &str,
        options: AMQPMessageOptions,
        payload: &S,
    ) -> impl Future<Output = Result<(), Error>>;
    fn broadcast_message(
        &self,
        options: AMQPMessageOptions,
        payload: &[u8],
    ) -> impl Future<Output = Result<(), Error>>;
    fn broadcast_json<S: Serialize>(
        &self,
        options: AMQPMessageOptions,
        payload: &S,
    ) -> impl Future<Output = Result<(), Error>>;
}

impl AMQPPoolExt for AMQPPool {
    async fn send_message(
        &self,
        routing_key: &str,
        options: AMQPMessageOptions,
        payload: &[u8],
    ) -> Result<(), Error> {
        self.send(ExchangeKind::Topic, routing_key, options, payload)
            .await
    }

    async fn send_json<S: Serialize>(
        &self,
        routing_key: &str,
        options: AMQPMessageOptions,
        payload: &S,
    ) -> Result<(), Error> {
        self.send(
            ExchangeKind::Topic,
            routing_key,
            options,
            &serde_json::to_vec(payload)?,
        )
        .await
    }

    async fn broadcast_message(
        &self,
        options: AMQPMessageOptions,
        payload: &[u8],
    ) -> Result<(), Error> {
        self.send(ExchangeKind::Fanout, "", options, payload).await
    }

    async fn broadcast_json<S: Serialize>(
        &self,
        options: AMQPMessageOptions,
        payload: &S,
    ) -> Result<(), Error> {
        self.send(
            ExchangeKind::Fanout,
            "",
            options,
            &serde_json::to_vec(payload)?,
        )
        .await
    }
}

// impl AMQPPool {
//     async fn set_delegate_with_exchange<D: ConsumerDelegate + Clone + 'static>(
//         &self,
//         exchange_kind: ExchangeKind,
//         queue: &str,
//         routing_key: Option<&str>,
//         delegate: D,
//     ) -> Result<(), Error> {
//         let mut service_options = AMQPChannelOptions::default()
//             .with_exchange(exchange_kind)
//             .with_durable();
//
//         if let Some(key) = routing_key {
//             service_options = service_options.with_routing_key(key);
//         }
//
//         self.set_delegate(queue, service_options, delegate).await
//     }
// }
