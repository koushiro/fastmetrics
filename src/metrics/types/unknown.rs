//! [Open Metrics Unknown](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#unknown) metric type.

pub use crate::metrics::raw::Number;
use crate::metrics::{MetricType, TypedMetric};

/// A marker trait for **unknown** metric value.
pub trait UnknownValue: Number {}

impl UnknownValue for i32 {}
impl UnknownValue for i64 {}
impl UnknownValue for isize {}
impl UnknownValue for u32 {}
impl UnknownValue for f32 {}
impl UnknownValue for f64 {}

/// Open Metrics [`Unknown`] metric, which **SHOULD NOT** be used. It **MAY** be used only when
/// it's impossible to determine the types of individual metrics from 3rd party systems.
///
/// # NOTE
///
/// A point in a metric with the [`Unknown`] type **MUST** have a single value.
#[derive(Clone, Debug)]
pub struct Unknown<T> {
    value: T,
}

impl<T: UnknownValue> Unknown<T> {
    /// Create an [`Unknown`] metric with the given unknown value.
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Get the unknown value of the [`Unknown`] metric.
    pub fn get(&self) -> &T {
        &self.value
    }
}

impl<T: UnknownValue> TypedMetric for Unknown<T> {
    const TYPE: MetricType = MetricType::Unknown;
}
