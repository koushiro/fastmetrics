mod common;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use crate::common::{setup_openmetrics_client_registry, setup_prometheus_client_registry};

fn prometheus_client_text_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("prometheus_client");

    let metric_counts = [10, 100];
    let observe_times = [10, 100, 1000, 10000];

    for metric_count in metric_counts {
        for observe_time in observe_times {
            let id = format!("text::encode: {metric_count} metrics * {observe_time} times");
            group.bench_function(&id, |b| {
                let registry = setup_prometheus_client_registry(metric_count, observe_time);

                let mut buffer = String::new();

                b.iter(|| {
                    prometheus_client::encoding::text::encode(&mut buffer, &registry).unwrap();
                    black_box(&mut buffer);
                });
            });
        }
    }

    group.finish();
}

fn openmetrics_client_text_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("openmetrics_client");

    let metric_counts = [10, 100];
    let observe_times = [10, 100, 1000, 10000];

    for metric_count in metric_counts {
        for observe_time in observe_times {
            let id = format!("text::encode: {metric_count} metrics * {observe_time} times");
            group.bench_function(&id, |b| {
                let registry = setup_openmetrics_client_registry(metric_count, observe_time);

                let mut buffer = String::new();

                b.iter(|| {
                    openmetrics_client::format::text::encode(&mut buffer, &registry).unwrap();
                    black_box(&mut buffer);
                });
            });
        }
    }

    group.finish();
}

criterion_group!(benches, prometheus_client_text_format, openmetrics_client_text_format);
criterion_main!(benches);
