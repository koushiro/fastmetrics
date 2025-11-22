use anyhow::Result;
use fastmetrics::{
    encoder::{EncodeLabelSet, EncodeLabelValue},
    format::text,
    metrics::{
        counter::Counter,
        family::Family,
        histogram::{Histogram, exponential_buckets},
    },
    raw::LabelSetSchema,
    registry::{Registry, Unit},
};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet, LabelSetSchema)]
struct Labels {
    #[label(rename = "op")]
    operation: Operation,
    error: Option<Error>,

    #[label(flatten)]
    extra: ExtraLabels,

    #[label(skip)]
    _skip: u64,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet, LabelSetSchema)]
struct ExtraLabels {
    region: &'static str,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
enum Operation {
    Read,
    Write,
    List,
    Delete,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
enum Error {
    NotFound,
    Fail,
}

impl Distribution<Labels> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Labels {
        let operation = match rng.random_range(0..=3) {
            0 => Operation::Read,
            1 => Operation::Write,
            2 => Operation::List,
            3 => Operation::Delete,
            _ => unreachable!(),
        };
        let error = match rng.random_range(0..=2) {
            0 => None,
            1 => Some(Error::NotFound),
            2 => Some(Error::Fail),
            _ => unreachable!(),
        };
        let extra = ExtraLabels {
            region: match rng.random_range(0..=2) {
                0 => "us-east-1",
                1 => "eu-west-1",
                2 => "ap-southeast-1",
                _ => unreachable!(),
            },
        };
        let skip = rng.random_range(1..=1_000_000);
        Labels { operation, error, extra, _skip: skip }
    }
}

fn main() -> Result<()> {
    let mut registry = Registry::builder().with_namespace("demo").build();

    let operation = Family::<Labels, Counter>::default();
    let operation_bytes =
        Family::<Labels, Histogram>::new(|| Histogram::new(exponential_buckets(1.0, 2.0, 10)));

    registry
        .register("operation", "Total number of operations", operation.clone())?
        .register_with_unit(
            "operation",
            "Operation size of bytes",
            Unit::Bytes,
            operation_bytes.clone(),
        )?;

    let mut rng = rand::rng();
    for _ in 0..100 {
        let labels = rng.random::<Labels>();
        operation.with_or_new(&labels, |counter| counter.inc());
        operation_bytes.with_or_new(&labels, |hist| hist.observe(rng.random_range(0f64..2048f64)));
    }

    let mut output = String::new();
    text::encode(&mut output, &registry)?;
    println!("{}", &output);

    Ok(())
}
