mod broadcast;

use crate::{Error, app::get_state};
use ::api_util::amqp::{AMQPChannelOptions, ExchangeKind};

pub async fn init_amqp() -> Result<(), Error> {
    let state = get_state();

    state
        .amqp
        .set_delegate(
            "system.broadcast",
            AMQPChannelOptions::default().with_exchange(ExchangeKind::Fanout).with_durable(),
            broadcast::consumer,
        )
        .await?;

    Ok(())
}
