use ::api_util::{Error, env};
use ::surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(serde::Deserialize)]
struct ServiceConfig {
    pub database: String,
    pub user: String,
    pub password: String,
}

pub trait Migration {
    async fn services_init(&self) -> Result<(), Error>;
}

impl Migration for Surreal<Client> {
    async fn services_init(&self) -> Result<(), Error> {
        let sql = include_str!("../../res/query/service/init.surql");

        let services_cfg: Vec<ServiceConfig> =
            serde_json::from_str(&env::get_var_or_default("SERVICES_DB_CFG", "[]"))?;

        for service_cfg in services_cfg {
            let sql = sql
                .replace("$namespace", &env::get_var_or_default("DB_NAMESPACE", "u2"))
                .replace("$database", &service_cfg.database)
                .replace("$user", &service_cfg.user)
                .replace("$password", &service_cfg.password);
            if let Ok(Some(true)) = self.query(sql).await?.take(0) {
                continue;
            }
            Err(Error::Unknown("service init failed"))?;
        }

        Ok(())
    }
}
