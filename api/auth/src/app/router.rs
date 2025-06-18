use ::api_util::{handler, prometheus};
use ::axum::{Router, middleware::from_fn, routing::get};

pub fn init_app() -> Router {
    Router::new()
        .route("/healthcheck", get(handler::healthcheck))
        .route_layer(from_fn(prometheus::track_metrics))
}
