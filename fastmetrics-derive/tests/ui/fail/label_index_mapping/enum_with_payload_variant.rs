use fastmetrics_derive::LabelIndexMapping;

#[derive(LabelIndexMapping)]
enum NotUnit {
    A(u8),
    B,
}

fn main() {}
