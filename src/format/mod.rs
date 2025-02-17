//! Wire formats
//!
//! This module provides implementations of different metric exposition formats.
//!
//! Supported formats:
//!
//! # Text format
//!
//! [OpenMetrics text format] MUST be supported and is the default.
//!
//! ## Example
//!
//! ```rust
//! # use openmetrics_client::{
//! #     format::text,
//! #     metrics::counter::Counter,
//! #     registry::Registry,
//! # };
//! #
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = Registry::default();
//!
//! let requests = <Counter>::default();
//! registry.register("requests", "Number of requests", requests.clone())?;
//! requests.inc();
//!
//! // Export metrics in text format
//! let mut output = String::new();
//! text::encode(&mut output, &registry)?;
//! println!("{}", output);
//! # Ok(())
//! # }
//! ```
//!
//! [OpenMetrics text format]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#text-format
//!
//! # Protobuf format (TODO)
//!
//! [OpenMetrics protobuf format] MUST follow the proto3 version of the protocol buffer language and
//! all payloads MUST be a single binary encoded MetricSet message, as defined by the [OpenMetrics
//! protobuf schema].
//!
//! ## Example
//!
//! TODO
//!
//! [OpenMetrics protobuf format]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#protobuf-format
//! [OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto

pub mod text;
