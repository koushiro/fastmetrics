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
    /// Not implemented yet.
    Histogram,
    /// Not implemented yet.
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
/// their corresponding `Open Metrics` types. Implementors must specify the metric type variant,
/// which can be one of the variants defined in [`MetricType`].
pub trait TypedMetric {
    /// The `Open Metrics` metric type associated with this type.
    const TYPE: MetricType;
}
