use ::api_util::{Error, amqp::AMQPPool, amqp_init, db_init, migrate::MigrateExt};
use ::std::sync::{Arc, OnceLock};
use ::surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Default)]
pub struct AppState {
    amqp: OnceLock<Arc<AMQPPool>>,
    db: OnceLock<Arc<Surreal<Client>>>,
}

impl AppState {
    pub async fn init(&self) -> Result<(), Error> {
        let amqp = amqp_init!();
        let db = db_init!();

        db.migrate_up().await?;

        self.amqp.get_or_init(|| amqp);
        self.db.get_or_init(|| db);

        Ok(())
    }

    pub fn amqp(&self) -> &Arc<AMQPPool> {
        self.amqp.get().expect("AMQP pool not initialized")
    }

    pub fn db(&self) -> &Arc<Surreal<Client>> {
        self.db.get().expect("Database not initialized")
    }
}
