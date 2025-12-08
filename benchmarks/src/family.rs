use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
// use pprof::criterion::{Output, PProfProfiler};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

mod common;
use self::common::with_metrics_recorder;

mod prometheus_setup {
    use prometheus::{HistogramVec, IntCounterVec, exponential_buckets, histogram_opts, opts};

    pub struct Families {
        pub counter: IntCounterVec,
        pub histogram: HistogramVec,
    }

    pub fn set_families(label_names: &[&str]) -> Families {
        let counter =
            IntCounterVec::new(opts! {"my_counter", "my_counter help"}, label_names).unwrap();
        let histogram = HistogramVec::new(
            histogram_opts! {
                "my_histogram",
                "my_histogram help",
                exponential_buckets(0.005f64, 2f64, 10).unwrap()
            },
            label_names,
        )
        .unwrap();
        Families { counter, histogram }
    }
}

mod prometheus_client_setup {
    use prometheus_client::metrics::{
        counter::Counter,
        family::Family,
        histogram::{Histogram, exponential_buckets},
    };

    pub struct Families<L> {
        pub counter: Family<L, Counter>,
        pub histogram: Family<L, Histogram>,
    }

    pub fn setup_families<L>() -> Families<L>
    where
        L: Clone + Eq + std::hash::Hash,
    {
        let counter = Family::<L, Counter>::default();
        let histogram = Family::<L, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.005f64, 2f64, 10))
        });

        Families { counter, histogram }
    }
}

mod fastmetrics_setup {
    use fastmetrics::metrics::{
        counter::Counter,
        family::Family,
        histogram::{Histogram, exponential_buckets},
    };

    pub struct Families<L> {
        pub counter: Family<L, Counter>,
        pub histogram: Family<L, Histogram>,
    }

