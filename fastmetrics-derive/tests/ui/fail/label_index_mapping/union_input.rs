use fastmetrics_derive::LabelIndexMapping;

#[derive(LabelIndexMapping)]
union NotSupported {
    x: u8,
}

fn main() {}
