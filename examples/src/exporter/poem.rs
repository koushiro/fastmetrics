use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Instant,
};

use anyhow::Result;
use fastmetrics::{
    format::{prost, text},
    registry::{Register, Registry},
};
use poem::{
    EndpointExt, Request, Response, Route, Server, get, handler, http::StatusCode,
    listener::TcpListener, web::Data,
};

mod common;
use self::common::{Metrics, canonical_method_label};

// Metrics moved to common.rs (shared across exporter examples)

#[derive(Clone)]
struct AppState {
    registry: Arc<Registry>,
    metrics: Metrics,
}

// Instrumentation is now done directly inside handler functions for better
// compatibility with the latest poem APIs.

#[handler]
async fn metrics_text(req: &Request, Data(state): Data<&AppState>) -> Response {
    let start = Instant::now();
    state.metrics.inc_in_flight();

    let mut output = String::new();
    let result = text::encode(&mut output, &state.registry);

    match result {
        Ok(()) => {
            let status = StatusCode::OK;
            let body = output;

            state
                .metrics
                .observe(canonical_method_label(req.method()), status.as_u16(), start);
            state.metrics.dec_in_flight();

            Response::builder().status(status).body(body)
        },
        Err(e) => {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let body = format!("text encode error: {e}");

            state
                .metrics
                .observe(canonical_method_label(req.method()), status.as_u16(), start);
            state.metrics.dec_in_flight();

            Response::builder().status(status).body(body)
        },
    }
}

#[handler]
async fn metrics_protobuf(req: &Request, Data(state): Data<&AppState>) -> Response {
    let start = Instant::now();
    state.metrics.inc_in_flight();

    let mut output = Vec::new();
    let result = prost::encode(&mut output, &state.registry);

    match result {
        Ok(()) => {
            let status = StatusCode::OK;
            let body = output;

            state
                .metrics
                .observe(canonical_method_label(req.method()), status.as_u16(), start);
            state.metrics.dec_in_flight();

            Response::builder().status(status).body(body)
        },
        Err(e) => {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let body = format!("protobuf encode error: {e}");

            state
                .metrics
                .observe(canonical_method_label(req.method()), status.as_u16(), start);
            state.metrics.dec_in_flight();

            Response::builder().status(status).body(body)
        },
    }
}

#[handler]
async fn not_found(req: &Request, Data(state): Data<&AppState>) -> Response {
    let start = Instant::now();
    state.metrics.inc_in_flight();

    let status = StatusCode::NOT_FOUND;
    let body = format!("Not found: {}", req.uri().path());

    state
        .metrics
        .observe(canonical_method_label(req.method()), status.as_u16(), start);
    state.metrics.dec_in_flight();

    Response::builder().status(status).body(body)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("poem").build();
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3002);
    println!("✅ Poem metrics exporter listening on {addr}");
    println!("   GET /metrics");
    println!("   GET /metrics/text");
    println!("   GET /metrics/protobuf");

    let listener = TcpListener::bind(addr);
    let app = {
        let state = AppState { registry: Arc::new(registry), metrics: metrics.clone() };
        Route::new()
            .at("/metrics", get(metrics_text))
            .at("/metrics/text", get(metrics_text))
            .at("/metrics/protobuf", get(metrics_protobuf))
            .at("/*path", get(not_found))
            .data(state.clone())
    };

    Server::new(listener).run(app).await?;

    Ok(())
}
