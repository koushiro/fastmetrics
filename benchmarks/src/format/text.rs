mod common;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use crate::common::{setup_openmetrics_client_registry, setup_prometheus_client_registry};

fn bench_text_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("text::encode");

    let metric_counts = [10, 100];
    let observe_times = [10, 100, 1_000, 10_000, 100_000];

    for count in metric_counts {
        for times in observe_times {
            let id = format!("prometheus_client: {count} metrics * {times} observe times");
            group.bench_function(id, |b| {
                let registry = setup_prometheus_client_registry(count, times);

                let mut buffer = String::new();

                b.iter(|| {
                    prometheus_client::encoding::text::encode(&mut buffer, &registry).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("openmetrics_client: {count} metrics * {times} observe times");
            group.bench_function(id, |b| {
                let registry = setup_openmetrics_client_registry(count, times);

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

criterion_group!(benches, bench_text_encoding);
criterion_main!(benches);
