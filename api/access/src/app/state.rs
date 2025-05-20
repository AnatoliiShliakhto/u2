use ::api_util::{Error, amqp::AMQPPool, amqp_init};
use ::std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    amqp: Mutex<Option<Arc<AMQPPool>>>,
}

impl AppState {
    pub async fn init(&self) -> Result<(), Error> {
        let amqp = amqp_init!();

        self.amqp
            .lock()
            .map_err(|_| Error::Unknown("failed to initialize AMQP pool"))?
            .replace(amqp);

        Ok(())
    }

    pub fn amqp(&self) -> Arc<AMQPPool> {
        self.amqp
            .lock()
            .expect("failed to lock AMQP pool")
            .as_ref()
            .expect("AMQP pool isn't initialized")
            .clone()
    }
}
