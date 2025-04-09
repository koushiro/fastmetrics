pub mod counter;
pub mod gauge;
pub mod gauge_histogram;
pub mod histogram;
pub mod info;
pub mod state_set;
pub mod summary;
pub mod unknown;

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
    /// Not implemented yet.
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

/// A trait that associates a type with a specific `Open Metrics` metric type.
///
/// This trait is used to enforce type-safe relationships between types representing metrics and
/// their corresponding `Open Metrics` types. Implementors must specify the metric type variant
/// and timestamp behavior through associated constants.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::{MetricType, TypedMetric};
/// struct MyGauge;
///
/// impl TypedMetric for MyGauge {
///     const TYPE: MetricType = MetricType::Gauge;
///     const WITH_TIMESTAMP: bool = true;
/// }
/// ```
pub trait TypedMetric {
    /// The `Open Metrics` metric type associated with this type.
    const TYPE: MetricType;

    /// Controls whether this metric includes timestamp information.
    ///
    /// When set to `true`, timestamps will be included when the metric is exported.
    /// This is useful for metrics that need to track when values were recorded.
    const WITH_TIMESTAMP: bool;
}
