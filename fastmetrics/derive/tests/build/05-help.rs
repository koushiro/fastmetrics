use fastmetrics::metrics::counter::Counter;
use fastmetrics::metrics::gauge::Gauge;
use fastmetrics_derive::Registrant;

#[derive(Registrant)]
struct Server {
    /// One line help
    requests: Counter,

    /// Muti-line help
    /// with a lot of text
    mem_usage: Gauge,
}

fn main() {}
