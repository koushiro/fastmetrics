use std::{borrow::Cow, time::Duration};

use super::prometheus_data_model;
use crate::{
    encoder::{
        self, EncodeCounterValue, EncodeExemplar, EncodeGaugeValue, EncodeLabel, EncodeLabelSet,
        EncodeMetric, EncodeUnknownValue, MetricFamilyEncoder as _,
    },
    error::{Error, Result},
    raw::{Metadata, MetricType, bucket::Bucket, quantile::Quantile},
    registry::Registry,
};

pub(super) fn encode(buffer: &mut impl prost::bytes::BufMut, registry: &Registry) -> Result<()> {
    let mut metric_families = vec![];
    Encoder::new(&mut metric_families, registry).encode()?;

    for metric_family in metric_families {
        prost::Message::encode_length_delimited(&metric_family, buffer)
            .map_err(|err| Error::unexpected(err.to_string()).set_source(err))?;
    }

    Ok(())
}

struct Encoder<'a> {
    metric_families: &'a mut Vec<prometheus_data_model::MetricFamily>,
    registry: &'a Registry,
}

impl<'a> Encoder<'a> {
    fn new(
        metric_families: &'a mut Vec<prometheus_data_model::MetricFamily>,
        registry: &'a Registry,
    ) -> Self {
        Self { metric_families, registry }
    }

    fn encode(&mut self) -> Result<()> {
        self.encode_registry(self.registry)
    }

    fn encode_registry(&mut self, registry: &Registry) -> Result<()> {
        for (metadata, metric) in &registry.metrics {
            MetricFamilyEncoder {
                metric_families: self.metric_families,
                namespace: registry.namespace(),
                const_labels: registry.constant_labels(),
            }
            .encode(metadata, metric)?;
        }
        for subsystem in registry.subsystems.values() {
            self.encode_registry(subsystem)?;
        }
        Ok(())
    }
}

struct MetricFamilyEncoder<'a> {
    metric_families: &'a mut Vec<prometheus_data_model::MetricFamily>,
    namespace: Option<&'a str>,
    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
}

impl encoder::MetricFamilyEncoder for MetricFamilyEncoder<'_> {
    fn encode(&mut self, metadata: &Metadata, metric: &dyn EncodeMetric) -> Result<()> {
        if metric.is_empty() {
            // skip empty metric family
            return Ok(());
        }

        let name = match self.namespace {
            Some(namespace) => format!("{}_{}", namespace, metadata.name()),
            None => metadata.name().to_owned(),
        };
        let family_name =
            if metadata.metric_type() == MetricType::Info { format!("{name}_info") } else { name };

        let mut metric_family = prometheus_data_model::MetricFamily {
            name: Some(family_name.clone()),
            help: Some(metadata.help().to_owned()),
            r#type: Some(prometheus_data_model::MetricType::from(metadata.metric_type()) as i32),
            unit: metadata.unit().map(|unit| unit.as_str().to_owned()),
            metric: vec![],
        };

        let mut labels = vec![];
        self.const_labels.encode(&mut LabelSetEncoder { labels: &mut labels })?;

        metric.encode(&mut MetricEncoder {
            metrics: &mut metric_family.metric,
            labels,
            timestamp_ms: metric.timestamp().map(into_prometheus_timestamp_millis),
            state_label_name: family_name,
        })?;

        self.metric_families.push(metric_family);

        Ok(())
    }
}

impl From<MetricType> for prometheus_data_model::MetricType {
    fn from(metric_type: MetricType) -> Self {
        match metric_type {
            MetricType::Unknown => prometheus_data_model::MetricType::Untyped,
            MetricType::Gauge => prometheus_data_model::MetricType::Gauge,
            MetricType::Counter => prometheus_data_model::MetricType::Counter,
            MetricType::StateSet => prometheus_data_model::MetricType::Gauge,
            MetricType::Info => prometheus_data_model::MetricType::Gauge,
            MetricType::Histogram => prometheus_data_model::MetricType::Histogram,
            MetricType::GaugeHistogram => prometheus_data_model::MetricType::GaugeHistogram,
            MetricType::Summary => prometheus_data_model::MetricType::Summary,
        }
    }
}

