use fastmetrics_derive::EncodeLabelValue;

// This should fail because named-field structs are not supported.
#[derive(EncodeLabelValue)]
struct NotAnEnum {
    field: String,
}

fn main() {}
