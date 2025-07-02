use crate::repository::migration::Migration;
use ::api_util::{Error, amqp::AMQPPool, amqp_init, env};
use ::surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use ::tokio::sync::OnceCell;

static APP: OnceCell<AppState> = OnceCell::const_new();

pub struct AppState {
    pub amqp: AMQPPool,
    pub db: Surreal<Client>,
}

pub async fn init_state() -> Result<&'static AppState, Error> {
    let amqp = amqp_init!();
    let db = <Surreal<Client>>::init();

    db.connect::<Ws>(env::get_var_or_default("DB_URL", "surrealdb:8000"))
        .await?;

    db.signin(Root {
        username: env::get_var_or_default("DB_USER", "root"),
        password: env::get_var_or_default("DB_PASS", "root"),
    })
    .await?;

    db.services_init().await?;

    let state = AppState { amqp, db };

    APP.set(state)
        .map_err(|_| Error::Unknown("Application state already set"))?;

    Ok(APP.get().unwrap())
}

pub fn get_state() -> &'static AppState {
    APP.get().expect("Application state is not set")
}
