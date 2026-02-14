//! Content negotiation helpers for exporter example endpoints.

use fastmetrics::format::text::{EscapingScheme, TextProfile};

/// Selects a text profile from `Accept` using a lightweight Accept header parser.
///
/// - `application/openmetrics-text; version=1.0.0` => OpenMetrics 1.0.0.
/// - `application/openmetrics-text; version=0.0.1` => OpenMetrics 0.0.1.
/// - `text/plain; version=1.0.0` => Prometheus 1.0.0.
/// - `text/plain` / `text/plain; version=0.0.4` => Prometheus 0.0.4.
/// - unsupported media types (or missing header) => fallback profile.
pub fn text_profile_from_accept(accept: Option<&str>) -> TextProfile {
    text_profile_from_accept_with_fallback(accept, TextProfile::PrometheusV0_0_4)
}

#[derive(Copy, Clone)]
struct MediaCandidate {
    profile: TextProfile,
    quality: f32,
    specificity: u8,
}

impl MediaCandidate {
    fn is_preferred_to(self, current: Option<Self>) -> bool {
        match current {
            None => true,
            Some(previous) => {
                self.quality > previous.quality
                    || (self.quality == previous.quality && self.specificity > previous.specificity)
            }
        }
    }
}

/// Selects a text profile from `Accept` using a lightweight Accept header parser.
///
/// - `application/openmetrics-text; version=1.0.0` => OpenMetrics 1.0.0.
/// - `application/openmetrics-text; version=0.0.1` => OpenMetrics 0.0.1.
/// - `text/plain; version=1.0.0` => Prometheus 1.0.0.
/// - `text/plain` / `text/plain; version=0.0.4` => Prometheus 0.0.4.
/// - `*/*` (wildcard) triggers the fallback profile when no better concrete match.
/// - unsupported media types (or missing header) => fallback.
pub fn text_profile_from_accept_with_fallback(
    accept: Option<&str>,
    fallback: TextProfile,
) -> TextProfile {
    // Fast path: if no Accept header is provided, keep previous behavior and
    // return the configured fallback profile.
    let accept = match accept {
        Some(value) if !value.trim().is_empty() => value,
        _ => return fallback,
    };

    let mut best: Option<MediaCandidate> = None;
    // `*/*` is treated as a fallback quality advertisement only. It does not
    // represent a concrete profile match and only activates when no better
    // concrete choice exists.
    let mut wildcard_quality = 0.0_f32;

    for segment in accept.split(',') {
        let segment = segment.trim();
        if segment.is_empty() {
            continue;
        }

        let mut parts = segment.split(';');
        let media_type = parts.next().unwrap_or_default().trim().to_ascii_lowercase();
        if media_type.is_empty() {
            continue;
        }

        let mut version: Option<&str> = None;
        let mut escaping: Option<EscapingScheme> = None;
        let mut quality = 1.0_f32;

        for part in parts {
            let part = part.trim();
            let (key, value) = match part.split_once('=') {
                Some((key, value)) => (key.trim().to_ascii_lowercase(), value.trim().trim_matches('"')),
                None => continue,
            };

            match key.as_str() {
                "version" => version = Some(value),
                "escaping" => escaping = parse_escaping_scheme(value),
                "q" => quality = value.parse::<f32>().unwrap_or(1.0_f32).clamp(0.0, 1.0),
                _ => {}
            }
        }

        // RFC 9110 allows */* as a wildcard. Keep only its highest q value.
        if media_type == "*/*" {
            if quality > wildcard_quality {
                wildcard_quality = quality;
            }
            continue;
        }

        // `q=0` is explicitly unacceptable. Skip the candidate but still
        // allow wildcard or fallback to be decided later.
        if quality <= 0.0 {
            continue;
        }

        let profile = parse_text_profile(media_type.as_str(), version, escaping);
        if let Some(profile) = profile {
            let specificity = {
                let mut value = 1_u8;
                if version.is_some() {
                    value += 1;
                }
                if escaping.is_some() {
                    value += 1;
                }
                value
            };

            // Keep the best concrete match:
            // higher quality wins, or higher specificity when q is equal.
            let candidate = MediaCandidate { profile, quality, specificity };
            if candidate.is_preferred_to(best) {
                best = Some(candidate);
            }
        }
    }

    // Only when wildcard quality strictly outranks the best concrete match do we
    // fall back to the configured profile.
    let best_quality = best.map_or(0.0_f32, |candidate| candidate.quality);
    if wildcard_quality > best_quality {
        return fallback;
    }

    best.map_or(fallback, |candidate| candidate.profile)
}

fn parse_text_profile(
    media_type: &str,
    version: Option<&str>,
    escaping: Option<EscapingScheme>,
) -> Option<TextProfile> {
    match (media_type, version) {
        ("application/openmetrics-text", Some("1.0.0")) => {
            Some(TextProfile::OpenMetricsV1_0_0 {
                escaping_scheme: escaping.unwrap_or_default(),
            })
        }
        ("application/openmetrics-text", Some("0.0.1")) => Some(TextProfile::OpenMetricsV0_0_1),
        ("text/plain", Some("1.0.0")) => Some(TextProfile::PrometheusV1_0_0 {
            escaping_scheme: escaping.unwrap_or_default(),
        }),
        ("text/plain", Some("0.0.4")) | ("text/plain", None) => {
            Some(TextProfile::PrometheusV0_0_4)
        }
        _ => None,
    }
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

    #[test]
    fn wildcard_uses_quality_weighting() {
        let fallback = TextProfile::PrometheusV0_0_4;
        let profile = text_profile_from_accept_with_fallback(
            Some("*/*;q=1, application/openmetrics-text; version=1.0.0; q=0.1"),
            fallback,
        );
        assert_eq!(profile, fallback);
    }

    #[test]
    fn specific_type_preferred_over_wildcard_when_quality_equal() {
        let profile = text_profile_from_accept_with_fallback(
            Some("*/*;q=1, application/openmetrics-text; version=1.0.0; q=1"),
            TextProfile::PrometheusV0_0_4,
        );
        assert!(matches!(
            profile,
            TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores }
        ));
    }

    #[test]
    fn prefer_more_specific_version_when_q_equal() {
        let profile = text_profile_from_accept_with_fallback(
            Some("text/plain; q=1, text/plain; version=1.0.0; q=1"),
            TextProfile::PrometheusV0_0_4,
        );
        assert!(matches!(
            profile,
            TextProfile::PrometheusV1_0_0 { escaping_scheme: EscapingScheme::Underscores }
        ));
    }

    #[test]
    fn wildcard_uses_fallback_when_plain_is_rejected() {
        let profile = text_profile_from_accept_with_fallback(
            Some("text/plain; q=0, */*; q=1"),
            TextProfile::PrometheusV0_0_4,
        );
        assert!(matches!(
            profile,
            TextProfile::PrometheusV0_0_4
        ));
    }
}
