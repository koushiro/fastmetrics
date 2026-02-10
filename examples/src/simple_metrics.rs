use fastmetrics::{
    error::Result,
    format::text,
    metrics::{
        counter::Counter,
        gauge::Gauge,
        histogram::{Histogram, exponential_buckets},
    },
    registry::{Register, Registry},
};
use rand::RngExt;

pub struct Metrics {
    counter: Counter,
    gauge: Gauge,
    histogram: Histogram,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            counter: Counter::default(),
            gauge: Gauge::default(),
            histogram: Histogram::new(exponential_buckets(1.0, 2.0, 10)),
        }
    }
}

impl Register for Metrics {
    fn register(&self, registry: &mut Registry) -> Result<()> {
        registry.register("my_counter", "My counter help", self.counter.clone())?;

        let subsystem = registry.subsystem("outer")?;
        subsystem.register("my_gauge", "My gauge help", self.gauge.clone())?;

        let subsystem = subsystem.subsystem("inner")?;
        subsystem.register("my_histogram", "My histogram help", self.histogram.clone())?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("demo").build()?;

    // 1. Using built-in metric types directly
    let counter = <Counter>::default();
    let gauge = <Gauge>::default();
    let histogram = Histogram::new(exponential_buckets(1.0, 2.0, 10));

    registry.register("my_counter", "My counter help", counter.clone())?;
    registry
        .subsystem("app")?
        .register("my_gauge", "My gauge help", gauge.clone())?
        .register("my_histogram", "My histogram help", histogram.clone())?;

    // 2. Using a custom struct that implements `Register` trait
    // let metrics = Metrics::default();
    // metrics.register(&mut registry)?;

    let mut rng = rand::rng();
    for _ in 0..100 {
        counter.inc();
        gauge.set(rng.random_range(-100..100i64));
        histogram.observe(rng.random_range(0f64..2048f64));
    }

    let mut output = String::new();
    text::encode(&mut output, &registry, text::TextProfile::OpenMetrics1)?;
    println!("{}", &output);

    Ok(())
}
