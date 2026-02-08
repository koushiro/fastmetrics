use fastmetrics_derive::{LabelIndexMapping, LabelSetSchema};

struct NoDefault;

#[derive(LabelSetSchema, LabelIndexMapping)]
struct Labels {
    #[label(skip)]
    skipped: NoDefault,
}

fn main() {}
