use std::hint::black_box;

// use pprof::criterion::{Output, PProfProfiler};
use criterion::{Criterion, criterion_group, criterion_main};

mod common;
use self::common::{
    setup_fastmetrics_registry, setup_metrics_exporter_prometheus_handle,
    setup_prometheus_client_registry, setup_prometheus_registry,
};

fn bench_protobuf_encoding(c: &mut Criterion) {
    let metric_counts = [10, 100];
    let observe_times = [100, 1_000, 10_000, 100_000];

    for count in metric_counts {
        for times in observe_times {
            let mut group = c.benchmark_group("protobuf::encode");

            let metric_id = format!("{count} metrics * {times} times");

            let id = format!("metrics_exporter_prometheus (prost/prometheus): {metric_id}");
            group.sample_size(10).bench_function(id, |b| {
                let handle = setup_metrics_exporter_prometheus_handle(count, times);
                b.iter(|| {
                    let payload = handle.render_protobuf();
                    black_box(payload);
                });
            });

            let id = format!("prometheus (protobuf/prometheus): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                let registry = setup_prometheus_registry(count, times);
                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    let metric_families = registry.gather();
                    prometheus::Encoder::encode(
                        &prometheus::ProtobufEncoder::new(),
                        &metric_families,
                        &mut buffer,
                    )
                    .unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("prometheus_client (prost/openmetrics): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                let registry = setup_prometheus_client_registry(count, times);
                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    let set = prometheus_client::encoding::protobuf::encode(&registry).unwrap();
                    prost_0_12::Message::encode(&set, &mut buffer).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (prost/prometheus): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                use fastmetrics::format::prost::{ProtobufProfile, encode};

                let registry = setup_fastmetrics_registry(count, times);
                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    encode(&mut buffer, &registry, ProtobufProfile::Prometheus).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (prost/openmetrics): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                use fastmetrics::format::prost::{ProtobufProfile, encode};

                let registry = setup_fastmetrics_registry(count, times);
                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    encode(&mut buffer, &registry, ProtobufProfile::OpenMetrics1).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (protobuf/promtheus): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                use fastmetrics::format::protobuf::{ProtobufProfile, encode};

                let registry = setup_fastmetrics_registry(count, times);
                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    encode(&mut buffer, &registry, ProtobufProfile::Prometheus).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics(protobuf/openmetrics): {metric_id}");
            group.sample_size(100).bench_function(id, |b| {
                use fastmetrics::format::protobuf::{ProtobufProfile, encode};

                let registry = setup_fastmetrics_registry(count, times);
                let mut buffer = Vec::new();
                b.iter(|| {
                    buffer.clear();
                    encode(&mut buffer, &registry, ProtobufProfile::OpenMetrics1).unwrap();
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
    targets = bench_protobuf_encoding
);
criterion_main!(benches);
