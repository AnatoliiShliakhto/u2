use super::config::AppConfig;
use crate::repository::PermissionsRepository;
use ::api_util::{Error, amqp::AMQPPool, amqp_init, db_init, migrate::MigrateExt};
use ::std::sync::Arc;
use ::surrealdb::{Surreal, engine::remote::ws::Client};
use ::tokio::sync::{OnceCell, RwLock};

static APP: OnceCell<AppState> = OnceCell::const_new();

pub struct AppState {
    pub cfg: Arc<AppConfig>,
    pub amqp: Arc<AMQPPool>,
    pub db: Arc<Surreal<Client>>,
    pub permissions_map: Arc<RwLock<Vec<u16>>>,
}

impl AppState {
    pub async fn update_permissions_map(&self) -> Result<(), Error> {
        let permissions = self.db.get_permissions_map().await?;
        {
            *self.permissions_map.write().await = permissions;
        }

        Ok(())
    }
}

pub async fn init_state() -> Result<&'static AppState, Error> {
    let cfg = Arc::new(AppConfig::new());
    let amqp = amqp_init!();
    let db = db_init!();

    db.migrate_up().await?;

    let state = AppState {
        cfg,
        amqp,
        db,
        permissions_map: Arc::new(RwLock::new(vec![])),
    };

    APP.set(state)
        .map_err(|_| Error::Unknown("Application state already set"))?;

    Ok(APP.get().unwrap())
}

pub fn get_state() -> &'static AppState {
    APP.get().expect("Application state is not set")
}
