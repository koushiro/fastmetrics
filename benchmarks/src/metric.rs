use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use rand::Rng;

fn bench_counter(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter::inc");
    group.bench_function("prometheus", |b| {
        use prometheus::IntCounter;
        let counter = IntCounter::new("my_counter", "My counter").unwrap();

        b.iter(|| counter.inc());
    });
    group.bench_function("prometheus_client", |b| {
        use prometheus_client::metrics::counter::Counter;
        let counter = <Counter>::default();

        b.iter(|| black_box(counter.inc()));
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::counter::Counter;
        let counter = <Counter>::default();

        b.iter(|| black_box(counter.inc()));
    });
    group.finish();
}

fn bench_gauge(c: &mut Criterion) {
    let mut group = c.benchmark_group("gauge::set");
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
            |input| black_box(gauge.set(black_box(input))),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| black_box(gauge.set(black_box(input))),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("gauge::inc_by");
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
            |input| black_box(gauge.inc_by(black_box(input))),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| black_box(gauge.inc_by(black_box(input))),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("gauge::dec_by");
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
            |input| black_box(gauge.dec_by(black_box(input))),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::gauge::Gauge;
        let gauge = <Gauge>::default();

        b.iter_batched(
            || rand::rng().random::<i64>(),
            |input| black_box(gauge.dec_by(black_box(input))),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_histogram(c: &mut Criterion) {
    let mut group = c.benchmark_group("histogram::observe");
    group.bench_function("prometheus", |b| {
        use prometheus::{exponential_buckets, histogram_opts, Histogram};
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
        use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};
        let histogram = Histogram::new(exponential_buckets(0.005f64, 2f64, 10));

        b.iter_batched(
            || rand::rng().random_range(0f64..100f64),
            |input| histogram.observe(black_box(input)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        use fastmetrics::metrics::histogram::{exponential_buckets, Histogram};
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
            |input| {
                black_box(histogram.observe(black_box(input)));
            },
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
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_counter, bench_gauge, bench_histogram
);
criterion_main!(benches);
