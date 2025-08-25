use anyhow::Result;
use fastmetrics::{
    encoder::{EncodeLabelSet, EncodeLabelValue},
    format::text,
    metrics::{
        counter::Counter,
        family::Family,
        histogram::{exponential_buckets, Histogram},
    },
    registry::{Registry, Unit},
};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
struct Labels {
    operation: Operation,
    error: Option<Error>,
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
        Labels { operation, error }
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
