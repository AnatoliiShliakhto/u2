use ::axum::http::StatusCode;
use ::serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Access forbidden")]
    AccessForbidden,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Unauthorized")]
    Unauthorized,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        let status_code = match self {
            Self::Unauthorized
            | Self::WrongCredentials
            | Self::InvalidToken
            | Self::MissingToken => StatusCode::UNAUTHORIZED,
            Self::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AccessForbidden => StatusCode::FORBIDDEN,
        };

        status_code
    }
}
