use crate::app::get_state;
use ::api_util::{Error, handler, prometheus};
use ::axum::{Router, middleware::from_fn, routing::get};
use ::axum_proxy::TrimPrefix;

use api_util::amqp::{AMQPMessageOptions, ExchangeKind};
use api_util::log::{error, info};
use axum::response::IntoResponse;

pub fn init_app() -> Router {
    let auth_proxy = axum_proxy::builder_http("auth")
        .expect("failed build auth proxy")
        .build(TrimPrefix("/auth"));

    Router::new()
        .route_service("/auth/{*path}", auth_proxy)
        .route("/broadcast-test", get(test))
        .route("/access-test", get(test1))
        .route("/auth-test", get(test2))
        .route("/logger-info", get(log))
        .route("/logger-error", get(log1))
        .route("/healthcheck", get(handler::healthcheck))
        .route_layer(from_fn(prometheus::track_metrics))
}

async fn test() -> Result<impl IntoResponse, Error> {
    get_state().amqp
        .send(
            ExchangeKind::Topic,
            "",
            AMQPMessageOptions::default()
                .app_id()
                .with_message_id("hello"),
            b"BROADCAST TEST",
        )
        .await?;

    Ok(())
}

async fn test1() -> Result<impl IntoResponse, Error> {
    get_state().amqp
        .send(
            ExchangeKind::Topic,
            "access.svc",
            AMQPMessageOptions::default(),
            b"hello from access service: self",
        )
        .await?;
    Ok(())
}

async fn test2() -> Result<impl IntoResponse, Error> {
    get_state().amqp
        .send(
            ExchangeKind::Topic,
            "auth.svc",
            AMQPMessageOptions::default(),
            b"hello from access service: auth",
        )
        .await?;
    Ok(())
}

async fn log() -> Result<impl IntoResponse, Error> {
    info!("Test logger: info");
    Ok(())
}

async fn log1() -> Result<impl IntoResponse, Error> {
    error!("Test logger: error");
    Ok(())
}
