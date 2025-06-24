use super::COOKIE_JWT;
use crate::{
    app::get_state,
    middleware::{Auth, Claims},
    model::Permissions,
    repository::{AccountAuthRepository, TokenRepository},
};
use ::api_util::{AuthError, Error};
use ::axum::{
    Json,
    extract::Query,
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse},
};
use ::axum_extra::extract::CookieJar;
use ::serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
pub struct AuthBody<'a> {
    pub token_type: &'a str,
    pub access_token: Cow<'a, str>,
}

pub async fn authorize(
    jar: CookieJar,
    Query(payload): Query<AuthPayload<'_>>,
) -> Result<impl IntoResponse, Error> {
    let state = get_state();
    let refresh_token_uuid;

    let (account_id, account_permissions) = if payload.login.is_none() {
        let cookie = jar.get(COOKIE_JWT).ok_or(AuthError::MissingToken)?;

        let current_refresh_token =
            Claims::from_refresh_token(cookie.value(), &state.cfg.security.jwt_keys.decoding)?
                .jti
                .ok_or(AuthError::MissingToken)?;

        let account = state.db.find_auth_by_token(&current_refresh_token).await?;

        refresh_token_uuid = state
            .db
            .update_refresh_token(
                current_refresh_token,
                state.cfg.security.jwt.refresh_expires_in,
            )
            .await?;

        account
    } else {
        let AuthPayload {
            login: Some(login),
            password: Some(password),
            device,
        } = payload
        else {
            Err(AuthError::WrongCredentials)?
        };

        let account = state.db.find_auth_by_credentials(login, password).await?;

        refresh_token_uuid = state
            .db
            .create_refresh_token(
                &account.0,
                state.cfg.security.jwt.refresh_expires_in,
                device,
            )
            .await?;

        account
    };

    // Build the access token
    let access_token = Claims::new()
        .with_issuer(&state.cfg.security.jwt.issuer)
        .with_subject(&state.cfg.security.jwt.subject)
        .with_expiration_in_seconds(state.cfg.security.jwt.access_expires_in)
        .with_auth(Auth {
            id: account_id,
            permissions: Permissions::init(
                &state.permissions_map.read().await,
                account_permissions,
            ),
        })
        .build_token(&state.cfg.security.jwt_keys.encoding)?;

    // Build the refresh token
    let refresh_token = Claims::new()
        .with_issuer(&state.cfg.security.jwt.issuer)
        .with_subject(&state.cfg.security.jwt.subject)
        .with_jti(&refresh_token_uuid)
        .with_expiration_in_seconds(state.cfg.security.jwt.refresh_expires_in)
        .build_token(&state.cfg.security.jwt_keys.encoding)?;

    // Send the authorized tokens
    Ok((
        AppendHeaders(vec![(
            SET_COOKIE,
            format!(
                "{COOKIE_JWT}={refresh_token}; Path=/api/auth; Max-Age={0}; {1}",
                state.cfg.security.jwt.refresh_expires_in, state.cfg.security.set_cookie,
            ),
        )]),
        Json(AuthBody {
            token_type: "Bearer",
            access_token,
        }),
    ))
}
