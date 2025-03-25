use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::Rng;

fn prometheus_client_baseline(c: &mut Criterion) {
    use prometheus_client::metrics::{
        counter::Counter,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    };

    let mut group = c.benchmark_group("prometheus_client");

    group.bench_function("counter::inc", |b| {
        let counter = <Counter>::default();

        b.iter(|| {
            let _ret = black_box(counter.inc());
        })
    });
    group.bench_function("gauge::set", |b| {
        let gauge = <Gauge>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random::<i64>(),
            |data| {
                let _ret = gauge.set(black_box(data));
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("histogram::observe", |b| {
        let buckets = exponential_buckets(0.005f64, 2f64, 10);
        let histogram = Histogram::new(buckets);
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random_range(0f64..100f64),
            |data| histogram.observe(black_box(data)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn openmetrics_client_baseline(c: &mut Criterion) {
    use openmetrics_client::metrics::{
        counter::Counter,
        gauge::Gauge,
        gauge_histogram::GaugeHistogram,
        histogram::{exponential_buckets, linear_buckets, Histogram},
        state_set::{StateSet, StateSetValue},
    };
    use rand::distr::{Distribution, StandardUniform};

    let mut group = c.benchmark_group("openmetrics_client");

    group.bench_function("counter::inc", |b| {
        let counter = <Counter>::default();

        b.iter(|| {
            let _ret = black_box(counter.inc());
        })
    });
    group.bench_function("gauge::set", |b| {
        let gauge = <Gauge>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random::<i64>(),
            |data| {
                let _ret = gauge.set(black_box(data));
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("histogram::observe", |b| {
        let buckets = exponential_buckets(0.005f64, 2f64, 10);
        let histogram = Histogram::new(buckets);
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random_range(0f64..100f64),
            |data| histogram.observe(black_box(data)),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("gauge_histogram::observe", |b| {
        let buckets = linear_buckets(-100f64, 10f64, 20);
        let histogram = GaugeHistogram::new(buckets);
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random_range(-100f64..200f64),
            |data| histogram.observe(black_box(data)),
            BatchSize::SmallInput,
        )
    });

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

    group.bench_function("stateset::set", |b| {
        let stateset = StateSet::<JobState>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random::<JobState>(),
            |data| stateset.set(black_box(data)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, prometheus_client_baseline, openmetrics_client_baseline);
criterion_main!(benches);
