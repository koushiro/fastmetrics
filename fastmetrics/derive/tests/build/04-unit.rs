use fastmetrics::metrics::gauge::Gauge;
use fastmetrics_derive::Registrant;

#[derive(Registrant)]
struct Server {
    #[registrant(unit = "bytes")]
    mem_usage: Gauge,

    #[registrant(unit = bytes)]
    invalid: Gauge,
}

fn main() {}
