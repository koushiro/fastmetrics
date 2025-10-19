use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(prometheus_client::encoding::EncodeLabelSet)]
#[derive(fastmetrics::encoder::EncodeLabelSet)]
pub struct Labels {
    method: Method,
    status: u16,
}

impl Labels {
    fn method(&self) -> &'static str {
        match self.method {
            Method::Get => "GET",
            Method::Put => "PUT",
        }
    }

    fn status(&self) -> &'static str {
        match self.status {
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            _ => "10",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(prometheus_client::encoding::EncodeLabelValue)]
#[derive(fastmetrics::encoder::EncodeLabelValue)]
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

pub fn setup_prometheus_registry(metric_count: u32, observe_time: u32) -> prometheus::Registry {
    use prometheus::{
        exponential_buckets, register_histogram_vec_with_registry,
        register_int_counter_vec_with_registry,
    };

    let mut rng = rand::rng();

    let registry = prometheus::Registry::default();
    let label_names = &["method", "status"];
    let buckets = exponential_buckets(0.005, 2.0, 10).unwrap();

    for i in 0..metric_count {
        let counter_vec = register_int_counter_vec_with_registry!(
            format!("my_counter_{}", i),
            "My counter",
            label_names,
            registry
        )
        .unwrap();
        let histogram_vec = register_histogram_vec_with_registry!(
            format!("my_histogram_{}", i),
            "My histogram",
            label_names,
            buckets.clone(),
            registry
        )
        .unwrap();

        for _ in 0..observe_time {
            let labels = rng.random::<Labels>();
            let label_values = [labels.method(), labels.status()];

            counter_vec.with_label_values(&label_values).inc();
            let observed_value = rng.random_range(0f64..100f64);
            histogram_vec.with_label_values(&label_values).observe(observed_value);
        }
    }

    registry
}

pub fn setup_prometheus_client_registry(
    metric_count: u32,
    observe_time: u32,
) -> prometheus_client::registry::Registry {
    use prometheus_client::metrics::{
        counter::Counter,
        family::Family,
        histogram::{Histogram, exponential_buckets},
    };

    let mut rng = rand::rng();

    let mut registry = prometheus_client::registry::Registry::default();

    for i in 0..metric_count {
        let counter_family = Family::<Labels, Counter>::default();
        let histogram_family = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.005, 2.0, 10))
        });

        registry.register(format!("my_counter_{i}"), "My counter", counter_family.clone());
        registry.register(format!("my_histogram_{i}"), "My histogram", histogram_family.clone());

        for _ in 0..observe_time {
            let labels = rng.random::<Labels>();
            counter_family.get_or_create(&labels).inc();
            let observed_value = rng.random_range(0f64..100f64);
            histogram_family.get_or_create(&labels).observe(observed_value);
        }
    }

    registry
}

pub fn setup_fastmetrics_registry(
    metric_count: u32,
    observe_time: u32,
) -> fastmetrics::registry::Registry {
    use fastmetrics::metrics::{
        counter::Counter,
        family::Family,
        histogram::{Histogram, exponential_buckets},
    };

    let mut rng = rand::rng();

    let mut registry = fastmetrics::registry::Registry::default();

    for i in 0..metric_count {
        let counter_family = Family::<Labels, Counter>::default();
        let histogram_family = Family::<Labels, Histogram>::new(|| {
            Histogram::new(exponential_buckets(0.005, 2.0, 10))
        });

        registry
            .register(format!("my_counter_{i}"), "My counter", counter_family.clone())
            .unwrap();
        registry
            .register(format!("my_histogram_{i}"), "My histogram", histogram_family.clone())
            .unwrap();

        for _ in 0..observe_time {
            let labels = rng.random::<Labels>();
            counter_family.with_or_new(&labels, |counter| counter.inc());
            let observed_value = rng.random_range(0f64..100f64);
            histogram_family.with_or_new(&labels, |hist| hist.observe(observed_value));
        }
    }

    registry
}
