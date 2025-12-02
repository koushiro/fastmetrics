use std::fmt;

/// [Open Metrics metric types](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#metric-types).
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MetricType {
    Unknown,
    Gauge,
    Counter,
    StateSet,
    Info,
    Histogram,
    GaugeHistogram,
    Summary,
}

impl MetricType {
    /// Return the string representation for the specified metric type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Gauge => "gauge",
            Self::Counter => "counter",
            Self::StateSet => "stateset",
            Self::Info => "info",
            Self::Histogram => "histogram",
            Self::GaugeHistogram => "gaugehistogram",
            Self::Summary => "summary",
        }
    }
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A trait that associates a type with a specific `Open Metrics` metric type.
///
/// This trait is used to enforce type-safe relationships between types representing metrics and
/// their corresponding `Open Metrics` types. Implementors must specify the metric type variant
/// through associated constants.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::raw::{MetricType, TypedMetric};
/// struct MyGauge;
///
/// impl TypedMetric for MyGauge {
///     const TYPE: MetricType = MetricType::Gauge;
/// }
/// ```
pub trait TypedMetric {
    /// The `Open Metrics` metric type associated with this type.
    const TYPE: MetricType;
}
