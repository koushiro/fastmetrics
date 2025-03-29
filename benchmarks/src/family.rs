use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

mod prometheus_client_setup {
    use prometheus_client::metrics::{
        counter::Counter,
        family::Family,
        histogram::{exponential_buckets, Histogram},
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

mod openmetrics_client_setup {
    use openmetrics_client::metrics::{
        counter::Counter,
        family::Family,
        histogram::{exponential_buckets, Histogram},
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
    group.bench_function("openmetrics_client", |b| {
        let families = openmetrics_client_setup::setup_families::<()>();

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
    group.bench_function("openmetrics_client", |b| {
        let families = openmetrics_client_setup::setup_families::<StrArrayLabels>();

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
    group.bench_function("openmetrics_client", |b| {
        let families = openmetrics_client_setup::setup_families::<StrVecLabels>();

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
    group.bench_function("openmetrics_client", |b| {
        let families = openmetrics_client_setup::setup_families::<OwnedStrVecLabels>();

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

fn bench_family_with_custom_labels(c: &mut Criterion) {
    let mut group = c.benchmark_group("family with custom labels");
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
    group.bench_function("openmetrics_client", |b| {
        let families = openmetrics_client_setup::setup_families::<Labels>();

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

criterion_group!(
    benches,
    bench_family_without_labels,
    bench_family_with_string_labels,
    bench_family_with_custom_labels
);
criterion_main!(benches);
