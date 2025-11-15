use fastmetrics_derive::EncodeLabelSet;

#[derive(EncodeLabelSet)]
struct MyLabels {
    service: String,
    endpoint: String,
    status_code: u16,

    #[label(flatten)]
    extra: ExtraLabels,

    #[label(skip)]
    _skip: u64,
}

#[derive(EncodeLabelSet)]
struct ExtraLabels {
    region: &'static str,
}

fn main() {
    // This just verifies compilation succeeds
    let _labels = MyLabels {
        service: "auth".to_string(),
        endpoint: "/login".to_string(),
        status_code: 200,
        extra: ExtraLabels { region: "us-east-1" },
        _skip: 42,
    };
}
