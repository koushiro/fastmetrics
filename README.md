# fastmetrics

[![Crates.io](https://img.shields.io/crates/v/fastmetrics.svg)](https://crates.io/crates/fastmetrics)
[![Documentation](https://docs.rs/fastmetrics/badge.svg)](https://docs.rs/fastmetrics)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A pure-Rust implementation of the [OpenMetrics] specification for transmitting cloud-native metrics at scale,
and it's compatible with Prometheus.

[OpenMetrics]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md

## Features

- Full support for [OpenMetrics] specification
- Fast encoding in both text and protobuf exposition format
- Type-safe metric creation and manipulation
- Hierarchical metric organization with namespaces and subsystems
- Support for variable and constant labels
- Derive macros to simplify code (e.g., like label handling, stateset value handling, etc.)

## TODO

- [ ] Summary metric type
- [ ] Exemplar feature in counter and histogram metric
- [ ] ...

## Usage

```rust
use fastmetrics::{
    encoder::{EncodeLabelSet, EncodeLabelValue},
    format::text,
    metrics::{counter::Counter, family::Family},
    registry::Registry,
};

// Define label types
// Need to enable `derive` feature to use `#[derive(EncodeLabelSet)]`
#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
struct Labels {
    method: Method,
    status: u32,
}

// Need to enable `derive` feature to use `#[derive(EncodeLabelValue)]`
#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
enum Method {
    Get,
    Put,
}

fn main() -> Box<dyn std::error::Error> {
    // Create a registry with a namespace and some constant labels
    let mut registry = Registry::builder()
        .with_namespace("myapp")
        .with_const_labels([("env", "prod")])
        .build();

    // Register a simple counter
    let requests = <Counter>::default();
    registry.register("requests", "Total requests processed", requests.clone())?;

    // Register a counter metric family for tracking requests with labels
    let http_requests = Family::<Labels, Counter>::default();
    registry.register(
        "http_requests",
        "Total HTTP requests",
        http_requests.clone()
    )?;

    // Update the simple counter
    requests.inc();
    assert_eq!(requests.total(), 1);

    // Update the counter family
    let labels = Labels { method: Method::Get, status: 200 };
    http_requests.with_or_new(&labels, |req| req.inc());
    assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));

    // Export metrics in text format
    let mut output = String::new();
    text::encode(&mut output, &registry)?;
    println!("{}", output);

    Ok(())
}
```

See [documentation](https://docs.rs/fastmetrics) and [examples](./examples) for more details.

## Performance

Compared with the existing rust client libraries, its text encoding is about 20%~30% faster than the fastest rust library (prometheus-client),
while its Protobuf encoding is on par with the fastest rust library (prometheus).

See [benchmarks](./benchmarks/README.md) for more details

## Acknowledgment

I drew a lot of inspiration from the following libraries, retaining the designs I thought were good and experimenting with some different ones.

- [prometheus/client_golang](https://github.com/prometheus/client_golang): Official prometheus client library for Golang applications.
- [prometheus/client_rust](https://github.com/prometheus/client_rust): Official prometheus client library for Rust applications.
- [tikv/rust-prometheus](https://github.com/tikv/rust-prometheus): Another prometheus instrumentation library for Rust applications.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
