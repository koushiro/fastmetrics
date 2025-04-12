//! Encoder module provides traits for encoding metrics and their metadata.

mod exemplar;
mod label_set;
mod value;

use std::{fmt, time::Duration};

pub use self::{exemplar::*, label_set::*, value::*};
use crate::raw::{bucket::Bucket, quantile::Quantile, Metadata, MetricType};

/// Trait for encoding metric with metadata.
///
/// This trait is responsible for encoding metric with the metadata, which includes:
///
/// - name
/// - TYPE (gauge, counter, etc.)
/// - HELP
/// - UNIT (if any)
pub trait MetricFamilyEncoder {
    /// Encodes metric with metadata.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata to encode, containing name, type, help and unit
    /// * `metric` - The metric to encode
    fn encode(&mut self, metadata: &Metadata, metric: &dyn EncodeMetric) -> fmt::Result;
}

/// Trait for encoding different types of metrics.
///
/// This trait provides methods for encoding all supported metric types in the [OpenMetrics].
/// Each method handles a specific metric type with its associated data format.
///
/// [OpenMetrics]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#metric-types
pub trait MetricEncoder {
    /// Encodes an unknown metric.
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> fmt::Result;

    /// Encodes a gauge metric.
    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> fmt::Result;

    /// Encodes a counter metric.
    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        exemplar: Option<&dyn EncodeExemplar>,
        created: Option<Duration>,
    ) -> fmt::Result;

    /// Encodes a stateset metric.
    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> fmt::Result;

    /// Encodes an info metric.
    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> fmt::Result;

    /// Encodes a histogram metric.
    ///
    /// **NOTE**: the slice length of `buckets` and `exemplars` should be the same.
    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: &[Option<&dyn EncodeExemplar>],
        count: u64,
        sum: f64,
        created: Option<Duration>,
    ) -> fmt::Result;

    /// Encodes a gauge histogram metric.
    ///
    /// **NOTE**: the slice length of `buckets` and `exemplars` should be the same.
    fn encode_gauge_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: &[Option<&dyn EncodeExemplar>],
        count: u64,
        sum: f64,
    ) -> fmt::Result;

    /// Encodes a summary metric.
    fn encode_summary(
        &mut self,
        quantiles: &[Quantile],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> fmt::Result;

    /// Encodes a metric with the specified label set.
    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> fmt::Result;
}

/// Trait for types that can be encoded as metrics.
///
/// This trait is implemented by all metric types and provides methods for encoding
/// the metric's value and obtaining its type information.
pub trait EncodeMetric: Send + Sync {
    /// Encodes this metric using the provided [`MetricEncoder`].
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result;

    /// Returns the type of this metric (counter, gauge, etc.).
    fn metric_type(&self) -> MetricType;

    /// Returns the unix timestamp of this metric.
    fn timestamp(&self) -> Option<Duration> {
        None
    }
}

impl EncodeMetric for Box<dyn EncodeMetric> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        (**self).encode(encoder)
    }

    fn metric_type(&self) -> MetricType {
        (**self).metric_type()
    }

    fn timestamp(&self) -> Option<Duration> {
        (**self).timestamp()
    }
}
