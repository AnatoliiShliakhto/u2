use ::api_util::{Error, amqp::AMQPPool, amqp_init};
use ::std::sync::{Arc, OnceLock};

#[derive(Default)]
pub struct AppState {
    amqp: OnceLock<Arc<AMQPPool>>,
}

impl AppState {
    pub async fn init(&self) -> Result<(), Error> {
        let amqp = amqp_init!();
        self.amqp.get_or_init(|| amqp);
        Ok(())
    }

    pub fn amqp(&self) -> &Arc<AMQPPool> {
        self.amqp.get().expect("AMQP pool not initialized")
    }
}
