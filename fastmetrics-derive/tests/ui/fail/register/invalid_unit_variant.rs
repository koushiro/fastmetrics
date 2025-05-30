use fastmetrics::metrics::histogram::Histogram;
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct Metrics {
    /// Duration of HTTP requests
    #[register(rename = "http_requests_duration", unit("Seconds"))]
    request_duration: Histogram,
}

fn main() {}
