use fastmetrics::metrics::counter::Counter;
use fastmetrics_derive::Registrant;

#[derive(Registrant)]
struct Server {
    #[registrant(rename = "http_requests")]
    requests: Counter,

    #[registrant(rename = http_requests)]
    invalid: Counter,
}

fn main() {}
