use std::{hint::black_box, sync::atomic::AtomicU64};

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
// use pprof::criterion::{Output, PProfProfiler};
use rand::Rng;

mod common;
use self::common::with_metrics_recorder;

fn bench_counter(c: &mut Criterion) {
    bench_counter_u64(c);
    bench_counter_f64(c);
}

fn bench_counter_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter(u64)::inc");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::counter!("my_counter");

            b.iter(|| handle.increment(1));
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::IntCounter;
        let counter = IntCounter::new("my_counter", "My counter").unwrap();

        b.iter(|| counter.inc());
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::counter::Counter;
        let counter = <Counter>::default();

        b.iter(|| counter.inc());
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::counter::Counter;
        let counter = <Counter>::default();

        b.iter(|| counter.inc());
    });
    group.finish();
}

fn bench_counter_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter(f64)::inc");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::counter!("my_counter_f64");

            b.iter(|| handle.increment(1));
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::Counter;
        let counter = Counter::new("my_counter", "My counter").unwrap();

        b.iter(|| counter.inc());
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::counter::Counter;
        let counter = Counter::<f64, AtomicU64>::default();

        b.iter(|| counter.inc());
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::counter::Counter;
        let counter = Counter::<f64>::default();

        b.iter(|| counter.inc());
    });
    group.finish();
}

fn bench_gauge(c: &mut Criterion) {
    bench_gauge_i64(c);
    bench_gauge_f64(c);
}

fn bench_gauge_i64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gauge(i64)::set");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::gauge!("my_gauge_i64_set");

            b.iter_batched(
                || rand::rng().random::<i64>(),
                |input| handle.set(black_box(input as f64)),
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::IntGauge;
        let gauge = IntGauge::new("my_gauge", "My gauge").unwrap();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.set(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.set(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.set(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("gauge(i64)::inc_by");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::gauge!("my_gauge_i64_inc");

            b.iter_batched(
                || rand::rng().random::<i64>(),
                |input| handle.increment(black_box(input as f64)),
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::IntGauge;
        let gauge = IntGauge::new("my_gauge", "My gauge").unwrap();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.add(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.inc_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.inc_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("gauge(i64)::dec_by");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::gauge!("my_gauge_i64_dec");

            b.iter_batched(
                || rand::rng().random::<i64>(),
                |input| handle.decrement(black_box(input as f64)),
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::IntGauge;
        let gauge = IntGauge::new("my_gauge", "My gauge").unwrap();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.sub(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.dec_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| gauge.dec_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_gauge_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gauge(f64)::set");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::gauge!("my_gauge_f64_set");

            b.iter_batched(
                || rand::rng().random::<f64>(),
                |input| handle.set(black_box(input)),
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::Gauge;
        let gauge = Gauge::new("my_gauge", "My gauge").unwrap();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.set(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::gauge::Gauge;
        let gauge = Gauge::<f64, AtomicU64>::default();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.set(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = Gauge::<f64>::default();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.set(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("gauge(f64)::inc_by");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::gauge!("my_gauge_f64_inc");

            b.iter_batched(
                || rand::rng().random::<f64>(),
                |input| handle.increment(black_box(input)),
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::Gauge;
        let gauge = Gauge::new("my_gauge", "My gauge").unwrap();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.add(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::gauge::Gauge;
        let gauge = Gauge::<f64, AtomicU64>::default();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.inc_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = Gauge::<f64>::default();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.inc_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("gauge(f64)::dec_by");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let handle = metrics::gauge!("my_gauge_f64_dec");

            b.iter_batched(
                || rand::rng().random::<f64>(),
                |input| handle.decrement(black_box(input)),
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::Gauge;
        let gauge = Gauge::new("my_gauge", "My gauge").unwrap();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.sub(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::gauge::Gauge;
        let gauge = Gauge::<f64, AtomicU64>::default();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.dec_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = Gauge::<f64>::default();

        b.iter_batched(
            || rand::rng().random::<f64>(),
            |input| gauge.dec_by(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_histogram(c: &mut Criterion) {
    let mut group = c.benchmark_group("histogram::observe");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let histogram = metrics::histogram!("my_histogram");

            b.iter_batched(
                || rand::rng().random_range(0f64..100f64),
                |input| histogram.record(black_box(input)),
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        use prometheus::{Histogram, exponential_buckets, histogram_opts};
        let histogram = Histogram::with_opts(histogram_opts!(
            "my_histogram",
            "My histogram",
            exponential_buckets(0.005f64, 2f64, 10).unwrap()
        ))
        .unwrap();

        b.iter_batched(
            || rand::rng().random_range(0f64..100f64),
            |input| histogram.observe(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::histogram::{Histogram, exponential_buckets};
        let histogram = Histogram::new(exponential_buckets(0.005f64, 2f64, 10));

        b.iter_batched(
            || rand::rng().random_range(0f64..100f64),
            |input| histogram.observe(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::histogram::{Histogram, exponential_buckets};
        let histogram = Histogram::new(exponential_buckets(0.005f64, 2f64, 10));

        b.iter_batched(
            || rand::rng().random_range(0f64..100f64),
            |input| histogram.observe(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

/*
fn bench_gauge_histogram(c: &mut Criterion) {
    let mut group = c.benchmark_group("gauge_histogram::observe");
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge_histogram::{linear_buckets, GaugeHistogram};
        let histogram = GaugeHistogram::new(linear_buckets(-100f64, 10f64, 20));

        b.iter_batched(
            || rand::rng().random_range(-100f64..200f64),
            |input| histogram.observe(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_stateset(c: &mut Criterion) {
    let mut group = c.benchmark_group("stateset::set");
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::state_set::{StateSet, StateSetValue};

        #[derive(Copy, Clone, Debug, PartialEq, Default, StateSetValue)]
        enum JobState {
            #[default]
            Pending,
            Running,
            Completed,
            Failed,
        }

        impl Distribution<JobState> for StandardUniform {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> JobState {
                match rng.random_range(0..=3) {
                    0 => JobState::Pending,
                    1 => JobState::Running,
                    2 => JobState::Completed,
                    _ => JobState::Failed,
                }
            }
        }

        let stateset = StateSet::<JobState>::default();

        b.iter_batched(
            || rand::rng().random::<JobState>(),
            |input| black_box(stateset.set(black_box(input))),
            BatchSize::SmallInput,
        );
    });
}
*/

criterion_group!(
    name = benches;
    config = Criterion::default()/*.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))*/;
    targets = bench_counter, bench_gauge, bench_histogram
);
criterion_main!(benches);
