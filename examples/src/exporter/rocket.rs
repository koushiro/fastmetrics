use std::{net::Ipv4Addr, sync::Arc, time::Instant};

use anyhow::Result;
use fastmetrics::{
    derive::*,
    format::{prost, text},
    registry::{Register, Registry},
};
use rocket::{
    Config, State,
    fairing::{Fairing, Info, Kind},
    http::{ContentType, Status},
    request::Request,
    response::Response,
};

#[path = "../metrics/mod.rs"]
mod metrics;

#[derive(Clone, Default, Register)]
pub struct Metrics {
    #[register(flatten)]
    pub http: metrics::http::HttpMetrics,

    #[register(subsystem = "process")]
    pub process: metrics::process::ProcessMetrics,
}

struct AppState {
    registry: Arc<Registry>,
    metrics: Metrics,
}

/// Rocket fairing that instruments requests.
struct MetricsFairing {
    metrics: Metrics,
}

impl MetricsFairing {
    fn new(metrics: Metrics) -> Self {
        Self { metrics }
    }
}

#[rocket::async_trait]
impl Fairing for MetricsFairing {
    fn info(&self) -> Info {
        Info { name: "Fastmetrics HTTP instrumentation", kind: Kind::Request | Kind::Response }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut rocket::Data<'_>) {
        // Save start time in request-local cache
        let _ = request.local_cache(Instant::now);
        self.metrics.http.inc_in_flight();
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let start = *request.local_cache(Instant::now);
        self.metrics
            .http
            .observe(request.method().as_str(), response.status().code, start);
        self.metrics.http.dec_in_flight();
    }
}

#[rocket::get("/metrics")]
async fn metrics_text(state: &State<AppState>) -> (Status, (ContentType, String)) {
    let mut output = String::new();
    let profile = text::TextProfile::PrometheusV0_0_4;
    if let Err(e) = text::encode(&mut output, &state.registry, profile) {
        return (
            Status::InternalServerError,
            (ContentType::Plain, format!("text encode error: {e}")),
        );
    }
    (
        Status::Ok,
        (ContentType::parse_flexible(profile.content_type()).unwrap_or(ContentType::Plain), output),
    )
}

#[rocket::get("/metrics/text")]
async fn metrics_text_explicit(state: &State<AppState>) -> (Status, (ContentType, String)) {
    metrics_text(state).await
}

#[rocket::get("/metrics/protobuf")]
async fn metrics_protobuf(state: &State<AppState>) -> (Status, (ContentType, Vec<u8>)) {
    let mut output = Vec::new();
    let profile = prost::ProtobufProfile::Prometheus;
    if let Err(e) = prost::encode(&mut output, &state.registry, profile) {
        return (
            Status::InternalServerError,
            (ContentType::Plain, format!("protobuf encode error: {e}").into_bytes()),
        );
    }
    (
        Status::Ok,
        (
            ContentType::parse_flexible(profile.content_type()).unwrap_or(ContentType::Binary),
            output,
        ),
    )
}

#[rocket::catch(404)]
fn not_found(req: &Request<'_>) -> (Status, String) {
    (Status::NotFound, format!("Not found: {}", req.uri()))
}

#[rocket::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("rocket").build()?;
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let ip_addr = Ipv4Addr::UNSPECIFIED.into();
    let port = 3000;
    println!("✅ Rocket metrics exporter listening on {ip_addr}:{port}");
    println!("   GET /metrics");
    println!("   GET /metrics/text");
    println!("   GET /metrics/protobuf");

    let state = AppState { registry: Arc::new(registry), metrics: metrics.clone() };
    let metrics = state.metrics.clone();

    rocket::custom(Config { address: ip_addr, port, ..Config::default() })
        .manage(state)
        .attach(MetricsFairing::new(metrics))
        .mount("/", rocket::routes![metrics_text, metrics_text_explicit, metrics_protobuf])
        .register("/", rocket::catchers![not_found])
        .launch()
        .await?;

    Ok(())
}
