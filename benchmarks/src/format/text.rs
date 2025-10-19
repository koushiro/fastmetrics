mod common;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

// use pprof::criterion::{Output, PProfProfiler};
use crate::common::{
    setup_fastmetrics_registry, setup_prometheus_client_registry, setup_prometheus_registry,
};

fn bench_text_encoding(c: &mut Criterion) {
    let metric_counts = [10, 100];
    let observe_times = [100, 1_000, 10_000, 100_000];

    for count in metric_counts {
        for times in observe_times {
            let mut group = c.benchmark_group("text::encode");

            let id = format!("prometheus: {count} metrics * {times} observe times");
            group.bench_function(id, move |b| {
                let registry = setup_prometheus_registry(count, times);

                let mut buffer = String::new();
                b.iter(|| {
                    let metric_families = registry.gather();
                    prometheus::TextEncoder::new()
                        .encode_utf8(&metric_families, &mut buffer)
                        .unwrap();
                    black_box(&mut buffer);
                })
            });

            let id = format!("prometheus_client: {count} metrics * {times} observe times");
            group.bench_function(id, |b| {
                let registry = setup_prometheus_client_registry(count, times);

                let mut buffer = String::new();

                b.iter(|| {
                    prometheus_client::encoding::text::encode(&mut buffer, &registry).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics: {count} metrics * {times} observe times");
            group.bench_function(id, |b| {
                let registry = setup_fastmetrics_registry(count, times);

                let mut buffer = String::new();

                b.iter(|| {
                    fastmetrics::format::text::encode(&mut buffer, &registry).unwrap();
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
