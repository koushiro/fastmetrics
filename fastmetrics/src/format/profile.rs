/// Text exposition profile.
///
/// This controls how metrics are serialized in text format.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum TextProfile {
    /// Prometheus text 0.0.4 profile.
    Prometheus004,
    /// OpenMetrics text 1.x profile.
    #[default]
    OpenMetrics1,
}

impl TextProfile {
    /// Returns the HTTP content type for this profile.
    pub const fn content_type(self) -> &'static str {
        match self {
            Self::OpenMetrics1 => "application/openmetrics-text; version=1.0.0; charset=utf-8",
            Self::Prometheus004 => "text/plain; version=0.0.4; charset=utf-8",
        }
    }
}

/// Protobuf exposition profile shared by protobuf backends.
///
/// This type is re-exported by both `format::prost` and `format::protobuf`.
#[cfg(any(feature = "prost", feature = "protobuf"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProtobufProfile {
    /// Prometheus protobuf profile (`io.prometheus.client.MetricFamily`, length-delimited).
    #[default]
    Prometheus,
    /// OpenMetrics protobuf profile (`openmetrics.MetricSet`).
    OpenMetrics1,
}

#[cfg(any(feature = "prost", feature = "protobuf"))]
impl ProtobufProfile {
    /// Returns the HTTP content type for this profile.
    pub const fn content_type(self) -> &'static str {
        match self {
            Self::Prometheus => {
                "application/vnd.google.protobuf; proto=io.prometheus.client.MetricFamily; encoding=delimited"
            },
            Self::OpenMetrics1 => {
                "application/openmetrics-protobuf; version=1.0.0; proto=openmetrics.MetricSet"
            },
        }
    }
}
