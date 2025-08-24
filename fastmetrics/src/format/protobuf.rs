//! Protobuf exposition format using [protobuf](https://github.com/stepancheg/rust-protobuf) crate.

use std::{borrow::Cow, fmt, io, time::Duration};

use crate::{
    encoder::{
        self, EncodeCounterValue, EncodeExemplar, EncodeGaugeValue, EncodeLabel, EncodeLabelSet,
        EncodeMetric, EncodeUnknownValue, MetricFamilyEncoder as _,
    },
    raw::{bucket::Bucket, quantile::Quantile, Metadata, MetricType},
    registry::Registry,
};

/// Data models that are automatically generated from [OpenMetrics protobuf schema].
///
/// [OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto
#[allow(missing_docs)]
#[allow(clippy::all)]
mod openmetrics_data_model {
    include!(concat!(env!("OUT_DIR"), "/protobuf/mod.rs"));

    pub use self::openmetrics_data_model::*;
}

/// Encodes metrics from a registry into the [OpenMetrics protobuf format](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#protobuf-format).
///
/// # Arguments
///
/// * `buffer` - A mutable reference to any type implementing [`io::Write`] trait where the encoded
///   protobuf data will be written.
/// * `registry` - A reference to the [`Registry`] containing the metrics to encode.
///
/// # Returns
///
/// Returns `Ok(())` if encoding was successful, or a [`io::Error`] if there was an error during
/// protobuf encoding.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::{
/// #     format::protobuf,
/// #     metrics::counter::Counter,
/// #     registry::Registry,
/// # };
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut registry = Registry::default();
///
/// // Register a counter
/// let requests = <Counter>::default();
/// registry.register(
///     "http_requests_total",
///     "Total number of HTTP requests",
///     requests.clone()
/// )?;
/// // Update a counter
/// requests.inc();
///
/// // Encode metrics in protobuf format
/// let mut output = Vec::new();
/// protobuf::encode(&mut output, &registry)?;
/// assert!(!output.is_empty());
/// # Ok(())
/// # }
/// ```
pub fn encode(buffer: &mut dyn io::Write, registry: &Registry) -> io::Result<()> {
    let mut metric_set = openmetrics_data_model::MetricSet::default();
    let mut encoder = Encoder::new(&mut metric_set, registry);
    encoder.encode().expect("fmt::Error should not be encountered");
    protobuf::Message::write_to_writer(&metric_set, buffer)?;
    Ok(())
}

struct Encoder<'a> {
    metric_set: &'a mut openmetrics_data_model::MetricSet,
    registry: &'a Registry,
}

impl<'a> Encoder<'a> {
    fn new(
        metric_set: &'a mut openmetrics_data_model::MetricSet,
        registry: &'a Registry,
    ) -> Encoder<'a> {
        Self { metric_set, registry }
    }

    fn encode(&mut self) -> fmt::Result {
        self.encode_registry(self.registry)
    }

    fn encode_registry(&mut self, registry: &Registry) -> fmt::Result {
        for (metadata, metric) in &registry.metrics {
            let metric_families = &mut self.metric_set.metric_families;
            MetricFamilyEncoder {
                metric_families,
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
    metric_families: &'a mut Vec<openmetrics_data_model::MetricFamily>,
    namespace: Option<&'a str>,
    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
}

impl From<MetricType> for openmetrics_data_model::MetricType {
    fn from(metric_type: MetricType) -> Self {
        match metric_type {
            MetricType::Unknown => openmetrics_data_model::MetricType::UNKNOWN,
            MetricType::Gauge => openmetrics_data_model::MetricType::GAUGE,
            MetricType::Counter => openmetrics_data_model::MetricType::COUNTER,
            MetricType::StateSet => openmetrics_data_model::MetricType::STATE_SET,
            MetricType::Info => openmetrics_data_model::MetricType::INFO,
            MetricType::Histogram => openmetrics_data_model::MetricType::HISTOGRAM,
            MetricType::GaugeHistogram => openmetrics_data_model::MetricType::GAUGE_HISTOGRAM,
            MetricType::Summary => openmetrics_data_model::MetricType::SUMMARY,
        }
    }
}

impl encoder::MetricFamilyEncoder for MetricFamilyEncoder<'_> {
    fn encode(&mut self, metadata: &Metadata, metric: &dyn EncodeMetric) -> fmt::Result {
        let mut metric_family = openmetrics_data_model::MetricFamily {
            name: {
                match self.namespace {
                    Some(namespace) => format!("{}_{}", namespace, metadata.name()),
                    None => metadata.name().to_owned(),
                }
            },
            type_: openmetrics_data_model::MetricType::from(metadata.metric_type()).into(),
            unit: if let Some(unit) = metadata.unit() {
                unit.as_str().to_owned()
            } else {
                String::new()
            },
            help: metadata.help().to_owned(),
            metrics: vec![],
            special_fields: protobuf::SpecialFields::new(),
        };

        let mut labels = vec![];
        self.const_labels.encode(&mut LabelSetEncoder { labels: &mut labels })?;

        metric.encode(&mut MetricEncoder {
            metrics: &mut metric_family.metrics,
            labels,
            timestamp: metric.timestamp(),
        })?;

        self.metric_families.push(metric_family);

        Ok(())
    }
}

struct MetricEncoder<'a> {
    metrics: &'a mut Vec<openmetrics_data_model::Metric>,
    labels: Vec<openmetrics_data_model::Label>,
    timestamp: Option<Duration>,
}

