//! Metric format encodings.
//!
//! This module provides implementations of different metric exposition formats.
//! Currently supported formats:
//!
//! - Text format: [OpenMetrics text format]((https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#text-format)), compatible with Prometheus
//!
//! # Example
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

pub mod text;
