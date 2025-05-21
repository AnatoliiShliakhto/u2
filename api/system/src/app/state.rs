use crate::repository::migration::Migration;
use ::api_util::{Error, amqp::AMQPPool, amqp_init, env, migrate::MigrateExt};
use ::std::sync::{Arc, OnceLock};
use ::surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

#[derive(Default)]
pub struct AppState {
    amqp: OnceLock<Arc<AMQPPool>>,
    db: OnceLock<Arc<Surreal<Client>>>,
}

impl AppState {
    pub async fn init(&self) -> Result<(), Error> {
        let amqp = amqp_init!();
        let db = Arc::new(<Surreal<Client>>::init());

        db.connect::<Ws>(env::get_var_or_default("DB_URL", "surrealdb:8000"))
            .await?;

        db.signin(Root {
            username: &env::get_var_or_default("DB_USER", "root"),
            password: &env::get_var_or_default("DB_PASS", "root"),
        })
        .await?;

        db.services_init().await?;
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
