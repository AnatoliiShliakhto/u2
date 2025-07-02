use super::{COOKIE_JWT, util::build_token_response};
use crate::{
    app::get_state,
    middleware::Claims,
    repository::{AuthRepository, TokenRepository},
};
use ::api_util::{AuthError, Error};
use ::axum::response::IntoResponse;
use ::axum_extra::extract::CookieJar;

pub async fn token(jar: CookieJar) -> Result<impl IntoResponse, Error> {
    let cookie = jar.get(COOKIE_JWT).ok_or(AuthError::MissingToken)?;

    let state = get_state();

    let current_refresh_token =
        Claims::from_refresh_token(cookie.value(), &state.cfg.security.jwt_keys.decoding)?
            .jti
            .ok_or(AuthError::MissingToken)?;

    let auth = state
        .db
        .find_auth_by_token(current_refresh_token.clone())
        .await?;

    let refresh_token_uuid = state
        .db
        .update_refresh_token(
            current_refresh_token,
            state.cfg.security.jwt.refresh_expires_in,
        )
        .await?;

    build_token_response(auth, refresh_token_uuid).await
}
