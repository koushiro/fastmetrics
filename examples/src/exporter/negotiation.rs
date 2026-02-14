//! Content negotiation helpers for exporter example endpoints.

use fastmetrics::format::text::{EscapingScheme, TextProfile};

/// Selects a text profile from `Accept` using a lightweight Accept header parser.
///
/// - `application/openmetrics-text; version=1.0.0` => OpenMetrics 1.0.0.
/// - `application/openmetrics-text; version=0.0.1` => OpenMetrics 0.0.1.
/// - `text/plain; version=1.0.0` => Prometheus 1.0.0.
/// - `text/plain` / `text/plain; version=0.0.4` => Prometheus 0.0.4.
/// - unsupported media types (or missing header) => `TextProfile::PrometheusV0_0_4`.
pub fn text_profile_from_accept(accept: Option<&str>) -> TextProfile {
    text_profile_from_accept_with_fallback(accept, TextProfile::PrometheusV0_0_4)
}

/// Selects a text profile from `Accept` using a lightweight Accept header parser.
///
/// - `application/openmetrics-text; version=1.0.0` => OpenMetrics 1.0.0.
/// - `application/openmetrics-text; version=0.0.1` => OpenMetrics 0.0.1.
/// - `text/plain; version=1.0.0` => Prometheus 1.0.0.
/// - `text/plain` / `text/plain; version=0.0.4` => Prometheus 0.0.4.
/// - unsupported media types (or missing header) => `fallback`.
pub fn text_profile_from_accept_with_fallback(
    accept: Option<&str>,
    fallback: TextProfile,
) -> TextProfile {
    let accept = match accept {
        Some(value) if !value.trim().is_empty() => value,
        _ => return fallback,
    };

    let mut selected = fallback;
    let mut best_q = 0.0_f32;

    for segment in accept.split(',') {
        let segment = segment.trim();
        if segment.is_empty() {
            continue;
        }

        let mut parts = segment.split(';');
        let media_type = parts.next().unwrap_or_default().trim().to_ascii_lowercase();
        if media_type.is_empty() || media_type == "*/*" {
            continue;
        }

        let mut version: Option<&str> = None;
        let mut escaping: Option<EscapingScheme> = None;
        let mut quality = 1.0_f32;

        for part in parts {
            let part = part.trim();
            let (key, value) = match part.split_once('=') {
                Some((key, value)) => {
                    (key.trim().to_ascii_lowercase(), value.trim().trim_matches('"'))
                },
                None => continue,
            };
            match key.as_str() {
                "version" => version = Some(value),
                "escaping" => escaping = parse_escaping_scheme(value),
                "q" => quality = value.parse::<f32>().unwrap_or(1.0_f32).clamp(0.0, 1.0),
                _ => {},
            }
        }

        if quality == 0.0 {
            continue;
        }

        let profile = match (media_type.as_str(), version) {
            ("application/openmetrics-text", Some("1.0.0")) => {
                Some(TextProfile::OpenMetricsV1_0_0 {
                    escaping_scheme: escaping.unwrap_or_default(),
                })
            },
            ("application/openmetrics-text", Some("0.0.1")) => Some(TextProfile::OpenMetricsV0_0_1),
            ("text/plain", Some("1.0.0")) => Some(TextProfile::PrometheusV1_0_0 {
                escaping_scheme: escaping.unwrap_or_default(),
            }),
            ("text/plain", Some("0.0.4")) | ("text/plain", None) => {
                Some(TextProfile::PrometheusV0_0_4)
            },
            _ => None,
        };

        if let Some(profile) = profile {
            if quality > best_q {
                selected = profile;
                best_q = quality;
            }
        }
    }

    selected
}

fn parse_escaping_scheme(value: &str) -> Option<EscapingScheme> {
    let value = value.trim().to_ascii_lowercase();
    match value.as_str() {
        "allow-utf-8" => Some(EscapingScheme::AllowUtf8),
        "underscores" => Some(EscapingScheme::Underscores),
        "dots" => Some(EscapingScheme::Dots),
        "values" => Some(EscapingScheme::Values),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_openmetrics_profile_from_accept() {
        let profile = text_profile_from_accept_with_fallback(
            Some("application/openmetrics-text; version=1.0.0; escaping=values"),
            TextProfile::PrometheusV0_0_4,
        );
        assert!(matches!(
            profile,
            TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Values }
        ));
    }

    #[test]
    fn select_prometheus_profile_from_accept() {
        let profile = text_profile_from_accept_with_fallback(
            Some("text/plain; version=1.0.0; escaping=allow-utf-8, text/plain; q=0.1"),
            TextProfile::PrometheusV0_0_4,
        );
        assert!(matches!(
            profile,
            TextProfile::PrometheusV1_0_0 { escaping_scheme: EscapingScheme::AllowUtf8 }
        ));
    }

    #[test]
    fn fallback_to_default_when_no_accept() {
        let expected = TextProfile::PrometheusV0_0_4;
        let profile = text_profile_from_accept(None);
        assert_eq!(profile, expected);
    }

    #[test]
    fn fallback_to_custom_when_no_accept() {
        let expected =
            TextProfile::PrometheusV1_0_0 { escaping_scheme: EscapingScheme::Underscores };
        let profile = text_profile_from_accept_with_fallback(None, expected);
        assert_eq!(profile, expected);
    }
}
