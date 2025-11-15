use fastmetrics::metrics::counter::Counter;
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct Metrics {
    #[register(unknown)]
    requests: Counter,
}

fn main() {}
