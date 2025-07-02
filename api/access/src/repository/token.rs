use std::borrow::Cow;
use ::api_util::{AuthError, Error};
use ::chrono::Utc;
use ::serde::Serialize;
use ::surrealdb::{Surreal, engine::remote::ws::Client};

pub trait TokenRepository {
    async fn create_refresh_token(
        &self,
        user_id: impl Into<String>,
        expiration: i64,
        device: Option<impl Into<String>>,
    ) -> Result<Cow<str>, Error>;
    async fn update_refresh_token(
        &self,
        id: impl Into<String>,
        expiration: i64,
    ) -> Result<Cow<str>, Error>;
    async fn delete_refresh_token(&self, id: impl Into<String>) -> Result<(), Error>;
    async fn delete_expired_refresh_tokens(&self) -> Result<(), Error>;
}

impl TokenRepository for Surreal<Client> {
    async fn create_refresh_token(
        &self,
        user_id: impl Into<String>,
        expiration: i64,
        device: Option<impl Into<String>>,
    ) -> Result<Cow<str>, Error> {
        #[derive(Serialize)]
        struct SqlParams {
            user_id: String,
            expiration_at: i64,
            device: Option<String>,
        }

        self.query(include_str!(
            "../../res/query/middleware/token/create.surql"
        ))
        .bind(SqlParams {
            user_id: user_id.into(),
            expiration_at: Utc::now().timestamp() + expiration,
            device: device.map(Into::into),
        })
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(Error::from(AuthError::TokenCreation))
    }

    async fn update_refresh_token(
        &self,
        id: impl Into<String>,
        expiration: i64,
    ) -> Result<Cow<str>, Error> {
        #[derive(Serialize)]
        struct SqlParams {
            token_id: String,
            expiration_at: i64,
        }

        self.query(include_str!(
            "../../res/query/middleware/token/update.surql"
        ))
        .bind(SqlParams {
            token_id: id.into(),
            expiration_at: Utc::now().timestamp() + expiration,
        })
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(Error::from(AuthError::InvalidToken))
    }

    async fn delete_refresh_token(&self, id: impl Into<String>) -> Result<(), Error> {
        self.query(include_str!(
            "../../res/query/middleware/token/delete.surql"
        ))
        .bind(("token_id", id.into()))
        .await?
        .check()?;

        Ok(())
    }

    async fn delete_expired_refresh_tokens(&self) -> Result<(), Error> {
        self.query(include_str!(
            "../../res/query/middleware/token/delete_expired.surql"
        ))
        .await?
        .check()?;

        Ok(())
    }
}
