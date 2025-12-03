use std::{
    convert::Infallible,
    fmt, io,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Instant,
};

use anyhow::Result;
use bytes::Bytes;
use fastmetrics::{
    format::{prost, text},
    registry::{Register, Registry},
};
use http_body_util::Full;
use hyper::{
    Method, Request, Response, StatusCode, body::Incoming, http, server::conn::http1,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};

mod common;
use self::common::Metrics;

type MetricsBody = Full<Bytes>;
type MetricsResponse = Response<MetricsBody>;

#[derive(Clone)]
struct AppState {
    registry: Arc<Registry>,
    metrics: Metrics,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("text encode error: {0}")]
    TextEncode(#[from] fmt::Error),
    #[error("protobuf encode error: {0}")]
    ProtobufEncode(#[from] io::Error),
    #[error("http response error: {0}")]
    Http(#[from] http::Error),
}

impl AppError {
    fn into_response(self) -> MetricsResponse {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Full::new(Bytes::from(self.to_string())))
            .expect("fallback response build should succeed")
    }
}

fn text_response(state: &AppState) -> Result<MetricsResponse, AppError> {
    let mut output = String::new();
    text::encode(&mut output, &state.registry)?;
    let body = Full::new(Bytes::from(output));

    Ok(Response::builder().status(StatusCode::OK).body(body)?)
}

fn protobuf_response(state: &AppState) -> Result<MetricsResponse, AppError> {
    let mut output = Vec::new();
    prost::encode(&mut output, &state.registry)?;
    let body = Full::new(Bytes::from(output));

    Ok(Response::builder().status(StatusCode::OK).body(body)?)
}

fn not_found_response(path: &str) -> Result<MetricsResponse, AppError> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from(format!("Not found: {path}"))))?)
}

enum MetricsRoute<'a> {
    Text,
    Protobuf,
    NotFound(&'a str),
}

fn classify_route<'a>(method: &'a Method, path: &'a str) -> MetricsRoute<'a> {
    if method != Method::GET {
        return MetricsRoute::NotFound(path);
    }

    match path {
        "/metrics" | "/metrics/text" => MetricsRoute::Text,
        "/metrics/protobuf" => MetricsRoute::Protobuf,
        _ => MetricsRoute::NotFound(path),
    }
}

async fn route_request(
    req: &Request<Incoming>,
    state: &AppState,
) -> Result<MetricsResponse, AppError> {
    match classify_route(req.method(), req.uri().path()) {
        MetricsRoute::Text => text_response(state),
        MetricsRoute::Protobuf => protobuf_response(state),
        MetricsRoute::NotFound(path) => not_found_response(path),
    }
}

async fn serve_http1(stream: TcpStream, state: AppState) -> Result<(), hyper::Error> {
    let _ = stream.set_nodelay(true);
    let io = TokioIo::new(stream);
    let service_state = state.clone();

    let service = service_fn(move |req: Request<Incoming>| {
        let state = service_state.clone();
        async move {
            let method = req.method().clone();
            let start = Instant::now();

            state.metrics.inc_in_flight();
            let response = match route_request(&req, &state).await {
                Ok(resp) => resp,
                Err(err) => err.into_response(),
            };
            state.metrics.observe(method, response.status().as_u16(), start);
            state.metrics.dec_in_flight();

            Ok::<_, Infallible>(response)
        }
    });

    http1::Builder::new().serve_connection(io, service).await
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("hyper").build();
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    println!("✅ Hyper metrics exporter listening on {addr}");
    println!("   GET /metrics");
    println!("   GET /metrics/text");
    println!("   GET /metrics/protobuf");

    let listener = TcpListener::bind(addr).await?;
    let state = AppState { registry: Arc::new(registry), metrics };

    loop {
        let (stream, _) = listener.accept().await?;
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(err) = serve_http1(stream, state).await {
                eprintln!("hyper connection error: {err}");
            }
        });
    }
}
