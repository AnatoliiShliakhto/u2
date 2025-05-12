use ::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn healthcheck() -> Response {
    (StatusCode::OK, "OK").into_response()
}
