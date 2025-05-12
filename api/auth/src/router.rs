use crate::app::*;
use ::api_util::{prometheus, handler};
use ::axum::{Router, middleware::from_fn, routing::get};
use ::std::sync::Arc;

pub async fn init_app(_state: &Arc<AppState>) -> Result<Router, Error> {
    let router = Router::new()
        .route("/healthcheck", get(handler::healthcheck))
        .route_layer(from_fn(prometheus::track_metrics));

    Ok(router)
}
