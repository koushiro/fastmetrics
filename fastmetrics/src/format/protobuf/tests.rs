use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use prost::Message as _;
use prost_types::Timestamp;

use super::{ProtobufProfile, encode, encode_with, prometheus_data_model as proto};
use crate::{
    encoder::{self, EncodeExemplar, EncodeLabel, EncodeLabelSet, EncodeMetric},
    error::{Error, Result},
    metrics::{counter::Counter, family::Family, info::Info, lazy_group::LazyGroup},
    raw::{MetricLabelSet, MetricType, TypedMetric, bucket::Bucket, quantile::Quantile},
    registry::{Registry, Unit},
};

fn decode_all(mut output: &[u8]) -> Vec<proto::MetricFamily> {
    let mut families = Vec::new();
    while !output.is_empty() {
        families.push(proto::MetricFamily::decode_length_delimited(&mut output).unwrap());
    }
    families
}

const ROUNDED_U64_MAX: f64 = 18_446_744_073_709_552_000.0;

fn label(name: &str, value: &str) -> proto::LabelPair {
    proto::LabelPair { name: Some(name.into()), value: Some(value.into()) }
}

fn family(
    name: &str,
    help: &str,
    kind: proto::MetricType,
    metric: Vec<proto::Metric>,
) -> proto::MetricFamily {
    proto::MetricFamily {
        name: Some(name.into()),
        help: Some(help.into()),
        r#type: Some(kind as i32),
        metric,
        ..Default::default()
    }
}

fn assert_families(
    output: &[u8],
    mut expected: Vec<proto::MetricFamily>,
) -> Vec<proto::MetricFamily> {
    let mut actual = decode_all(output);
    actual.sort_by(|a, b| a.name().cmp(b.name()));
    expected.sort_by(|a, b| a.name().cmp(b.name()));
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(&expected) {
        assert_eq!(actual, expected, "family={}", expected.name());
    }
    actual
}

fn expected_test_labels() -> Vec<proto::LabelPair> {
    vec![
        label("method", "GET"),
        label("integer", "340282366920938463463374607431768211455"),
        label("float", "-0.0"),
        label("bool", "true"),
    ]
}

fn expected_fixture() -> Vec<proto::MetricFamily> {
    let mut unknown = Vec::new();
    let mut gauge = Vec::new();
    let mut counter = Vec::new();
    let mut states = Vec::new();
    let mut info = Vec::new();
    let mut histograms = Vec::new();
    let mut gauge_histograms = Vec::new();
    let mut summaries = Vec::new();
    let points = [
        ("left", 0, Timestamp::default(), 0),
        ("right", i64::MAX, Timestamp { seconds: -1, nanos: 999_999_999 }, 1),
    ];
    for (branch, timestamp_ms, created, exemplar_seconds) in points {
        let base = proto::Metric {
            label: vec![label("site", "test"), label("branch", branch)],
            timestamp_ms: Some(timestamp_ms),
            ..Default::default()
        };
        unknown.push(proto::Metric {
            untyped: Some(proto::Untyped { value: Some(-0.0) }),
            ..base.clone()
        });
        gauge.push(proto::Metric {
            gauge: Some(proto::Gauge { value: Some(-42.0) }),
            ..base.clone()
        });
        let mut exemplar_labels = expected_test_labels();
        exemplar_labels.push(label("trace_id", "abc"));
        let exemplar = proto::Exemplar {
            label: exemplar_labels,
            value: Some(2.5),
            timestamp: Some(Timestamp { seconds: exemplar_seconds, nanos: 123_456_789 }),
        };
        counter.push(proto::Metric {
            counter: Some(proto::Counter {
                value: Some(ROUNDED_U64_MAX),
                exemplar: Some(exemplar.clone()),
                created_timestamp: Some(created),
            }),
            ..base.clone()
        });
        for (state, value) in [("ready", 1.0), ("idle", 0.0)] {
            let mut labels = base.label.clone();
            labels.push(label("app_states", state));
            states.push(proto::Metric {
                label: labels,
                gauge: Some(proto::Gauge { value: Some(value) }),
                ..base.clone()
            });
        }
        let mut info_labels = base.label.clone();
        info_labels.extend(expected_test_labels());
        info.push(proto::Metric {
            label: info_labels,
            gauge: Some(proto::Gauge { value: Some(1.0) }),
            ..base.clone()
        });
        let buckets = vec![
            proto::Bucket {
                cumulative_count: Some(u64::MAX),
                upper_bound: Some(1.0),
                exemplar: Some(exemplar),
                ..Default::default()
            },
            proto::Bucket {
                cumulative_count: Some(u64::MAX),
                upper_bound: Some(f64::INFINITY),
                exemplar: Some(proto::Exemplar::default()),
                ..Default::default()
            },
        ];
        histograms.push(proto::Metric {
            histogram: Some(proto::Histogram {
                sample_count: Some(u64::MAX),
                sample_sum: Some(3.5),
                bucket: buckets.clone(),
                created_timestamp: Some(created),
                ..Default::default()
            }),
            ..base.clone()
        });
        gauge_histograms.push(proto::Metric {
            histogram: Some(proto::Histogram {
                sample_count: Some(u64::MAX),
                sample_sum: Some(-3.5),
                bucket: buckets,
                ..Default::default()
            }),
            ..base.clone()
        });
        summaries.push(proto::Metric {
            summary: Some(proto::Summary {
                sample_count: Some(2),
                sample_sum: Some(5.0),
                quantile: vec![
                    proto::Quantile { quantile: Some(0.5), value: Some(1.5) },
                    proto::Quantile { quantile: Some(0.99), value: Some(3.5) },
                ],
                created_timestamp: Some(created),
            }),
            ..base
        });
    }
    vec![
        family("app_unknown", "Unknown", proto::MetricType::Untyped, unknown),
        family("app_gauge", "Gauge", proto::MetricType::Gauge, gauge),
        family("app_counter", "Counter", proto::MetricType::Counter, counter),
        family("app_states", "States", proto::MetricType::Gauge, states),
        family("app_info_info", "Info", proto::MetricType::Gauge, info),
        proto::MetricFamily {
            unit: Some("seconds".into()),
            ..family("app_histogram", "Histogram", proto::MetricType::Histogram, histograms)
        },
        family(
            "app_gauge_histogram",
            "Gauge histogram",
            proto::MetricType::GaugeHistogram,
            gauge_histograms,
        ),
        family("app_sub_summary", "Summary", proto::MetricType::Summary, summaries),
    ]
}

