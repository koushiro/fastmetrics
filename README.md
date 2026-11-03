# FastMetrics

[![](https://github.com/koushiro/fastmetrics/actions/workflows/ci.yml/badge.svg)][actions]
[![](https://img.shields.io/docsrs/fastmetrics)][docs.rs]
[![](https://img.shields.io/crates/v/fastmetrics)][crates.io]
[![](https://img.shields.io/crates/l/fastmetrics)][crates.io]
[![](https://img.shields.io/crates/d/fastmetrics)][crates.io]
[![](https://img.shields.io/badge/MSRV-1.95.0-green?logo=rust)][whatrustisit]
[![](https://deepwiki.com/badge.svg)][deepwiki]

[actions]: https://github.com/koushiro/fastmetrics/actions
[docs.rs]: https://docs.rs/fastmetrics
[crates.io]: https://crates.io/crates/fastmetrics
[whatrustisit]: https://www.whatrustisit.com
[deepwiki]: https://deepwiki.com/koushiro/fastmetrics

A pure-Rust implementation of the [OpenMetrics] specification for transmitting cloud-native metrics at scale,
and it's compatible with [Prometheus].

[Prometheus]: https://prometheus.io
[OpenMetrics]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md

## Features

- Full support for [OpenMetrics] specification
- Fast encoding in both text and (optional) protobuf exposition format
  - Text
    - Prometheus text: `0.0.4`, `1.0.0`
    - OpenMetrics text: `0.0.1`, `1.0.0`
    - V1 escaping schemes: `allow-utf-8`, `underscores`, `dots`, `values`
  - Protobuf
    - [Prometheus protobuf schema]
    - [OpenMetrics protobuf schema]
- Customizable metric types (currently a set of commonly used metric types are provided)
- Hierarchical metric organization with namespaces and subsystems
- Support for variable and constant labels
- Derive macros to simplify code (e.g., like registering metrics, label handling, etc.)

[Prometheus protobuf schema]: https://github.com/prometheus/client_model/blob/master/io/prometheus/client/metrics.proto
[OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto

## Usage

### Implementing traits

Implement the label traits explicitly and register each metric with `Registry`:

```rust
use fastmetrics::{
    encoder::{EncodeLabel, EncodeLabelSet, EncodeLabelValue, LabelEncoder, LabelSetEncoder},
    error::Result,
    format::text::{self, TextProfile},
    metrics::{counter::Counter, family::Family},
    raw::LabelSetSchema,
    registry::Registry,
};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Labels {
    method: Method,
    status: u16,
}

impl LabelSetSchema for Labels {
    fn names() -> Option<&'static [&'static str]> {
        Some(&["method", "status"])
    }
}

impl EncodeLabelSet for Labels {
    fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
        encoder.encode(&("method", &self.method))?;
        encoder.encode(&("status", &self.status))?;
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Method {
    Get,
    Put,
}

impl EncodeLabelValue for Method {
    fn encode(&self, encoder: &mut dyn LabelEncoder) -> Result<()> {
        match self {
            Self::Get => encoder.encode_str_value("Get"),
            Self::Put => encoder.encode_str_value("Put"),
        }
    }
}

fn main() -> Result<()> {
    // Create a registry with a namespace and some constant labels
    let mut registry = Registry::builder()
        .with_namespace("myapp")
        .with_const_labels([("env", "prod")])
        .build()?;

    // Register a simple counter
    let requests = <Counter>::default();
    registry.register("requests", "Total requests processed", requests.clone())?;

    // Register a counter family for tracking requests with labels
    let http_requests = Family::<Labels, Counter>::default();
    registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;

    // Update the simple counter
    requests.inc();
    assert_eq!(requests.total(), 1);

    // Update the counter family
    let labels = Labels { method: Method::Get, status: 200 };
    http_requests.with_or_new(&labels, |req| req.inc());
    assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));

    // Export metrics in text format
    let mut output = String::new();
    text::encode(&mut output, &registry, TextProfile::default())?;
    assert!(output.contains(r#"myapp_http_requests_total{env="prod",method="Get",status="200"} 1"#));
    println!("{}", output);

    Ok(())
}
```

### Using derive macros

Enable the `derive` feature to generate the label and registration trait implementations.
This example produces the same metrics as the manual implementation above:

```rust
# #[cfg(feature = "derive")]
use fastmetrics::{
    derive::*,
    error::Result,
    format::text::{self, TextProfile},
    metrics::{counter::Counter, family::Family},
    registry::*,
};

// Define label types
# #[cfg(feature = "derive")]
#[derive(Clone, Eq, PartialEq, Hash, LabelSet)]
struct Labels {
    method: Method,
    status: u16,
}

# #[cfg(feature = "derive")]
#[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
enum Method {
    Get,
    Put,
}

# #[cfg(feature = "derive")]
#[derive(Default, Register)]
struct Metrics {
    /// Total requests processed
    requests: Counter,
    /// Total HTTP requests
    http_requests: Family<Labels, Counter>,
}

# #[cfg(feature = "derive")]
fn main() -> Result<()> {
    // Create a registry with a namespace and some constant labels
    let mut registry = Registry::builder()
        .with_namespace("myapp")
        .with_const_labels([("env", "prod")])
        .build()?;

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
    text::encode(&mut output, &registry, TextProfile::default())?;
    assert!(output.contains(r#"myapp_http_requests_total{env="prod",method="Get",status="200"} 1"#));
    println!("{}", output);

    Ok(())
}
# #[cfg(not(feature = "derive"))]
# fn main() {}
```

See [documentation](https://docs.rs/fastmetrics) and [examples](https://github.com/koushiro/fastmetrics/tree/main/examples) for more details.

## Performance

See [benchmarks](https://github.com/koushiro/fastmetrics/blob/main/benchmarks/README.md) for more details

## Acknowledgment

I drew a lot of inspiration from the following libraries, retaining the designs I thought were good and experimenting with some different ones.

- [prometheus/client_golang](https://github.com/prometheus/client_golang): Official prometheus client library for Golang applications.
- [prometheus/client_rust](https://github.com/prometheus/client_rust): Official prometheus client library for Rust applications.
- [tikv/rust-prometheus](https://github.com/tikv/rust-prometheus): Another prometheus instrumentation library for Rust applications.

## License

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>
