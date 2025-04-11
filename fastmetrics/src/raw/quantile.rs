//!  Provides quantile-related functionality for summary metrics in the OpenMetrics.

/// The label that defines the quantile in a summary.
pub const QUANTILE_LABEL: &str = "quantile";

/// Represents a single quantile measurement with its value.
///
/// A quantile combines a specific quantile point (e.g., 0.5 for median, 0.99 for 99th percentile)
/// with its corresponding value in the distribution.
#[derive(Copy, Clone, Debug)]
pub struct Quantile {
    quantile: f64,
    value: f64,
}

impl Quantile {
    /// Creates a new [`Quantile`] instance.
    ///
    /// # Arguments
    ///
    /// * `quantile` - The quantile point (e.g., 0.5 for median, 0.99 for 99th percentile), MUST be
    ///   between 0 and 1 inclusive.
    /// * `value` - The value at this quantile point, MUST NOT be negative
    pub const fn new(quantile: f64, value: f64) -> Self {
        Self { quantile, value }
    }

    /// Returns the quantile point.
    ///
    /// The quantile is a number between 0 and 1 representing where this measurement sits in the
    /// distribution (e.g., 0.5 for median).
    pub const fn quantile(&self) -> f64 {
        self.quantile
    }

    /// Returns the value at this quantile point.
    ///
    /// This is the actual measured value corresponding to this quantile in the distribution.
    pub const fn value(&self) -> f64 {
        self.value
    }
}
