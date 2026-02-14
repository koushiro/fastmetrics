use std::sync::LazyLock;

use anyhow::Result;
use fastmetrics::{
    derive::*,
    format::text::{self, TextProfile},
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{Histogram, exponential_buckets},
    },
    registry::*,
};
use rand::RngExt;

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet, LabelSetSchema)]
struct HttpLabels {
    method: HttpMethod,
    status: u16,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Register)]
struct Metrics {
    /// Total number of HTTP requests
    http_requests_total: Family<HttpLabels, Counter>,
    /// Duration of HTTP requests
    #[register(unit(Seconds))]
    http_request_duration: Family<HttpLabels, Histogram>,
    /// Number of active connections
    active_connections: Gauge,
    /// Current cache size in bytes
    #[register(unit(Bytes))]
    cache_size: Gauge,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            http_requests_total: Default::default(),
            http_request_duration: Family::<HttpLabels, Histogram>::new(|| {
                Histogram::new(exponential_buckets(0.001, 2.0, 20))
            }),
            active_connections: Default::default(),
            cache_size: Default::default(),
        }
    }
}

// Define static metrics using LazyLock
static METRICS: LazyLock<Metrics> = LazyLock::new(|| {
    let metrics = Metrics::default();
    metrics.register_global().expect("Failed to register metrics");
    metrics
});

// Helper functions to update metrics from anywhere in the application
fn record_http_request(method: HttpMethod, status: u16, duration_seconds: f64) {
    let labels = HttpLabels { method, status };

    // Increment request counter
    METRICS.http_requests_total.with_or_new(&labels, |counter| counter.inc());

    // Record request duration
    METRICS.http_request_duration.with_or_new(&labels, |histogram| {
        histogram.observe(duration_seconds);
    });
}

// Simulate some application logic
fn simulate_http_server() {
    let mut rng = rand::rng();

    println!("Simulating HTTP server activity...");

    for i in 0..50 {
        // Simulate different HTTP requests
        let method = match rng.random_range(0..4) {
            0 => HttpMethod::Get,
            1 => HttpMethod::Post,
            2 => HttpMethod::Put,
            _ => HttpMethod::Delete,
        };

        let status = if rng.random_range(0..100) < 90 {
            200 // 90% success
        } else {
            500 // 10% error
        };

        let duration = rng.random_range(0.001..2.0);

        // Record the request
        record_http_request(method, status, duration);

        // Simulate connection changes
        if i % 10 == 0 {
            METRICS.active_connections.inc();
        }
        if i % 15 == 0 {
            METRICS.active_connections.dec();
        }

        // Update cache size
        let cache_size = rng.random_range(1024..1024 * 1024 * 10);
        METRICS.cache_size.set(cache_size);
    }

    println!("Simulation completed!");
}

fn main() -> Result<()> {
    // Optionally set a custom global registry
    let registry = Registry::builder().with_namespace("myapp").build()?;
    set_global_registry(registry)?;

    // Simulate application activity
    simulate_http_server();

    // Export metrics
    let mut output = String::new();
    with_global_registry(|registry| text::encode(&mut output, registry, TextProfile::default()))?;

    println!("\n=== Exported Metrics ===");
    println!("{output}");

    Ok(())
}
