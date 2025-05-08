use fastmetrics_derive::EncodeLabelSet;

#[derive(EncodeLabelSet)]
struct MyLabels {
    service: String,
    endpoint: String,
    status_code: u16,
}

fn main() {
    // This just verifies compilation succeeds
    let _labels =
        MyLabels { service: "auth".to_string(), endpoint: "/login".to_string(), status_code: 200 };
}
