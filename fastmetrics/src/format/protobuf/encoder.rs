use std::time::Duration;

use super::{prometheus_data_model, wire};
use crate::{
    encoder::{
        self, EncodeCounterValue, EncodeExemplar, EncodeGaugeValue, EncodeLabel, EncodeLabelSet,
        EncodeMetric, EncodeUnknownValue,
    },
    error::{Error, Result},
    raw::{Metadata, MetricType, bucket::Bucket, quantile::Quantile},
    registry::Registry,
};

#[derive(Default)]
struct Encoder {
    output: Vec<u8>,
    family: Vec<u8>,
    name: String,
    scratch: Scratch,
}

pub(super) fn encode(buffer: &mut impl prost::bytes::BufMut, registry: &Registry) -> Result<()> {
    let mut encoder = Encoder::default();
    encoder.encode_registry(registry)?;
    let required = encoder.output.len();
    let remaining = buffer.remaining_mut();
    if required > remaining {
        return Err(Error::unexpected("insufficient buffer capacity to encode protobuf metrics")
            .with_context("required", required)
            .with_context("remaining", remaining));
    }
    // Commit only after collection and the capacity check succeed.
    buffer.put_slice(&encoder.output);
    Ok(())
}

impl Encoder {
    fn encode_registry(&mut self, registry: &Registry) -> Result<()> {
        for (metadata, metric) in &registry.metrics {
            self.encode_family(metadata, metric, registry)?;
        }
        for subsystem in registry.subsystems.values() {
            self.encode_registry(subsystem)?;
        }
        Ok(())
    }

    fn encode_family(
        &mut self,
        metadata: &Metadata,
        metric: &dyn EncodeMetric,
        registry: &Registry,
    ) -> Result<()> {
        if metric.is_empty() {
            return Ok(());
        }

        self.name.clear();
        if let Some(namespace) = registry.namespace() {
            self.name.push_str(namespace);
            self.name.push('_');
        }
        self.name.push_str(metadata.name());
        if metadata.metric_type() == MetricType::Info {
            self.name.push_str("_info");
        }

        self.family.clear();
        wire::string(&mut self.family, 1, &self.name);
        wire::string(&mut self.family, 2, metadata.help());
        wire::enumeration(
            &mut self.family,
            3,
            prometheus_data_model::MetricType::from(metadata.metric_type()) as i32,
        );
        if let Some(unit) = metadata.unit() {
            wire::string(&mut self.family, 5, unit.as_str());
        }

        self.scratch.labels.bytes.clear();
        registry.constant_labels().encode(&mut self.scratch.labels)?;
        metric.encode(&mut MetricEncoder {
            family: &mut self.family,
            scratch: &mut self.scratch,
            timestamp_ms: metric.timestamp().map(timestamp_millis),
            state_label_name: &self.name,
        })?;

        wire::delimited(&mut self.output, &self.family);
        Ok(())
    }
}

#[derive(Default)]
struct Scratch {
    value: Vec<u8>,
    bucket: Vec<u8>,
    labels: Labels,
    exemplar: Exemplar,
}

struct MetricEncoder<'a> {
    family: &'a mut Vec<u8>,
    scratch: &'a mut Scratch,
    timestamp_ms: Option<i64>,
    state_label_name: &'a str,
}

impl MetricEncoder<'_> {
    fn scalar(&mut self, tag: u32, value: f64) {
        self.emit_direct(tag, wire::double_len(1), |buffer| wire::double(buffer, 1, value));
    }

    fn emit_direct(&mut self, tag: u32, value_len: usize, encode: impl FnOnce(&mut Vec<u8>)) {
        write_metric(
            self.family,
            &self.scratch.labels.bytes,
            self.timestamp_ms,
            tag,
            value_len,
            encode,
        );
    }

    fn emit(&mut self, tag: u32) {
        write_metric(
            self.family,
            &self.scratch.labels.bytes,
            self.timestamp_ms,
            tag,
            self.scratch.value.len(),
            |buffer| buffer.extend_from_slice(&self.scratch.value),
        );
    }
}

fn write_metric(
    family: &mut Vec<u8>,
    labels: &[u8],
    timestamp: Option<i64>,
    tag: u32,
    value_len: usize,
    encode: impl FnOnce(&mut Vec<u8>),
) {
    let timestamp_len = timestamp.map_or(0, |value| wire::uint64_len(6, value as u64));
    let metric_len = labels.len() + wire::message_len(tag, value_len) + timestamp_len;
    wire::message_header(family, 4, metric_len);
    family.extend_from_slice(labels);
    wire::message_header(family, tag, value_len);
    encode(family);
    if let Some(timestamp) = timestamp {
        wire::int64(family, 6, timestamp);
    }
}

