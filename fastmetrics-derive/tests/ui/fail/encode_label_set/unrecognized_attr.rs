use fastmetrics_derive::EncodeLabelSet;

#[derive(EncodeLabelSet)]
struct Labels {
    #[label(unknown)]
    a: u8,
}

fn main() {}
