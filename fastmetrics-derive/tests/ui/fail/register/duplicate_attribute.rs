use fastmetrics::metrics::counter::Counter;
use fastmetrics_derive::Register;

#[derive(Default, Register)]
struct DupRenameOneAttr {
    #[register(rename = "a", rename = "b")]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupRenameTwoAttrs {
    #[register(rename = "a")]
    #[register(rename = "b")]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupHelpOneAttr {
    #[register(help = "help1", help = "help2")]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupHelpTwoAttrs {
    #[register(help = "help1")]
    #[register(help = "help2")]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupUnitStringOneAttr {
    #[register(unit = "bytes", unit = "bytes")]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupUnitStringTwoAttrs {
    #[register(unit = "bytes")]
    #[register(unit = "bytes")]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupUnitPathOneAttr {
    #[register(unit(Bytes), unit(Bytes))]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupUnitPathTwoAttrs {
    #[register(unit(Bytes))]
    #[register(unit(Bytes))]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupUnitMixedOneAttr {
    #[register(unit = "bytes", unit(Bytes))]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupUnitMixedTwoAttrs {
    #[register(unit = "bytes")]
    #[register(unit(Bytes))]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupSkipOneAttr {
    #[register(skip, skip)]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupSkipTwoAttrs {
    #[register(skip)]
    #[register(skip)]
    counter: Counter,
}

#[derive(Default, Register)]
struct DupFlattenOneAttr {
    #[register(flatten, flatten)]
    inner: InnerMetrics,
}

#[derive(Default, Register)]
struct DupFlattenTwoAttrs {
    #[register(flatten)]
    #[register(flatten)]
    inner: InnerMetrics,
}

#[derive(Default, Register)]
struct DupSubsystemOneAttr {
    #[register(subsystem = "s1", subsystem = "s2")]
    inner: InnerMetrics,
}

#[derive(Default, Register)]
struct DupSubsystemTwoAttrs {
    #[register(subsystem = "s1")]
    #[register(subsystem = "s2")]
    inner: InnerMetrics,
}

#[derive(Default, Register)]
struct InnerMetrics {
    counter: Counter,
}

fn main() {}