#[inline]
fn into_prometheus_timestamp_millis(duration: Duration) -> i64 {
    let millis = duration.as_millis();
    if millis > i64::MAX as u128 { i64::MAX } else { millis as i64 }
}

#[inline]
fn into_prost_timestamp(duration: Duration) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
    }
}

struct MetricEncoder<'a> {
    metrics: &'a mut Vec<prometheus_data_model::Metric>,
    labels: Vec<prometheus_data_model::LabelPair>,
    timestamp_ms: Option<i64>,
    state_label_name: String,
}

impl encoder::MetricEncoder for MetricEncoder<'_> {
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> Result<()> {
        let mut v = UnknownValueEncoder::default();
        value.encode(&mut v)?;

        self.metrics.push(prometheus_data_model::Metric {
            label: self.labels.clone(),
            untyped: Some(prometheus_data_model::Untyped { value: Some(v.value) }),
            timestamp_ms: self.timestamp_ms,
            ..Default::default()
        });
        Ok(())
    }

    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> Result<()> {
        let mut v = GaugeValueEncoder::default();
        value.encode(&mut v)?;

        self.metrics.push(prometheus_data_model::Metric {
            label: self.labels.clone(),
            gauge: Some(prometheus_data_model::Gauge { value: Some(v.value) }),
            timestamp_ms: self.timestamp_ms,
            ..Default::default()
        });
        Ok(())
    }

    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        exemplar: Option<&dyn EncodeExemplar>,
        created: Option<Duration>,
    ) -> Result<()> {
        let mut v = CounterValueEncoder::default();
        total.encode(&mut v)?;

        let exemplar = if let Some(exemplar) = exemplar {
            let mut e = prometheus_data_model::Exemplar::default();
            exemplar.encode(&mut ExemplarEncoder { exemplar: &mut e })?;
            Some(e)
        } else {
            None
        };

        self.metrics.push(prometheus_data_model::Metric {
            label: self.labels.clone(),
            counter: Some(prometheus_data_model::Counter {
                value: Some(v.value),
                exemplar,
                created_timestamp: created.map(into_prost_timestamp),
            }),
            timestamp_ms: self.timestamp_ms,
            ..Default::default()
        });
        Ok(())
    }

    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> Result<()> {
        for (state, enabled) in states {
            let mut labels = self.labels.clone();
            labels.push(prometheus_data_model::LabelPair {
                name: Some(self.state_label_name.clone()),
                value: Some(state.to_owned()),
            });

            self.metrics.push(prometheus_data_model::Metric {
                label: labels,
                gauge: Some(prometheus_data_model::Gauge {
                    value: Some(if enabled { 1.0 } else { 0.0 }),
                }),
                timestamp_ms: self.timestamp_ms,
                ..Default::default()
            });
        }
        Ok(())
    }

    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> Result<()> {
        let mut labels = self.labels.clone();
        label_set.encode(&mut LabelSetEncoder { labels: &mut labels })?;

        self.metrics.push(prometheus_data_model::Metric {
            label: labels,
            gauge: Some(prometheus_data_model::Gauge { value: Some(1.0) }),
            timestamp_ms: self.timestamp_ms,
            ..Default::default()
        });
        Ok(())
    }

    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
        count: u64,
        sum: f64,
        created: Option<Duration>,
    ) -> Result<()> {
        let exemplars = exemplars.inspect(|exemplars| {
            assert_eq!(buckets.len(), exemplars.len(), "buckets and exemplars count mismatch");
        });

        let mut cumulative_count = 0_u64;
        let mut histogram_buckets = Vec::with_capacity(buckets.len());

        for (idx, bucket) in buckets.iter().enumerate() {
            cumulative_count = cumulative_count.saturating_add(bucket.count());

            let exemplar = if let Some(exemplars) = exemplars {
                if let Some(exemplar) = exemplars[idx] {
                    let mut e = prometheus_data_model::Exemplar::default();
                    exemplar.encode(&mut ExemplarEncoder { exemplar: &mut e })?;
                    Some(e)
                } else {
                    None
                }
            } else {
                None
            };

            histogram_buckets.push(prometheus_data_model::Bucket {
                cumulative_count: Some(cumulative_count),
                upper_bound: Some(bucket.upper_bound()),
                exemplar,
                ..Default::default()
            });
        }

        self.metrics.push(prometheus_data_model::Metric {
            label: self.labels.clone(),
            histogram: Some(prometheus_data_model::Histogram {
                sample_count: Some(count),
                sample_sum: Some(sum),
                bucket: histogram_buckets,
                created_timestamp: created.map(into_prost_timestamp),
                ..Default::default()
            }),
            timestamp_ms: self.timestamp_ms,
            ..Default::default()
        });

        Ok(())
    }

    fn encode_gauge_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
        count: u64,
        sum: f64,
    ) -> Result<()> {
        self.encode_histogram(buckets, exemplars, count, sum, None)
    }

    fn encode_summary(
        &mut self,
        quantiles: &[Quantile],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> Result<()> {
        let quantile = quantiles
            .iter()
            .map(|q| prometheus_data_model::Quantile {
                quantile: Some(q.quantile()),
                value: Some(q.value()),
            })
            .collect::<Vec<_>>();

        self.metrics.push(prometheus_data_model::Metric {
            label: self.labels.clone(),
            summary: Some(prometheus_data_model::Summary {
                sample_count: Some(count),
                sample_sum: Some(sum),
                quantile,
                created_timestamp: created.map(into_prost_timestamp),
            }),
            timestamp_ms: self.timestamp_ms,
            ..Default::default()
        });
        Ok(())
    }

    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> Result<()> {
        let mut labels = self.labels.clone();
        label_set.encode(&mut LabelSetEncoder { labels: &mut labels })?;

        metric.encode(&mut MetricEncoder {
            metrics: self.metrics,
            labels,
            timestamp_ms: metric.timestamp().map(into_prometheus_timestamp_millis),
            state_label_name: self.state_label_name.clone(),
        })
    }
}

