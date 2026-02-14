use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Instant,
};

use anyhow::Result;
use fastmetrics::{
    derive::*,
    format::{prost, text},
    registry::{Register, Registry},
};
use warp::{
    Filter, Rejection, Reply,
    http::{Method, StatusCode, header},
};

#[path = "../metrics/mod.rs"]
mod metrics;
mod negotiation;

#[derive(Clone, Default, Register)]
pub struct Metrics {
    #[register(flatten)]
    pub http: metrics::http::HttpMetrics,

    #[register(subsystem = "process")]
    pub process: metrics::process::ProcessMetrics,
}

#[derive(Clone)]
struct AppState {
    registry: Arc<Registry>,
    metrics: Metrics,
}

#[derive(Debug)]
struct TextEncodeReject;
impl warp::reject::Reject for TextEncodeReject {}

fn metrics_encode_text(
    accept: Option<String>,
    state: AppState,
) -> Result<impl Reply, TextEncodeReject> {
    let mut output = String::new();
    let profile = negotiation::text_profile_from_accept(accept.as_deref());
    text::encode(&mut output, &state.registry, profile).map_err(|_| TextEncodeReject)?;
    Ok(warp::reply::with_header(
        warp::reply::with_status(output, StatusCode::OK),
        header::CONTENT_TYPE,
        profile.content_type(),
    ))
}

#[derive(Debug)]
struct ProtobufEncodeReject;
impl warp::reject::Reject for ProtobufEncodeReject {}

fn metrics_encode_protobuf(state: AppState) -> Result<impl Reply, ProtobufEncodeReject> {
    let mut output = Vec::new();
    let profile = prost::ProtobufProfile::Prometheus;
    prost::encode(&mut output, &state.registry, profile).map_err(|_| ProtobufEncodeReject)?;
    Ok(warp::reply::with_header(
        warp::reply::with_status(output, StatusCode::OK),
        header::CONTENT_TYPE,
        profile.content_type(),
    ))
}

async fn text_endpoint(
    method: Method,
    accept: Option<String>,
    state: AppState,
) -> Result<impl Reply, Rejection> {
    let start = Instant::now();
    state.metrics.http.inc_in_flight();
    let result = metrics_encode_text(accept, state.clone());
    match result {
        Ok(reply) => {
            state.metrics.http.observe(method, StatusCode::OK.as_u16(), start);
            state.metrics.http.dec_in_flight();
            Ok(reply)
        },
        Err(err) => {
            state
                .metrics
                .http
                .observe(method, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), start);
            state.metrics.http.dec_in_flight();
            Err(warp::reject::custom(err))
        },
    }
}

async fn protobuf_endpoint(method: Method, state: AppState) -> Result<impl Reply, Rejection> {
    let start = Instant::now();
    state.metrics.http.inc_in_flight();
    let result = metrics_encode_protobuf(state.clone());
    match result {
        Ok(reply) => {
            state.metrics.http.observe(method, StatusCode::OK.as_u16(), start);
            state.metrics.http.dec_in_flight();
            Ok(reply)
        },
        Err(err) => {
            state
                .metrics
                .http
                .observe(method, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), start);
            state.metrics.http.dec_in_flight();
            Err(warp::reject::custom(err))
        },
    }
}

async fn not_found_endpoint(
    path: warp::path::FullPath,
    method: Method,
    state: AppState,
) -> Result<impl Reply, Rejection> {
    let start = Instant::now();
    state.metrics.http.inc_in_flight();

    let reply = format!("Not found: {}", path.as_str());
    let reply = warp::reply::with_status(reply, StatusCode::NOT_FOUND);

    state.metrics.http.observe(method, StatusCode::NOT_FOUND.as_u16(), start);
    state.metrics.http.dec_in_flight();
    Ok(reply)
}

fn build_filters(
    state: AppState,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // /metrics (text)
    let metrics_text_route = warp::path!("metrics")
        .and(warp::method())
        .and(warp::header::optional::<String>("accept"))
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and_then(|method, accept, state| text_endpoint(method, accept, state));

    // /metrics/text
    let metrics_text_explicit = warp::path!("metrics" / "text")
        .and(warp::method())
        .and(warp::header::optional::<String>("accept"))
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and_then(|method, accept, state| text_endpoint(method, accept, state));

    // /metrics/protobuf
    let metrics_protobuf_route = warp::path!("metrics" / "protobuf")
        .and(warp::method())
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and_then(protobuf_endpoint);

    // Not found catch-all
    let not_found = warp::path::full()
        .and(warp::method())
        .and(warp::any().map({
            let state = state.clone();
            move || state.clone()
        }))
        .and_then(not_found_endpoint);

    metrics_text_route
        .or(metrics_text_explicit)
        .or(metrics_protobuf_route)
        .or(not_found)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("warp").build()?;
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    println!("✅ Warp metrics exporter listening on {addr}");
    println!("   GET /metrics");
    println!("   GET /metrics/text");
    println!("   GET /metrics/protobuf");

    let state = AppState { registry: Arc::new(registry), metrics: metrics.clone() };
    let filters = build_filters(state);

    warp::serve(filters).run(addr).await;

    Ok(())
}
