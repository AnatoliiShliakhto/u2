use ::axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ::serde::Serialize;
use ::serde_json::{Value, json};
use ::tracing::error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
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
pub struct ErrorBody {
    pub message: String,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub details: Value,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (code, message, details) = match self {
            Self::JsonRejection(err) => (
                StatusCode::BAD_REQUEST,
                err.to_string(),
                Value::Null,
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_string(), Value::Null),
        };
        
        error!("{message}");
        
        let body = json!({
            "message": message,
            "details": details,
        });
        
        (code, Json(body)).into_response()
    }
}