struct LabelSetEncoder<'a> {
    labels: &'a mut Vec<prometheus_data_model::LabelPair>,
}

impl encoder::LabelSetEncoder for LabelSetEncoder<'_> {
    fn encode(&mut self, label: &dyn EncodeLabel) -> Result<()> {
        self.labels.push(prometheus_data_model::LabelPair::default());
        label.encode(&mut LabelEncoder {
            label: self.labels.last_mut().expect("labels must not be none"),
        })
    }
}

struct LabelEncoder<'a> {
    label: &'a mut prometheus_data_model::LabelPair,
}

macro_rules! encode_integer_value_impls {
    ($($integer:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $integer _value>](&mut self, value: $integer) -> Result<()> {
                self.label
                    .value
                    .get_or_insert_with(String::new)
                    .push_str(itoa::Buffer::new().format(value));
                Ok(())
            }
        )* }
    )
}

macro_rules! encode_float_value_impls {
    ($($float:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $float _value>](&mut self, value: $float) -> Result<()> {
                self.label
                    .value
                    .get_or_insert_with(String::new)
                    .push_str(zmij::Buffer::new().format(value));
                Ok(())
            }
        )* }
    )
}

impl encoder::LabelEncoder for LabelEncoder<'_> {
    fn encode_label_name(&mut self, name: &str) -> Result<()> {
        self.label.name.get_or_insert_with(String::new).push_str(name);
        Ok(())
    }

    fn encode_str_value(&mut self, value: &str) -> Result<()> {
        self.label.value.get_or_insert_with(String::new).push_str(value);
        Ok(())
    }

    fn encode_bool_value(&mut self, value: bool) -> Result<()> {
        self.label.value.get_or_insert_with(String::new).push_str(if value {
            "true"
        } else {
            "false"
        });
        Ok(())
    }

    encode_integer_value_impls! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize
    }

    encode_float_value_impls! { f32, f64 }
}

#[derive(Default)]
struct UnknownValueEncoder {
    value: f64,
}

