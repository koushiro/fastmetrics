//! Label metadata contracts shared across the crate.
//!
//! This module provides two lightweight traits:
//! - [`LabelSetSchema`]: describes the schema of a label set type.
//! - [`MetricLabelSet`]: declares which [`LabelSetSchema`] a metric type is bound to.
//!
//! Together they form the foundation for reasoning about whether a metric
//! supports labels, and if so, which label names it expects.

/// Describes the schema (names) of a label set.
///
/// Implement this trait for every label set structure.
pub trait LabelSetSchema {
    /// Returns the canonical label names for this schema.
    ///
    /// A return value of `None` means the schema carries no labels.
    fn names() -> Option<&'static [&'static str]>;
}

/// Types that do not carry any labels can simply rely on the unit type `()` implementation.
impl LabelSetSchema for () {
    fn names() -> Option<&'static [&'static str]> {
        None
    }
}

/// Declares the label set schema associated with a metric type.
///
/// Metric implementations should set `LabelSet` to the label structure they
/// expect. For metrics without labels, use the unit type `()`.
pub trait MetricLabelSet {
    /// The label set schema used by this metric.
    type LabelSet: LabelSetSchema;
}
