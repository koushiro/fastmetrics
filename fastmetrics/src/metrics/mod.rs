//! Core metric types and traits.
//!
//! This module provides the fundamental metric types specified by OpenMetrics:
//!
//! - [Counter]: Monotonically increasing values (e.g., request count)
//! - [Gauge]: Values that can go up and down (e.g., temperature)
//! - [StateSet]: A set of states that can be active/inactive
//! - [Info]: Static key-value information about the target
//! - [Histogram]: Statistical distribution of values
//! - [GaugeHistogram]: Like histogram but values can decrease
//! - [Summary] (Not Implemented): Similar to histogram, with quantiles
//!
//! The module also provides:
//!
//! - [Family]: Collections of metrics with the same name but different labels
//! - [IndexedFamily]: Fixed-cardinality metric families indexed by label values
//! - [LabelIndexMapping]: Trait for stable label-to-index mapping
//! - [LabelIndex]: Reusable index token for indexed family lookups
//!
//! [Counter]: self::counter
//! [Gauge]: self::gauge
//! [StateSet]: self::state_set
//! [Info]: self::info
//! [Histogram]: self::histogram
//! [GaugeHistogram]: self::gauge_histogram
//! [Summary]: self::summary
//! [Family]: self::family::Family
//! [IndexedFamily]: self::family::IndexedFamily
//! [LabelIndexMapping]: self::family::LabelIndexMapping
//! [LabelIndex]: self::family::LabelIndex

pub mod family;
mod internal;
pub mod lazy_group;
mod types;

pub use self::types::*;

#[cfg(test)]
fn check_text_encoding<S, H>(setup: S, handle: H)
where
    S: Fn(&mut crate::registry::Registry),
    H: Fn(String),
{
    use crate::{
        format::text::{self, TextProfile},
        registry::Registry,
    };

    let mut registry = Registry::default();

    setup(&mut registry);

    let mut output = String::new();
    text::encode(&mut output, &registry, TextProfile::default()).unwrap();

    handle(output);
}
