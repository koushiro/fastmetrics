use fastmetrics_derive::LabelSetSchema;

// This should fail because LabelSetSchema can only be derived for structs with named fields.
#[derive(LabelSetSchema)]
enum NotAStruct {
    Variant,
}

fn main() {}
