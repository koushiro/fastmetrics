use std::time::Instant;

use fastmetrics::{
    derive::*,
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{Histogram, exponential_buckets},
    },
};

/// Labels attached to HTTP request metrics.
/// `method` is canonical uppercase (GET/POST/PUT/DELETE/...) or "OTHER".
#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet, LabelSetSchema)]
pub struct HttpLabels {
    pub status: u16,
    pub method: &'static str,
}

/// HTTP metrics used by all HTTP exporter examples.
#[derive(Clone, Register)]
pub struct HttpMetrics {
    /// Total number of HTTP requests received.
    #[register(rename = "http_requests_total")]
    http_requests: Family<HttpLabels, Counter>,

    /// Latency of HTTP requests in seconds (histogram).
    #[register(rename = "http_request_duration_seconds", unit(Seconds))]
    http_request_duration: Family<HttpLabels, Histogram>,

    /// Number of in-flight (currently processing) HTTP requests.
    #[register(rename = "http_requests_in_flight")]
    http_in_flight: Gauge,
}

impl Default for HttpMetrics {
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

impl HttpMetrics {
    /// Observe a completed request.
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
