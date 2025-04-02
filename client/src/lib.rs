//! OpenMetrics client library for Rust.
//!
//! This library provides a pure-Rust implementation of [OpenMetrics](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md),
//! a standard for transmitting cloud-native metrics at scale.
//! It is compatible with Prometheus and supports the text-based exposition format.
//!
//! # Features
//!
//! - Full OpenMetrics data model support
//! - Type-safe metric creation and manipulation
//! - Hierarchical metric organization with namespaces and subsystems
//! - Support for all metric types: Counter, Gauge, StateSet, Info
//! - Label sets and constant labels
//! - Text exposition format encoding
//!
//! # Usage
//!
//! The main components of this library are:
//!
//! - [Registry] - Central collection of all metrics
//! - Metric types in the [metrics] module (Counter, Gauge, etc.)
//! - [Family] for collecting metrics with the same label name but different label values
//! - Text format encoding via [format::text]
//!
//! [Registry]: crate::registry::Registry
//! [metrics]: crate::metrics
//! [Family]: crate::metrics::family::Family
//! [format::text]: crate::format::text
//!
//! # Example
//!
//! ```rust
//! use openmetrics_client::{
//!     encoder::{EncodeLabelSet, EncodeLabelValue, EncodeLabel, LabelSetEncoder, LabelEncoder},
//!     format::text,
//!     metrics::{counter::Counter, family::Family},
//!     registry::Registry,
//! };
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
//! #[derive(Clone, Eq, PartialEq, Hash)]
//! struct Labels {
//!     method: Method,
//!     status: u32,
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
//! // Register a counter metric family for tracking requests with labels
//! let http_requests = Family::<Labels, Counter>::default();
//! registry.register(
//!     "http_requests",
//!     "Total HTTP requests",
//!     http_requests.clone()
//! )?;
//!
//! // Update metrics
//! requests.inc();
//! assert_eq!(requests.total(), 1);
//!
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
