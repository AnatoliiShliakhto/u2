use super::{COOKIE_JWT, util::build_token_response};
use crate::{
    app::get_state,
    middleware::Claims,
    repository::{AuthRepository, TokenRepository},
};
use ::api_util::{AuthError, Error};
use ::axum::{extract::Query, response::IntoResponse};
use ::axum_extra::extract::CookieJar;
use ::serde::Deserialize;
use ::std::borrow::Cow;

#[derive(Deserialize)]
pub struct AuthPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Cow<'a, str>>,
}

pub async fn authorize(
    jar: CookieJar,
    Query(payload): Query<AuthPayload<'_>>,
) -> Result<impl IntoResponse, Error> {
    let state = get_state();
    let refresh_token_uuid;

    let auth = if payload.login.is_none() {
        let cookie = jar.get(COOKIE_JWT).ok_or(AuthError::MissingToken)?;

        let current_refresh_token =
            Claims::from_refresh_token(cookie.value(), &state.cfg.security.jwt_keys.decoding)?
                .jti
                .ok_or(AuthError::MissingToken)?;

        let auth = state
            .db
            .find_auth_by_token(current_refresh_token.clone())
            .await?;

        refresh_token_uuid = state
            .db
            .update_refresh_token(
                current_refresh_token,
                state.cfg.security.jwt.refresh_expires_in,
            )
            .await?;

        auth
    } else {
        let AuthPayload {
            login: Some(login),
            password: Some(password),
            device,
        } = payload
        else {
            Err(AuthError::WrongCredentials)?
        };

        let auth = state.db.find_auth_by_credentials(login, password).await?;

        refresh_token_uuid = state
            .db
            .create_refresh_token(
                auth.id.clone(),
                state.cfg.security.jwt.refresh_expires_in,
                device,
            )
            .await?;

        auth
    };

    build_token_response(auth, refresh_token_uuid).await
}
