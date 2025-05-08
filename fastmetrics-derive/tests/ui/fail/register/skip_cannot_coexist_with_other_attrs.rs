use fastmetrics::metrics::counter::Counter;
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct Metrics {
    #[register(skip)]
    _skip: (),

    /// Total HTTP requests
    #[register(rename = "http_requests", skip)]
    requests: Counter,
}

fn main() {}
