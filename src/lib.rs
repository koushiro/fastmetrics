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
//!     format::text,
//!     metrics::{counter::Counter, gauge::Gauge, family::Family},
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
//! // Create a metric family for tracking requests with labels
//! type Labels = Vec<(&'static str, &'static str)>;
//! let requests_by_path = Family::<Labels, Counter>::default();
//! registry.register(
//!     "requests_by_path",
//!     "Requests broken down by path",
//!     requests_by_path.clone()
//! )?;
//!
//! // Update metrics
//! requests.inc();
//! assert_eq!(requests.total(), 1);
//!
//! let labels = vec![("path", "/api/v1/users")];
//! requests_by_path.with_or_default(&labels, |req| req.inc());
//! assert_eq!(requests_by_path.with(&labels, |req| req.total()), Some(1));
//!
//! // Export metrics in text format
//! let mut output = String::new();
//! text::encode(&mut output, &registry)?;
//! // println!("{}", output);
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
