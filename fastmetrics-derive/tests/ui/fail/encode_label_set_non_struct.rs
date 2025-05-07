use fastmetrics_derive::EncodeLabelSet;

// This should fail because EncodeLabelSet can only be derived for structs
#[derive(EncodeLabelSet)]
enum NotAStruct {
    Variant1,
    Variant2,
}

fn main() {}
