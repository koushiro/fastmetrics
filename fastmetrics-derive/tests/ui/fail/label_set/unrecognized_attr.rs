use fastmetrics_derive::LabelSet;

#[derive(LabelSet)]
struct Labels {
    #[label(unknown)]
    value: u64,
}

fn main() {}
