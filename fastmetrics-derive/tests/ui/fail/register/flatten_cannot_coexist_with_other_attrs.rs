use fastmetrics::metrics::{counter::Counter, histogram::Histogram};
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct Metrics {
    #[register(flatten)]
    inner1: InnerMetrics1,

    #[register(flatten, skip)]
    inner2: InnerMetrics2,
}

#[derive(Default, Register)]
struct InnerMetrics1 {
    /// Total HTTP requests
    #[register(rename = "http_requests")]
    requests: Counter,
}

#[derive(Default, Register)]
struct InnerMetrics2 {
    /// Total HTTP requests
    #[register(rename = "http_requests_duration", unit(Seconds))]
    request_duration: Histogram,
}

fn main() {}
