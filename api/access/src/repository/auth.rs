use ::api_util::{AuthError, Error};
use ::serde::{Deserialize, Serialize};
use ::std::{borrow::Cow, collections::HashMap};
use ::surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Deserialize)]
struct AuthEntity<'a> {
    id: Cow<'a, str>,
    permissions: Cow<'a, [(u16, u8)]>,
}

pub struct AuthEntityDto<'a> {
    pub id: Cow<'a, str>,
    pub permissions: HashMap<u16, u8>,
}

pub trait AuthRepository {
    async fn find_auth_by_credentials(
        &self,
        login: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<AuthEntityDto, Error>;
    async fn find_auth_by_token(&self, refresh_token: impl Into<String>)
    -> Result<AuthEntityDto, Error>;
}

impl AuthRepository for Surreal<Client> {
    async fn find_auth_by_credentials(
        &self,
        login: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<AuthEntityDto, Error> {
        #[derive(Serialize)]
        struct SqlParams {
            login: String,
            password: String,
        }

        let user = self
            .query(include_str!(
                "../../res/query/middleware/auth/by_credentials.surql"
            ))
            .bind(SqlParams {
                login: login.into(),
                password: password.into(),
            })
            .await?
            .take::<Option<AuthEntity>>(0)?
            .ok_or(AuthError::WrongCredentials)?;

        Ok(entity_to_dto(user))
    }

    async fn find_auth_by_token(
        &self,
        refresh_token: impl Into<String>,
    ) -> Result<AuthEntityDto, Error> {
        let user = self.query(include_str!(
            "../../res/query/middleware/auth/by_token.surql"
        ))
        .bind(("refresh_token", refresh_token.into()))
        .await?
        .take::<Option<AuthEntity>>(0)?
        .ok_or(AuthError::WrongCredentials)?;
        
        Ok(entity_to_dto(user))
    }
}

fn entity_to_dto(auth: AuthEntity) -> AuthEntityDto {
    let permissions = auth
        .permissions
        .iter()
        .map(|&(id, perm)| (id, perm))
        .collect::<HashMap<u16, u8>>();

    AuthEntityDto {
        id: auth.id,
        permissions,
    }
}
