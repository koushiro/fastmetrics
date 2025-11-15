use fastmetrics_derive::StateSetValue;

// This should fail because StateSetValue is designed for enums
#[derive(StateSetValue)]
struct NotAnEnum {
    field: bool,
}

fn main() {}
