//! Content negotiation helpers for exporter example endpoints.

use fastmetrics::format::text::{EscapingScheme, TextProfile};

/// Selects a text profile from `Accept` using a lightweight Accept header parser.
///
/// - `application/openmetrics-text; version=1.0.0` => OpenMetrics 1.0.0.
/// - `application/openmetrics-text; version=0.0.1` => OpenMetrics 0.0.1.
/// - `text/plain; version=1.0.0` => Prometheus 1.0.0.
/// - `text/plain` / `text/plain; version=0.0.4` => Prometheus 0.0.4.
/// - `*/*` (wildcard) is considered as a fallback media-type candidate.
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

/// Selects a text profile from `Accept` using a lightweight Accept header parser.
///
/// - `application/openmetrics-text; version=1.0.0` => OpenMetrics 1.0.0.
/// - `application/openmetrics-text; version=0.0.1` => OpenMetrics 0.0.1.
/// - `text/plain; version=1.0.0` => Prometheus 1.0.0.
/// - `text/plain` / `text/plain; version=0.0.4` => Prometheus 0.0.4.
/// - `*/*` (wildcard) acts as fallback media type when no better match exists.
/// - unsupported media types (or missing header) => fallback.
pub fn text_profile_from_accept_with_fallback(
    accept: Option<&str>,
    fallback: TextProfile,
) -> TextProfile {
    // Step 0: missing or empty header keeps legacy behavior.
    let accept = match accept {
        Some(value) if !value.trim().is_empty() => value,
        _ => return fallback,
    };

    // Step 1: collect negotiation candidates.
    // - `best` records the best concrete match (non-"*/*") by quality + specificity.
    // - `wildcard_quality` tracks the best quality advertised by "*/*".
    // - `fallback_rejected_by_accept` tracks q=0 on the fallback media type.
    let mut best: Option<MediaCandidate> = None;
    let mut wildcard_quality = 0.0_f32;
    let mut fallback_rejected_by_accept = false;

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

        // Parse simple media-range parameters (`q`, `version`, `escaping`).
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

        // `*/*` is not a concrete type; keep its best q for step-2 resolution.
        if media_type == "*/*" {
            if quality > wildcard_quality {
                wildcard_quality = quality;
            }
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

        // A media type can still be rejected (q=0), which is important when
        // wildcard fallback is considered later.
        if quality == 0.0 {
            fallback_rejected_by_accept |=
                does_reject_fallback(&fallback, media_type.as_str(), version);
            continue;
        }

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

        if let Some(profile) = profile {
            let candidate = MediaCandidate { profile, quality, specificity };

            // Compare quality first, then specificity on tie.
            if is_preferred_candidate(candidate, best) {
                best = Some(candidate);
            }
        }
    }

    // Step 2: apply wildcard preference only when it outranks concrete best.
    // This preserves the HTTP rule that a later explicit type with equal q
    // should win over wildcard, while still allowing wildcard to be used.
    let best_concrete_q = best.map_or(0.0_f32, |candidate| candidate.quality);
    if wildcard_quality > best_concrete_q {
        if fallback_rejected_by_accept {
            return fallback_profile_for_wildcard(fallback);
        }
        return fallback;
    }

    // Step 3: keep best concrete match, or fallback when nothing concrete matched.
    best.map_or(fallback, |candidate| candidate.profile)
}

fn is_preferred_candidate(candidate: MediaCandidate, current: Option<MediaCandidate>) -> bool {
    match current {
        None => true,
        Some(previous) => {
            candidate.quality > previous.quality
                || (candidate.quality == previous.quality
                    && candidate.specificity > previous.specificity)
        },
    }
}

fn does_reject_fallback(fallback: &TextProfile, media_type: &str, version: Option<&str>) -> bool {
    match fallback {
        TextProfile::PrometheusV0_0_4 => {
            media_type == "text/plain" && (version.is_none() || version == Some("0.0.4"))
        },
        TextProfile::PrometheusV1_0_0 { .. } => {
            media_type == "text/plain" && (version.is_none() || version == Some("1.0.0"))
        },
        TextProfile::OpenMetricsV0_0_1 => {
            media_type == "application/openmetrics-text" && version == Some("0.0.1")
        },
        TextProfile::OpenMetricsV1_0_0 { .. } => {
            media_type == "application/openmetrics-text"
                && (version.is_none() || version == Some("1.0.0"))
        },
        _ => false,
    }
}

fn fallback_profile_for_wildcard(fallback: TextProfile) -> TextProfile {
    // When wildcard is selected but the configured fallback is explicitly
    // rejected, switch to another concrete supported profile.
    match fallback {
        TextProfile::OpenMetricsV0_0_1 | TextProfile::OpenMetricsV1_0_0 { .. } => {
            TextProfile::PrometheusV0_0_4
        },
        _ => TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores },
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
    fn wildcard_fallback_does_not_return_rejected_plain_type() {
        // The client explicitly rejects text/plain, so fallback may not choose
        // PrometheusV0_0_4 when wildcard is the only remaining candidate.
        let profile = text_profile_from_accept_with_fallback(
            Some("text/plain; q=0, */*; q=1"),
            TextProfile::PrometheusV0_0_4,
        );
        assert!(matches!(
            profile,
            TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores }
        ));
    }
}
