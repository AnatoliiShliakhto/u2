use super::COOKIE_JWT;
use crate::{
    app::get_state,
    middleware::{Auth, Claims},
    model::Permissions,
    repository::AuthEntityDto,
};
use ::api_util::Error;
use ::axum::{
    Json,
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse},
};
use ::serde::Serialize;
use ::std::borrow::Cow;

#[derive(Serialize)]
pub struct TokenBody<'a> {
    #[serde(rename = "type")]
    pub token_type: &'a str,
    #[serde(rename = "token")]
    pub access_token: Cow<'a, str>,
}

pub async fn build_token_response(
    auth: AuthEntityDto<'_>,
    refresh_token_uuid: Cow<'_, str>,
) -> Result<impl IntoResponse, Error> {
    let state = get_state();

    // Build the access token
    let access_token = Claims::new()
        .with_issuer(state.cfg.security.jwt.issuer)
        .with_subject(state.cfg.security.jwt.subject)
        .with_expiration_in_seconds(state.cfg.security.jwt.access_expires_in)
        .with_auth(Auth {
            id: auth.id,
            permissions: Permissions::init(&state.permissions_map.read().await, auth.permissions),
        })
        .build_token(&state.cfg.security.jwt_keys.encoding)?;

    // Build the refresh token
    let refresh_token = Claims::new()
        .with_issuer(state.cfg.security.jwt.issuer)
        .with_subject(state.cfg.security.jwt.subject)
        .with_jti(refresh_token_uuid)
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
        Json(TokenBody {
            token_type: "Bearer",
            access_token: Cow::Owned(access_token),
        }),
    ))
}
