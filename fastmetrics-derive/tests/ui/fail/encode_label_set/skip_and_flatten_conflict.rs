use fastmetrics_derive::EncodeLabelSet;

#[derive(EncodeLabelSet)]
struct Inner {
    x: u8,
}

#[derive(EncodeLabelSet)]
struct Labels {
    #[label(skip, flatten)]
    inner: Inner,
}

fn main() {}
