//! OpenMetrics client library for Rust.
//!
//! This library provides a pure-Rust implementation of [OpenMetrics], a standard for transmitting
//! cloud-native metrics at scale. And this library is compatible with Prometheus.
//!
//! # Features
//!
//! - Full support for [OpenMetrics]  specification
//! - Fast encoding in both text and protobuf exposition format
//! - Type-safe metric creation and manipulation
//! - Hierarchical metric organization with namespaces and subsystems
//! - Support for variable and constant labels
//! - Derive macros to simplify code (e.g., like label handling, stateset value handling, etc.)
//!
//! [OpenMetrics]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md
//!
//! # Example
//!
//! ```rust
//! # use openmetrics_client::{
//! #     encoder::{EncodeLabel, EncodeLabelSet, EncodeLabelValue, LabelSetEncoder, LabelEncoder},
//! #     format::text,
//! #     metrics::{counter::Counter, family::Family},
//! #     registry::Registry,
//! # };
//!
//! #[derive(Clone, Eq, PartialEq, Hash)]
//! struct Labels {
//!     method: Method,
//!     status: u16,
//! }
//!
//! // Can use `#[derive(EncodeLabelSet)]` to simplify the code, but need to enable `derive` feature
//! impl EncodeLabelSet for Labels {
//!     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> std::fmt::Result {
//!         encoder.encode(&("method", &self.method))?;
//!         encoder.encode(&("status", &self.status))?;
//!         Ok(())
//!     }
//! }
//!
//! #[derive(Clone, Eq, PartialEq, Hash)]
//! enum Method {
//!     Get,
//!     Put,
//! }
//!
//! // Can use `#[derive(EncodeLabelValue)]` to simplify the code, but need to enable `derive` feature
//! impl EncodeLabelValue for Method {
//!     fn encode(&self, encoder: &mut dyn LabelEncoder) -> std::fmt::Result {
//!         match self {
//!             Self::Get => encoder.encode_str_value("Get"),
//!             Self::Put => encoder.encode_str_value("Put"),
//!         }
//!     }
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a registry with a namespace and some constant labels
//! let mut registry = Registry::builder()
//!     .with_namespace("myapp")
//!     .with_const_labels([("env", "prod")])
//!     .build();
//!
//! // Register a simple counter
//! let requests = <Counter>::default();
//! registry.register("requests", "Total requests processed", requests.clone())?;
//!
//! // Register a counter metric family for tracking requests with labels
//! let http_requests = Family::<Labels, Counter>::default();
//! registry.register(
//!     "http_requests",
//!     "Total HTTP requests",
//!     http_requests.clone()
//! )?;
//!
//! // Update the simple counter
//! requests.inc();
//! assert_eq!(requests.total(), 1);
//!
//! // Update the counter family
//! let labels = Labels { method: Method::Get, status: 200 };
//! http_requests.with_or_new(&labels, |req| req.inc());
//! assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));
//!
//! // Export metrics in text format
//! let mut output = String::new();
//! text::encode(&mut output, &registry)?;
//! // println!("{}", output);
//! assert!(output.contains(r#"myapp_http_requests_total{env="prod",method="Get",status="200"} 1"#));
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_crate_dependencies)]

pub mod encoder;
pub mod format;
pub mod metrics;
pub mod registry;
