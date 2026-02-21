use fastmetrics_derive::LabelSet;

#[derive(LabelSet)]
struct Labels {
    #[label(skip, rename = "value")]
    value: u64,
}

fn main() {}
