use ::api_util::{Error, amqp::AMQPPool, db_init, amqp_init};
use ::std::sync::{Arc, Mutex};
use ::surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Default)]
pub struct AppState {
    amqp: Mutex<Option<Arc<AMQPPool>>>,
    database: Mutex<Option<Arc<Surreal<Client>>>>,
}

impl AppState {
    pub async fn init(&self) -> Result<(), Error> {
        let amqp = amqp_init!();
        let database = db_init!();
        
        self.amqp
            .lock()
            .map_err(|_| Error::Unknown("failed to initialize AMQP pool"))?
            .replace(amqp);

        self.database
            .lock()
            .map_err(|_| Error::Unknown("failed to initialize AMQP pool"))?
            .replace(database);

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

    pub fn db(&self) -> Arc<Surreal<Client>> {
        self.database
            .lock()
            .expect("failed to lock SurrealDB pool")
            .as_ref()
            .expect("SurrealDB pool isn't initialized")
            .clone()
    }    
}
