//! [Open Metrics Unknown](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#unknown) metric type.

use std::fmt;

use crate::{
    encoder::{EncodeMetric, EncodeUnknownValue, MetricEncoder},
    raw::{MetricType, Number, TypedMetric},
};

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
    const WITH_TIMESTAMP: bool = false;
}

impl<T: EncodeUnknownValue + UnknownValue> EncodeMetric for Unknown<T> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_unknown(self.get())
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::check_text_encoding;

    #[test]
    fn test_text_encoding() {
        check_text_encoding(
            |registry| {
                let unknown = Unknown::new(1);
                registry.register("my_unknown", "My unknown help", unknown.clone()).unwrap();
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_unknown unknown
                    # HELP my_unknown My unknown help
                    my_unknown 1
                    # EOF
                "#};
                assert_eq!(output, expected);
            },
        );
    }
}
