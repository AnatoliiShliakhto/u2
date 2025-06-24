use crate::controller::auth;
use ::api_util::{handler, prometheus};
use ::axum::{Router, middleware::from_fn, routing::get};
use ::axum_reverse_proxy::ReverseProxy;

pub fn init_app() -> Router {
    Router::new()
        .merge(ReverseProxy::new("/api/audit", "http://audit:80"))
        .route("/api/auth/token", get(auth::token))
        .route("/api/auth", get(auth::authorize).delete(auth::revoke))
        .route("/healthcheck", get(handler::healthcheck))
        .route_layer(from_fn(prometheus::track_metrics))
}
