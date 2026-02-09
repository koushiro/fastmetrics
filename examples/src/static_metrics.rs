use std::sync::LazyLock;

use anyhow::Result;
use fastmetrics::{
    derive::*,
    format::text,
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

// Define static metrics using LazyLock
static HTTP_REQUESTS_TOTAL: LazyLock<Family<HttpLabels, Counter>> = LazyLock::new(|| {
    register(
        "http_requests_total",
        "Total number of HTTP requests",
        Family::<HttpLabels, Counter>::default(),
    )
    .expect("Failed to register http_requests_total")
});

static HTTP_REQUEST_DURATION: LazyLock<Family<HttpLabels, Histogram>> = LazyLock::new(|| {
    register_with_unit(
        "http_request_duration",
        "Duration of HTTP requests",
        Unit::Seconds,
        Family::<HttpLabels, Histogram>::new(|| {
            Histogram::new(exponential_buckets(0.001, 2.0, 20))
        }),
    )
    .expect("Failed to register http_request_duration")
});

static ACTIVE_CONNECTIONS: LazyLock<Gauge> = LazyLock::new(|| {
    register("active_connections", "Number of active connections", Gauge::default())
        .expect("Failed to register active_connections")
});

static CACHE_SIZE: LazyLock<Gauge> = LazyLock::new(|| {
    register_with_unit("cache_size", "Current cache size in bytes", Unit::Bytes, Gauge::default())
        .expect("Failed to register cache_size")
});

// Helper functions to update metrics from anywhere in the application
fn record_http_request(method: HttpMethod, status: u16, duration_seconds: f64) {
    let labels = HttpLabels { method, status };

    // Increment request counter
    HTTP_REQUESTS_TOTAL.with_or_new(&labels, |counter| counter.inc());

    // Record request duration
    HTTP_REQUEST_DURATION.with_or_new(&labels, |histogram| {
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
            ACTIVE_CONNECTIONS.inc();
        }
        if i % 15 == 0 {
            ACTIVE_CONNECTIONS.dec();
        }

        // Update cache size
        let cache_size = rng.random_range(1024..1024 * 1024 * 10);
        CACHE_SIZE.set(cache_size);
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
    with_global_registry(|registry| text::encode(&mut output, registry))?;

    println!("\n=== Exported Metrics ===");
    println!("{output}");

    Ok(())
}
