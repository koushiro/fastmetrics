use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
// use pprof::criterion::{Output, PProfProfiler};
use rand::RngExt;

mod common;
use self::common::with_metrics_recorder;

mod measured_setup {
    use measured::{CounterVec, HistogramVec, label::LabelGroupSet, metric::histogram::Thresholds};

    pub struct Families<L: LabelGroupSet> {
        pub counter: CounterVec<L>,
        pub histogram: HistogramVec<L, 10>,
    }

    pub fn setup_families<L, F>(mut new_label_set: F) -> Families<L>
    where
        L: LabelGroupSet,
        F: FnMut() -> L,
    {
        let counter = CounterVec::with_label_set(new_label_set());
        let histogram = HistogramVec::with_label_set_and_metadata(
            new_label_set(),
            Thresholds::<10>::exponential_buckets(0.005f64, 2f64),
        );

        Families { counter, histogram }
    }

    #[derive(Clone, PartialEq, Eq, Hash, measured::LabelGroup)]
    #[label(set = HttpLabelsSet)]
    pub struct MeasuredLabels {
        method: MeasuredMethod,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, measured::FixedCardinalityLabel)]
    pub enum MeasuredMethod {
        Get,
        Put,
    }

    impl From<super::Method> for MeasuredMethod {
        fn from(value: super::Method) -> Self {
            match value {
                super::Method::Get => Self::Get,
                super::Method::Put => Self::Put,
            }
        }
    }

    impl From<super::Labels> for MeasuredLabels {
        fn from(value: super::Labels) -> Self {
            Self { method: value.method.into() }
        }
    }

    pub fn setup_http_families() -> Families<HttpLabelsSet> {
        setup_families(HttpLabelsSet::new)
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, measured::LabelGroup)]
    #[label(set = EmptyLabelsSet)]
    pub struct EmptyLabels {}

    pub fn setup_empty_families() -> Families<EmptyLabelsSet> {
        setup_families(EmptyLabelsSet::new)
    }
}

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
        family::{Family, IndexedFamily, LabelIndexMapping},
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

    pub struct IndexedFamilies<L> {
        pub counter: IndexedFamily<L, Counter>,
        pub histogram: IndexedFamily<L, Histogram>,
    }

    pub fn setup_indexed_families<L: LabelIndexMapping>() -> IndexedFamilies<L> {
        let counter = IndexedFamily::<L, Counter>::default();
        let histogram = IndexedFamily::<L, Histogram>::new(|| {
            Histogram::new(exponential_buckets(0.005f64, 2f64, 10))
        });
        IndexedFamilies { counter, histogram }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, fastmetrics::derive::LabelIndexMapping)]
enum Method {
    Get,
    Put,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, fastmetrics::derive::LabelIndexMapping)]
struct Labels {
    method: Method,
}

impl Labels {
    const fn new(method: Method) -> Self {
        Self { method }
    }

    const fn method(&self) -> &'static str {
        match self.method {
            Method::Get => "GET",
            Method::Put => "PUT",
        }
    }
}

struct Input {
    labels: Labels,
    value: f64,
}

fn setup_input() -> Input {
    let mut rng = rand::rng();
    let method = match rng.random_ratio(8, 10) {
        true => Method::Get,
        false => Method::Put,
    };
    let labels = Labels::new(method);
    let value = rng.random_range(0f64..100f64);
    Input { labels, value }
}

