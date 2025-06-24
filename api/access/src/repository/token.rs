use ::api_util::{AuthError, Error};
use ::chrono::Utc;
use ::std::borrow::Cow;
use ::surrealdb::{Surreal, engine::remote::ws::Client};

pub trait TokenRepository {
    async fn create_refresh_token(
        &self,
        user_id: impl ToString,
        expiration: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>, Error>;
    async fn update_refresh_token(
        &self,
        id: impl ToString,
        expiration: i64,
    ) -> Result<Cow<str>, Error>;
    async fn delete_refresh_token(&self, id: impl ToString) -> Result<(), Error>;
    async fn delete_expired_refresh_tokens(&self) -> Result<(), Error>;
}

impl TokenRepository for Surreal<Client> {
    async fn create_refresh_token(
        &self,
        user_id: impl ToString,
        expiration: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>, Error> {
        self.query(include_str!(
            "../../res/query/middleware/token/create.surql"
        ))
        .bind(("user_id", user_id.to_string()))
        .bind(("expiration_at", Utc::now().timestamp() + expiration))
        .bind((
            "device",
            device.map_or("web".to_string(), |v| v.to_string()),
        ))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(AuthError::TokenCreation.into())
    }

    async fn update_refresh_token(
        &self,
        id: impl ToString,
        expiration: i64,
    ) -> Result<Cow<str>, Error> {
        self.query(include_str!(
            "../../res/query/middleware/token/update.surql"
        ))
        .bind(("token_id", id.to_string()))
        .bind(("expiration_at", Utc::now().timestamp() + expiration))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(AuthError::InvalidToken.into())
    }

    async fn delete_refresh_token(&self, id: impl ToString) -> Result<(), Error> {
        self.query(include_str!(
            "../../res/query/middleware/token/delete.surql"
        ))
        .bind(("token_id", id.to_string()))
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
