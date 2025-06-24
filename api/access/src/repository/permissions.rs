use ::api_util::Error;
use ::surrealdb::{Surreal, engine::remote::ws::Client};

pub trait PermissionsRepository {
    async fn get_permissions_map(&self) -> Result<Vec<u16>, Error>;
}

impl PermissionsRepository for Surreal<Client> {
    async fn get_permissions_map(&self) -> Result<Vec<u16>, Error> {
        self.query(include_str!("../../res/query/permissions/map.surql"))
            .await?
            .take(0)
            .map(Ok)?
    }
}
