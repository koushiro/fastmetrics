use fastmetrics::metrics::{counter::Counter, histogram::Histogram};
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct Metrics {
    #[register(flatten)]
    inner1: InnerMetrics1,

    #[register(flatten, subsystem = "inner")]
    inner2: InnerMetrics2,

    #[register(flatten, rename = "inner")]
    inner3: InnerMetrics3,
}

#[derive(Default, Register)]
struct InnerMetrics1 {
    /// Total HTTP requests
    #[register(rename = "http_requests")]
    requests: Counter,
}

#[derive(Default, Register)]
struct InnerMetrics2 {
    /// Duration of HTTP requests
    #[register(rename = "http_requests_duration", unit(Seconds))]
    request_duration: Histogram,
}

#[derive(Default, Register)]
struct InnerMetrics3 {
    /// Inner counter
    counter: Counter,
}

fn main() {}
