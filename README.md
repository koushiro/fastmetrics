# openmetrics-client

[![Crates.io](https://img.shields.io/crates/v/openmetrics-client.svg)](https://crates.io/crates/openmetrics-client)
[![Documentation](https://docs.rs/openmetrics-client/badge.svg)](https://docs.rs/openmetrics-client)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A pure-Rust implementation of the [OpenMetrics](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md) specification for transmitting cloud-native metrics at scale. This library is compatible with Prometheus and supports both text-based and protobuf exposition formats.

## Features

- Full support for [OpenMetrics](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md) specification
- Type-safe metric creation and manipulation
- Hierarchical metric organization with namespaces and subsystems
- Support for variable and constant labels
- Encoding in bot text and protobuf exposition format
- Optional derive macros to simplify code (e.g., like label handling, stateset value handling, etc.)

## TODO

- [ ] Summary metric type
- [ ] Exemplar feature in counter and histogram metric
- [ ] More check about metric restrictions
- [ ] ...

## Usage

```rust
use openmetrics_client::{
    encoder::{EncodeLabelSet, EncodeLabelValue},
    format::text,
    metrics::{counter::Counter, family::Family},
    registry::Registry,
};

// Create a registry with a namespace and some constant labels
let mut registry = Registry::builder()
    .with_namespace("myapp")
    .with_const_labels([("env", "prod")])
    .build();

// Register a simple counter
let requests = <Counter>::default();
registry.register("requests", "Total requests processed", requests.clone())?;

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

// Register a counter metric family for tracking requests with labels
let http_requests = Family::<Labels, Counter>::default();
registry.register(
    "http_requests",
    "Total HTTP requests",
    http_requests.clone()
)?;

// Update metrics
requests.inc();
let labels = Labels { method: Method::Get, status: 200 };
http_requests.with_or_default(&labels, |req| req.inc());

// Export metrics in text format
let mut output = String::new();
text::encode(&mut output, &registry)?;
println!("{}", output);
```

See [documentation](https://docs.rs/openmetrics-client) and [examples](./examples/) for more details.

## Acknowledgment

I drew a lot of inspiration from the following libraries, retaining the designs I thought were good and experimenting with some different ones.

- [prometheus/client_golang](https://github.com/prometheus/client_golang): Official prometheus client library for Golang applications.
- [prometheus/client_rust](https://github.com/prometheus/client_rust): Official prometheus client library for Rust applications.
- [tikv/rust-prometheus](https://github.com/tikv/rust-prometheus): Anthor prometheus instrumentation library for Rust applications.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
