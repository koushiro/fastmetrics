use super::*;
use crate::{
    encoder::*,
    error::{ErrorKind, Result},
    metrics::{
        counter::Counter,
        family::Family,
        gauge_histogram::GaugeHistogram,
        info::Info,
        state_set::{StateSet, StateSetValue},
        unknown::Unknown,
    },
    raw::{LabelSetSchema, MetricLabelSet, MetricType, TypedMetric},
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
    assert_eq!(err.kind(), ErrorKind::Invalid);

    let err = encode(&mut output, &registry, TextProfile::OpenMetricsV0_0_1).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Invalid);
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

#[test]
fn v1_underscores_rejects_const_label_name_collisions_after_escaping() {
    let mut registry = Registry::builder()
        .with_name_rule(NameRule::Utf8)
        .with_const_labels([("a-b", "x"), ("a/b", "y")])
        .build()
        .unwrap();
    registry.register("req_total", "help", Unknown::new(1_i64)).unwrap();

    let mut output = String::new();
    let err = encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Duplicated);
    assert_eq!(err.message(), "label names collide after escaping");
}

#[test]
fn v1_dots_rejects_family_label_name_collisions_after_escaping() {
    #[derive(Clone, Eq, PartialEq, Hash)]
    struct CollisionLabels {
        left: &'static str,
        right: &'static str,
    }

    impl LabelSetSchema for CollisionLabels {
        fn names() -> Option<&'static [&'static str]> {
            Some(&["a-b", "a/b"])
        }
    }

    impl EncodeLabelSet for CollisionLabels {
        fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
            encoder.encode(&("a-b", self.left))?;
            encoder.encode(&("a/b", self.right))?;
            Ok(())
        }
    }

    let mut registry = Registry::builder().with_name_rule(NameRule::Utf8).build().unwrap();
    let family = Family::<CollisionLabels, Counter>::default();
    registry.register("req_total", "help", family.clone()).unwrap();
    family.with_or_new(&CollisionLabels { left: "get", right: "200" }, |counter| counter.inc());

    let mut output = String::new();
    let err = encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Dots },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Duplicated);
    assert_eq!(err.message(), "label names collide after escaping");
}

#[test]
fn v1_underscores_rejects_stateset_label_name_collisions_after_escaping() {
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TestState {
        Ready,
    }

    impl StateSetValue for TestState {
        fn variants() -> &'static [Self] {
            &[Self::Ready]
        }

        fn as_str(&self) -> &str {
            "ready"
        }
    }

    let mut registry = Registry::builder()
        .with_name_rule(NameRule::Utf8)
        .with_const_labels([("state/label", "prod")])
        .build()
        .unwrap();
    registry
        .register("state-label", "help", StateSet::new(TestState::Ready))
        .unwrap();

    let mut output = String::new();
    let err = encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores },
    )
    .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Duplicated);
    assert_eq!(err.message(), "label names collide after escaping");
}

#[test]
fn v1_dots_stateset_escapes_state_label_name_once() {
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TestState {
        Ready,
    }

    impl StateSetValue for TestState {
        fn variants() -> &'static [Self] {
            &[Self::Ready]
        }

        fn as_str(&self) -> &str {
            "ready"
        }
    }

    let mut registry = Registry::builder().with_name_rule(NameRule::Utf8).build().unwrap();
    registry
        .register("state.metric", "help", StateSet::new(TestState::Ready))
        .unwrap();

    let mut output = String::new();
    encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Dots },
    )
    .unwrap();

    assert!(
        output.contains(r#"state_dot_metric{state_dot_metric="ready"} 1"#),
        "stateset label name should be escaped once in dots mode: {output}"
    );
}

#[test]
fn v1_underscores_rejects_exemplar_label_name_collisions_after_escaping() {
    struct CollidingExemplar;

    impl EncodeExemplar for CollidingExemplar {
        fn encode(&self, encoder: &mut dyn ExemplarEncoder) -> Result<()> {
            encoder.encode(&[("a-b", "x"), ("a/b", "y")], 1.0, None)
        }
    }

    #[derive(Copy, Clone)]
    struct ExemplarCounterMetric;

    impl TypedMetric for ExemplarCounterMetric {
        const TYPE: MetricType = MetricType::Counter;
    }

    impl MetricLabelSet for ExemplarCounterMetric {
        type LabelSet = ();
    }

    impl EncodeMetric for ExemplarCounterMetric {
        fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
            encoder.encode_counter(&1_u64, Some(&CollidingExemplar), None)
        }
    }

    let mut registry = Registry::builder().with_name_rule(NameRule::Utf8).build().unwrap();
    registry.register("exemplar_metric", "help", ExemplarCounterMetric).unwrap();

    let mut output = String::new();
    let err = encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores },
    )
    .unwrap_err();

    assert_eq!(err.kind(), ErrorKind::Duplicated);
    assert_eq!(err.message(), "label names collide after escaping");
}

#[test]
fn legacy_profiles_reject_exemplar_label_name_collisions_after_escaping() {
    struct CollidingExemplar;

    impl EncodeExemplar for CollidingExemplar {
        fn encode(&self, encoder: &mut dyn ExemplarEncoder) -> Result<()> {
            encoder.encode(&[("a-b", "x"), ("a/b", "y")], 1.0, None)
        }
    }

    #[derive(Copy, Clone)]
    struct ExemplarCounterMetric;

    impl TypedMetric for ExemplarCounterMetric {
        const TYPE: MetricType = MetricType::Counter;
    }

    impl MetricLabelSet for ExemplarCounterMetric {
        type LabelSet = ();
    }

    impl EncodeMetric for ExemplarCounterMetric {
        fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
            encoder.encode_counter(&1_u64, Some(&CollidingExemplar), None)
        }
    }

    let mut registry = Registry::default();
    registry.register("exemplar_metric", "help", ExemplarCounterMetric).unwrap();

    let mut output = String::new();
    let err = encode(
        &mut output,
        &registry,
        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: EscapingScheme::Underscores },
    )
    .unwrap_err();

    assert_eq!(err.kind(), ErrorKind::Duplicated);
    assert_eq!(err.message(), "label names collide after escaping");
}
