use fastmetrics::{
    format::text,
    metrics::{counter::Counter, family::Family, gauge::Gauge, histogram::Histogram},
    registry::{Register, Registry},
};
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct Metrics {
    /// My counter help
    #[register(rename = "my_counter")]
    counter_family: Family<(), Counter>,

    /// My gauge help line 1
    /// help line 2
    /// help line 3
    #[register(rename = "my_gauge")]
    gauge: Gauge,

    // No help
    #[register(unit(Bytes))]
    counter: Counter,

    /**

    My histogram help line 1

    help line 2
    help line 3

    */
    #[register(rename = "my_histogram", unit = "bytes")]
    histogram: Histogram,

    #[register(subsystem = "inner")]
    inner: InnerMetrics,

    #[register(flatten)]
    flatten: FlattenMetrics,

    // skip the field
    #[register(skip)]
    _skip: (),
}

#[derive(Default, Register)]
struct InnerMetrics {
    /// Inner counter help
    counter: Counter,

    #[register(subsystem = "innermost")]
    innermost: InnermostMetrics,
}

#[derive(Default, Register)]
struct InnermostMetrics {
    /// Innermost counter help
    counter: Counter,
}

#[derive(Default, Register)]
struct FlattenMetrics {
    /// Flatten gauge help
    gauge: Gauge,
}

fn main() {
    let mut registry = Registry::default();
    let metrics = Metrics::default();
    metrics.register(&mut registry).unwrap();

    let mut output = String::new();
    text::encode(&mut output, &registry).unwrap();
    println!("{}", output);
}