fn into_protobuf_timestamp(duration: Duration) -> protobuf::well_known_types::timestamp::Timestamp {
    protobuf::well_known_types::timestamp::Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
        special_fields: protobuf::SpecialFields::new(),
    }
}

impl encoder::MetricEncoder for MetricEncoder<'_> {
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> fmt::Result {
        let mut v = openmetrics_data_model::unknown_value::Value::IntValue(0);
        value.encode(&mut UnknownValueEncoder { value: &mut v })?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::UnknownValue(
                    openmetrics_data_model::UnknownValue {
                        value: Some(v),
                        special_fields: protobuf::SpecialFields::new(),
                    },
                )),
                timestamp: self.timestamp.map(into_protobuf_timestamp).into(),
                special_fields: protobuf::SpecialFields::new(),
            }],
            special_fields: protobuf::SpecialFields::new(),
        });

        Ok(())
    }

    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> fmt::Result {
        let mut v = openmetrics_data_model::gauge_value::Value::IntValue(0);
        value.encode(&mut GaugeValueEncoder { value: &mut v })?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::GaugeValue(
                    openmetrics_data_model::GaugeValue {
                        value: Some(v),
                        special_fields: protobuf::SpecialFields::new(),
                    },
                )),
                timestamp: self.timestamp.map(into_protobuf_timestamp).into(),
                special_fields: protobuf::SpecialFields::new(),
            }],
            special_fields: protobuf::SpecialFields::new(),
        });

        Ok(())
    }

    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        exemplar: Option<&dyn EncodeExemplar>,
        created: Option<Duration>,
    ) -> fmt::Result {
        let mut t = openmetrics_data_model::counter_value::Total::IntValue(0);
        total.encode(&mut CounterValueEncoder { total: &mut t })?;

        let exemplar = if let Some(exemplar) = exemplar {
            let mut e = openmetrics_data_model::Exemplar::default();
            exemplar.encode(&mut ExemplarEncoder { exemplar: &mut e })?;
            Some(e)
        } else {
            None
        };

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::CounterValue(
                    openmetrics_data_model::CounterValue {
                        total: Some(t),
                        created: created.map(into_protobuf_timestamp).into(),
                        exemplar: exemplar.into(),
                        special_fields: protobuf::SpecialFields::new(),
                    },
                )),
                timestamp: self.timestamp.map(into_protobuf_timestamp).into(),
                special_fields: protobuf::SpecialFields::new(),
            }],
            special_fields: protobuf::SpecialFields::new(),
        });

        Ok(())
    }

    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> fmt::Result {
        let states = states
            .into_iter()
            .map(|(state, enabled)| openmetrics_data_model::state_set_value::State {
                name: state.to_owned(),
                enabled,
                special_fields: protobuf::SpecialFields::new(),
            })
            .collect::<Vec<_>>();

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::StateSetValue(
                    openmetrics_data_model::StateSetValue {
                        states,
                        special_fields: protobuf::SpecialFields::new(),
                    },
                )),
                timestamp: self.timestamp.map(into_protobuf_timestamp).into(),
                special_fields: protobuf::SpecialFields::new(),
            }],
            special_fields: protobuf::SpecialFields::new(),
        });

        Ok(())
    }

    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> fmt::Result {
        let mut info_labels = vec![];
        label_set.encode(&mut LabelSetEncoder { labels: &mut info_labels })?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::InfoValue(
                    openmetrics_data_model::InfoValue {
                        info: info_labels,
                        special_fields: protobuf::SpecialFields::new(),
                    },
                )),
                timestamp: self.timestamp.map(into_protobuf_timestamp).into(),
                special_fields: protobuf::SpecialFields::new(),
            }],
            special_fields: protobuf::SpecialFields::new(),
        });

        Ok(())
    }

    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: &[Option<&dyn EncodeExemplar>],
        count: u64,
        sum: f64,
        created: Option<Duration>,
    ) -> fmt::Result {
        assert_eq!(buckets.len(), exemplars.len(), "buckets and exemplars count mismatch");

        let buckets = buckets
            .iter()
            .zip(exemplars.iter())
            .map(|(b, e)| {
                Ok(openmetrics_data_model::histogram_value::Bucket {
                    count: b.count(),
                    upper_bound: b.upper_bound(),
                    exemplar: if let Some(exemplar) = e {
                        let mut e = openmetrics_data_model::Exemplar::default();
                        exemplar.encode(&mut ExemplarEncoder { exemplar: &mut e })?;
                        Some(e)
                    } else {
                        None
                    }
                    .into(),
                    special_fields: protobuf::SpecialFields::new(),
                })
            })
            .collect::<Result<Vec<_>, fmt::Error>>()?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::HistogramValue(
                    openmetrics_data_model::HistogramValue {
                        buckets,
                        count,
                        sum: Some(openmetrics_data_model::histogram_value::Sum::DoubleValue(sum)),
                        created: created.map(into_protobuf_timestamp).into(),
                        special_fields: protobuf::SpecialFields::new(),
                    },
                )),
                timestamp: self.timestamp.map(into_protobuf_timestamp).into(),
                special_fields: protobuf::SpecialFields::new(),
            }],
            special_fields: protobuf::SpecialFields::new(),
        });

        Ok(())
    }

    fn encode_gauge_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: &[Option<&dyn EncodeExemplar>],
        count: u64,
        sum: f64,
    ) -> fmt::Result {
        self.encode_histogram(buckets, exemplars, count, sum, None)
    }

    fn encode_summary(
        &mut self,
        quantiles: &[Quantile],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> fmt::Result {
        let quantile = quantiles
            .iter()
            .map(|q| openmetrics_data_model::summary_value::Quantile {
                quantile: q.quantile(),
                value: q.value(),
                special_fields: protobuf::SpecialFields::new(),
            })
            .collect::<Vec<_>>();

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::SummaryValue(
                    openmetrics_data_model::SummaryValue {
                        quantile,
                        count,
                        sum: Some(openmetrics_data_model::summary_value::Sum::DoubleValue(sum)),
                        created: created.map(into_protobuf_timestamp).into(),
                        special_fields: protobuf::SpecialFields::new(),
                    },
                )),
                timestamp: self.timestamp.map(into_protobuf_timestamp).into(),
                special_fields: protobuf::SpecialFields::new(),
            }],
            special_fields: protobuf::SpecialFields::new(),
        });

        Ok(())
    }

    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> fmt::Result {
        let mut labels = self.labels.clone();
        label_set.encode(&mut LabelSetEncoder { labels: &mut labels })?;
        metric.encode(&mut MetricEncoder {
            metrics: self.metrics,
            labels,
            timestamp: metric.timestamp(),
        })
    }
}

