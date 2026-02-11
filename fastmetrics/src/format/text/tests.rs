use super::*;
use crate::{
    error::ErrorKind,
    metrics::{
        gauge_histogram::GaugeHistogram,
        info::Info,
        state_set::{StateSet, StateSetValue},
        unknown::Unknown,
    },
    registry::{NameRule, Registry},
};

#[test]
fn escape_help_and_label_values() {
    let mut registry = Registry::default();
    let info = Info::new(vec![("quote", "a\"b"), ("slash", "a\\b"), ("newline", "a\nb")]);

    registry
        .register("build", r#"Help with \"quote\", \\ slash and \n newline"#, info)
        .unwrap();

    let mut output = String::new();
    encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::AllowUtf8 },
    )
    .unwrap();

    assert!(
        output.contains(r#"# HELP build Help with \"quote\", \\ slash and \n newline"#),
        "help line should keep canonical escaping: {output}"
    );
    assert!(output.contains(r#"quote="a\"b""#), "quote label should be escaped: {output}");
    assert!(output.contains(r#"slash="a\\b""#), "slash label should be escaped: {output}");
    assert!(output.contains(r#"newline="a\nb""#), "newline label should be escaped: {output}");
}

#[test]
fn prometheus_profile_maps_unknown_to_untyped() {
    let mut registry = Registry::default();
    registry.register("raw_value", "Raw value", Unknown::new(42_i64)).unwrap();

    let mut output = String::new();
    encode(&mut output, &registry, TextProfile::PrometheusV0_0_4).unwrap();

    assert!(output.contains("# TYPE raw_value untyped"), "unknown should map to untyped: {output}");
    assert!(output.contains("raw_value 42"), "unknown sample missing: {output}");
}

#[test]
fn prometheus_profile_rejects_info() {
    let mut registry = Registry::default();

    let info = Info::new(vec![("version", "1.0.0")]);
    registry.register("release_version", "Build info", info).unwrap();

    let mut output = String::new();

    let err = encode(&mut output, &registry, TextProfile::PrometheusV0_0_4).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);

    let err = encode(
        &mut output,
        &registry,
        TextProfile::PrometheusV1_0_0 { escaping_scheme: Default::default() },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);
}

#[test]
fn prometheus_profile_rejects_stateset() {
    let mut registry = Registry::default();

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TestState {
        Ready,
        Busy,
    }

    impl StateSetValue for TestState {
        fn variants() -> &'static [Self] {
            &[Self::Ready, Self::Busy]
        }

        fn as_str(&self) -> &str {
            match self {
                Self::Ready => "ready",
                Self::Busy => "busy",
            }
        }
    }

    let stateset = StateSet::new(TestState::Ready);
    registry.register("worker_state", "Worker state", stateset).unwrap();

    let mut output = String::new();
    let err = encode(&mut output, &registry, TextProfile::PrometheusV0_0_4).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);

    let err = encode(
        &mut output,
        &registry,
        TextProfile::PrometheusV1_0_0 { escaping_scheme: Default::default() },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);
}

#[test]
fn prometheus_profile_rejects_gauge_histogram() {
    let mut registry = Registry::default();
    registry
        .register("temperature_distribution", "Temperature distribution", GaugeHistogram::default())
        .unwrap();

    let mut output = String::new();
    let err = encode(&mut output, &registry, TextProfile::PrometheusV0_0_4).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);

    let err = encode(
        &mut output,
        &registry,
        TextProfile::PrometheusV1_0_0 { escaping_scheme: Default::default() },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);
}

#[test]
fn legacy_profiles_reject_utf8_metric_name() {
    let mut registry = Registry::builder().with_name_rule(NameRule::Utf8).build().unwrap();

    registry.register("指标", "UTF-8 metric name", Unknown::new(1_i64)).unwrap();

    let mut output = String::new();

    let err = encode(&mut output, &registry, TextProfile::PrometheusV0_0_4).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);

    let err = encode(&mut output, &registry, TextProfile::OpenMetricsV0_0_1).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Unsupported);
}

#[test]
fn v1_allow_utf8_keeps_utf8_metric_and_label_names() {
    let mut registry = Registry::builder()
        .with_name_rule(NameRule::Utf8)
        .with_const_labels([("标签", "值")])
        .build()
        .unwrap();

    registry.register("指标", "UTF-8 metric name", Unknown::new(1_i64)).unwrap();

    let mut output = String::new();
    encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::AllowUtf8 },
    )
    .unwrap();

    assert!(output.contains("指标{标签=\"值\"} 1"), "allow-utf-8 must keep UTF-8 names: {output}");
}

#[test]
fn v1_underscores_escapes_utf8_metric_and_label_names() {
    let mut registry = Registry::builder()
        .with_name_rule(NameRule::Utf8)
        .with_const_labels([("b.温", "v")])
        .build()
        .unwrap();

    registry.register("a.温", "UTF-8 metric name", Unknown::new(1_i64)).unwrap();

    let mut output = String::new();
    encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores },
    )
    .unwrap();

    assert!(
        output.contains("a__{b__=\"v\"} 1"),
        "underscores scheme must rewrite metric/label names: {output}"
    );
}

#[test]
fn v1_dots_and_values_escape_metric_names() {
    let mut registry = Registry::builder().with_name_rule(NameRule::Utf8).build().unwrap();

    registry.register("foo_bar.baz", "help", Unknown::new(1_i64)).unwrap();

    let mut output = String::new();
    encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Dots },
    )
    .unwrap();
    assert!(output.contains("foo__bar_dot_baz 1"), "dots escaping mismatch: {output}");

    output.clear();
    encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Values },
    )
    .unwrap();
    assert!(output.contains("U__foo__bar_2E_baz 1"), "values escaping mismatch: {output}");
}

#[test]
fn v1_underscores_rejects_family_name_collisions_after_escaping() {
    let mut registry = Registry::builder().with_name_rule(NameRule::Utf8).build().unwrap();
    registry.register("a-b", "help", Unknown::new(1_i64)).unwrap();
    registry.register("a/b", "help", Unknown::new(2_i64)).unwrap();

    let mut output = String::new();
    let err = encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Duplicated);
    assert_eq!(err.message(), "metric family names collide after escaping");
}

#[test]
fn v1_dots_rejects_family_name_collisions_after_escaping() {
    let mut registry = Registry::builder().with_name_rule(NameRule::Utf8).build().unwrap();
    registry.register("a-b", "help", Unknown::new(1_i64)).unwrap();
    registry.register("a/b", "help", Unknown::new(2_i64)).unwrap();

    let mut output = String::new();
    let err = encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Dots },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Duplicated);
    assert_eq!(err.message(), "metric family names collide after escaping");
}
