use std::{
    future::Future,
    io,
    net::{Ipv4Addr, SocketAddr},
    pin::Pin,
    sync::Arc,
    task::{ready, Context, Poll},
    time::{Duration, Instant},
};

use anyhow::Result;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, Router},
    ServiceExt,
};
use fastmetrics::{
    encoder::EncodeLabelSet,
    format::{prost, text},
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::{Register, Registry},
};
use pin_project::pin_project;
use tokio::net::TcpListener;
use tower::{Layer, Service};
use tower_http::normalize_path::NormalizePathLayer;

#[derive(Clone, Default, Register)]
pub struct Metrics {
    /// Total number of HTTP requests
    http_requests: Family<RequestsLabels, Counter>,
    /// Duration of HTTP request
    #[register(unit(Seconds))]
    http_request_duration: Family<RequestsLabels, Histogram>,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
pub struct RequestsLabels {
    pub status: u16,
}

impl Metrics {
    pub fn observe(&self, status: StatusCode, duration: Duration) {
        let labels = RequestsLabels { status: status.as_u16() };
        self.http_requests.with_or_new(&labels, |req| req.inc());
        self.http_request_duration
            .with_or_new(&labels, |hist| hist.observe(duration.as_secs_f64()))
    }
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
        let inner_future = self.inner.call(req);
        MetricsFuture { inner: inner_future, start, metrics: self.metrics.clone() }
    }
}

#[pin_project]
struct MetricsFuture<F> {
    #[pin]
    inner: F,
    start: Instant,
    metrics: Metrics,
}

impl<F, ResBody, E> Future for MetricsFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let duration = this.start.elapsed();

        let response = ready!(this.inner.poll(cx))?;
        this.metrics.observe(response.status(), duration);

        Poll::Ready(Ok(response))
    }
}

#[derive(Clone)]
struct AppState {
    registry: Arc<Registry>,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
enum AppError {
    #[error("prometheus encode error: {0}")]
    WriteFmt(#[from] std::fmt::Error),
    #[error("protobuf encode error: {0}")]
    ProtobufEncode(#[from] io::Error),
    #[error("{0}")]
    Http(#[from] axum::http::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_msg = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, error_msg).into_response()
    }
}

async fn text_handler(state: State<AppState>) -> Result<Response, AppError> {
    let mut output = String::new();
    text::encode(&mut output, &state.registry)?;

    let response = Response::builder().status(StatusCode::OK).body(Body::from(output))?;
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
    let mut registry = Registry::builder().with_namespace("axum").build();

    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    let listener = TcpListener::bind(addr).await?;
    println!("✅ Axum server is listening on: {addr}");

    let state = AppState { registry: Arc::new(registry) };
    let router = Router::new()
        .route("/metrics", get(text_handler))
        .nest(
            "/metrics",
            Router::new()
                .route("/text", get(text_handler))
                .route("/protobuf", get(protobuf_handler)),
        )
        .route("/{other}", get(not_found_handler))
        .with_state(state)
        .layer(MetricsLayer { metrics });

    let app = {
        let normalized = NormalizePathLayer::trim_trailing_slash().layer(router);
        ServiceExt::<Request>::into_make_service(normalized)
    };

    axum::serve(listener, app).await?;
    Ok(())
}
