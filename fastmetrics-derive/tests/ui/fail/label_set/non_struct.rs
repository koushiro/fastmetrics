use fastmetrics_derive::LabelSet;

// This should fail because LabelSet can only be derived for structs with named fields.
#[derive(LabelSet)]
enum NotAStruct {
    Variant1,
    Variant2,
}

fn main() {}
