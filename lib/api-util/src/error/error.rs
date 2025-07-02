use ::axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ::serde::Serialize;
use ::serde_json::Value;
use ::tracing::error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error[transparent]]
    AuthError(#[from] super::AuthError),
    #[error[transparent]]
    IoError(#[from] std::io::Error),
    #[error[transparent]]
    HttpError(#[from] axum::http::Error),
    #[error[transparent]]
    SerdeJsonError(#[from] serde_json::Error),
    #[error[transparent]]
    DatabaseError(#[from] surrealdb::Error),
    #[error("{0}")]
    Amqp(String),
    #[error[transparent]]
    JsonRejection(#[from] JsonRejection),
    #[error("{0}")]
    Unknown(&'static str),
}

#[derive(Serialize)]
pub struct ErrorBody<'a> {
    pub message: &'a str,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub details: Value,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{self:?}");

        let (code, message, details) = match self {
            Self::AuthError(err) => (err.status_code(), &*err.to_string(), Value::Null),
            Self::JsonRejection(err) => (StatusCode::BAD_REQUEST, &*err.to_string(), Value::Null),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong",
                Value::Null,
            ),
        };

        let body = ErrorBody { message, details };

        (code, Json(body)).into_response()
    }
}
