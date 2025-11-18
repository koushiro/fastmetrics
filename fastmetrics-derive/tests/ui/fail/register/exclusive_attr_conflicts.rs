use fastmetrics::metrics::counter::Counter;
use fastmetrics_derive::Register;

// This file intentionally combines multiple exclusivity conflict scenarios
// for #[register(...)] attributes.

#[derive(Default, Register)]
struct SkipConflictOneAttr {
    // Conflict: skip with another non-exclusive attribute (rename) within a single attribute
    #[register(skip, rename = "should_fail")]
    counter: Counter,
}

#[derive(Default, Register)]
struct SkipConflictTwoAttrs {
    // Conflict: skip with another non-exclusive attribute (rename) within the separate attributes
    #[register(skip)]
    #[register(rename = "should_fail")]
    counter: Counter,
}

#[derive(Default, Register)]
struct FlattenConflictOneAttr {
    // Conflict: flatten with another non-exclusive attribute (rename) within a single attribute
    #[register(flatten, rename = "also_fail")]
    inner: Inner,
}

#[derive(Default, Register)]
struct FlattenConflictTwoAttrs {
    // Conflict: flatten with another non-exclusive attribute (rename) within a single attribute
    #[register(flatten)]
    #[register(rename = "also_fail")]
    inner: Inner,
}

#[derive(Default, Register)]
struct SubsystemConflictOneAttr {
    // Conflict: subsystem with another non-exclusive attribute (unit) within a single attribute
    #[register(subsystem = "inner", unit(Bytes))]
    inner: Inner,
}

#[derive(Default, Register)]
struct SubsystemConflictTwoAttrs {
    // Conflict: subsystem with another non-exclusive attribute (unit) within the separate attributes
    #[register(subsystem = "inner")]
    #[register(unit(Bytes))]
    inner: Inner,
}

#[derive(Default, Register)]
struct Inner {
    // Just a simple metric to allow nesting
    counter: Counter,
}

#[derive(Default, Register)]
struct IntraAttributeConflictOneAttr1 {
    // Conflict: skip + flatten (single attribute)
    #[register(skip, flatten)]
    a: Counter,
}

#[derive(Default, Register)]
struct IntraAttributeConflictOneAttr2 {
    // Conflict: flatten + subsystem (single attribute)
    #[register(flatten, subsystem = "inner")]
    inner: Inner,
}

#[derive(Default, Register)]
struct IntraAttributeConflictOneAttr3 {
    // Conflict: subsystem + skip (single attribute)
    #[register(subsystem = "inner", skip)]
    inner: Inner,
}

#[derive(Default, Register)]
struct IntraAttributeConflictThreeAttrs {
    // Conflict: skip + flatten + subsystem (separate attributes)
    #[register(skip)]
    #[register(flatten)]
    #[register(subsystem = "inner")]
    inner: Inner,
}

fn main() {}
