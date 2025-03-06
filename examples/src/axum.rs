use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Result;
use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use openmetrics_client::{
    encoder::{EncodeLabelSet, EncodeLabelValue},
    format::{protobuf, text},
    metrics::{counter::Counter, family::Family},
    registry::Registry,
};
use tokio::net::TcpListener;

#[derive(Clone, Default)]
pub struct Metrics {
    http_requests: Family<RequestsLabels, Counter>,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
pub struct RequestsLabels {
    pub method: Method,
    pub status: u16,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
pub enum Method {
    Get,
    Post,
}

impl Metrics {
    pub fn observe_http_req(&self, method: Method, status: StatusCode) {
        let labels = RequestsLabels { method, status: status.as_u16() };
        self.http_requests.with_or_default(&labels, |req| req.inc());
    }

    pub fn observe_http_get_req(&self, status: StatusCode) {
        self.observe_http_req(Method::Get, status);
    }
}

#[derive(Clone)]
struct AppState {
    registry: Arc<Registry>,
    metrics: Metrics,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
enum AppError {
    #[error("prometheus encode error: {0}")]
    WriteFmt(#[from] std::fmt::Error),
    #[error("protobuf encode error: {0}")]
    ProtobufEncode(#[from] protobuf::EncodeError),
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

    let status = StatusCode::OK;
    state.metrics.observe_http_get_req(status);

    Ok(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/openmetrics-text; version=1.0.0; charset=utf-8")
        .body(Body::from(output))?)
}

async fn protobuf_handler(state: State<AppState>) -> Result<Response, AppError> {
    let mut output = Vec::new();
    protobuf::encode(&mut output, &state.registry)?;

    let status = StatusCode::OK;
    state.metrics.observe_http_get_req(status);

    Ok(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/openmetrics-protobuf; version=1.0.0; charset=utf-8")
        .body(Body::from(output))?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("axum").build();

    let metrics = Metrics::default();

    // Register metrics
    registry.register(
        "http_requests",
        "Total number of HTTP requests",
        metrics.http_requests.clone(),
    )?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    let listener = TcpListener::bind(addr).await?;
    println!("✅ Axum server is listening on: {addr}");

    let state = AppState { registry: Arc::new(registry), metrics };
    let router = Router::new()
        .route("/metrics", get(text_handler))
        .nest(
            "/metrics",
            Router::new()
                .route("/text", get(text_handler))
                .route("/protobuf", get(protobuf_handler)),
        )
        .route(
            "/{other}",
            get(|state: State<AppState>| async move {
                let status = StatusCode::NOT_FOUND;
                state.metrics.observe_http_get_req(status);
                status
            }),
        )
        .with_state(state);

    axum::serve(listener, router).await?;
    Ok(())
}
