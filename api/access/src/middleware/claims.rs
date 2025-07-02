#![allow(dead_code)]
use crate::{
    app::{AppState, get_state},
    middleware::Auth,
    model::Capabilities,
};
use ::api_util::{AuthError, Error, env};
use ::axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use ::axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use ::chrono::{Duration, Utc};
use ::jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use ::serde::{Deserialize, Serialize};
use ::std::sync::LazyLock;
use std::borrow::Cow;

const MIN_AUTH_ID_LENGTH: usize = 20; // UUID v4
static JWT_ACCESS_EXPIRATION: LazyLock<usize> = LazyLock::new(|| {
    env::get_var_or_default("JWT_ACCESS_EXPIRATION", "600")
        .parse()
        .unwrap_or(600)
});

#[derive(Serialize, Deserialize)]
pub struct Claims<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<Cow<'a, str>>,
    pub iat: usize,
    pub exp: usize,
    #[serde(
        rename = "sid",
        skip_serializing_if = "Option::is_none",
        serialize_with = "Auth::serialize",
        deserialize_with = "Auth::deserialize",
        default
    )]
    pub auth: Option<Auth<'a>>,
}

impl<'a> Claims<'a> {
    pub fn new() -> Self {
        let timestamp_now = Self::current_timestamp();
        Self {
            iss: None,
            sub: None,
            jti: None,
            iat: timestamp_now,
            exp: timestamp_now + *JWT_ACCESS_EXPIRATION,
            auth: None,
        }
    }

    pub fn with_issuer(mut self, iss: impl Into<Cow<'a, str>>) -> Self {
        self.iss = Some(iss.into());
        self
    }

    pub fn with_subject(mut self, sub: impl Into<Cow<'a, str>>) -> Self {
        self.sub = Some(sub.into());
        self
    }

    pub fn with_jti(mut self, jti: impl Into<Cow<'a, str>>) -> Self {
        self.jti = Some(jti.into());
        self
    }

    pub fn with_expiration(mut self, timestamp: usize) -> Self {
        self.exp = timestamp;
        self
    }

    pub fn with_expiration_duration(mut self, duration: Duration) -> Self {
        self.exp = Self::calculate_expiration_timestamp(duration);
        self
    }

    pub fn with_expiration_in_seconds(self, seconds: i64) -> Self {
        self.with_expiration_duration(Duration::seconds(seconds))
    }

    pub fn with_expiration_in_minutes(self, minutes: i64) -> Self {
        self.with_expiration_duration(Duration::minutes(minutes))
    }

    pub fn with_expiration_in_hours(self, hours: i64) -> Self {
        self.with_expiration_duration(Duration::hours(hours))
    }

    pub fn with_expiration_in_days(self, days: i64) -> Self {
        self.with_expiration_duration(Duration::days(days))
    }

    pub fn with_auth(mut self, auth: Auth<'a>) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn is_expired(&self) -> bool {
        self.exp < Self::current_timestamp()
    }

    pub fn build_token(&self, encoding_key: &EncodingKey) -> Result<String, AuthError> {
        encode(&Header::default(), self, encoding_key).map_err(|_| AuthError::TokenCreation)
    }

    pub fn from_refresh_token(token: &str, decoding_key: &DecodingKey) -> Result<Self, AuthError> {
        let token_data = decode::<Claims>(token, decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Self::validate_refresh_token(&token_data.claims)?;
        Ok(token_data.claims)
    }

    pub fn id(&self) -> Option<&str> {
        self.auth.as_ref().map(|v| v.id.as_ref())
    }

    pub fn has_capabilities(
        &self,
        index: u16,
        capabilities: Capabilities,
    ) -> Result<(), AuthError> {
        let auth = self.auth.as_ref().ok_or(AuthError::AccessForbidden)?;

        if auth
            .permissions
            .get_or_default(index)
            .contains(capabilities)
        {
            Ok(())
        } else {
            Err(AuthError::AccessForbidden)
        }
    }

    fn current_timestamp() -> usize {
        Utc::now().timestamp() as usize
    }

    fn calculate_expiration_timestamp(duration: Duration) -> usize {
        (Utc::now() + duration).timestamp() as usize
    }

    fn validate_refresh_token(claims: &Claims) -> Result<(), AuthError> {
        if claims.jti.is_none() {
            Err(AuthError::InvalidToken)
        } else {
            Ok(())
        }
    }

    fn validate_access_token_auth(&self) -> Result<(), AuthError> {
        match &self.auth {
            None => Err(AuthError::InvalidToken),
            Some(auth) if auth.id.len() < MIN_AUTH_ID_LENGTH => Err(AuthError::InvalidToken),
            Some(_) => Ok(()),
        }
    }

    fn validate_permissions_consistency(&self, state: &AppState) -> Result<(), AuthError> {
        if let Some(auth) = &self.auth {
            let permissions_map_len = state
                .permissions_map
                .try_read()
                .map(|guard| guard.len())
                .unwrap_or(0);

            if permissions_map_len != auth.permissions.len() {
                return Err(AuthError::InvalidToken);
            }
        }
        Ok(())
    }
}

impl Default for Claims<'_> {
    fn default() -> Self {
        let timestamp_now = Self::current_timestamp();
        Self {
            iss: None,
            sub: None,
            jti: None,
            iat: timestamp_now,
            exp: timestamp_now + *JWT_ACCESS_EXPIRATION,
            auth: Some(Auth::default()),
        }
    }
}

impl<S> FromRequestParts<S> for Claims<'_>
where
    S: Send + Sync + Clone,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let state = get_state();

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingToken)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &state.cfg.security.jwt_keys.decoding,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        let claims = token_data.claims;
        claims.validate_access_token_auth()?;
        claims.validate_permissions_consistency(state)?;

        Ok(claims)
    }
}
