//! [Open Metrics Info](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#info) metric type.

use crate::raw::{MetricType, TypedMetric};

/// Open Metrics [`Info`] metric, which is used to expose textual information which SHOULD NOT
/// change during process lifetime.
#[derive(Clone, Debug)]
pub struct Info<LS> {
    label_set: LS,
}

impl<LS> Info<LS> {
    /// Creates an [`Info`] metric with the given label set.
    pub fn new(label_set: LS) -> Self {
        Self { label_set }
    }

    /// Gets the label set of the [`Info`].
    pub fn get(&self) -> &LS {
        &self.label_set
    }
}

impl<LS> TypedMetric for Info<LS> {
    const TYPE: MetricType = MetricType::Info;
    const WITH_TIMESTAMP: bool = false;
}
