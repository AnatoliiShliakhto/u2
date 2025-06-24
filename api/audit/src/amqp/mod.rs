mod entity;

use crate::app::get_state;
use ::api_util::{
    Error,
    amqp::{AMQPChannelOptions, ExchangeKind},
};

pub async fn init_amqp() -> Result<(), Error> {
    let state = get_state();

    state
        .amqp
        .set_delegate(
            "audit.entity",
            AMQPChannelOptions::default()
                .with_exchange(ExchangeKind::Topic)
                .with_routing_key("entity.*")
                .with_durable(),
            entity::consumer,
        )
        .await?;

    Ok(())
}