impl encoder::MetricEncoder for MetricEncoder<'_> {
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> Result<()> {
        let mut encoded = UnknownValueEncoder::default();
        value.encode(&mut encoded)?;
        self.scalar(5, encoded.value);
        Ok(())
    }

    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> Result<()> {
        let mut encoded = GaugeValueEncoder::default();
        value.encode(&mut encoded)?;
        self.scalar(2, encoded.value);
        Ok(())
    }

    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        exemplar: Option<&dyn EncodeExemplar>,
        created: Option<Duration>,
    ) -> Result<()> {
        let mut encoded = CounterValueEncoder::default();
        total.encode(&mut encoded)?;
        if exemplar.is_none() && created.is_none() {
            self.scalar(3, encoded.value);
            return Ok(());
        }
        self.scratch.value.clear();
        wire::double(&mut self.scratch.value, 1, encoded.value);
        if let Some(exemplar) = exemplar {
            self.scratch.exemplar.collect(exemplar)?;
            wire::message(&mut self.scratch.value, 2, &self.scratch.exemplar.bytes);
        }
        if let Some(created) = created {
            wire::timestamp(&mut self.scratch.value, 3, created);
        }
        self.emit(3);
        Ok(())
    }

    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> Result<()> {
        let len = self.scratch.labels.bytes.len();
        for (state, enabled) in states {
            self.scratch.labels.append(self.state_label_name, state);
            self.scalar(2, if enabled { 1.0 } else { 0.0 });
            self.scratch.labels.bytes.truncate(len);
        }
        Ok(())
    }

    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> Result<()> {
        let len = self.scratch.labels.bytes.len();
        let result = label_set.encode(&mut self.scratch.labels);
        if result.is_ok() {
            self.scalar(2, 1.0);
        }
        self.scratch.labels.bytes.truncate(len);
        result
    }

    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
        count: u64,
        sum: f64,
        created: Option<Duration>,
    ) -> Result<()> {
        if let Some(exemplars) = exemplars {
            assert_eq!(buckets.len(), exemplars.len(), "buckets and exemplars count mismatch");
        }
        if exemplars.is_none() && created.is_none() {
            let mut len = wire::uint64_len(1, count) + wire::double_len(2);
            let mut cumulative = 0_u64;
            // Both passes use the same snapshot without collecting the metric again.
            for bucket in buckets {
                cumulative = cumulative.saturating_add(bucket.count());
                let bucket_len = wire::uint64_len(1, cumulative) + wire::double_len(2);
                len += wire::message_len(3, bucket_len);
            }
            self.emit_direct(7, len, |buffer| {
                wire::uint64(buffer, 1, count);
                wire::double(buffer, 2, sum);
                let mut cumulative = 0_u64;
                for bucket in buckets {
                    cumulative = cumulative.saturating_add(bucket.count());
                    let bucket_len = wire::uint64_len(1, cumulative) + wire::double_len(2);
                    wire::message_header(buffer, 3, bucket_len);
                    wire::uint64(buffer, 1, cumulative);
                    wire::double(buffer, 2, bucket.upper_bound());
                }
            });
            return Ok(());
        }
        let Scratch { value, bucket, exemplar, .. } = &mut self.scratch;
        value.clear();
        wire::uint64(value, 1, count);
        wire::double(value, 2, sum);
        let mut cumulative_count = 0_u64;
        for (idx, source) in buckets.iter().enumerate() {
            cumulative_count = cumulative_count.saturating_add(source.count());
            bucket.clear();
            wire::uint64(bucket, 1, cumulative_count);
            wire::double(bucket, 2, source.upper_bound());
            if let Some(source) = exemplars.and_then(|exemplars| exemplars[idx]) {
                exemplar.collect(source)?;
                wire::message(bucket, 3, &exemplar.bytes);
            }
            wire::message(value, 3, bucket);
        }
        if let Some(created) = created {
            wire::timestamp(value, 15, created);
        }
        self.emit(7);
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
        let Scratch { value, bucket, .. } = &mut self.scratch;
        value.clear();
        wire::uint64(value, 1, count);
        wire::double(value, 2, sum);
        for quantile in quantiles {
            bucket.clear();
            wire::double(bucket, 1, quantile.quantile());
            wire::double(bucket, 2, quantile.value());
            wire::message(value, 3, bucket);
        }
        if let Some(created) = created {
            wire::timestamp(value, 4, created);
        }
        self.emit(4);
        Ok(())
    }

    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> Result<()> {
        let len = self.scratch.labels.bytes.len();
        let result = label_set.encode(&mut self.scratch.labels).and_then(|()| {
            metric.encode(&mut MetricEncoder {
                family: self.family,
                scratch: self.scratch,
                timestamp_ms: metric.timestamp().map(timestamp_millis),
                state_label_name: self.state_label_name,
            })
        });
        self.scratch.labels.bytes.truncate(len);
        result
    }
}

