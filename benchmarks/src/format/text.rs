use std::hint::black_box;

// use pprof::criterion::{Output, PProfProfiler};
use criterion::{Criterion, criterion_group, criterion_main};

mod common;
use self::common::{
    setup_fastmetrics_registry, setup_metrics_exporter_prometheus_handle,
    setup_prometheus_client_registry, setup_prometheus_registry,
};

fn bench_text_encoding(c: &mut Criterion) {
    let metric_counts = [10, 100];
    let observe_times = [100, 1_000, 10_000, 100_000];

    for count in metric_counts {
        for times in observe_times {
            let mut group = c.benchmark_group("text::encode");

            let metric_id = format!("{count} metrics * {times} observe times");

            let id = format!("metrics_exporter_prometheus: {metric_id}");
            group.sample_size(20);
            group.bench_function(id, |b| {
                let handle = setup_metrics_exporter_prometheus_handle(count, times);
                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    handle.render_to_write(&mut buffer).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("prometheus: {metric_id}");
            group.sample_size(100);
            group.bench_function(id, move |b| {
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

            let id = format!("prometheus_client: {metric_id}");
            group.sample_size(100);
            group.bench_function(id, |b| {
                let registry = setup_prometheus_client_registry(count, times);
                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    prometheus_client::encoding::text::encode(&mut buffer, &registry).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics: {metric_id}");
            group.sample_size(100);
            group.bench_function(id, |b| {
                let registry = setup_fastmetrics_registry(count, times);
                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
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
