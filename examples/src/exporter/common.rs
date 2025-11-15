//! Common metrics definitions and helpers shared by exporter examples.
//!
//! This module centralizes the HTTP metrics (request count, duration histogram,
//! and in-flight gauge).

use std::time::Instant;

use fastmetrics::{
    encoder::EncodeLabelSet,
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{Histogram, exponential_buckets},
    },
    registry::Register,
};

/// Labels attached to HTTP request metrics.
/// `method` is canonical uppercase (GET/POST/PUT/DELETE/...) or "OTHER".
#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
pub struct HttpLabels {
    pub status: u16,
    pub method: &'static str,
}

/// Top-level metrics used by all HTTP exporter examples.
#[derive(Clone, Register)]
pub struct Metrics {
    /// Total number of HTTP requests received.
    #[register(rename = "http_requests_total", help = "Total number of HTTP requests")]
    http_requests: Family<HttpLabels, Counter>,

    /// Latency of HTTP requests in seconds (histogram).
    #[register(
        rename = "http_request_duration_seconds",
        unit(Seconds),
        help = "HTTP request latencies (seconds)"
    )]
    http_request_duration: Family<HttpLabels, Histogram>,

    /// Number of in-flight (currently processing) HTTP requests.
    #[register(rename = "http_requests_in_flight", help = "In-flight HTTP requests")]
    http_in_flight: Gauge,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            http_requests: Default::default(),
            http_request_duration: Family::<HttpLabels, Histogram>::new(|| {
                // Exponential buckets roughly spanning 1ms .. ~13s
                Histogram::new(exponential_buckets(0.001, 1.7, 15))
            }),
            http_in_flight: Gauge::default(),
        }
    }
}

impl Metrics {
    /// Observe a completed request.
    ///
    /// `method` must be a canonical label produced by one of the [`canonical_method_label`]
    /// function.
    pub fn observe(&self, method: impl AsRef<str>, status: u16, start: Instant) {
        let elapsed = start.elapsed();
        let labels = HttpLabels { status, method: canonical_method_label(method) };
        self.http_requests.with_or_new(&labels, |c| c.inc());
        self.http_request_duration
            .with_or_new(&labels, |h| h.observe(elapsed.as_secs_f64()));
    }

    /// Increment in-flight gauge.
    pub fn inc_in_flight(&self) {
        self.http_in_flight.inc();
    }

    /// Decrement in-flight gauge.
    pub fn dec_in_flight(&self) {
        self.http_in_flight.dec();
    }
}

/// Normalize a raw method string (e.g. from `Method::as_str()`) to canonical label.
fn canonical_method_label(method: impl AsRef<str>) -> &'static str {
    match method.as_ref() {
        "GET" => "GET",
        "POST" => "POST",
        "PUT" => "PUT",
        "DELETE" => "DELETE",
        "PATCH" => "PATCH",
        "HEAD" => "HEAD",
        "OPTIONS" => "OPTIONS",
        "CONNECT" => "CONNECT",
        "TRACE" => "TRACE",
        _ => "OTHER",
    }
}
