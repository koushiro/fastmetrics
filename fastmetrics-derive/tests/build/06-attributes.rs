#![allow(unused_imports)]
use fastmetrics::metrics::counter::Counter;
use fastmetrics::metrics::gauge::Gauge;
use fastmetrics_derive::Registrant;

#[derive(Registrant)]
struct Server {
    #[registrant(rename = "memory_usage", unit = "bytes")] // mutiple attributes in single parenthesis
    mem_usage: Gauge,

    #[registrant(rename = "tcp_retransmitted")]
    #[registrant(unit = "segments")]
    tcp_retrans: Gauge,
}

fn main() {}
