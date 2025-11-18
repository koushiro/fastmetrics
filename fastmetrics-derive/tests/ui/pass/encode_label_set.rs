use fastmetrics_derive::{EncodeLabelSet, EncodeLabelValue};

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
struct Labels {
    #[label(rename = "op")]
    operation: Operation,
    error: Option<Error>,

    #[label(flatten)]
    extra: ExtraLabels,

    #[label(skip)]
    _skip: u64,
}

#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
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

fn main() {
    // This just verifies compilation succeeds
    let _labels = Labels {
        operation: Operation::Read,
        error: Some(Error::NotFound),
        extra: ExtraLabels { region: "us-east-1" },
        _skip: 42,
    };
}
