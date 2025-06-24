#![allow(dead_code)]
use crate::{app::get_state, middleware::Auth, model::Capabilities};
use ::api_util::{AuthError, Error, env};
use ::axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use ::axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use ::chrono::{Duration, Utc};
use ::jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use ::serde::{Deserialize, Serialize};
use ::std::{borrow::Cow, sync::LazyLock};

static JWT_ACCESS_EXPIRATION: LazyLock<usize> = LazyLock::new(|| {
    env::get_var_or_default("JWT_ACCESS_EXPIRATION", "600")
        .parse()
        .unwrap_or(600)
});

#[derive(Serialize, Deserialize)]
pub struct Claims {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    pub iat: usize,
    pub exp: usize,
    #[serde(
        rename = "sid",
        skip_serializing_if = "Option::is_none",
        serialize_with = "Auth::serialize",
        deserialize_with = "Auth::deserialize",
        default
    )]
    pub auth: Option<Auth>,
}

impl Claims {
    pub fn new() -> Self {
        let timestamp_now = Utc::now().timestamp() as usize;
        Self {
            iss: None,
            sub: None,
            jti: None,
            iat: timestamp_now,
            exp: timestamp_now + *JWT_ACCESS_EXPIRATION,
            auth: None,
        }
    }

    pub fn with_issuer(mut self, iss: &str) -> Self {
        if !iss.is_empty() {
            self.iss = Some(iss.to_string());
        }
        self
    }

    pub fn with_subject(mut self, sub: &str) -> Self {
        if !sub.is_empty() {
            self.sub = Some(sub.to_string());
        }
        self
    }

    pub fn with_jti(mut self, jti: &str) -> Self {
        if !jti.is_empty() {
            self.jti = Some(jti.to_string());
        }
        self
    }

    pub fn with_expiration(mut self, timestamp: usize) -> Self {
        self.exp = timestamp;
        self
    }

    pub fn with_expiration_in_seconds(mut self, seconds: i64) -> Self {
        self.exp = (Utc::now() + Duration::seconds(seconds)).timestamp() as usize;
        self
    }

    pub fn with_expiration_in_minutes(mut self, minutes: i64) -> Self {
        self.exp = (Utc::now() + Duration::minutes(minutes)).timestamp() as usize;
        self
    }

    pub fn with_expiration_in_hours(mut self, hours: i64) -> Self {
        self.exp = (Utc::now() + Duration::hours(hours)).timestamp() as usize;
        self
    }

    pub fn with_expiration_in_days(mut self, days: i64) -> Self {
        self.exp = (Utc::now() + Duration::days(days)).timestamp() as usize;
        self
    }

    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn is_expired(&self) -> bool {
        self.exp < Utc::now().timestamp() as usize
    }

    pub fn build_token(&self, encoding_key: &EncodingKey) -> Result<Cow<'static, str>, AuthError> {
        encode(&Header::default(), self, encoding_key)
            .map_err(|_| AuthError::TokenCreation)
            .map(|v| Ok(Cow::Owned(v)))?
    }

    pub fn from_refresh_token(token: &str, decoding_key: &DecodingKey) -> Result<Self, AuthError> {
        let token_data = decode::<Claims>(token, decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.jti.is_none() {
            Err(AuthError::InvalidToken)?
        }

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
        let Some(auth) = &self.auth else {
            Err(AuthError::AccessForbidden)?
        };

        if auth
            .permissions
            .get_or_default(index)
            .contains(capabilities)
        {
            Ok(())
        } else {
            Err(AuthError::AccessForbidden)?
        }
    }
}

impl Default for Claims {
    fn default() -> Self {
        let timestamp_now = Utc::now().timestamp() as usize;

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

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync + Clone,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let state = get_state();

        let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        else {
            Err(AuthError::MissingToken)?
        };

        let token_data = decode::<Claims>(
            bearer.token(),
            &state.cfg.security.jwt_keys.decoding,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.auth.is_none() {
            Err(AuthError::InvalidToken)?
        }

        if let Some(auth) = &token_data.claims.auth {
            if state.permissions_map.read().await.len() != auth.permissions.len() {
                Err(AuthError::InvalidToken)?
            }
        }

        Ok(token_data.claims)
    }
}
