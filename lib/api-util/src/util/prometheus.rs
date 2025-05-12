use ::axum::{
    Router,
    extract::{MatchedPath, Request},
    middleware::Next,
    response::IntoResponse,
    routing::get,
};
use ::axum_server::Handle;
use ::metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use ::std::{
    future::ready,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Instant,
};

fn metrics_app() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

pub async fn start_metrics_server(host: &str, shutdown_handle: Option<Handle>) {
    let mut metric_srv = axum_server::bind(host.parse().unwrap_or(SocketAddr::V4(
        SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3001),
    )));

    if let Some(handler) = shutdown_handle {
        metric_srv = metric_srv.handle(handler)
    }

    metric_srv
        .serve(metrics_app().into_make_service())
        .await
        .expect("failed to start metrics server");
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

/// Tracks metrics for HTTP requests including the method, path, status, and duration.
///
/// This function is used as middleware in an async web framework to collect
/// metrics for each incoming HTTP request and response. It records the total
/// number of requests (`http_requests_total`) and the request duration
/// (`http_requests_duration_seconds`) using a metrics lib.
///
/// # Metrics
///
/// - `http_requests_total`: A counter representing the total number of HTTP requests processed, labeled by
///   `method`, `path`, and `status`.
/// - `http_requests_duration_seconds`: A histogram that measures the duration of HTTP requests in seconds,
///   labeled by `method`, `path`, and `status`.
///
/// # Example
///
/// ```
/// use ::axum::{middleware, Router};
/// use ::api_util::prometheus::{start_metrics_server, track_metrics};
///
/// let app = Router::new()
///     .route("/", axum::routing::get(|| async { "Hello, World!" }))
///     .layer(middleware::from_fn(track_metrics));
/// ```
/// The `/metrics` endpoint should not be publicly available. If behind a reverse access, this
///  can be achieved by rejecting requests to `/metrics`. In this example, a second server is
///  started on another port to expose `/metrics`.
///
///  Let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrics_server());
pub async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    response
}
