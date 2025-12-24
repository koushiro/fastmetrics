//! Protobuf exposition format using [prost](https://github.com/tokio-rs/prost) crate.

use std::{borrow::Cow, time::Duration};

use crate::{
    encoder::{
        self, EncodeCounterValue, EncodeExemplar, EncodeGaugeValue, EncodeLabel, EncodeLabelSet,
        EncodeMetric, EncodeUnknownValue, MetricFamilyEncoder as _,
    },
    error::{Error, Result},
    raw::{Metadata, MetricType, bucket::Bucket, quantile::Quantile},
    registry::Registry,
};

/// Data models that are automatically generated from [OpenMetrics protobuf schema].
///
/// [OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto
#[allow(missing_docs)]
#[allow(clippy::all)]
mod openmetrics_data_model {
    include!(concat!(env!("OUT_DIR"), "/prost/openmetrics.rs"));
}

/// Encodes metrics from a registry into the [OpenMetrics protobuf format](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#protobuf-format).
///
/// # Arguments
///
/// * `buffer` - A mutable reference to any type implementing `BufMut` trait where the encoded
///   protobuf data will be written.
/// * `registry` - A reference to the [`Registry`] containing the metrics to encode.
///
/// # Returns
///
/// Returns `Ok(())` if encoding was successful, or a [`Error`] if there was an error
/// during protobuf encoding.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::{
/// #     error::Result,
/// #     format::prost,
/// #     metrics::counter::Counter,
/// #     registry::Registry,
/// # };
/// #
/// # fn main() -> Result<()> {
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
/// prost::encode(&mut output, &registry)?;
/// assert!(!output.is_empty());
/// # Ok(())
/// # }
/// ```
pub fn encode(buffer: &mut impl prost::bytes::BufMut, registry: &Registry) -> Result<()> {
    let mut metric_set = openmetrics_data_model::MetricSet::default();
    Encoder::new(&mut metric_set, registry).encode()?;
    prost::Message::encode(&metric_set, buffer)
        .map_err(|err| Error::unexpected(err.to_string()).set_source(err))?;
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

    fn encode(&mut self) -> Result<()> {
        self.encode_registry(self.registry)
    }

    fn encode_registry(&mut self, registry: &Registry) -> Result<()> {
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
            MetricType::Unknown => openmetrics_data_model::MetricType::Unknown,
            MetricType::Gauge => openmetrics_data_model::MetricType::Gauge,
            MetricType::Counter => openmetrics_data_model::MetricType::Counter,
            MetricType::StateSet => openmetrics_data_model::MetricType::StateSet,
            MetricType::Info => openmetrics_data_model::MetricType::Info,
            MetricType::Histogram => openmetrics_data_model::MetricType::Histogram,
            MetricType::GaugeHistogram => openmetrics_data_model::MetricType::GaugeHistogram,
            MetricType::Summary => openmetrics_data_model::MetricType::Summary,
        }
    }
}

impl encoder::MetricFamilyEncoder for MetricFamilyEncoder<'_> {
    fn encode(&mut self, metadata: &Metadata, metric: &dyn EncodeMetric) -> Result<()> {
        if metric.is_empty() {
            // skip empty metric family
            return Ok(());
        }

        let mut metric_family = openmetrics_data_model::MetricFamily {
            name: {
                match self.namespace {
                    Some(namespace) => format!("{}_{}", namespace, metadata.name()),
                    None => metadata.name().to_owned(),
                }
            },
            r#type: {
                let metric_type = openmetrics_data_model::MetricType::from(metadata.metric_type());
                metric_type as i32
            },
            unit: if let Some(unit) = metadata.unit() {
                unit.as_str().to_owned()
            } else {
                String::new()
            },
            help: metadata.help().to_owned(),
            metrics: vec![],
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

fn into_prost_timestamp(duration: Duration) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
    }
}

