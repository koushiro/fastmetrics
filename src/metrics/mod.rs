//! Core metric types and traits.
//!
//! This module provides the fundamental metric types specified by OpenMetrics:
//!
//! - [`Counter`]: Monotonically increasing values (e.g., request count)
//! - [`Gauge`]: Values that can go up and down (e.g., temperature)
//! - [`StateSet`]: A set of states that can be active/inactive
//! - [`Info`]: Static key-value information about the target
//! - [`Histogram`] (TODO): Statistical distribution of values
//! - [`GaugeHistogram`] (TODO): Like histogram but values can decrease
//! - [`Summary`] (TODO): Similar to histogram, with quantiles
//!
//! Each metric type comes in three variants:
//!
//! - Regular: Thread-safe metrics that can be shared between threads
//! - `Const`: Immutable metrics with constant values
//! - `Local`: Thread-local metrics for better performance
//!
//! The module also provides:
//!
//! - [`Family`]: Collections of metrics with the same name but different labels
//! - Common traits and types used by all metrics
//!
//! # Example
//!
//! ```rust
//! use openmetrics_client::metrics::{
//!     counter::Counter,
//!     gauge::Gauge,
//!     family::Family,
//! };
//!
//! // Create a simple counter
//! let requests = <Counter>::default();
//! requests.inc();
//!
//! // Create a gauge with an initial value
//! let temperature = Gauge::<f64>::new(23.5);
//! temperature.set(24.0);
//!
//! // Create a family of counters with labels
//! let requests_by_path = Family::<Vec<(String, String)>, Counter>::default();
//! requests_by_path
//!     .get_or_create(&vec![("path".into(), "/api/users".into())])
//!     .inc();
//! ```

pub mod family;
mod raw;
mod types;

pub use self::{raw::*, types::*};
