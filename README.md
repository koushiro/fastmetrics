# fastmetrics

[![](https://github.com/koushiro/fastmetrics/actions/workflows/ci.yml/badge.svg)](https://github.com/koushiro/fastmetrics/actions)
[![](https://img.shields.io/crates/v/fastmetrics)](https://crates.io/crates/fastmetrics)
[![](https://img.shields.io/docsrs/fastmetrics)](https://docs.rs/fastmetrics)
[![](https://img.shields.io/crates/l/fastmetrics)](LICENSE)
[![](https://img.shields.io/badge/MSRV-1.81.0-green?logo=rust)](https://www.whatrustisit.com)
[![](https://deepwiki.com/badge.svg)](https://deepwiki.com/koushiro/fastmetrics)

A pure-Rust implementation of the [OpenMetrics] specification for transmitting cloud-native metrics at scale,
and it's compatible with Prometheus.

[OpenMetrics]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md

## Features

- Full support for [OpenMetrics] specification
- Fast encoding in both text and (optional) protobuf exposition format
- Customizable metric types (currently a set of commonly used metric types are provided)
- Hierarchical metric organization with namespaces and subsystems
- Support for variable and constant labels
- Derive macros to simplify code (e.g., like registering metrics, label handling, etc.)

## Usage

```rust
use fastmetrics::{
    encoder::{EncodeLabelSet, EncodeLabelValue},
    format::text,
    metrics::{counter::Counter, family::Family},
    registry::{Register, Registry},
};

// Define label types
// Need to enable `derive` feature to use `#[derive(EncodeLabelSet)]`
#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
struct Labels {
    method: Method,
    status: u16,
}

// Need to enable `derive` feature to use `#[derive(EncodeLabelValue)]`
#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
enum Method {
    Get,
    Put,
}

// Need to enable `derive` feature to use `#[derive(Register)]`
#[derive(Default, Register)]
struct Metrics {
    /// Total requests processed
    requests: Counter,
    /// Total HTTP requests
    http_requests: Family<Labels, Counter>,
}

fn main() -> Box<dyn std::error::Error> {
    // Create a registry with a namespace and some constant labels
    let mut registry = Registry::builder()
        .with_namespace("myapp")
        .with_const_labels([("env", "prod")])
        .build();

    // Register metrics
    let metrics = Metrics::default();
    metrics.register(&mut registry)?;

    // Update the simple counter
    metrics.requests.inc();
    assert_eq!(metrics.requests.total(), 1);

    // Update the counter family
    let labels = Labels { method: Method::Get, status: 200 };
    metrics.http_requests.with_or_new(&labels, |req| req.inc());
    assert_eq!(metrics.http_requests.with(&labels, |req| req.total()), Some(1));

    // Export metrics in text format
    let mut output = String::new();
    text::encode(&mut output, &registry)?;
    println!("{}", output);

    Ok(())
}
```

See [documentation](https://docs.rs/fastmetrics) and [examples](https://github.com/koushiro/fastmetrics/tree/main/examples) for more details.

## Performance

Compared with the existing rust client libraries, its text encoding is about 20%~30% faster than the fastest rust library (prometheus-client),
while its Protobuf encoding is on par with the fastest rust library (prometheus).

See [benchmarks](https://github.com/koushiro/fastmetrics/blob/main/benchmarks/README.md) for more details

## Acknowledgment

I drew a lot of inspiration from the following libraries, retaining the designs I thought were good and experimenting with some different ones.

- [prometheus/client_golang](https://github.com/prometheus/client_golang): Official prometheus client library for Golang applications.
- [prometheus/client_rust](https://github.com/prometheus/client_rust): Official prometheus client library for Rust applications.
- [tikv/rust-prometheus](https://github.com/tikv/rust-prometheus): Another prometheus instrumentation library for Rust applications.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
