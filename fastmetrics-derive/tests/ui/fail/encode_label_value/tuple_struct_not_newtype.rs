use fastmetrics_derive::EncodeLabelValue;

// This should fail because only single-field tuple structs are supported.
#[derive(EncodeLabelValue)]
struct InvalidTupleStruct(u16, u16);

fn main() {}