struct Fixture<const KIND: u8>;

impl<const KIND: u8> TypedMetric for Fixture<KIND> {
    const TYPE: MetricType = match KIND {
        0 => MetricType::Unknown,
        1 => MetricType::Gauge,
        2 => MetricType::Counter,
        3 => MetricType::StateSet,
        4 => MetricType::Info,
        5 => MetricType::Histogram,
        6 => MetricType::GaugeHistogram,
        7 => MetricType::Summary,
        _ => MetricType::Unknown,
    };
}

impl<const KIND: u8> MetricLabelSet for Fixture<KIND> {
    type LabelSet = ();
}

impl<const KIND: u8> EncodeMetric for Fixture<KIND> {
    fn encode(&self, encoder: &mut dyn encoder::MetricEncoder) -> Result<()> {
        encoder.encode(&[("branch", "left")], &Point::<KIND>(0))?;
        encoder.encode(&[("branch", "right")], &Point::<KIND>(1))
    }
}

struct Point<const KIND: u8>(u64);

impl<const KIND: u8> EncodeMetric for Point<KIND> {
    fn encode(&self, encoder: &mut dyn encoder::MetricEncoder) -> Result<()> {
        let created = Some(if self.0 == 0 { Duration::ZERO } else { Duration::MAX });
        let exemplar = TestExemplar(self.0);
        let exemplars: [Option<&dyn EncodeExemplar>; 2] = [Some(&exemplar), Some(&())];
        let buckets = [Bucket::new(1.0, u64::MAX), Bucket::new(f64::INFINITY, 1)];
        match KIND {
            0 => encoder.encode_unknown(&-0.0_f64),
            1 => encoder.encode_gauge(&-42_i64),
            2 => encoder.encode_counter(&u64::MAX, Some(&exemplar), created),
            3 => encoder.encode_stateset(vec![("ready", true), ("idle", false)]),
            4 => encoder.encode_info(&TestLabels),
            5 => encoder.encode_histogram(&buckets, Some(&exemplars), u64::MAX, 3.5, created),
            6 => encoder.encode_gauge_histogram(&buckets, Some(&exemplars), u64::MAX, -3.5),
            7 => encoder.encode_summary(
                &[Quantile::new(0.5, 1.5), Quantile::new(0.99, 3.5)],
                5.0,
                2,
                created,
            ),
            _ => unreachable!(),
        }
    }

