use fastmetrics_derive::EncodeLabelValue;

#[derive(EncodeLabelValue)]
enum Status {
    Success,
    Error,
    Pending,
}

#[derive(EncodeLabelValue)]
struct HttpStatus(u16);

#[derive(EncodeLabelValue)]
struct OptionalError(Option<&'static str>);

fn main() {
    // This just verifies compilation succeeds
    let _status = Status::Success;
    let _code = HttpStatus(200);
    let _error = OptionalError(None);
}
