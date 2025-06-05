use anyhow::Result;
use fastmetrics::{
    encoder::{EncodeLabelSet, EncodeLabelValue},
    format::text,
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::{Register, Registry},
};
use rand::Rng;

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
struct Labels {
    number: Number,
}

impl Labels {
    fn odd() -> Self {
        Self { number: Number::Odd }
    }

    fn even() -> Self {
        Self { number: Number::Even }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
enum Number {
    Odd,
    Even,
}

#[derive(Default, Register)]
struct DemoMetrics {
    /// My counter help
    #[register(rename = "my_counter")]
    counter_family: Family<Labels, Counter>,

    /// My gauge help line 1
    /// help line 2
    /// help line 3
    #[register(rename = "my_gauge")]
    gauge: Gauge,

    // No help
    #[register(unit(Bytes))]
    counter: Counter,

    /// This doc comment will be ignored
    #[register(help = "Custom help text that override doc comments")]
    override_help_counter: Counter,

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

impl DemoMetrics {
    fn new() -> Self {
        Self {
            counter_family: Default::default(),
            gauge: Default::default(),
            counter: Default::default(),
            override_help_counter: Default::default(),
            histogram: Histogram::new(exponential_buckets(1.0, 2.0, 10)),
            inner: Default::default(),
            flatten: Default::default(),
            _skip: (),
        }
    }
}

fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("demo").build();

    let metrics = DemoMetrics::new();
    metrics.register(&mut registry)?;

    let mut rng = rand::rng();
    for _ in 0..100 {
        let random = rng.random_range(0..100u64);
        let labels = if random % 2 == 0 { Labels::even() } else { Labels::odd() };
        metrics.counter_family.with_or_new(&labels, |counter| counter.inc());

        metrics.gauge.set(rng.random_range(-100..100i64));
        metrics.counter.inc();
        metrics.histogram.observe(rng.random_range(0f64..2048f64));
    }

    let mut output = String::new();
    text::encode(&mut output, &registry)?;
    println!("{}", &output);

    Ok(())
}
