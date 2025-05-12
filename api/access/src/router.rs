use crate::state::AppState;
use ::api_util::{Error, handler, prometheus};
use ::axum::{Router, middleware::from_fn, routing::get};
use ::axum_proxy::{TrimPrefix, TrimSuffix};
use ::std::sync::Arc;

use api_util::logger::{error, info};
use axum::extract::State;
use axum::response::IntoResponse;

pub async fn init_app(state: &Arc<AppState>) -> Result<Router, Error> {
    let surrealdb_proxy =
        axum_proxy::builder_http("surrealdb:8000")?.build(TrimPrefix("/surrealdb"));
    let grafana_proxy = axum_proxy::builder_http("grafana:3000")?.build(TrimSuffix("/"));
    let auth_proxy = axum_proxy::builder_http("auth")?.build(TrimPrefix("/auth"));

    let router = Router::new()
        .route_service("/auth/{*path}", auth_proxy)
        .route_service("/grafana/{*path}", grafana_proxy.clone())
        .route_service("/grafana/", grafana_proxy.clone())
        .route_service("/grafana", grafana_proxy)
        .route_service("/surrealdb/{*path}", surrealdb_proxy.clone())
        .route_service("/surrealdb", surrealdb_proxy)
        .route("/broadcast-test", get(test))
        .route("/access-test", get(test1))
        .route("/auth-test", get(test2))
        .route("/logger-info", get(log))
        .route("/logger-error", get(log1))
        .route("/healthcheck", get(handler::healthcheck))
        .with_state(state.clone())
        .route_layer(from_fn(prometheus::track_metrics));

    Ok(router)
}

async fn test(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    state
        .amqp
        .broadcast("hello", b"hello from access service: broadcast")
        .await?;

    Ok(())
}

async fn test1(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    state
        .amqp
        .send("access", "hello", b"hello from access service: self")
        .await?;
    Ok(())
}

async fn test2(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    state
        .amqp
        .send("auth", "hello", b"hello from access service: auth")
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
