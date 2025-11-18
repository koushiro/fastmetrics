use fastmetrics_derive::EncodeLabelSet;

// Duplicated `rename` within a single attribute list
#[derive(EncodeLabelSet)]
struct DupRenameOneAttr {
    #[label(rename = "a", rename = "b")]
    a: u8,
}

// Duplicated `rename` across separate attributes
#[derive(EncodeLabelSet)]
struct DupRenameTwoAttrs {
    #[label(rename = "a")]
    #[label(rename = "b")]
    a: u8,
}

// Duplicated `skip` within a single attribute list
#[derive(EncodeLabelSet)]
struct DupSkipOneAttr {
    #[label(skip, skip)]
    a: u8,
}

// Duplicated `skip` across separate attributes
#[derive(EncodeLabelSet)]
struct DupSkipTwoAttrs {
    #[label(skip)]
    #[label(skip)]
    a: u8,
}

// Duplicated `flatten` within a single attribute list
#[derive(EncodeLabelSet)]
struct DupFlattenOneAttr {
    #[label(flatten, flatten)]
    inner: Inner,
}

// Duplicated `flatten` across separate attributes
#[derive(EncodeLabelSet)]
struct DupFlattenTwoAttrs {
    #[label(flatten)]
    #[label(flatten)]
    inner: Inner,
}

// A simple inner struct to use with `flatten` tests
#[derive(EncodeLabelSet)]
struct Inner {
    x: u8,
}

fn main() {}
