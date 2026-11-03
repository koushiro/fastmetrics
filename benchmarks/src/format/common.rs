use rand::{
    RngExt,
    distr::{Distribution, StandardUniform},
};

pub fn setup_metrics_exporter_prometheus_handle(
    metric_count: u32,
    observe_time: u32,
) -> metrics_exporter_prometheus::PrometheusHandle {
    let recorder = metrics_exporter_prometheus::PrometheusBuilder::new().build_recorder();
    let handle = recorder.handle();

    metrics::with_local_recorder(&recorder, || {
        let mut rng = rand::rng();

        for i in 0..metric_count {
            let counter_name = format!("my_counter_{i}");
            let histogram_name = format!("my_histogram_{i}");

            for _ in 0..observe_time {
                let labels = rng.random::<Labels>();
                let method = labels.method();
                let status = labels.status();

                metrics::counter!(
                    counter_name.clone(),
                    "method" => method,
                    "status" => status
                )
                .increment(1);

                let observed_value = rng.random_range(0f64..100f64);
                metrics::histogram!(
                    histogram_name.clone(),
                    "method" => method,
                    "status" => status
                )
                .record(observed_value);
            }
        }
    });

    handle.run_upkeep();
    handle
}

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

// Validate metric names at registration so encoding can use the stored bytes directly.
struct MeasuredName(String);

impl MeasuredName {
    fn new(name: String) -> Self {
        measured::metric::name::MetricName::try_from_str(&name)
            .expect("measured metric name validation should succeed");
        Self(name)
    }
}

impl measured::metric::name::MetricNameEncoder for MeasuredName {
    fn encode_utf8(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())
    }

    fn encode_len(&self) -> usize {
        self.0.len()
    }
}

pub struct MeasuredRegistry {
    counters: Vec<(MeasuredName, measured::CounterVec<MeasuredLabelsSet>)>,
    histograms: Vec<(MeasuredName, measured::HistogramVec<MeasuredLabelsSet, 10>)>,
}

impl MeasuredRegistry {
    fn register_counter(&mut self, name: String, counter: measured::CounterVec<MeasuredLabelsSet>) {
        self.counters.push((MeasuredName::new(name), counter));
    }

    fn register_histogram(
        &mut self,
        name: String,
        histogram: measured::HistogramVec<MeasuredLabelsSet, 10>,
    ) {
        self.histograms.push((MeasuredName::new(name), histogram));
    }
}

impl<E: measured::metric::group::Encoding> measured::MetricGroup<E> for MeasuredRegistry {
    fn collect_group_into(&self, encoder: &mut E) -> Result<(), E::Err> {
        use measured::metric::{MetricFamilyEncoding, group::FamilyMeta};

        for (name, counter) in &self.counters {
            counter.collect_family_into(
                name,
                FamilyMeta::new().with_help("My counter"),
                encoder,
            )?;
        }
        for (name, histogram) in &self.histograms {
            histogram.collect_family_into(
                name,
                FamilyMeta::new().with_help("My histogram"),
                encoder,
            )?;
        }
        Ok(())
    }
}

pub fn setup_measured_registry(metric_count: u32, observe_time: u32) -> MeasuredRegistry {
    use measured::{CounterVec, HistogramVec, metric::histogram::Thresholds};

    let mut rng = rand::rng();

    let mut registry = MeasuredRegistry {
        counters: Vec::with_capacity(metric_count as usize),
        histograms: Vec::with_capacity(metric_count as usize),
    };

    for i in 0..metric_count {
        let counter = CounterVec::with_label_set(MeasuredLabelsSet::default());
        let histogram = HistogramVec::with_label_set_and_metadata(
            MeasuredLabelsSet::default(),
            Thresholds::<10>::exponential_buckets(0.005f64, 2f64),
        );

        for _ in 0..observe_time {
            let labels = MeasuredLabels::random(&mut rng);
            counter.inc(labels);
            let observed_value = rng.random_range(0f64..100f64);
            histogram.observe(labels, observed_value);
        }

        registry.register_counter(format!("my_counter_{i}"), counter);
        registry.register_histogram(format!("my_histogram_{i}"), histogram);
    }

    registry
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
            format!("my_counter_{i}"),
            "My counter",
            label_names,
            registry
        )
        .unwrap();
        let histogram_vec = register_histogram_vec_with_registry!(
            format!("my_histogram_{i}"),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(prometheus_client::encoding::EncodeLabelSet)]
#[derive(fastmetrics::derive::EncodeLabelSet, fastmetrics::derive::LabelSetSchema)]
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
#[derive(fastmetrics::derive::EncodeLabelValue)]
pub enum Method {
    Get,
    Put,
}

impl Distribution<Labels> for StandardUniform {
    fn sample<R: RngExt + ?Sized>(&self, rng: &mut R) -> Labels {
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
    name_rule: fastmetrics::registry::NameRule,
) -> fastmetrics::registry::Registry {
    use fastmetrics::metrics::{
        counter::Counter,
        family::Family,
        histogram::{Histogram, exponential_buckets},
    };

    let mut rng = rand::rng();

    let mut registry = fastmetrics::registry::Registry::builder()
        .with_name_rule(name_rule)
        .build()
        .unwrap();

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
