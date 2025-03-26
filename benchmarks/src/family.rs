use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Labels {
    method: Method,
    status: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Method {
    Get,
    Put,
}

impl Labels {
    const fn method(&self) -> &'static str {
        stringify!(self.method)
    }

    const fn status(&self) -> &'static str {
        stringify!(self.status)
    }
}

impl Distribution<Labels> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Labels {
        let method = match rng.random_ratio(8, 10) {
            true => Method::Get,
            false => Method::Put,
        };
        let status = rng.random_range(1..=10);
        Labels { method, status }
    }
}

fn prometheus_client_family(c: &mut Criterion) {
    use prometheus_client::metrics::{counter::Counter, family::Family};

    let mut group = c.benchmark_group("prometheus_client");

    group.bench_function("counter family without labels", |b| {
        let family = Family::<(), Counter>::default();

        b.iter(|| {
            let _ret = family.get_or_create(black_box(&())).inc();
        });
    });
    group.bench_function("counter family with [(&'static str, &'static str)] labels", |b| {
        let family = Family::<[(&'static str, &'static str); 2], Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || {
                let labels = rng.random::<Labels>();
                [("method", labels.method()), ("status", labels.status())]
            },
            |labels| {
                let _ret = family.get_or_create(black_box(&labels)).inc();
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("counter family with Vec<(&'static str, &'static str)> labels", |b| {
        let family = Family::<Vec<(&'static str, &'static str)>, Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || {
                let labels = rng.random::<Labels>();
                vec![("method", labels.method()), ("status", labels.status())]
            },
            |labels| {
                let _ret = family.get_or_create(&black_box(labels)).inc();
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("counter family with Vec<(String, String)> labels", |b| {
        let family = Family::<Vec<(String, String)>, Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || {
                let labels = rng.random::<Labels>();
                vec![
                    ("method".to_owned(), labels.method().to_owned()),
                    ("status".to_owned(), labels.status().to_owned()),
                ]
            },
            |labels| {
                let _ret = family.get_or_create(&black_box(labels)).inc();
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("counter family with custom labels", |b| {
        let family = Family::<Labels, Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random::<Labels>(),
            |labels| {
                let _ret = family.get_or_create(&black_box(labels)).inc();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn openmetrics_client_family(c: &mut Criterion) {
    use openmetrics_client::metrics::{counter::Counter, family::Family};

    let mut group = c.benchmark_group("openmetrics_client");

    group.bench_function("counter family without labels", |b| {
        let family = Family::<(), Counter>::default();
        family.with_or_default(&(), |_| {});

        b.iter(|| {
            let _ret = family.with(black_box(&()), |counter| counter.inc());
        })
    });
    group.bench_function("counter family with [(&'static str, &'static str)] labels", |b| {
        let family = Family::<[(&'static str, &'static str); 2], Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || {
                let labels = rng.random::<Labels>();
                [("method", labels.method()), ("status", labels.status())]
            },
            |labels| {
                let _ret = family.with_or_default(black_box(&labels), |counter| counter.inc());
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("counter family with Vec<(&'static str, &'static str)> labels", |b| {
        let family = Family::<Vec<(&'static str, &'static str)>, Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || {
                let labels = rng.random::<Labels>();
                vec![("method", labels.method()), ("status", labels.status())]
            },
            |labels| {
                let _ret = family.with_or_default(black_box(&labels), |counter| counter.inc());
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("counter family with Vec<(String, String)> labels", |b| {
        let family = Family::<Vec<(String, String)>, Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || {
                let labels = rng.random::<Labels>();
                vec![
                    ("method".to_owned(), labels.method().to_owned()),
                    ("status".to_owned(), labels.status().to_owned()),
                ]
            },
            |labels| {
                let _ret = family.with_or_default(black_box(&labels), |counter| counter.inc());
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("counter family with custom labels", |b| {
        let family = Family::<Labels, Counter>::default();
        let mut rng = rand::rng();

        b.iter_batched(
            || rng.random::<Labels>(),
            |labels| {
                let _ret = family.with_or_default(black_box(&labels), |counter| counter.inc());
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, prometheus_client_family, openmetrics_client_family);
criterion_main!(benches);