fn bench_family_with_empty_labels(c: &mut Criterion) {
    let mut group = c.benchmark_group("family with empty labels");
    group.bench_function("metrics_cached", |b| {
        with_metrics_recorder(|| {
            let labels: &[(&str, &str)] = &[];
            let counter = metrics::counter!("with_empty_labels", labels);
            let histogram = metrics::histogram!("with_empty_labels", labels);

            b.iter_batched(
                || {
                    let mut rng = rand::rng();
                    rng.random_range(0f64..100f64)
                },
                |input| {
                    let value = black_box(input);
                    counter.increment(1);
                    histogram.record(value);
                },
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("metrics_dynamic", |b| {
        with_metrics_recorder(|| {
            let labels: &[(&str, &str)] = &[];

            b.iter_batched(
                || {
                    let mut rng = rand::rng();
                    rng.random_range(0f64..100f64)
                },
                |input| {
                    let value = black_box(input);
                    metrics::counter!("with_empty_labels", labels).increment(1);
                    metrics::histogram!("with_empty_labels", labels).record(value);
                },
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("measured", |b| {
        let families = measured_setup::setup_empty_families();

        b.iter_batched(
            || {
                let mut rng = rand::rng();
                rng.random_range(0f64..100f64)
            },
            |input| {
                let value = black_box(input);
                let labels = black_box(measured_setup::EmptyLabels {});
                families.counter.inc(labels);
                families.histogram.observe(labels, value);
            },
            BatchSize::SmallInput,
        );
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
                let value = black_box(input);
                families.counter.with_label_values(black_box(empty_labels)).inc();
                families.histogram.with_label_values(black_box(empty_labels)).observe(value);
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
                let value = black_box(input);
                families.counter.get_or_create(black_box(&())).inc();
                families.histogram.get_or_create(black_box(&())).observe(value);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics_cached", |b| {
        let families = fastmetrics_setup::setup_families::<()>();
        let labels = ();
        let counter = families.counter.with_or_new(&labels, Clone::clone);
        let histogram = families.histogram.with_or_new(&labels, Clone::clone);

        b.iter_batched(
            || {
                let mut rng = rand::rng();
                rng.random_range(0f64..100f64)
            },
            |input| {
                let value = black_box(input);
                counter.inc();
                histogram.observe(value);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics_dynamic", |b| {
        let families = fastmetrics_setup::setup_families::<()>();

        b.iter_batched(
            || {
                let mut rng = rand::rng();
                rng.random_range(0f64..100f64)
            },
            |input| {
                let value = black_box(input);
                families.counter.with_or_new(black_box(&()), |counter| counter.inc());
                families.histogram.with_or_new(black_box(&()), |hist| hist.observe(value));
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics_indexed", |b| {
        let families = fastmetrics_setup::setup_indexed_families::<()>();
        let labels = ();
        let label_index = fastmetrics::metrics::family::LabelIndex::new(&labels);
        let counter = families.counter.get_by_index(label_index);
        let histogram = families.histogram.get_by_index(label_index);

        b.iter_batched(
            || {
                let mut rng = rand::rng();
                rng.random_range(0f64..100f64)
            },
            |input| {
                let value = black_box(input);
                counter.inc();
                histogram.observe(value);
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_family_with_custom_labels(c: &mut Criterion) {
    let mut group = c.benchmark_group("family with custom labels");
    group.bench_function("metrics_cached", |b| {
        with_metrics_recorder(|| {
            let counter_get = metrics::counter!("family_custom_labels_counter", "method" => "GET");
            let counter_put = metrics::counter!("family_custom_labels_counter", "method" => "PUT");
            let hist_get = metrics::histogram!("family_custom_labels_histogram", "method" => "GET");
            let hist_put = metrics::histogram!("family_custom_labels_histogram", "method" => "PUT");

            b.iter_batched(
                setup_input,
                |input| {
                    let value = black_box(input.value);
                    match input.labels.method {
                        Method::Get => {
                            counter_get.increment(1);
                            hist_get.record(value);
                        },
                        Method::Put => {
                            counter_put.increment(1);
                            hist_put.record(value);
                        },
                    }
                },
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("metrics_dynamic", |b| {
        with_metrics_recorder(|| {
            b.iter_batched(
                setup_input,
                |input| {
                    let method = black_box(input.labels.method());
                    let value = black_box(input.value);
                    metrics::counter!("family_custom_labels_counter", "method" => method)
                        .increment(1);
                    metrics::histogram!("family_custom_labels_histogram", "method" => method)
                        .record(value);
                },
                BatchSize::SmallInput,
            );
        });
    });
    group.bench_function("measured", |b| {
        let families = measured_setup::setup_http_families();

        b.iter_batched(
            setup_input,
            |input| {
                let labels = black_box(measured_setup::MeasuredLabels::from(input.labels));
                let value = black_box(input.value);
                families.counter.inc(labels.clone());
                families.histogram.observe(labels, value);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("prometheus", |b| {
        let families = prometheus_setup::set_families(&["method"]);

        b.iter_batched(
            || {
                let input = setup_input();
                ([input.labels.method()], input.value)
            },
            |(labels, value)| {
                let value = black_box(value);
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
                let value = black_box(input.value);
                families.counter.get_or_create(black_box(&input.labels)).inc();
                families.histogram.get_or_create(black_box(&input.labels)).observe(value);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics_cached", |b| {
        let families = fastmetrics_setup::setup_families::<Labels>();
        let labels_get = Labels::new(Method::Get);
        let labels_put = Labels::new(Method::Put);
        let counter_get = families.counter.with_or_new(&labels_get, Clone::clone);
        let counter_put = families.counter.with_or_new(&labels_put, Clone::clone);
        let hist_get = families.histogram.with_or_new(&labels_get, Clone::clone);
        let hist_put = families.histogram.with_or_new(&labels_put, Clone::clone);

        b.iter_batched(
            setup_input,
            |input| {
                let value = black_box(input.value);
                match input.labels.method {
                    Method::Get => {
                        counter_get.inc();
                        hist_get.observe(value);
                    },
                    Method::Put => {
                        counter_put.inc();
                        hist_put.observe(value);
                    },
                }
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics_dynamic", |b| {
        let families = fastmetrics_setup::setup_families::<Labels>();

        b.iter_batched(
            setup_input,
            |input| {
                let value = black_box(input.value);
                families.counter.with_or_new(black_box(&input.labels), |counter| counter.inc());
                families
                    .histogram
                    .with_or_new(black_box(&input.labels), |hist| hist.observe(value));
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("fastmetrics_indexed", |b| {
        let families = fastmetrics_setup::setup_indexed_families::<Labels>();

        b.iter_batched(
            setup_input,
            |input| {
                let labels = black_box(input.labels);
                let value = black_box(input.value);
                let label_index = fastmetrics::metrics::family::LabelIndex::new(&labels);
                families.counter.get_by_index(label_index).inc();
                families.histogram.get_by_index(label_index).observe(value);
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
                let value = black_box(value);
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
                let value = black_box(value);
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
                let value = black_box(value);
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
                let value = black_box(value);
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
                let value = black_box(value);
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
                let value = black_box(value);
                families.counter.with_or_new(black_box(&labels), |counter| counter.inc());
                families.histogram.with_or_new(black_box(&labels), |hist| hist.observe(value));
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()/*.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))*/;
    targets = bench_family_with_empty_labels, bench_family_with_custom_labels, bench_family_with_string_labels
);
criterion_main!(benches);