    fn timestamp(&self) -> Option<Duration> {
        Some(if self.0 == 0 { Duration::ZERO } else { Duration::MAX })
    }
}

struct TestLabels;

impl EncodeLabelSet for TestLabels {
    fn encode(&self, encoder: &mut dyn encoder::LabelSetEncoder) -> Result<()> {
        encoder.encode(&ChunkedLabel)?;
        encoder.encode(&("integer", u128::MAX))?;
        encoder.encode(&("float", -0.0_f32))?;
        encoder.encode(&("bool", true))?;
        encoder.encode(&("missing", None::<u8>))
    }
}

struct ChunkedLabel;

impl EncodeLabel for ChunkedLabel {
    fn encode(&self, encoder: &mut dyn encoder::LabelEncoder) -> Result<()> {
        encoder.encode_label_name("met")?;
        encoder.encode_label_name("hod")?;
        encoder.encode_str_value("G")?;
        encoder.encode_str_value("ET")
    }
}

struct TestExemplar(u64);

impl EncodeExemplar for TestExemplar {
    fn encode(&self, encoder: &mut dyn encoder::ExemplarEncoder) -> Result<()> {
        encoder.encode(&TestLabels, 1.5, Some(Duration::ZERO))?;
        encoder.encode(&[("trace_id", "abc")], 2.5, Some(Duration::new(self.0, 123_456_789)))
    }
}

fn fixture_registry() -> Registry {
    let mut registry = Registry::builder()
        .with_namespace("app")
        .with_const_labels([("site", "test")])
        .build()
        .unwrap();
    registry.register("unknown", "Unknown", Fixture::<0>).unwrap();
    registry.register("gauge", "Gauge", Fixture::<1>).unwrap();
    registry.register("counter", "Counter", Fixture::<2>).unwrap();
    registry.register("states", "States", Fixture::<3>).unwrap();
    registry.register("info", "Info", Fixture::<4>).unwrap();
    registry
        .register_with_unit("histogram", "Histogram", Unit::Seconds, Fixture::<5>)
        .unwrap();
    registry.register("gauge_histogram", "Gauge histogram", Fixture::<6>).unwrap();
    registry
        .subsystem("sub")
        .unwrap()
        .register("summary", "Summary", Fixture::<7>)
        .unwrap();
    registry
        .register("empty", "Empty family", Family::<(), Counter>::default())
        .unwrap();
    registry
}

#[test]
fn encoding_matches_expected_message_trees() {
    let registry = fixture_registry();
    let mut output = Vec::new();
    encode(&mut output, &registry, ProtobufProfile::Prometheus).unwrap();
    let families = assert_families(&output, expected_fixture());
    let unknown = families.iter().find(|family| family.name() == "app_unknown").unwrap();
    for metric in &unknown.metric {
        assert_eq!(metric.untyped.as_ref().unwrap().value().to_bits(), (-0.0_f64).to_bits());
    }
}

#[test]
fn encoding_preserves_samples_across_buffer_growth() {
    struct Samples;

    impl TypedMetric for Samples {
        const TYPE: MetricType = MetricType::Gauge;
    }

    impl MetricLabelSet for Samples {
        type LabelSet = ();
    }

    impl EncodeMetric for Samples {
        fn encode(&self, encoder: &mut dyn encoder::MetricEncoder) -> Result<()> {
            for index in 0..256 {
                encoder.encode(&[("series", index)], &Point::<1>(index))?;
            }
            Ok(())
        }
    }

    let region = "x".repeat(128);
    let mut registry = Registry::builder()
        .with_const_labels([("region", region.clone())])
        .build()
        .unwrap();
    registry.register("readings", "Readings", Samples).unwrap();
    let mut output = Vec::new();
    encode(&mut output, &registry, ProtobufProfile::Prometheus).unwrap();
    assert_families(
        &output,
        vec![family(
            "readings",
            "Readings",
            proto::MetricType::Gauge,
            (0..256)
                .map(|index| proto::Metric {
                    label: vec![label("region", &region), label("series", &index.to_string())],
                    gauge: Some(proto::Gauge { value: Some(-42.0) }),
                    timestamp_ms: Some(if index == 0 { 0 } else { i64::MAX }),
                    ..Default::default()
                })
                .collect(),
        )],
    );
}

