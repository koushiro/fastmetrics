use fastmetrics::{
    format::text,
    metrics::{counter::Counter, histogram::Histogram},
    registry::{Register, Registry},
};
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct Metrics {
    /// Total HTTP requests
    http_requests: Counter,
    /// Duration of HTTP requests
    #[register(rename = "http_request_duration", unit(Seconds))]
    request_duration: Histogram,

    #[register(skip)]
    _skip: (),
}

fn main() {
    let mut registry = Registry::default();
    let metrics = Metrics::default();
    metrics.register(&mut registry).unwrap();

    let mut output = String::new();
    text::encode(&mut output, &registry).unwrap();
    println!("{}", output);
}
