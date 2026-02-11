use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
// use pprof::criterion::{Output, PProfProfiler};
use rand::RngExt;

mod common;
use self::common::{
    setup_fastmetrics_registry, setup_metrics_exporter_prometheus_handle,
    setup_prometheus_client_registry, setup_prometheus_registry,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, measured::FixedCardinalityLabel)]
#[label(crate = measured)]
enum MeasuredStatus {
    #[label(rename = "1")]
    One,
    #[label(rename = "2")]
    Two,
    #[label(rename = "3")]
    Three,
    #[label(rename = "4")]
    Four,
    #[label(rename = "5")]
    Five,
    #[label(rename = "6")]
    Six,
    #[label(rename = "7")]
    Seven,
    #[label(rename = "8")]
    Eight,
    #[label(rename = "9")]
    Nine,
    #[label(rename = "10")]
    Ten,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, measured::FixedCardinalityLabel)]
#[label(crate = measured)]
enum MeasuredMethod {
    #[label(rename = "GET")]
    Get,
    #[label(rename = "PUT")]
    Put,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, measured::LabelGroup)]
#[label(crate = measured, set = MeasuredLabelsSet)]
struct MeasuredLabels {
    method: MeasuredMethod,
    status: MeasuredStatus,
}

impl MeasuredLabels {
    fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let method =
            if rng.random_ratio(8, 10) { MeasuredMethod::Get } else { MeasuredMethod::Put };
        let status = match rng.random_range(1..=10) {
            1 => MeasuredStatus::One,
            2 => MeasuredStatus::Two,
            3 => MeasuredStatus::Three,
            4 => MeasuredStatus::Four,
            5 => MeasuredStatus::Five,
            6 => MeasuredStatus::Six,
            7 => MeasuredStatus::Seven,
            8 => MeasuredStatus::Eight,
            9 => MeasuredStatus::Nine,
            _ => MeasuredStatus::Ten,
        };
        MeasuredLabels { method, status }
    }
}

pub struct MeasuredRegistry {
    counters: Vec<(String, measured::CounterVec<MeasuredLabelsSet>)>,
    histograms: Vec<(String, measured::HistogramVec<MeasuredLabelsSet, 10>)>,
}

impl MeasuredRegistry {
    pub fn encode_text(&self) -> bytes::Bytes {
        use measured::{
            metric::{MetricFamilyEncoding, name::MetricName},
            text::BufferedTextEncoder,
        };

        let mut encoder = BufferedTextEncoder::new();
        for (name, counter) in &self.counters {
            let metric_name = MetricName::try_from_str(name)
                .expect("measured metric name validation should succeed");
            counter
                .collect_family_into(metric_name, &mut encoder)
                .expect("BufferedTextEncoder collection is infallible");
        }
        for (name, histogram) in &self.histograms {
            let metric_name = MetricName::try_from_str(name)
                .expect("measured metric name validation should succeed");
            histogram
                .collect_family_into(metric_name, &mut encoder)
                .expect("BufferedTextEncoder collection is infallible");
        }
        encoder.finish()
    }
}

pub fn setup_measured_registry(metric_count: u32, observe_time: u32) -> MeasuredRegistry {
    use measured::{CounterVec, HistogramVec, metric::histogram::Thresholds};

    let mut rng = rand::rng();

    let mut counters = Vec::with_capacity(metric_count as usize);
    let mut histograms = Vec::with_capacity(metric_count as usize);

    for i in 0..metric_count {
        let counter_vec = CounterVec::with_label_set(MeasuredLabelsSet::default());
        let histogram_vec = HistogramVec::with_label_set_and_metadata(
            MeasuredLabelsSet::default(),
            Thresholds::<10>::exponential_buckets(0.005f64, 2f64),
        );

        for _ in 0..observe_time {
            let labels = MeasuredLabels::random(&mut rng);
            counter_vec.inc(labels);
            let observed_value = rng.random_range(0f64..100f64);
            histogram_vec.observe(labels, observed_value);
        }

        counters.push((format!("my_counter_{i}"), counter_vec));
        histograms.push((format!("my_histogram_{i}"), histogram_vec));
    }

    MeasuredRegistry { counters, histograms }
}

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
                let registry = setup_measured_registry(count, times);
                b.iter(|| {
                    let buffer = registry.encode_text();
                    black_box(buffer);
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
                use fastmetrics::format::text::{TextProfile, encode};

                let registry = setup_fastmetrics_registry(count, times);
                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(&mut buffer, &registry, TextProfile::PrometheusV0_0_4).unwrap();
                    black_box(&mut buffer);
                });
            });

            let id = format!("fastmetrics (openmetrics 1.0.0): {metric_id}");
            group.sample_size(50).bench_function(id, |b| {
                use fastmetrics::format::text::{TextProfile, encode};

                let registry = setup_fastmetrics_registry(count, times);
                let mut buffer = String::new();
                b.iter(|| {
                    buffer.clear();
                    encode(
                        &mut buffer,
                        &registry,
                        TextProfile::OpenMetricsV1_0_0 { escaping_scheme: Default::default() },
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
