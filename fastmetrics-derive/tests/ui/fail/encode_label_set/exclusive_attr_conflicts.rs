use fastmetrics_derive::EncodeLabelSet;

// This file intentionally combines multiple exclusivity conflict scenarios
// for #[label(...)] attributes.

// Conflict: skip + flatten within a single attribute
#[derive(EncodeLabelSet)]
struct IntraAttributeConflictOneAttr1 {
    #[label(skip, flatten)]
    a: u8,
}

// Conflict: flatten + skip within a single attribute
#[derive(EncodeLabelSet)]
struct IntraAttributeConflictOneAttr2 {
    #[label(flatten, skip)]
    a: u8,
}

// Conflict: skip + flatten across separate attributes (skip then flatten)
#[derive(EncodeLabelSet)]
struct InterAttributeConflictTwoAttrs1 {
    #[label(skip)]
    #[label(flatten)]
    inner: Inner,
}

// Conflict: skip + flatten across separate attributes (flatten then skip)
#[derive(EncodeLabelSet)]
struct InterAttributeConflictTwoAttrs2 {
    #[label(flatten)]
    #[label(skip)]
    inner: Inner,
}

#[derive(EncodeLabelSet)]
struct Inner {
    x: u8,
}

fn main() {}