impl encoder::MetricEncoder for MetricEncoder<'_> {
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> Result<()> {
        let mut v = openmetrics_data_model::unknown_value::Value::IntValue(0);
        value.encode(&mut UnknownValueEncoder { value: &mut v })?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::UnknownValue(
                    openmetrics_data_model::UnknownValue { value: Some(v) },
                )),
                timestamp: self.timestamp.map(into_prost_timestamp),
            }],
        });

        Ok(())
    }

    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> Result<()> {
        let mut v = openmetrics_data_model::gauge_value::Value::IntValue(0);
        value.encode(&mut GaugeValueEncoder { value: &mut v })?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::GaugeValue(
                    openmetrics_data_model::GaugeValue { value: Some(v) },
                )),
                timestamp: self.timestamp.map(into_prost_timestamp),
            }],
        });

        Ok(())
    }

    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        exemplar: Option<&dyn EncodeExemplar>,
        created: Option<Duration>,
    ) -> Result<()> {
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
                        created: created.map(into_prost_timestamp),
                        exemplar,
                    },
                )),
                timestamp: self.timestamp.map(into_prost_timestamp),
            }],
        });

        Ok(())
    }

    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> Result<()> {
        let states = states
            .into_iter()
            .map(|(state, enabled)| openmetrics_data_model::state_set_value::State {
                name: state.to_owned(),
                enabled,
            })
            .collect::<Vec<_>>();

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::StateSetValue(
                    openmetrics_data_model::StateSetValue { states },
                )),
                timestamp: self.timestamp.map(into_prost_timestamp),
            }],
        });

        Ok(())
    }

    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> Result<()> {
        let mut info_labels = vec![];
        label_set.encode(&mut LabelSetEncoder { labels: &mut info_labels })?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::InfoValue(
                    openmetrics_data_model::InfoValue { info: info_labels },
                )),
                timestamp: self.timestamp.map(into_prost_timestamp),
            }],
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

        let buckets = buckets
            .iter()
            .enumerate()
            .map(|(idx, b)| {
                Ok(openmetrics_data_model::histogram_value::Bucket {
                    count: b.count(),
                    upper_bound: b.upper_bound(),
                    exemplar: if let Some(exemplars) = exemplars {
                        if let Some(exemplar) = exemplars[idx] {
                            let mut e = openmetrics_data_model::Exemplar::default();
                            exemplar.encode(&mut ExemplarEncoder { exemplar: &mut e })?;
                            Some(e)
                        } else {
                            None
                        }
                    } else {
                        None
                    },
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        self.metrics.push(openmetrics_data_model::Metric {
            labels: self.labels.clone(),
            metric_points: vec![openmetrics_data_model::MetricPoint {
                value: Some(openmetrics_data_model::metric_point::Value::HistogramValue(
                    openmetrics_data_model::HistogramValue {
                        buckets,
                        count,
                        sum: Some(openmetrics_data_model::histogram_value::Sum::DoubleValue(sum)),
                        created: created.map(into_prost_timestamp),
                    },
                )),
                timestamp: self.timestamp.map(into_prost_timestamp),
            }],
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
            .map(|q| openmetrics_data_model::summary_value::Quantile {
                quantile: q.quantile(),
                value: q.value(),
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
                        created: created.map(into_prost_timestamp),
                    },
                )),
                timestamp: self.timestamp.map(into_prost_timestamp),
            }],
        });

        Ok(())
    }

    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> Result<()> {
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
    fn encode(&mut self, label: &dyn EncodeLabel) -> Result<()> {
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
            fn [<encode_ $integer _value>](&mut self, value: $integer) -> Result<()> {
                self.label.value.push_str(itoa::Buffer::new().format(value));
                Ok(())
            }
        )* }
    )
}

macro_rules! encode_float_value_impls {
    ($($float:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $float _value>](&mut self, value: $float) -> Result<()> {
                self.label.value.push_str(dtoa::Buffer::new().format(value));
                Ok(())
            }
        )* }
    )
}

impl encoder::LabelEncoder for LabelEncoder<'_> {
    fn encode_label_name(&mut self, name: &str) -> Result<()> {
        self.label.name.push_str(name);
        Ok(())
    }

    fn encode_str_value(&mut self, value: &str) -> Result<()> {
        self.label.value.push_str(value);
        Ok(())
    }

    fn encode_bool_value(&mut self, value: bool) -> Result<()> {
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
    fn encode_i32(&mut self, value: i32) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_i64(&mut self, value: i64) -> Result<()> {
        *self.value = openmetrics_data_model::unknown_value::Value::IntValue(value);
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_u32(&mut self, value: u32) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_f32(&mut self, value: f32) -> Result<()> {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> Result<()> {
        *self.value = openmetrics_data_model::unknown_value::Value::DoubleValue(value);
        Ok(())
    }
}

struct GaugeValueEncoder<'a> {
    value: &'a mut openmetrics_data_model::gauge_value::Value,
}

impl encoder::GaugeValueEncoder for GaugeValueEncoder<'_> {
    fn encode_i32(&mut self, value: i32) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_i64(&mut self, value: i64) -> Result<()> {
        *self.value = openmetrics_data_model::gauge_value::Value::IntValue(value);
        Ok(())
    }

    fn encode_isize(&mut self, value: isize) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_u32(&mut self, value: u32) -> Result<()> {
        self.encode_i64(value as i64)
    }

    fn encode_u64(&mut self, value: u64) -> Result<()> {
        if value <= i64::MAX as u64 {
            *self.value = openmetrics_data_model::gauge_value::Value::IntValue(value as i64);
            Ok(())
        }
        // value > i64::MAX
        else {
            // For gauge metrics that support the u64 type, the openmetrics protobuf format does not
            // support encoding values exceeding i64::MAX.
            Err(Error::unsupported(
                "OpenMetrics protobuf format does not support encoding gauge values exceeding i64::MAX",
            ))
        }
    }

    fn encode_f32(&mut self, value: f32) -> Result<()> {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> Result<()> {
        *self.value = openmetrics_data_model::gauge_value::Value::DoubleValue(value);
        Ok(())
    }
}

struct CounterValueEncoder<'a> {
    total: &'a mut openmetrics_data_model::counter_value::Total,
}

impl encoder::CounterValueEncoder for CounterValueEncoder<'_> {
    fn encode_u32(&mut self, value: u32) -> Result<()> {
        self.encode_u64(value as u64)
    }

    fn encode_u64(&mut self, value: u64) -> Result<()> {
        *self.total = openmetrics_data_model::counter_value::Total::IntValue(value);
        Ok(())
    }

    fn encode_usize(&mut self, value: usize) -> Result<()> {
        self.encode_u64(value as u64)
    }

    fn encode_f32(&mut self, value: f32) -> Result<()> {
        self.encode_f64(value as f64)
    }

    fn encode_f64(&mut self, value: f64) -> Result<()> {
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
    ) -> Result<()> {
        label_set.encode(&mut LabelSetEncoder { labels: &mut self.exemplar.label })?;
        self.exemplar.value = value;
        self.exemplar.timestamp = timestamp.map(into_prost_timestamp);
        Ok(())
    }
}
