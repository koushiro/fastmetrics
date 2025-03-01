//! Encoder module provides traits for encoding metrics and their metadata.

mod label_set;
mod number;

use std::{fmt, time::Duration};

pub use self::{label_set::*, number::*};
use crate::metrics::{
    counter::*, family::*, gauge::*, histogram::*, info::*, state_set::*, summary::*, unknown::*,
    MetricType, TypedMetric,
};

/// Trait for encoding metric metadata.
///
/// This trait is responsible for encoding the metadata associated with metrics, including:
///
/// - name
/// - TYPE (gauge, counter, etc.)
/// - HELP
/// - UNIT (if any)
pub trait MetricFamilyEncoder {
    /// Encodes metadata of a metric and returns a [`MetricEncoder`] to encode the metric itself.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata to encode, containing name, type, help and unit
    ///
    /// # Returns
    ///
    /// Returns a [`MetricEncoder`] that can be used to encode the actual metric values.
    fn encode_metadata<'s>(
        &'s mut self,
        metadata: &'s Metadata,
    ) -> Result<Box<dyn MetricEncoder + 's>, fmt::Error>;
}

/// Trait for encoding different types of metrics.
///
/// This trait provides methods for encoding all supported metric types in the [OpenMetrics specification](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#metric-types).
/// Each method handles a specific metric type with its associated data format.
pub trait MetricEncoder {
    /// Encodes an unknown metric.
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> fmt::Result;

    /// Encodes a gauge metric.
    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> fmt::Result;

    /// Encodes a counter metric.
    ///
    /// This method handles both the `total` value and optional `created` timestamp.
    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        created: Option<Duration>,
    ) -> fmt::Result;

    /// Encodes a stateset metric.
    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> fmt::Result;

    /// Encodes an info metric.
    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> fmt::Result;

    /// Encodes a histogram metric.
    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> fmt::Result;

    /// Encodes a summary metric.
    fn encode_summary(
        &mut self,
        quantiles: &[Quantile],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> fmt::Result;

    /// Creates an encoder for a metric family with the specified label set.
    fn encode_family<'s>(
        &'s mut self,
        label_set: &'s dyn EncodeLabelSet,
    ) -> Result<Box<dyn MetricEncoder + 's>, fmt::Error>;
}

/// Trait for types that can be encoded as metrics.
///
/// This trait is implemented by all metric types and provides methods for encoding
/// the metric's value and obtaining its type information.
pub trait EncodeMetric {
    /// Encodes this metric using the provided [`MetricEncoder`].
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result;

    /// Returns the type of this metric (counter, gauge, etc.).
    fn metric_type(&self) -> MetricType;
}

////////////////////////////////////////////////////////////////////////////////

impl<T: EncodeUnknownValue + UnknownValue> EncodeMetric for Unknown<T> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_unknown(self.get())
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Unknown
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<N: EncodeGaugeValue + GaugeValue> EncodeMetric for Gauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_gauge(&self.get())
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

impl<N: EncodeGaugeValue + GaugeValue> EncodeMetric for ConstGauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_gauge(&self.get())
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

impl<N: EncodeGaugeValue + GaugeValue> EncodeMetric for LocalGauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_gauge(&self.get())
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<N: EncodeCounterValue + CounterValue> EncodeMetric for Counter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let (total, created) = self.get();
        encoder.encode_counter(&total, created)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Counter
    }
}

impl<N: EncodeCounterValue + CounterValue> EncodeMetric for ConstCounter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let (total, created) = self.get();
        encoder.encode_counter(&total, created)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Counter
    }
}

impl<N: EncodeCounterValue + CounterValue> EncodeMetric for LocalCounter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let (total, created) = self.get();
        encoder.encode_counter(&total, created)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Counter
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<T: StateSetValue> EncodeMetric for StateSet<T> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let states = self.get();
        encoder.encode_stateset(states)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::StateSet
    }
}

impl<T: StateSetValue> EncodeMetric for ConstStateSet<T> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let states = self.get();
        encoder.encode_stateset(states)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::StateSet
    }
}

impl<T: StateSetValue> EncodeMetric for LocalStateSet<T> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let states = self.get();
        encoder.encode_stateset(states)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::StateSet
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<LS: EncodeLabelSet> EncodeMetric for Info<LS> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_info(self.get())
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Info
    }
}

////////////////////////////////////////////////////////////////////////////////

impl EncodeMetric for Histogram {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let (buckets, sum, count, created) = self.get();
        encoder.encode_histogram(&buckets, sum, count, created)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Histogram
    }
}

impl EncodeMetric for ConstHistogram {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let (buckets, sum, count, created) = self.get();
        encoder.encode_histogram(&buckets, sum, count, created)
    }

    fn metric_type(&self) -> MetricType {
        MetricType::Histogram
    }
}

////////////////////////////////////////////////////////////////////////////////

// GaugeHistogram

////////////////////////////////////////////////////////////////////////////////

// Summary

////////////////////////////////////////////////////////////////////////////////

impl<LS, M, S> EncodeMetric for Family<LS, M, S>
where
    LS: EncodeLabelSet,
    M: EncodeMetric + TypedMetric,
{
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let guard = self.read();
        for (labels, metric) in guard.iter() {
            let mut encoder = encoder.encode_family(labels)?;
            metric.encode(encoder.as_mut())?;
        }
        Ok(())
    }

    fn metric_type(&self) -> MetricType {
        <M as TypedMetric>::TYPE
    }
}
