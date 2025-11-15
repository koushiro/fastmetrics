use fastmetrics_derive::EncodeLabelSet;

#[derive(EncodeLabelSet)]
struct Inner {
    x: u8,
}

#[derive(EncodeLabelSet)]
struct DupSkip {
    #[label(skip)]
    #[label(skip)]
    a: u8,
}

#[derive(EncodeLabelSet)]
struct DupFlatten {
    #[label(flatten)]
    #[label(flatten)]
    inner: Inner,
}

fn main() {}