impl encoder::UnknownValueEncoder for UnknownValueEncoder {
    fn encode_i32(&mut self, value: i32) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_i64(&mut self, value: i64) -> Result<()> {
        self.value = value as f64;
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_u32(&mut self, value: u32) -> Result<()> {
        self.value = value as f64;
        Ok(())
    }

    fn encode_f32(&mut self, value: f32) -> Result<()> {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> Result<()> {
        self.value = value;
        Ok(())
    }
}

#[derive(Default)]
struct GaugeValueEncoder {
    value: f64,
}

impl encoder::GaugeValueEncoder for GaugeValueEncoder {
    fn encode_i32(&mut self, value: i32) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_i64(&mut self, value: i64) -> Result<()> {
        self.value = value as f64;
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_f32(&mut self, value: f32) -> Result<()> {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> Result<()> {
        self.value = value;
        Ok(())
    }
}

#[derive(Default)]
struct CounterValueEncoder {
    value: f64,
}

impl encoder::CounterValueEncoder for CounterValueEncoder {
    fn encode_u32(&mut self, value: u32) -> Result<()> {
        self.encode_u64(value as u64)
    }

    fn encode_u64(&mut self, value: u64) -> Result<()> {
        self.value = value as f64;
        Ok(())
    }

    fn encode_usize(&mut self, value: usize) -> Result<()> {
        self.encode_u64(value as u64)
    }

    fn encode_f32(&mut self, value: f32) -> Result<()> {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> Result<()> {
        self.value = value;
        Ok(())
    }
}

struct ExemplarEncoder<'a> {
    exemplar: &'a mut prometheus_data_model::Exemplar,
}

impl encoder::ExemplarEncoder for ExemplarEncoder<'_> {
    fn encode(
        &mut self,
        label_set: &dyn EncodeLabelSet,
        value: f64,
        timestamp: Option<Duration>,
    ) -> Result<()> {
        label_set.encode(&mut LabelSetEncoder { labels: &mut self.exemplar.label })?;
        self.exemplar.value = Some(value);
        self.exemplar.timestamp = timestamp.map(into_prost_timestamp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        metrics::{counter::Counter, info::Info},
        registry::Registry,
    };

    #[test]
    fn encode_prometheus_counter_profile() {
        let mut registry = Registry::default();
        let counter = Counter::<u64>::default();
        registry
            .register("http_requests_total", "Total requests", counter.clone())
            .unwrap();
        counter.inc();

        let mut output = Vec::new();
        super::encode(&mut output, &registry).unwrap();

        let mut input = output.as_slice();
        let family =
            <prometheus_data_model::MetricFamily as prost::Message>::decode_length_delimited(
                &mut input,
            )
            .expect("must decode a single length-delimited MetricFamily");
        assert!(input.is_empty(), "unexpected trailing bytes");

        assert_eq!(family.name.as_deref(), Some("http_requests_total"));
        assert_eq!(family.r#type, Some(prometheus_data_model::MetricType::Counter as i32));

        let metric = family.metric.first().expect("missing metric sample");
        let counter = metric.counter.as_ref().expect("counter payload is required");
        assert_eq!(counter.value, Some(1.0));
    }

    #[test]
    fn encode_prometheus_info_profile() {
        let mut registry = Registry::default();
        let info = Info::new(vec![("version", "1.0.0")]);
        registry.register("build", "Build info", info).expect("register info metric");

        let mut output = Vec::new();
        super::encode(&mut output, &registry).unwrap();

        let mut input = output.as_slice();
        let family =
            <prometheus_data_model::MetricFamily as prost::Message>::decode_length_delimited(
                &mut input,
            )
            .expect("must decode a single length-delimited MetricFamily");
        assert!(input.is_empty(), "unexpected trailing bytes");

        assert_eq!(family.name.as_deref(), Some("build_info"));
        assert_eq!(family.r#type, Some(prometheus_data_model::MetricType::Gauge as i32));

        let metric = family.metric.first().expect("missing metric sample");
        let gauge = metric.gauge.as_ref().expect("gauge payload is required");
        assert_eq!(gauge.value, Some(1.0));
        assert!(metric.label.iter().any(|label| label.name.as_deref() == Some("version")
            && label.value.as_deref() == Some("1.0.0")));
    }
}
