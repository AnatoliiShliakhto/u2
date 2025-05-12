use ::api_util::{prometheus, handler, Error};
use ::axum::{Router, middleware::from_fn, routing::get};
use ::axum_proxy::{TrimPrefix, TrimSuffix};
use ::std::sync::Arc;

pub async fn init_app(_state: &Arc<AppState>) -> Result<Router, Error> {
    let surrealdb_proxy =
        axum_proxy::builder_http("surrealdb:8000")?.build(TrimPrefix("/surrealdb"));
    let grafana_proxy = axum_proxy::builder_http("grafana:3000")?.build(TrimSuffix("/"));
    let auth_proxy = axum_proxy::builder_http("auth-svc:80")?.build(TrimPrefix("/auth"));
    let rabbitmq_proxy = axum_proxy::builder_http("rabbitmq:15672")?.build(TrimPrefix("/rabbitmq"));

    let router = Router::new()
        .route_service("/auth/{*path}", auth_proxy.clone())
        .route_service("/auth", auth_proxy)
        .route_service("/grafana/{*path}", grafana_proxy.clone())
        .route_service("/grafana/", grafana_proxy.clone())
        .route_service("/grafana", grafana_proxy)
        .route_service("/rabbitmq/{*path}", rabbitmq_proxy.clone())
        .route_service("/rabbitmq/", rabbitmq_proxy.clone())
        .route_service("/rabbitmq", rabbitmq_proxy)
        .route_service("/surrealdb/{*path}", surrealdb_proxy)
        .route("/healthcheck", get(handler::healthcheck))
        .route_layer(from_fn(prometheus::track_metrics));

    Ok(router)
}