#[inline]
fn timestamp_millis(duration: Duration) -> i64 {
    let millis = duration.as_millis();
    if millis > i64::MAX as u128 { i64::MAX } else { millis as i64 }
}

#[derive(Default)]
struct Labels {
    bytes: Vec<u8>,
    label: Label,
}

impl Labels {
    fn append(&mut self, name: &str, value: &str) {
        let len = wire::message_len(1, name.len()) + wire::message_len(2, value.len());
        wire::message_header(&mut self.bytes, 1, len);
        wire::string(&mut self.bytes, 1, name);
        wire::string(&mut self.bytes, 2, value);
    }
}

impl encoder::LabelSetEncoder for Labels {
    fn encode(&mut self, label: &dyn EncodeLabel) -> Result<()> {
        self.label.name.clear();
        self.label.value.clear();
        self.label.has_name = false;
        self.label.has_value = false;
        label.encode(&mut self.label)?;
        if !self.label.has_name && !self.label.has_value {
            return Ok(());
        }
        let len = if self.label.has_name { wire::message_len(1, self.label.name.len()) } else { 0 }
            + if self.label.has_value { wire::message_len(2, self.label.value.len()) } else { 0 };
        wire::message_header(&mut self.bytes, 1, len);
        if self.label.has_name {
            wire::string(&mut self.bytes, 1, &self.label.name);
        }
        if self.label.has_value {
            wire::string(&mut self.bytes, 2, &self.label.value);
        }
        Ok(())
    }
}

#[derive(Default)]
struct Label {
    name: String,
    value: String,
    has_name: bool,
    has_value: bool,
}

macro_rules! encode_integer_value_impls {
    ($($integer:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $integer _value>](&mut self, value: $integer) -> Result<()> {
                self.encode_str_value(itoa::Buffer::new().format(value))
            }
        )* }
    )
}

macro_rules! encode_float_value_impls {
    ($($float:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $float _value>](&mut self, value: $float) -> Result<()> {
                self.encode_str_value(zmij::Buffer::new().format(value))
            }
        )* }
    )
}

impl encoder::LabelEncoder for Label {
    fn encode_label_name(&mut self, name: &str) -> Result<()> {
        self.has_name = true;
        self.name.push_str(name);
        Ok(())
    }

    fn encode_str_value(&mut self, value: &str) -> Result<()> {
        self.has_value = true;
        self.value.push_str(value);
        Ok(())
    }

    fn encode_bool_value(&mut self, value: bool) -> Result<()> {
        self.encode_str_value(if value { "true" } else { "false" })
    }

    encode_integer_value_impls! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize
    }

    encode_float_value_impls! { f32, f64 }
}

#[derive(Default)]
struct Exemplar {
    bytes: Vec<u8>,
    labels: Labels,
    value: Option<f64>,
    timestamp: Option<Duration>,
}

impl Exemplar {
    fn collect(&mut self, source: &dyn EncodeExemplar) -> Result<()> {
        self.bytes.clear();
        self.labels.bytes.clear();
        self.value = None;
        self.timestamp = None;
        source.encode(self)?;
        self.bytes.extend_from_slice(&self.labels.bytes);
        if let Some(value) = self.value {
            wire::double(&mut self.bytes, 2, value);
        }
        if let Some(timestamp) = self.timestamp {
            wire::timestamp(&mut self.bytes, 3, timestamp);
        }
        Ok(())
    }
}

impl encoder::ExemplarEncoder for Exemplar {
    fn encode(
        &mut self,
        label_set: &dyn EncodeLabelSet,
        value: f64,
        timestamp: Option<Duration>,
    ) -> Result<()> {
        label_set.encode(&mut self.labels)?;
        self.value = Some(value);
        self.timestamp = timestamp;
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
