use fastmetrics::metrics::family::LabelIndexMapping;
use fastmetrics_derive::{LabelIndexMapping, LabelSetSchema};

#[derive(Clone, Copy, Eq, PartialEq, Hash, LabelIndexMapping)]
enum Method {
    Get,
    Put,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, LabelSetSchema, LabelIndexMapping)]
struct ExtraLabels {
    secure: bool,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, LabelSetSchema, LabelIndexMapping)]
struct Labels {
    method: Method,

    #[label(flatten)]
    extra: ExtraLabels,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, LabelSetSchema, LabelIndexMapping)]
struct LabelsWithSkip {
    method: Method,
    #[label(skip)]
    skipped: bool,
}

fn main() {
    assert_eq!(<Method as LabelIndexMapping>::CARDINALITY, 2);
    let get = Method::Get;
    assert_eq!(<Method as LabelIndexMapping>::index(&get), 0);
    let put = <Method as LabelIndexMapping>::from_index(1);
    assert!(matches!(put, Method::Put));

    assert_eq!(<Labels as LabelIndexMapping>::CARDINALITY, 4);

    let labels = Labels { method: Method::Put, extra: ExtraLabels { secure: true } };
    let index = <Labels as LabelIndexMapping>::index(&labels);
    assert_eq!(index, 3);

    let roundtrip = <Labels as LabelIndexMapping>::from_index(index);
    assert!(matches!(roundtrip.method, Method::Put));
    assert!(roundtrip.extra.secure);

    assert_eq!(<LabelsWithSkip as LabelIndexMapping>::CARDINALITY, 2);
    let skipped = LabelsWithSkip { method: Method::Put, skipped: true };
    let skipped_index = <LabelsWithSkip as LabelIndexMapping>::index(&skipped);
    assert_eq!(skipped_index, 1);
    let skipped_roundtrip = <LabelsWithSkip as LabelIndexMapping>::from_index(skipped_index);
    assert!(matches!(skipped_roundtrip.method, Method::Put));
    assert!(!skipped_roundtrip.skipped);
}
