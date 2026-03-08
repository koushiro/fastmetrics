use fastmetrics::raw::LabelSetSchema;
use fastmetrics_derive::LabelSetSchema;

#[derive(Clone, Eq, PartialEq, Hash, LabelSetSchema)]
struct Labels {
    #[label(rename = "method")]
    method: &'static str,

    #[label(flatten)]
    inner: InnerLabels,

    #[label(skip)]
    _skip: u64,
}

#[derive(Clone, Eq, PartialEq, Hash, LabelSetSchema)]
struct InnerLabels {
    region: &'static str,
}

fn main() {
    // Ensure the derived traits are usable.
    let _labels = Labels { method: "GET", inner: InnerLabels { region: "us-east-1" }, _skip: 42 };
    assert_eq!(<Labels as LabelSetSchema>::names(), Some(&["method", "region"][..]));
}
