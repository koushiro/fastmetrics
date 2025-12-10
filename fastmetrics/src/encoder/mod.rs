//! Encoder module provides traits for encoding metrics and their metadata.

mod exemplar;
mod label_set;
mod value;

use std::time::Duration;

pub use self::{exemplar::*, label_set::*, value::*};
use crate::{
    error::Result,
    raw::{Metadata, bucket::Bucket, quantile::Quantile},
};

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
    fn encode(&mut self, metadata: &Metadata, metric: &dyn EncodeMetric) -> Result<()>;
}

/// Trait for encoding different types of metrics.
///
/// This trait provides methods for encoding all supported metric types in the [OpenMetrics].
/// Each method handles a specific metric type with its associated data format.
///
/// [OpenMetrics]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#metric-types
pub trait MetricEncoder {
    /// Encodes an unknown metric.
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> Result<()>;

    /// Encodes a gauge metric.
    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> Result<()>;

    /// Encodes a counter metric.
    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        exemplar: Option<&dyn EncodeExemplar>,
        created: Option<Duration>,
    ) -> Result<()>;

    /// Encodes a stateset metric.
    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> Result<()>;

    /// Encodes an info metric.
    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> Result<()>;

    /// Encodes a histogram metric.
    ///
    /// **NOTE**: when `exemplars` is provided, its slice length must match `buckets`.
    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
        count: u64,
        sum: f64,
        created: Option<Duration>,
    ) -> Result<()>;

    /// Encodes a gauge histogram metric.
    ///
    /// **NOTE**: when `exemplars` is provided, its slice length must match `buckets`.
    fn encode_gauge_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
        count: u64,
        sum: f64,
    ) -> Result<()>;

    /// Encodes a summary metric.
    fn encode_summary(
        &mut self,
        quantiles: &[Quantile],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> Result<()>;

    /// Encodes a metric with the specified label set.
    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> Result<()>;
}

/// Trait for types that can be encoded as metrics.
///
/// This trait is implemented by all metric types and provides methods for encoding
/// the metric's value and collecting its type information.
pub trait EncodeMetric: Send + Sync {
    /// Encodes this metric using the provided [`MetricEncoder`].
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()>;

    /// Returns the unix timestamp of this metric.
    fn timestamp(&self) -> Option<Duration> {
        None
    }

    /// Returns `true` if the metric is empty.
    ///
    /// An "empty" metric is one that has no data points to expose.
    /// This is especially useful for metric families, which can be considered empty
    /// if they contain no individual metrics. In such cases, the `# TYPE`, `# HELP`, and `# UNIT`
    /// lines for the metric family will not be rendered.
    ///
    /// The definition of "empty" depends on the metric type:
    // - `Counter`, `Gauge`, `Histogram` and `GaugeHistogram` are never considered empty.
    ///   A value of `0` is a valid and meaningful state that should be exposed.
    ///   An unobserved `Histogram` is still represented with a `_count` of `0`, a `_sum` of `0`,
    ///   and bucket counts of `0`.
    /// - `StateSet` and `Info` are never considered empty.
    /// - A `Summary` might be considered empty if it has not recorded any observations.
    /// - A `Family` is empty if it contains no labeled metrics.
    ///
    /// By default, this method returns `false`, assuming a metric is not empty.
    fn is_empty(&self) -> bool {
        false
    }
}

impl EncodeMetric for Box<dyn EncodeMetric> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
        (**self).encode(encoder)
    }

    fn timestamp(&self) -> Option<Duration> {
        (**self).timestamp()
    }

    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}