#[test]
fn fused_samples_match_expected_message_trees_at_length_boundaries() {
    struct PlainHistogram {
        buckets: Vec<Bucket>,
        count: u64,
        timestamp: Duration,
    }

    impl TypedMetric for PlainHistogram {
        const TYPE: MetricType = MetricType::Histogram;
    }

    impl MetricLabelSet for PlainHistogram {
        type LabelSet = ();
    }

    impl EncodeMetric for PlainHistogram {
        fn encode(&self, encoder: &mut dyn encoder::MetricEncoder) -> Result<()> {
            encoder.encode_histogram(&self.buckets, None, self.count, 3.5, None)
        }

        fn timestamp(&self) -> Option<Duration> {
            Some(self.timestamp)
        }
    }

    for label_len in [0, 127, 128, 16383, 16384] {
        for (count, counter_value, sample_count, cumulative_counts) in [
            (0, 0.0, 2, [0, 1, 2]),
            (127, 127.0, 129, [127, 128, 129]),
            (128, 128.0, 130, [128, 129, 130]),
            (u64::MAX, ROUNDED_U64_MAX, u64::MAX, [u64::MAX; 3]),
        ] {
            let region = "x".repeat(label_len);
            let mut registry = Registry::builder()
                .with_const_labels([("region", region.clone())])
                .build()
                .unwrap();
            let counter = Counter::<u64>::default();
            counter.inc_by(count);
            registry.register("counter", "Counter", counter).unwrap();
            let histogram = PlainHistogram {
                buckets: vec![
                    Bucket::new(0.005, count),
                    Bucket::new(2.56, 1),
                    Bucket::new(f64::INFINITY, 1),
                ],
                count: count.saturating_add(2),
                timestamp: if count == 0 { Duration::ZERO } else { Duration::MAX },
            };
            registry.register("histogram", "Histogram", histogram).unwrap();
            let mut output = Vec::new();
            encode(&mut output, &registry, ProtobufProfile::Prometheus).unwrap();
            let labels = vec![label("region", &region)];
            assert_families(
                &output,
                vec![
                    family(
                        "counter",
                        "Counter",
                        proto::MetricType::Counter,
                        vec![proto::Metric {
                            label: labels.clone(),
                            counter: Some(proto::Counter {
                                value: Some(counter_value),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }],
                    ),
                    family(
                        "histogram",
                        "Histogram",
                        proto::MetricType::Histogram,
                        vec![proto::Metric {
                            label: labels,
                            histogram: Some(proto::Histogram {
                                sample_count: Some(sample_count),
                                sample_sum: Some(3.5),
                                bucket: vec![
                                    proto::Bucket {
                                        cumulative_count: Some(cumulative_counts[0]),
                                        upper_bound: Some(0.005),
                                        ..Default::default()
                                    },
                                    proto::Bucket {
                                        cumulative_count: Some(cumulative_counts[1]),
                                        upper_bound: Some(2.56),
                                        ..Default::default()
                                    },
                                    proto::Bucket {
                                        cumulative_count: Some(cumulative_counts[2]),
                                        upper_bound: Some(f64::INFINITY),
                                        ..Default::default()
                                    },
                                ],
                                ..Default::default()
                            }),
                            timestamp_ms: Some(if count == 0 { 0 } else { i64::MAX }),
                            ..Default::default()
                        }],
                    ),
                ],
            );
        }
    }
}

struct FailingMetric;

impl TypedMetric for FailingMetric {
    const TYPE: MetricType = MetricType::Gauge;
}

impl MetricLabelSet for FailingMetric {
    type LabelSet = ();
}

impl EncodeMetric for FailingMetric {
    fn encode(&self, encoder: &mut dyn encoder::MetricEncoder) -> Result<()> {
        encoder.encode_gauge(&1_i64)?;
        Err(Error::invalid("collection failed"))
    }
}

#[test]
fn encoding_is_isolated_after_collection_errors() {
    let good = fixture_registry();
    let mut bad = Registry::default();
    bad.register("failure", "Failure", FailingMetric).unwrap();
    let mut baseline = Vec::new();
    encode(&mut baseline, &good, ProtobufProfile::Prometheus).unwrap();
    let mut output = b"prefix".to_vec();
    assert!(encode(&mut output, &bad, ProtobufProfile::Prometheus).is_err());
    assert_eq!(output, b"prefix");

    encode(&mut output, &Registry::default(), ProtobufProfile::Prometheus).unwrap();
    assert_eq!(output, b"prefix");
    encode(&mut output, &good, ProtobufProfile::Prometheus).unwrap();
    assert_eq!(&output[6..], baseline);
}

#[test]
fn encoding_samples_lazy_groups_once_per_scrape() {
    let calls = Arc::new(AtomicUsize::new(0));
    let group = LazyGroup::new({
        let calls = calls.clone();
        move || calls.fetch_add(1, Ordering::Relaxed) as u64
    });
    let mut registry = Registry::default();
    registry.register("first", "First", group.counter(|value| *value)).unwrap();
    registry.register("second", "Second", group.counter(|value| *value)).unwrap();
    let mut output = Vec::new();
    for expected in 0..2 {
        output.clear();
        encode(&mut output, &registry, ProtobufProfile::Prometheus).unwrap();
        let families = decode_all(&output);
        assert_eq!(families.len(), 2);
        assert!(families.iter().all(|family| family.metric[0].counter.as_ref().unwrap().value() == expected as f64));
        assert_eq!(calls.load(Ordering::Relaxed), expected + 1);
    }
}

#[test]
fn explicit_scope_is_dropped_after_collection_error() {
    let state = AtomicUsize::new(0);
    struct Guard<'a>(&'a AtomicUsize);
    impl Drop for Guard<'_> {
        fn drop(&mut self) {
            self.0.store(2, Ordering::Relaxed);
        }
    }
    let mut registry = Registry::default();
    registry.register("failure", "Failure", FailingMetric).unwrap();
    let result = encode_with(&mut Vec::new(), &registry, ProtobufProfile::Prometheus, || {
        state.store(1, Ordering::Relaxed);
        Guard(&state)
    });
    assert!(result.is_err());
    assert_eq!(state.load(Ordering::Relaxed), 2);
}

