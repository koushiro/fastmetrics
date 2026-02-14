//! # FastMetrics
//!
//! [![](https://github.com/koushiro/fastmetrics/actions/workflows/ci.yml/badge.svg)][actions]
//! [![](https://img.shields.io/docsrs/fastmetrics)][docs.rs]
//! [![](https://img.shields.io/crates/v/fastmetrics)][crates.io]
//! [![](https://img.shields.io/crates/l/fastmetrics)][crates.io]
//! [![](https://img.shields.io/crates/d/fastmetrics)][crates.io]
//! [![](https://img.shields.io/badge/MSRV-1.85.0-green?logo=rust)][whatrustisit]
//! [![](https://deepwiki.com/badge.svg)][deepwiki]
//!
//! [actions]: https://github.com/koushiro/fastmetrics/actions
//! [docs.rs]: https://docs.rs/fastmetrics
//! [crates.io]: https://crates.io/crates/fastmetrics
//! [whatrustisit]: https://www.whatrustisit.com
//! [deepwiki]: https://deepwiki.com/koushiro/fastmetrics
//!
//! OpenMetrics / Prometheus client library for Rust.
//!
//! A pure-Rust implementation of the [OpenMetrics] specification for transmitting cloud-native
//! metrics at scale, and it's compatible with [Prometheus].
//!
//! ## Features
//!
//! - Full support for [OpenMetrics] specification
//! - Fast encoding in both text and protobuf exposition format
//!   - Text
//!     - Prometheus text: `0.0.4`, `1.0.0`
//!     - OpenMetrics text: `0.0.1`, `1.0.0`
//!     - V1 escaping schemes: `allow-utf-8`, `underscores`, `dots`, `values`
//!   - Protobuf
//!     - [Prometheus protobuf schema]
//!     - [OpenMetrics protobuf schema]
//! - Customizable metric types (currently a set of commonly used metric types are provided)
//! - Hierarchical metric organization with namespaces and subsystems
//! - Support for variable and constant labels
//! - Derive macros to simplify code (e.g., like registering metrics, label handling, etc.)
//!
//! [Prometheus]: https://prometheus.io
//! [OpenMetrics]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md
//! [Prometheus protobuf schema]: https://github.com/prometheus/client_model/blob/master/io/prometheus/client/metrics.proto
//! [OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto
//!
//! ## Example
//!
//! ```rust
//! use fastmetrics::{
//!     encoder::{EncodeLabel, EncodeLabelSet, EncodeLabelValue, LabelSetEncoder, LabelEncoder},
//!     error::Result,
//!     format::text::{self, TextProfile},
//!     metrics::{counter::Counter, family::Family},
//!     raw::LabelSetSchema,
//!     registry::Registry,
//! };
//!
//! #[derive(Clone, Eq, PartialEq, Hash)]
//! struct Labels {
//!     method: Method,
//!     status: u16,
//! }
//!
//! // Can use `#[derive(EncodeLabelSet, LabelSetSchema)]` to simplify the code, but need to enable `derive` feature
//!
//! impl LabelSetSchema for Labels {
//!     fn names() -> Option<&'static [&'static str]> {
//!         Some(&["method", "status"])
//!     }
//! }
//!
//! impl EncodeLabelSet for Labels {
//!     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
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
//!     fn encode(&self, encoder: &mut dyn LabelEncoder) -> Result<()> {
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
//!     .build()?;
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
//! text::encode(&mut output, &registry, TextProfile::default())?;
//! // println!("{}", output);
//! assert!(output.contains(r#"myapp_http_requests_total{env="prod",method="Get",status="200"} 1"#));
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Enforce platform requirements without interfering with crate-level inner docs (`//!`).
// This must live after the crate docs block.
#[cfg(not(target_has_atomic = "64"))]
compile_error!("fastmetrics requires 64-bit atomic support (target_has_atomic = \"64\").");

#[cfg(feature = "derive")]
pub use fastmetrics_derive as derive;

pub mod encoder;
pub mod error;
pub mod format;
pub mod metrics;
pub mod raw;
pub mod registry;
