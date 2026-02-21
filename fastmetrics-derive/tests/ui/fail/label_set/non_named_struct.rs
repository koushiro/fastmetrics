use fastmetrics_derive::LabelSet;

// This should fail because LabelSet requires named fields.
#[derive(LabelSet)]
struct TupleStruct(u64);

fn main() {}
