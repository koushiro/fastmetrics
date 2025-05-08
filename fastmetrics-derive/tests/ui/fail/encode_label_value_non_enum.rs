use fastmetrics_derive::EncodeLabelValue;

// This should fail because EncodeLabelValue is designed for enums
#[derive(EncodeLabelValue)]
struct NotAnEnum {
    field: String,
}

fn main() {}
