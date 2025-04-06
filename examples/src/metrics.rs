use anyhow::Result;
use fastmetrics::{
    format::text,
    metrics::{
        counter::Counter,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use rand::Rng;

fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("demo").build();

    let counter = <Counter>::default();
    let gauge = <Gauge>::default();
    let histogram = Histogram::new(exponential_buckets(1.0, 2.0, 10));

    registry
        .register("my_counter", "My counter help", counter.clone())?
        .register("my_gauge", "My gauge help", gauge.clone())?
        .register("my_histogram", "My histogram help", histogram.clone())?;

    let mut rng = rand::rng();
    for _ in 0..100 {
        counter.inc();
        gauge.set(rng.random_range(-100..100i64));
        histogram.observe(rng.random_range(0f64..2048f64));
    }

    let mut output = String::new();
    text::encode(&mut output, &registry)?;
    println!("{}", &output);

    Ok(())
}
