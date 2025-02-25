//! Core metric types and traits.
//!
//! This module provides the fundamental metric types specified by OpenMetrics:
//!
//! - [Counter]: Monotonically increasing values (e.g., request count)
//! - [Gauge]: Values that can go up and down (e.g., temperature)
//! - [StateSet]: A set of states that can be active/inactive
//! - [Info]: Static key-value information about the target
//! - [Histogram] (TODO): Statistical distribution of values
//! - [GaugeHistogram] (TODO): Like histogram but values can decrease
//! - [Summary] (TODO): Similar to histogram, with quantiles
//!
//! Each metric type comes in three variants:
//!
//! - Regular: Thread-safe metrics that can be shared between threads
//! - `Const`: Immutable metrics with constant values
//! - `Local`: Thread-local metrics for better performance
//!
//! The module also provides:
//!
//! - [Family]: Collections of metrics with the same name but different labels
//! - Common traits and types used by all metrics
//!
//! [Counter]: self::counter
//! [Gauge]: self::gauge
//! [StateSet]: self::state_set
//! [Info]: self::info
//! [Histogram]: self::histogram
//! [GaugeHistogram]: self::gauge_histogram
//! [Summary]: self::summary
//! [Family]: self::family::Family

pub mod family;
pub mod raw;
mod types;

pub use self::types::*;
