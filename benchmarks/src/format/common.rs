use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(prometheus_client::encoding::EncodeLabelSet)]
#[derive(openmetrics_client::encoder::EncodeLabelSet)]
pub struct Labels {
    method: Method,
    status: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(prometheus_client::encoding::EncodeLabelValue)]
#[derive(openmetrics_client::encoder::EncodeLabelValue)]
pub enum Method {
    Get,
    Put,
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

pub fn setup_prometheus_client_registry(
    metric_count: u32,
    observe_time: u32,
) -> prometheus_client::registry::Registry {
    use prometheus_client::metrics::{
        counter::Counter,
        family::Family,
        histogram::{exponential_buckets, Histogram},
    };

    let mut rng = rand::rng();

    let mut registry = prometheus_client::registry::Registry::default();

    for i in 0..metric_count {
        let counter_family = Family::<Labels, Counter>::default();
        let histogram_family = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.005, 2.0, 10))
        });

        registry.register(format!("my_counter_{}", i), "My counter", counter_family.clone());
        registry.register(format!("my_histogram_{}", i), "My histogram", histogram_family.clone());

        for _ in 0..observe_time {
            let labels = rng.random::<Labels>();
            counter_family.get_or_create(&labels).inc();
            let observed_value = rng.random_range(0f64..1_000_000f64);
            histogram_family.get_or_create(&labels).observe(observed_value);
        }
    }

    registry
}

pub fn setup_openmetrics_client_registry(
    metric_count: u32,
    observe_time: u32,
) -> openmetrics_client::registry::Registry {
    use openmetrics_client::metrics::{counter::Counter, family::Family, histogram::Histogram};

    let mut rng = rand::rng();

    let mut registry = openmetrics_client::registry::Registry::default();

    for i in 0..metric_count {
        let counter_family = Family::<Labels, Counter>::default();
        let histogram_family = Family::<Labels, Histogram>::default();

        registry
            .register(format!("my_counter_{}", i), "My counter", counter_family.clone())
            .unwrap();
        registry
            .register(format!("my_histogram_{}", i), "My histogram", histogram_family.clone())
            .unwrap();

        for _ in 0..observe_time {
            let labels = rng.random::<Labels>();
            counter_family.with_or_default(&labels, |counter| counter.inc());
            let observed_value = rng.random_range(0f64..1_000_000f64);
            histogram_family.with_or_default(&labels, |hist| hist.observe(observed_value));
        }
    }

    registry
}
