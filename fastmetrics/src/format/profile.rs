/// Text exposition profile.
///
/// This controls how metrics are serialized in text format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TextProfile {
    /// Prometheus text 0.0.4 profile.
    PrometheusV0_0_4,
    /// Prometheus text 1.0.0 profile.
    PrometheusV1_0_0 {
        /// Escaping scheme for UTF-8 metric and label names.
        escaping_scheme: EscapingScheme,
    },
    /// OpenMetrics text 0.0.1 profile.
    OpenMetricsV0_0_1,
    /// OpenMetrics text 1.0.0 profile.
    OpenMetricsV1_0_0 {
        /// Escaping scheme for UTF-8 metric and label names.
        escaping_scheme: EscapingScheme,
    },
}

/// UTF-8 metric and label name escaping scheme for text exposition.
///
/// These values correspond to the `escaping=<scheme>` parameter used in scrape
/// content negotiation for text 1.0 profiles.
///
/// Per Prometheus escaping-scheme guidance, when the scraper does not send an
/// explicit `escaping` parameter, `underscores` SHOULD be assumed. FastMetrics
/// does not perform HTTP negotiation internally, so callers should select the
/// scheme explicitly.
///
/// Reference:
/// <https://prometheus.io/docs/instrumenting/escaping_schemes/>
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum EscapingScheme {
    /// `escaping=allow-utf-8`
    ///
    /// Keep metric and label names as UTF-8 without name translation.
    ///
    /// Escaping in the text payload still applies where required by the text
    /// format (for example `\`, `\n`, and `"` in quoted contexts).
    AllowUtf8,
    /// `escaping=underscores`
    ///
    /// Replace every character outside the legacy name character set
    /// (`a-zA-Z0-9_:`) with `_`.
    ///
    /// Example: `metric.name/with/slashes` -> `metric_name_with_slashes`.
    #[default]
    Underscores,
    /// `escaping=dots`
    ///
    /// Apply a dot-preserving transformation:
    /// - `.` -> `_dot_`
    /// - `_` -> `__`
    /// - Other legacy-invalid characters -> `_`
    ///
    /// Example: `metric.name.with.dots` -> `metric_dot_name_dot_with_dot_dots`.
    Dots,
    /// `escaping=values`
    ///
    /// Encode names in a fully round-trippable form:
    /// - Prefix with `U__`
    /// - Legacy-invalid characters become `_HEX_` (Unicode code point in hex)
    /// - `_` -> `__`
    ///
    /// Example: `metric.name` -> `U__metric_2E_name`.
    Values,
}

impl Default for TextProfile {
    fn default() -> Self {
        Self::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::default() }
    }
}

macro_rules! text_v1_content_type {
    ($base:literal, $escaping_scheme:expr) => {
        match $escaping_scheme {
            EscapingScheme::AllowUtf8 => concat!($base, "; escaping=allow-utf-8"),
            EscapingScheme::Underscores => concat!($base, "; escaping=underscores"),
            EscapingScheme::Dots => concat!($base, "; escaping=dots"),
            EscapingScheme::Values => concat!($base, "; escaping=values"),
        }
    };
}

impl TextProfile {
    /// Returns the HTTP content type for this profile.
    pub const fn content_type(self) -> &'static str {
        match self {
            Self::PrometheusV0_0_4 => "text/plain; version=0.0.4; charset=utf-8",
            Self::PrometheusV1_0_0 { escaping_scheme } => {
                text_v1_content_type!("text/plain; version=1.0.0; charset=utf-8", escaping_scheme)
            },
            Self::OpenMetricsV0_0_1 => "application/openmetrics-text; version=0.0.1; charset=utf-8",
            Self::OpenMetricsV1_0_0 { escaping_scheme } => {
                text_v1_content_type!(
                    "application/openmetrics-text; version=1.0.0; charset=utf-8",
                    escaping_scheme
                )
            },
        }
    }
}

impl TextProfile {
    /// Returns the escaping scheme for this profile.
    pub const fn escaping_scheme(self) -> Option<EscapingScheme> {
        match self {
            Self::PrometheusV0_0_4 | Self::OpenMetricsV0_0_1 => None,
            Self::PrometheusV1_0_0 { escaping_scheme }
            | Self::OpenMetricsV1_0_0 { escaping_scheme } => Some(escaping_scheme),
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
