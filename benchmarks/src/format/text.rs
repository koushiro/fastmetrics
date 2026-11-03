use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
// use pprof::criterion::{Output, PProfProfiler};

mod common;
use self::common::{
    setup_fastmetrics_registry, setup_measured_registry, setup_metrics_exporter_prometheus_handle,
    setup_prometheus_client_registry, setup_prometheus_registry,
};

fn bench_text_encoding(c: &mut Criterion) {
    let metric_counts = [10, 100];
    let observe_times = [100, 1_000, 10_000, 100_000];

    for count in metric_counts {
        for times in observe_times {
            let mut group = c.benchmark_group("text::encode");

            let metric_id = format!("{count} metrics * {times} times");

            let id = format!("metrics_exporter_prometheus (prometheus 0.0.4): {metric_id}");
            group.sample_size(20).bench_function(id, |b| {
                let handle = setup_metrics_exporter_prometheus_handle(count, times);

                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    handle.render_to_write(&mut buffer).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("measured (prometheus 0.0.4): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                use measured::MetricGroup;
                let registry = setup_measured_registry(count, times);

                b.iter(|| {
                    let mut encoder = measured_prometheus_text::BufferedTextEncoder::new();
                    registry.collect_group_into(&mut encoder).unwrap();
                    black_box(encoder.finish());
                });
            });

            let id = format!("prometheus (prometheus 0.0.4): {metric_id}");
            group.sample_size(100).bench_function(id, move |b| {
                let registry = setup_prometheus_registry(count, times);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    let metric_families = registry.gather();
                    prometheus::TextEncoder::new()
                        .encode_utf8(&metric_families, &mut buffer)
                        .unwrap();
                    black_box(&mut buffer);
                })
            });

            let id = format!("prometheus_client (openmetrics 0.0.1): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                let registry = setup_prometheus_client_registry(count, times);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    prometheus_client::encoding::text::encode(&mut buffer, &registry).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (prometheus 0.0.4): {metric_id}");
            group.sample_size(50).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Legacy);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(&mut buffer, &registry, TextProfile::PrometheusV0_0_4).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (prometheus 1.0.0, legacy + underscores): {metric_id}");
            group.sample_size(30).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{EscapingScheme, TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Legacy);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(
                        &mut buffer,
                        &registry,
                        TextProfile::PrometheusV1_0_0 {
                            escaping_scheme: EscapingScheme::Underscores,
                        },
                    )
                    .unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (prometheus 1.0.0, allow utf8): {metric_id}");
            group.sample_size(50).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{EscapingScheme, TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Utf8);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(
                        &mut buffer,
                        &registry,
                        TextProfile::PrometheusV1_0_0 {
                            escaping_scheme: EscapingScheme::AllowUtf8,
                        },
                    )
                    .unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (prometheus 1.0.0, utf8 + underscores): {metric_id}");
            group.sample_size(30).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{EscapingScheme, TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Utf8);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(
                        &mut buffer,
                        &registry,
                        TextProfile::PrometheusV1_0_0 {
                            escaping_scheme: EscapingScheme::Underscores,
                        },
                    )
                    .unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (openmetrics 0.0.1): {metric_id}");
            group.sample_size(50).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Legacy);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(&mut buffer, &registry, TextProfile::OpenMetricsV0_0_1).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (openmetrics 1.0.0, legacy + underscores): {metric_id}");
            group.sample_size(30).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{EscapingScheme, TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Legacy);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(
                        &mut buffer,
                        &registry,
                        TextProfile::OpenMetricsV1_0_0 {
                            escaping_scheme: EscapingScheme::Underscores,
                        },
                    )
                    .unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (openmetrics 1.0.0, allow utf8): {metric_id}");
            group.sample_size(50).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{EscapingScheme, TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Utf8);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(
                        &mut buffer,
                        &registry,
                        TextProfile::OpenMetricsV1_0_0 {
                            escaping_scheme: EscapingScheme::AllowUtf8,
                        },
                    )
                    .unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (openmetrics 1.0.0, utf8 + underscores): {metric_id}");
            group.sample_size(30).bench_function(id, |b| {
                use fastmetrics::{
                    format::text::{EscapingScheme, TextProfile, encode},
                    registry::NameRule,
                };

                let registry = setup_fastmetrics_registry(count, times, NameRule::Utf8);

                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(
                        &mut buffer,
                        &registry,
                        TextProfile::OpenMetricsV1_0_0 {
                            escaping_scheme: EscapingScheme::Underscores,
                        },
                    )
                    .unwrap();
                    black_box(&mut buffer);
                });
            });

            group.finish();
        }
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default()/*.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))*/;
    targets = bench_text_encoding
);
criterion_main!(benches);
