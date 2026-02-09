use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, ready},
    time::Instant,
};

use anyhow::Result;
use axum::{
    ServiceExt,
    body::Body,
    extract::{Request, State},
    http::{Method, StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::{Router, get},
};
use fastmetrics::{
    derive::*,
    format::{prost, text},
    registry::{Register, Registry},
};
use pin_project::pin_project;
use tokio::net::TcpListener;
use tower::{Layer, Service};
use tower_http::normalize_path::NormalizePathLayer;

#[path = "../metrics/mod.rs"]
mod metrics;

#[derive(Clone, Default, Register)]
pub struct Metrics {
    #[register(flatten)]
    pub http: metrics::http::HttpMetrics,

    #[register(subsystem = "process")]
    pub process: metrics::process::ProcessMetrics,
}

#[derive(Clone)]
struct MetricsLayer {
    metrics: Metrics,
}

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MetricsService { inner, metrics: self.metrics.clone() }
    }
}

#[derive(Clone)]
struct MetricsService<S> {
    inner: S,
    metrics: Metrics,
}

impl<S, R, ResBody> Service<Request<R>> for MetricsService<S>
where
    S: Service<Request<R>, Response = Response<ResBody>>,
{
    type Response = Response<ResBody>;
    type Error = S::Error;
    type Future = MetricsFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<R>) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        self.metrics.http.inc_in_flight();
        let inner_future = self.inner.call(req);
        MetricsFuture {
            inner: inner_future,
            start,
            metrics: self.metrics.clone(),
            method,
            done: false,
        }
    }
}

#[pin_project(PinnedDrop)]
struct MetricsFuture<F> {
    #[pin]
    inner: F,
    start: Instant,
    metrics: Metrics,
    method: Method,
    done: bool,
}

impl<F, ResBody, E> Future for MetricsFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let result = ready!(this.inner.poll(cx));
        match result {
            Ok(ref response) => {
                this.metrics.http.observe(this.method, response.status().as_u16(), *this.start);
            },
            Err(_) => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                this.metrics.http.observe(this.method, status.as_u16(), *this.start);
            },
        }

        this.metrics.http.dec_in_flight();
        *this.done = true;
        Poll::Ready(result)
    }
}

#[pin_project::pinned_drop]
impl<F> PinnedDrop for MetricsFuture<F> {
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();
        if !*this.done {
            this.metrics.http.dec_in_flight();
        }
    }
}

#[derive(Clone)]
struct AppState {
    registry: Arc<Registry>,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
enum AppError {
    #[error("metrics encode error: {0}")]
    Encode(#[from] fastmetrics::error::Error),
    #[error("{0}")]
    Http(#[from] axum::http::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

async fn text_handler(state: State<AppState>) -> Result<Response, AppError> {
    let mut output = String::new();
    let profile = text::TextProfile::Prometheus004;
    text::encode_profile(&mut output, &state.registry, profile)?;
    let response = Response::builder()
        .header(header::CONTENT_TYPE, profile.content_type())
        .status(StatusCode::OK)
        .body(Body::from(output))?;
    Ok(response)
}

async fn protobuf_handler(state: State<AppState>) -> Result<Response, AppError> {
    let mut output = Vec::new();
    prost::encode(&mut output, &state.registry)?;
    let response = Response::builder().status(StatusCode::OK).body(Body::from(output))?;
    Ok(response)
}

async fn not_found_handler(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Not found: {}", uri.path()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("axum").build()?;
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    println!("✅ Axum metrics exporter listening on {addr}");
    println!("   GET /metrics");
    println!("   GET /metrics/text");
    println!("   GET /metrics/protobuf");

    let listener = TcpListener::bind(addr).await?;
    let app = {
        let router = Router::new()
            .route("/metrics", get(text_handler))
            .nest(
                "/metrics",
                Router::new()
                    .route("/text", get(text_handler))
                    .route("/protobuf", get(protobuf_handler)),
            )
            .route("/{other}", get(not_found_handler))
            .with_state(AppState { registry: Arc::new(registry) })
            .layer(MetricsLayer { metrics });

        // Normalize paths (trim trailing slashes)
        let normalized = NormalizePathLayer::trim_trailing_slash().layer(router);
        ServiceExt::<Request>::into_make_service(normalized)
    };

    axum::serve(listener, app).await?;

    Ok(())
}
