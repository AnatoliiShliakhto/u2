use ::api_util::{AuthError, Error};
use ::serde::Deserialize;
use ::std::collections::HashMap;
use ::surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Deserialize)]
struct AccountEntity {
    id: String,
    permissions: Vec<(u16, u8)>,
}

pub trait AccountAuthRepository {
    async fn find_auth_by_credentials<T: ToString>(
        &self,
        login: T,
        password: T,
    ) -> Result<(String, HashMap<u16, u8>), Error>;
    async fn find_auth_by_token(
        &self,
        refresh_token: impl ToString,
    ) -> Result<(String, HashMap<u16, u8>), Error>;
}

impl AccountAuthRepository for Surreal<Client> {
    async fn find_auth_by_credentials<T: ToString>(
        &self,
        login: T,
        password: T,
    ) -> Result<(String, HashMap<u16, u8>), Error> {
        let account = self
            .query(include_str!(
                "../../res/query/middleware/auth/by_credentials.surql"
            ))
            .bind(("login", login.to_string()))
            .bind(("password", password.to_string()))
            .await?
            .take::<Option<AccountEntity>>(0)?
            .ok_or(AuthError::WrongCredentials)?;

        let permissions = account
            .permissions
            .iter()
            .map(|&(id, perm)| (id, perm))
            .collect::<HashMap<u16, u8>>();

        Ok((account.id, permissions))
    }

    async fn find_auth_by_token(
        &self,
        refresh_token: impl ToString,
    ) -> Result<(String, HashMap<u16, u8>), Error> {
        let account = self
            .query(include_str!(
                "../../res/query/middleware/auth/by_token.surql"
            ))
            .bind(("refresh_token", refresh_token.to_string()))
            .await?
            .take::<Option<AccountEntity>>(0)?
            .ok_or(AuthError::WrongCredentials)?;

        let permissions = account
            .permissions
            .iter()
            .map(|&(id, caps)| (id, caps))
            .collect::<HashMap<u16, u8>>();

        Ok((account.id, permissions))
    }
}
