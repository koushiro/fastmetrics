use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Instant,
};

use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorInternalServerError,
    http::StatusCode,
    web::{self, Data},
};
use anyhow::Result;
use fastmetrics::{
    format::{prost, text},
    registry::{Register, Registry},
};
use futures::future::{LocalBoxFuture, Ready, ready};

mod common;
use self::common::{Metrics, canonical_method_label};

struct AppState {
    registry: Arc<Registry>,
    _metrics: Metrics,
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
        metrics.inc_in_flight();
        let start = Instant::now();

        let fut = self.inner.call(req);

        Box::pin(async move {
            let res = fut.await;
            match &res {
                Ok(sr) => metrics.observe(
                    canonical_method_label(method),
                    sr.response().status().as_u16(),
                    start,
                ),
                Err(_) => metrics.observe(
                    canonical_method_label(method),
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    start,
                ),
            }
            metrics.dec_in_flight();
            res
        })
    }
}

async fn text_handler(state: Data<AppState>) -> Result<impl Responder, Error> {
    let mut output = String::new();
    text::encode(&mut output, &state.registry).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(output))
}

async fn protobuf_handler(state: Data<AppState>) -> Result<impl Responder, Error> {
    let mut output = Vec::new();
    prost::encode(&mut output, &state.registry).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(output))
}

#[actix_web::main]
async fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("actix").build();
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 3000);
    println!("✅ Actix-Web metrics exporter listening on {addr}");
    println!("   GET /metrics");
    println!("   GET /metrics/text");
    println!("   GET /metrics/protobuf");

    let state = Data::new(AppState { registry: Arc::new(registry), _metrics: metrics.clone() });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(MetricsLayer::new(metrics.clone()))
            .route("/metrics", web::get().to(text_handler))
            .service(
                web::scope("/metrics")
                    .route("/text", web::get().to(text_handler))
                    .route("/protobuf", web::get().to(protobuf_handler)),
            )
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
