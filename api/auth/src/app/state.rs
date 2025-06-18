use ::api_util::{Error, amqp::AMQPPool, amqp_init, db_init, migrate::MigrateExt};
use ::std::sync::Arc;
use ::surrealdb::{Surreal, engine::remote::ws::Client};
use ::tokio::sync::OnceCell;

static APP: OnceCell<AppState> = OnceCell::const_new();

pub struct AppState {
    pub amqp: Arc<AMQPPool>,
    pub db: Arc<Surreal<Client>>,
}

pub async fn init_state() -> Result<&'static AppState, Error> {
    let amqp = amqp_init!();
    let db = db_init!();

    db.migrate_up().await?;

    let state = AppState { amqp, db };

    APP.set(state)
        .map_err(|_| Error::Unknown("Application state already set"))?;

    Ok(APP.get().unwrap())
}

pub fn get_state() -> &'static AppState {
    APP.get().expect("Application state is not set")
}