use crate::env;
use ::std::sync::Arc;
use ::surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Database,
    Surreal, Result,
};

pub async fn initialize_database() -> Result<Arc<Surreal<Client>>> {
    let database = <Surreal<Client>>::init();

    database
        .connect::<Ws>(env::get_var_or_default("DB_URL", "surrealdb:8000"))
        .await?;

    database
        .signin(Database {
            namespace: &env::get_var_or_default("DB_NAMESPACE", "u2"),
            database: &env::get_var_or_default("DB_DATABASE", "core"),
            username: &env::get_var_or_default("DB_USER", "root"),
            password: &env::get_var_or_default("DB_PASS", "root"),
        })
        .await?;

    Ok(Arc::new(database))
}

#[macro_export]
macro_rules! db_init {
    () => {{
        ::api_util::database::initialize_database().await?
    }};
}