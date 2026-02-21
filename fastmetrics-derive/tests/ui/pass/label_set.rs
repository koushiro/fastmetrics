use fastmetrics_derive::{EncodeLabelValue, LabelSet};

#[derive(Clone, Eq, PartialEq, Hash, LabelSet)]
struct Labels {
    #[label(rename = "op")]
    operation: Operation,
    error: Option<Error>,

    #[label(flatten)]
    extra: ExtraLabels,

    #[label(skip)]
    _skip: u64,
}

#[derive(Clone, Eq, PartialEq, Hash, LabelSet)]
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
    let labels = Labels {
        operation: Operation::Read,
        error: Some(Error::NotFound),
        extra: ExtraLabels { region: "us-east-1" },
        _skip: 42,
    };

    let _as_label_set: &dyn fastmetrics::encoder::EncodeLabelSet = &labels;
    assert_eq!(
        <Labels as fastmetrics::raw::LabelSetSchema>::names(),
        Some(&["op", "error", "region"][..])
    );
}