    pub fn setup_families<L>() -> Families<L> {
        let counter = Family::<L, Counter>::default();
        let histogram =
            Family::<L, Histogram>::new(|| Histogram::new(exponential_buckets(0.005f64, 2f64, 10)));
        Families { counter, histogram }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Labels {
    method: Method,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Method {
    Get,
    Put,
}

impl Labels {
    const fn method(&self) -> &'static str {
        match self.method {
            Method::Get => "GET",
            Method::Put => "PUT",
        }
    }
}

impl Distribution<Labels> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Labels {
        let method = match rng.random_ratio(8, 10) {
            true => Method::Get,
            false => Method::Put,
        };
        Labels { method }
    }
}

struct Input {
    labels: Labels,
    value: f64,
}

fn setup_input() -> Input {
    let mut rng = rand::rng();
    let labels = rng.random::<Labels>();
    let value = rng.random_range(0f64..100f64);
    Input { labels, value }
}

fn bench_family_without_labels(c: &mut Criterion) {
    let mut group = c.benchmark_group("family without labels");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            let counter = metrics::counter!("without_labels");
            let histogram = metrics::histogram!("without_labels");

            b.iter_batched(
                || {
                    let mut rng = rand::rng();
                    rng.random_range(0f64..100f64)
                },
                |input| {
                    counter.increment(1);
                    histogram.record(black_box(input));
                },
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        let empty_labels: &[&str] = &[];
        let families = prometheus_setup::set_families(empty_labels);

        b.iter_batched(
            || {
                let mut rng = rand::rng();
                rng.random_range(0f64..100f64)
            },
            |input| {
                families.counter.with_label_values(black_box(empty_labels)).inc();
                families.histogram.with_label_values(black_box(empty_labels)).observe(input);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        let families = prometheus_client_setup::setup_families::<()>();

        b.iter_batched(
            || {
                let mut rng = rand::rng();
                rng.random_range(0f64..100f64)
            },
            |input| {
                families.counter.get_or_create(black_box(&())).inc();
                families.histogram.get_or_create(black_box(&())).observe(input);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        let families = fastmetrics_setup::setup_families::<()>();

        b.iter_batched(
            || {
                let mut rng = rand::rng();
                rng.random_range(0f64..100f64)
            },
            |input| {
                families.counter.with_or_new(black_box(&()), |counter| counter.inc());
                families.histogram.with_or_new(black_box(&()), |hist| hist.observe(input));
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_family_with_custom_labels(c: &mut Criterion) {
    let mut group = c.benchmark_group("family with custom labels");
    group.bench_function("metrics", |b| {
        with_metrics_recorder(|| {
            // The metric handles should be created outside the batched iteration to only measure
            // the label lookup and operation overhead, matching the pattern used by other libraries.
            // But the metrics-rs API doesn't provide this pattern due to its design.

            b.iter_batched(
                setup_input,
                |input| {
                    let method = black_box(input.labels.method());
                    metrics::counter!("family_custom_labels_counter", "method" => method)
                        .increment(1);
                    metrics::histogram!("family_custom_labels_histogram", "method" => method)
                        .record(input.value);
                },
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("prometheus", |b| {
        let families = prometheus_setup::set_families(&["method"]);

        b.iter_batched(
            || {
                let input = setup_input();
                ([input.labels.method()], input.value)
            },
            |(labels, value)| {
                families.counter.with_label_values(black_box(&labels)).inc();
                families.histogram.with_label_values(black_box(&labels)).observe(value);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus_client", |b| {
        let families = prometheus_client_setup::setup_families::<Labels>();

        b.iter_batched(
            setup_input,
            |input| {
                families.counter.get_or_create(black_box(&input.labels)).inc();
                families.histogram.get_or_create(black_box(&input.labels)).observe(input.value);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        let families = fastmetrics_setup::setup_families::<Labels>();

        b.iter_batched(
            setup_input,
            |input| {
                families.counter.with_or_new(black_box(&input.labels), |counter| counter.inc());
                families
                    .histogram
                    .with_or_new(black_box(&input.labels), |hist| hist.observe(input.value));
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_family_with_string_labels(c: &mut Criterion) {
    type StrArrayLabels = [(&'static str, &'static str); 1];
    let mut group = c.benchmark_group("family with [(&'static str, &'static str)] labels");
    group.bench_function("prometheus_client", |b| {
        let families = prometheus_client_setup::setup_families::<StrArrayLabels>();

        b.iter_batched(
            || {
                let input = setup_input();
                ([("method", input.labels.method())], input.value)
            },
            |(labels, value)| {
                families.counter.get_or_create(black_box(&labels)).inc();
                families.histogram.get_or_create(black_box(&labels)).observe(value)
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        let families = fastmetrics_setup::setup_families::<StrArrayLabels>();

        b.iter_batched(
            || {
                let input = setup_input();
                ([("method", input.labels.method())], input.value)
            },
            |(labels, value)| {
                families.counter.with_or_new(black_box(&labels), |counter| counter.inc());
                families.histogram.with_or_new(black_box(&labels), |hist| hist.observe(value));
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();

    type StrVecLabels = Vec<(&'static str, &'static str)>;
    let mut group = c.benchmark_group("family with Vec<(&'static str, &'static str)> labels");
    group.bench_function("prometheus_client", |b| {
        let families = prometheus_client_setup::setup_families::<StrVecLabels>();

        b.iter_batched(
            || {
                let input = setup_input();
                (vec![("method", input.labels.method())], input.value)
            },
            |(labels, value)| {
                families.counter.get_or_create(black_box(&labels)).inc();
                families.histogram.get_or_create(black_box(&labels)).observe(value)
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        let families = fastmetrics_setup::setup_families::<StrVecLabels>();

        b.iter_batched(
            || {
                let input = setup_input();
                (vec![("method", input.labels.method())], input.value)
            },
            |(labels, value)| {
                families.counter.with_or_new(black_box(&labels), |counter| counter.inc());
                families.histogram.with_or_new(black_box(&labels), |hist| hist.observe(value));
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();

    type OwnedStrVecLabels = Vec<(String, String)>;
    let mut group = c.benchmark_group("family with Vec<(String, String)> labels");
    group.bench_function("prometheus_client", |b| {
        let families = prometheus_client_setup::setup_families::<OwnedStrVecLabels>();

        b.iter_batched(
            || {
                let input = setup_input();
                (vec![("method".to_owned(), input.labels.method().to_owned())], input.value)
            },
            |(labels, value)| {
                families.counter.get_or_create(black_box(&labels)).inc();
                families.histogram.get_or_create(black_box(&labels)).observe(value)
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics", |b| {
        let families = fastmetrics_setup::setup_families::<OwnedStrVecLabels>();

        b.iter_batched(
            || {
                let input = setup_input();
                (vec![("method".to_owned(), input.labels.method().to_owned())], input.value)
            },
            |(labels, value)| {
                families.counter.with_or_new(black_box(&labels), |counter| counter.inc());
                families.histogram.with_or_new(black_box(&labels), |hist| hist.observe(value));
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_family_concurrent_metric_creation(c: &mut Criterion) {
    #[derive(Clone, PartialEq, Eq, Hash)]
    #[derive(prometheus_client::encoding::EncodeLabelSet)]
    #[derive(fastmetrics::derive::EncodeLabelSet, fastmetrics::derive::LabelSetSchema)]
    struct WorkerLabels {
        worker: String,
    }

    impl WorkerLabels {
        fn new(index: usize) -> Self {
            Self { worker: format!("worker-{index}") }
        }
    }

    const THREADS: usize = 4;
    const LABELS_PER_THREAD: usize = 512;

    let total_labels = THREADS * LABELS_PER_THREAD;
    let label_pool: Vec<WorkerLabels> = (0..total_labels).map(WorkerLabels::new).collect();
    let chunk_size = std::cmp::max(label_pool.len() / THREADS, 1);

    let mut group = c.benchmark_group("family concurrent new metric creation");
    group.bench_function("prometheus_client", |b| {
        let label_pool = &label_pool;
        b.iter_batched(
            prometheus_client_setup::setup_families::<WorkerLabels>,
            |families| {
                std::thread::scope(|scope| {
                    for chunk in label_pool.chunks(chunk_size) {
                        let histogram = families.histogram.clone();
                        scope.spawn(move || {
                            for labels in chunk {
                                let labels = black_box(labels);
                                let _ = black_box(histogram.get_or_create(labels));
                            }
                        });
                    }
                });
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("fastmetrics", |b| {
        let label_pool = &label_pool;
        b.iter_batched(
            fastmetrics_setup::setup_families::<WorkerLabels>,
            |families| {
                std::thread::scope(|scope| {
                    for chunk in label_pool.chunks(chunk_size) {
                        let histogram = families.histogram.clone();
                        scope.spawn(move || {
                            for labels in chunk {
                                let labels = black_box(labels);
                                histogram.with_or_new(labels, |metric| {
                                    black_box(metric as *const _);
                                });
                            }
                        });
                    }
                });
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()/*.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))*/;
    targets = bench_family_without_labels, bench_family_with_custom_labels, bench_family_with_string_labels, bench_family_concurrent_metric_creation
);
criterion_main!(benches);
