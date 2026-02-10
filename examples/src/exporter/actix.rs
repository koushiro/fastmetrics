use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Instant,
};

use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorInternalServerError,
    http::{StatusCode, header},
    web::{self, Data},
};
use anyhow::Result;
use fastmetrics::{
    derive::*,
    format::{prost, text},
    registry::{Register, Registry},
};
use futures::future::{LocalBoxFuture, Ready, ready};

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
}

struct MetricsLayer {
    metrics: Metrics,
}

impl MetricsLayer {
    fn new(metrics: Metrics) -> Self {
        Self { metrics }
    }
}

impl<S, B> Transform<S, ServiceRequest> for MetricsLayer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(MetricsService { inner: service, metrics: self.metrics.clone() }))
    }
}

struct MetricsService<S> {
    inner: S,
    metrics: Metrics,
}

impl<S, B> Service<ServiceRequest> for MetricsService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let metrics = self.metrics.clone();
        let method = req.method().clone();
        metrics.http.inc_in_flight();
        let start = Instant::now();

        let fut = self.inner.call(req);

        Box::pin(async move {
            let res = fut.await;
            match &res {
                Ok(sr) => metrics.http.observe(method, sr.response().status().as_u16(), start),
                Err(_) => {
                    let status = StatusCode::INTERNAL_SERVER_ERROR;
                    metrics.http.observe(method, status.as_u16(), start)
                },
            }
            metrics.http.dec_in_flight();
            res
        })
    }
}

async fn text_handler(state: Data<AppState>) -> Result<impl Responder, Error> {
    let mut output = String::new();
    let profile = text::TextProfile::Prometheus004;
    text::encode(&mut output, &state.registry, profile).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, profile.content_type()))
        .body(output))
}

async fn protobuf_handler(state: Data<AppState>) -> Result<impl Responder, Error> {
    let mut output = Vec::new();
    let profile = prost::ProtobufProfile::Prometheus;
    prost::encode(&mut output, &state.registry, profile).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, profile.content_type()))
        .body(output))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("actix").build()?;
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    println!("✅ Actix-Web metrics exporter listening on {addr}");
    println!("   GET /metrics");
    println!("   GET /metrics/text");
    println!("   GET /metrics/protobuf");

    let state = Data::new(AppState { registry: Arc::new(registry) });
    let app = {
        let metrics = metrics.clone();
        move || {
            App::new()
                .app_data(state.clone())
                .wrap(MetricsLayer::new(metrics.clone()))
                .route("/metrics", web::get().to(text_handler))
                .service(
                    web::scope("/metrics")
                        .route("/text", web::get().to(text_handler))
                        .route("/protobuf", web::get().to(protobuf_handler)),
                )
        }
    };

    HttpServer::new(app).bind(addr)?.run().await?;

    Ok(())
}