struct LabelSetEncoder<'a> {
    labels: &'a mut Vec<openmetrics_data_model::Label>,
}

impl encoder::LabelSetEncoder for LabelSetEncoder<'_> {
    fn encode(&mut self, label: &dyn EncodeLabel) -> fmt::Result {
        self.labels.push(openmetrics_data_model::Label::default());
        label.encode(&mut LabelEncoder {
            label: self.labels.last_mut().expect("labels must not be none"),
        })
    }
}

struct LabelEncoder<'a> {
    label: &'a mut openmetrics_data_model::Label,
}

macro_rules! encode_integer_value_impls {
    ($($integer:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $integer _value>](&mut self, value: $integer) -> fmt::Result {
                self.label.value.push_str(itoa::Buffer::new().format(value));
                Ok(())
            }
        )* }
    )
}

macro_rules! encode_float_value_impls {
    ($($float:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $float _value>](&mut self, value: $float) -> fmt::Result {
                self.label.value.push_str(dtoa::Buffer::new().format(value));
                Ok(())
            }
        )* }
    )
}

impl encoder::LabelEncoder for LabelEncoder<'_> {
    fn encode_label_name(&mut self, name: &str) -> fmt::Result {
        self.label.name.push_str(name);
        Ok(())
    }

    fn encode_str_value(&mut self, value: &str) -> fmt::Result {
        self.label.value.push_str(value);
        Ok(())
    }

    fn encode_bool_value(&mut self, value: bool) -> fmt::Result {
        self.label.value.push_str(if value { "true" } else { "false" });
        Ok(())
    }

    encode_integer_value_impls! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize
    }

    encode_float_value_impls! { f32, f64 }
}

