use fastmetrics::metrics::counter::Counter;
use fastmetrics::metrics::gauge::Gauge;
use fastmetrics_derive::Registrant;

#[derive(Registrant)]
struct Server {
    /// Number of HTTP requests received
    /// from the client
    #[registrant(rename = "http_requests")]
    requests: Counter,
    /// Memory usage in bytes
    /// of the server
    #[registrant(unit = "bytes")]
    memory_usage: Gauge,
}

fn main() {}
