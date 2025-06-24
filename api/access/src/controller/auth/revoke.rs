use super::COOKIE_JWT;
use crate::{app::get_state, middleware::Claims, repository::TokenRepository};
use ::api_util::{AuthError, Error};
use ::axum::{
    http::header::SET_COOKIE,
    response::{AppendHeaders, IntoResponse},
};
use ::axum_extra::extract::CookieJar;

pub async fn revoke(jar: CookieJar) -> Result<impl IntoResponse, Error> {
    let cookie = jar.get(COOKIE_JWT).ok_or(AuthError::MissingToken)?;

    let state = get_state();

    if let Some(refresh_token) =
        Claims::from_refresh_token(cookie.value(), &state.cfg.security.jwt_keys.decoding)?.jti
    {
        state.db.delete_refresh_token(refresh_token).await?;
    }

    Ok(AppendHeaders(vec![(
        SET_COOKIE,
        format!(
            "{COOKIE_JWT}=; Path=/api/auth; Expires=Thu, 01 Jan 1970 00:00:00 GMT; {}",
            state.cfg.security.set_cookie
        ),
    )]))
}
