use ::api_util::{env, handler, prometheus};
use ::axum::{Router, extract::DefaultBodyLimit, http::Method, middleware::from_fn, routing::get};
use ::axum_reverse_proxy::{RetryLayer, ReverseProxy};
use ::tower::ServiceBuilder;
use ::tower_http::{compression::CompressionLayer, cors::CorsLayer};

pub fn init_app() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin(["*".parse().unwrap()])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_credentials(true);

    let compression_layer = CompressionLayer::new().br(true).gzip(true).zstd(true);

    Router::new()
        .merge(ReverseProxy::new(
            "/",
            env::get_var_or_default("ACCESS_URL", "http://access:80"),
        ))
        .layer(ServiceBuilder::new().layer(RetryLayer::new(3)))
        .route("/healthcheck", get(handler::healthcheck))
        .route_layer(from_fn(prometheus::track_metrics))
        .layer(compression_layer)
        .layer(cors_layer)
        .layer(DefaultBodyLimit::max(104_857_600))
}