struct UnknownValueEncoder<'a> {
    value: &'a mut openmetrics_data_model::unknown_value::Value,
}

impl encoder::UnknownValueEncoder for UnknownValueEncoder<'_> {
    fn encode_i32(&mut self, value: i32) -> fmt::Result {
        self.encode_i64(value as i64)
    }

    fn encode_i64(&mut self, value: i64) -> fmt::Result {
        *self.value = openmetrics_data_model::unknown_value::Value::IntValue(value);
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> fmt::Result {
        self.encode_i64(value as i64)
    }

    fn encode_u32(&mut self, value: u32) -> fmt::Result {
        self.encode_i64(value as i64)
    }

    fn encode_f32(&mut self, value: f32) -> fmt::Result {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> fmt::Result {
        *self.value = openmetrics_data_model::unknown_value::Value::DoubleValue(value);
        Ok(())
    }
}

struct GaugeValueEncoder<'a> {
    value: &'a mut openmetrics_data_model::gauge_value::Value,
}

impl encoder::GaugeValueEncoder for GaugeValueEncoder<'_> {
    fn encode_i32(&mut self, value: i32) -> fmt::Result {
        self.encode_i64(value as i64)
    }

    fn encode_i64(&mut self, value: i64) -> fmt::Result {
        *self.value = openmetrics_data_model::gauge_value::Value::IntValue(value);
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> fmt::Result {
        self.encode_i64(value as i64)
    }

    fn encode_u32(&mut self, value: u32) -> fmt::Result {
        self.encode_i64(value as i64)
    }

    fn encode_u64(&mut self, value: u64) -> fmt::Result {
        if value <= i64::MAX as u64 {
            *self.value = openmetrics_data_model::gauge_value::Value::IntValue(value as i64);
            Ok(())
        }
        // value > i64::MAX
        else {
            // For gauge metrics that support the u64 type, the openmetrics protobuf format does not
            // support encoding values exceeding i64::MAX.
            // panic!("Can't encode gauge value in protobuf format: value {value} > i64::MAX");

            // For large u64 values that exceed i64::MAX, encode as a double to avoid errors
            // Note: This may result in precision loss when converting to f64.
            *self.value = openmetrics_data_model::gauge_value::Value::DoubleValue(value as f64);
            Ok(())
        }
    }

    fn encode_f32(&mut self, value: f32) -> fmt::Result {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> fmt::Result {
        *self.value = openmetrics_data_model::gauge_value::Value::DoubleValue(value);
        Ok(())
    }
}

struct CounterValueEncoder<'a> {
    total: &'a mut openmetrics_data_model::counter_value::Total,
}

impl encoder::CounterValueEncoder for CounterValueEncoder<'_> {
    fn encode_u32(&mut self, value: u32) -> fmt::Result {
        self.encode_u64(value as u64)
    }

    fn encode_u64(&mut self, value: u64) -> fmt::Result {
        *self.total = openmetrics_data_model::counter_value::Total::IntValue(value);
        Ok(())
    }

    fn encode_usize(&mut self, value: usize) -> fmt::Result {
        self.encode_u64(value as u64)
    }

    fn encode_f32(&mut self, value: f32) -> fmt::Result {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> fmt::Result {
        *self.total = openmetrics_data_model::counter_value::Total::DoubleValue(value);
        Ok(())
    }
}

struct ExemplarEncoder<'a> {
    exemplar: &'a mut openmetrics_data_model::Exemplar,
}

impl encoder::ExemplarEncoder for ExemplarEncoder<'_> {
    fn encode(
        &mut self,
        label_set: &dyn EncodeLabelSet,
        value: f64,
        timestamp: Option<Duration>,
    ) -> fmt::Result {
        label_set.encode(&mut LabelSetEncoder { labels: &mut self.exemplar.label })?;
        self.exemplar.value = value;
        self.exemplar.timestamp = timestamp.map(into_protobuf_timestamp).into();
        Ok(())
    }
}
