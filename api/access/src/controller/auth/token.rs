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
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse},
};
use ::axum_extra::extract::CookieJar;
use ::serde::Serialize;
use ::std::borrow::Cow;

#[derive(Serialize)]
pub struct RefreshTokenBody<'a> {
    pub token_type: &'a str,
    pub access_token: Cow<'a, str>,
}

pub async fn token(jar: CookieJar) -> Result<impl IntoResponse, Error> {
    let cookie = jar.get(COOKIE_JWT).ok_or(AuthError::MissingToken)?;

    let state = get_state();

    let current_refresh_token =
        Claims::from_refresh_token(cookie.value(), &state.cfg.security.jwt_keys.decoding)?
            .jti
            .ok_or(AuthError::MissingToken)?;

    let account = state.db.find_auth_by_token(&current_refresh_token).await?;

    let refresh_token_uuid = state
        .db
        .update_refresh_token(
            current_refresh_token,
            state.cfg.security.jwt.refresh_expires_in,
        )
        .await?;

    // Build the access token
    let access_token = Claims::new()
        .with_issuer(&state.cfg.security.jwt.issuer)
        .with_subject(&state.cfg.security.jwt.subject)
        .with_expiration_in_seconds(state.cfg.security.jwt.access_expires_in)
        .with_auth(Auth {
            id: account.0,
            permissions: Permissions::init(&state.permissions_map.read().await, account.1),
        })
        .build_token(&state.cfg.security.jwt_keys.encoding)?;

    // Build the refresh token
    let refresh_token = Claims::new()
        .with_issuer(&state.cfg.security.jwt.issuer)
        .with_subject(&state.cfg.security.jwt.subject)
        .with_jti(&refresh_token_uuid)
        .with_expiration_in_seconds(state.cfg.security.jwt.refresh_expires_in)
        .build_token(&state.cfg.security.jwt_keys.encoding)?;

    Ok((
        AppendHeaders(vec![(
            SET_COOKIE,
            format!(
                "{COOKIE_JWT}={refresh_token}; Path=/api/auth; Max-Age={0}; {1}",
                state.cfg.security.jwt.refresh_expires_in, state.cfg.security.set_cookie,
            ),
        )]),
        Json(RefreshTokenBody {
            token_type: "Bearer",
            access_token,
        }),
    ))
}