#[test]
fn encode_prometheus_counter_profile() {
    let mut registry = Registry::default();
    let counter = Counter::<u64>::default();
    registry
        .register("http_requests_total", "Total requests", counter.clone())
        .unwrap();
    counter.inc();
    let mut output = Vec::new();
    encode(&mut output, &registry, ProtobufProfile::Prometheus).unwrap();
    assert_families(
        &output,
        vec![family(
            "http_requests_total",
            "Total requests",
            proto::MetricType::Counter,
            vec![proto::Metric {
                counter: Some(proto::Counter { value: Some(1.0), ..Default::default() }),
                ..Default::default()
            }],
        )],
    );
}

#[test]
fn encode_prometheus_info_profile() {
    let mut registry = Registry::default();
    registry
        .register("build", "Build info", Info::new(vec![("version", "1.0.0")]))
        .unwrap();
    let mut output = Vec::new();
    encode(&mut output, &registry, ProtobufProfile::Prometheus).unwrap();
    assert_families(
        &output,
        vec![family(
            "build_info",
            "Build info",
            proto::MetricType::Gauge,
            vec![proto::Metric {
                label: vec![label("version", "1.0.0")],
                gauge: Some(proto::Gauge { value: Some(1.0) }),
                ..Default::default()
            }],
        )],
    );
}

#[test]
fn optional_labels_are_omitted_without_losing_empty_values() {
    let mut registry = Registry::default();
    let info = Info::new(vec![("missing", None), ("empty", Some("")), ("present", Some("value"))]);
    registry.register("labels", "Labels", info).unwrap();
    let mut output = Vec::new();
    encode(&mut output, &registry, ProtobufProfile::Prometheus).unwrap();
    assert_families(
        &output,
        vec![family(
            "labels_info",
            "Labels",
            proto::MetricType::Gauge,
            vec![proto::Metric {
                label: vec![label("empty", ""), label("present", "value")],
                gauge: Some(proto::Gauge { value: Some(1.0) }),
                ..Default::default()
            }],
        )],
    );
}

#[test]
fn fixed_capacity_buffers_are_checked_before_writing() {
    let registry = fixture_registry();
    let expected = expected_fixture();
    let required: usize = expected
        .iter()
        .map(|family| {
            let len = family.encoded_len();
            prost::length_delimiter_len(len) + len
        })
        .sum();
    let mut exact = vec![0xaa; required];
    let mut cursor = exact.as_mut_slice();
    encode(&mut cursor, &registry, ProtobufProfile::Prometheus).unwrap();
    assert!(cursor.is_empty());
    assert_families(&exact, expected);

    let mut short = vec![0xaa; required - 1];
    let mut cursor = short.as_mut_slice();
    let error = encode(&mut cursor, &registry, ProtobufProfile::Prometheus).unwrap_err();
    assert_eq!(error.kind(), crate::error::ErrorKind::Unexpected);
    assert_eq!(cursor.len(), required - 1);
    assert!(short.iter().all(|byte| *byte == 0xaa));
}
