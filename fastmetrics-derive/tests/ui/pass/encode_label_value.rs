use fastmetrics_derive::EncodeLabelValue;

#[derive(EncodeLabelValue)]
enum Status {
    Success,
    Error,
    Pending,
}

fn main() {
    // This just verifies compilation succeeds
    let _status = Status::Success;
}
