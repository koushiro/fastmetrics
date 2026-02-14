use super::{EscapingScheme, TextProfile};

#[derive(Clone, Copy)]
pub(super) struct ProfileConfig {
    pub(super) emit_eof: bool,
    pub(super) emit_unit: bool,
    pub(super) append_unit_suffix: bool,
    pub(super) append_counter_total_suffix: bool,
    pub(super) emit_created_series: bool,
    pub(super) emit_exemplars: bool,
    pub(super) prometheus_type_compat: bool,
    pub(super) timestamp_format: TimestampFormat,
    pub(super) name_policy: NamePolicy,
}

#[derive(Clone, Copy)]
pub(super) enum TimestampFormat {
    SecondsMillis,
    MillisecondsInteger,
}

#[derive(Clone, Copy)]
pub(super) enum NamePolicy {
    Legacy,
    V1Escaping(EscapingScheme),
}

impl NamePolicy {
    pub(super) const fn is_lossy(self) -> bool {
        matches!(self, Self::V1Escaping(EscapingScheme::Underscores | EscapingScheme::Dots))
    }
}

impl From<TextProfile> for ProfileConfig {
    fn from(profile: TextProfile) -> Self {
        match profile {
            TextProfile::PrometheusV0_0_4 => Self {
                emit_eof: false,
                emit_unit: false,
                append_unit_suffix: true,
                append_counter_total_suffix: false,
                emit_created_series: false,
                emit_exemplars: false,
                prometheus_type_compat: true,
                timestamp_format: TimestampFormat::MillisecondsInteger,
                name_policy: NamePolicy::Legacy,
            },
            TextProfile::PrometheusV1_0_0 { escaping_scheme } => Self {
                emit_eof: false,
                emit_unit: false,
                append_unit_suffix: true,
                append_counter_total_suffix: false,
                emit_created_series: false,
                emit_exemplars: false,
                prometheus_type_compat: true,
                timestamp_format: TimestampFormat::MillisecondsInteger,
                name_policy: NamePolicy::V1Escaping(escaping_scheme),
            },
            TextProfile::OpenMetricsV0_0_1 => Self {
                emit_eof: true,
                emit_unit: true,
                append_unit_suffix: true,
                append_counter_total_suffix: true,
                emit_created_series: true,
                emit_exemplars: true,
                prometheus_type_compat: false,
                timestamp_format: TimestampFormat::SecondsMillis,
                name_policy: NamePolicy::Legacy,
            },
            TextProfile::OpenMetricsV1_0_0 { escaping_scheme } => Self {
                emit_eof: true,
                emit_unit: true,
                append_unit_suffix: true,
                append_counter_total_suffix: true,
                emit_created_series: true,
                emit_exemplars: true,
                prometheus_type_compat: false,
                timestamp_format: TimestampFormat::SecondsMillis,
                name_policy: NamePolicy::V1Escaping(escaping_scheme),
            },
        }
    }
}
