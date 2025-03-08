//! Text exposition format.

use std::{borrow::Cow, collections::HashMap, fmt, time::Duration};

use crate::{
    encoder::{
        self, EncodeCounterValue, EncodeGaugeValue, EncodeLabelSet, EncodeLabelValue,
        EncodeUnknownValue, MetricFamilyEncoder as _,
    },
    metrics::{
        family::{Metadata, Unit},
        raw::{
            bucket::{Bucket, BUCKET_LABEL},
            quantile::{Quantile, QUANTILE_LABEL},
        },
    },
    registry::{Registry, RegistrySystem},
};

/// Encodes metrics from a registry into the [OpenMetrics text format](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#text-format).
///
/// The text format is human-readable and follows the format:
/// ```text
/// # TYPE metric_name type
/// # HELP metric_name help_text
/// # UNIT metric_name unit
/// metric_name{label="value"} value
/// ```
///
/// # Arguments
///
/// * `writer` - A mutable reference to any type implementing `fmt::Write` where the text format
///   will be written.
/// * `registry` - A reference to the [`Registry`] containing the metrics to encode.
///
/// # Returns
///
/// Returns `Ok(())` if encoding was successful, or a [`fmt::Error`] if there was an error writing
/// to the output.
///
/// # Example
///
/// ```rust
/// # use openmetrics_client::{
/// #     format::text,
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
/// // Encode metrics in text format
/// let mut output = String::new();
/// text::encode(&mut output, &registry)?;
/// assert!(output.contains("http_requests_total"));
/// # Ok(())
/// # }
/// ```
pub fn encode(writer: &mut impl fmt::Write, registry: &Registry) -> fmt::Result {
    Encoder::new(writer, registry).encode()
}

struct Encoder<'a, W> {
    writer: &'a mut W,
    registry: &'a Registry,
}

impl<'a, W> Encoder<'a, W>
where
    W: fmt::Write,
{
    fn new(writer: &'a mut W, registry: &'a Registry) -> Self {
        Self { writer, registry }
    }

    fn encode(&mut self) -> fmt::Result {
        self.encode_registry()?;
        self.encode_eof()
    }

    fn encode_registry(&mut self) -> fmt::Result {
        for (metadata, metric) in &self.registry.metrics {
            let mut family_encoder = MetricFamilyEncoder::new(&mut self.writer)
                .with_namespace(self.registry.namespace())
                .with_const_labels(&self.registry.const_labels);
            let mut metric_encoder = family_encoder.encode_metadata(metadata)?;
            metric.encode(metric_encoder.as_mut())?
        }
        self.encode_registry_system(&self.registry.subsystems)?;
        Ok(())
    }

    fn encode_registry_system(&mut self, systems: &HashMap<String, RegistrySystem>) -> fmt::Result {
        for system in systems.values() {
            for (metadata, metric) in &system.metrics {
                let mut family_encoder = MetricFamilyEncoder::new(&mut self.writer)
                    .with_namespace(Some(system.namespace()))
                    .with_const_labels(&system.const_labels);
                let mut metric_encoder = family_encoder.encode_metadata(metadata)?;
                metric.encode(metric_encoder.as_mut())?
            }
            for subsystem in system.subsystems.values() {
                self.encode_registry_system(&subsystem.subsystems)?
            }
        }
        Ok(())
    }

    fn encode_eof(&mut self) -> fmt::Result {
        self.writer.write_str("# EOF\n")
    }
}

struct MetricFamilyEncoder<'a, W> {
    writer: &'a mut W,
    namespace: Option<&'a str>,
    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
}

impl<'a, W> MetricFamilyEncoder<'a, W>
where
    W: fmt::Write,
{
    fn new(writer: &'a mut W) -> MetricFamilyEncoder<'a, W> {
        Self { writer, namespace: None, const_labels: &[] }
    }

    fn with_namespace(mut self, namespace: Option<&'a str>) -> MetricFamilyEncoder<'a, W> {
        self.namespace = namespace;
        self
    }

    fn with_const_labels(
        mut self,
        labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
    ) -> MetricFamilyEncoder<'a, W> {
        self.const_labels = labels;
        self
    }

    fn encode_type(&mut self, metadata: &Metadata) -> fmt::Result {
        self.writer.write_str("# TYPE ")?;
        self.encode_metric_name(metadata)?;
        self.writer.write_str(" ")?;
        self.writer.write_str(metadata.metric_type().as_str())?;
        self.encode_newline()?;
        Ok(())
    }

    fn encode_help(&mut self, metadata: &Metadata) -> fmt::Result {
        self.writer.write_str("# HELP ")?;
        self.encode_metric_name(metadata)?;
        self.writer.write_str(" ")?;
        self.writer.write_str(metadata.help())?;
        self.encode_newline()?;
        Ok(())
    }

    fn encode_unit(&mut self, metadata: &Metadata) -> fmt::Result {
        if let Some(unit) = metadata.unit() {
            self.writer.write_str("# UNIT ")?;
            self.encode_metric_name(metadata)?;
            self.writer.write_str(" ")?;
            self.writer.write_str(unit.as_str())?;
            self.encode_newline()?;
        }
        Ok(())
    }

    fn encode_metric_name(&mut self, metadata: &Metadata) -> fmt::Result {
        MetricNameEncoder {
            writer: self.writer,
            namespace: self.namespace,
            name: metadata.name(),
            unit: metadata.unit(),
        }
        .encode()
    }

    fn encode_newline(&mut self) -> fmt::Result {
        self.writer.write_str("\n")
    }
}

impl<W> encoder::MetricFamilyEncoder for MetricFamilyEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode_metadata<'s>(
        &'s mut self,
        metadata: &'s Metadata,
    ) -> Result<Box<dyn encoder::MetricEncoder + 's>, fmt::Error> {
        self.encode_type(metadata)?;
        self.encode_help(metadata)?;
        self.encode_unit(metadata)?;

        Ok(Box::new(MetricEncoder::<'s, W> {
            writer: self.writer,
            namespace: self.namespace,
            name: metadata.name(),
            unit: metadata.unit(),
            const_labels: self.const_labels,
            family_labels: None,
        }))
    }
}

struct MetricEncoder<'a, W> {
    writer: &'a mut W,

    namespace: Option<&'a str>,
    name: &'a str,
    unit: Option<&'a Unit>,

    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
    family_labels: Option<&'a dyn EncodeLabelSet>,
}

impl<W> MetricEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode_metric_name(&mut self) -> fmt::Result {
        MetricNameEncoder {
            writer: self.writer,
            namespace: self.namespace,
            name: self.name,
            unit: self.unit,
        }
        .encode()
    }

    fn encode_suffix(&mut self, suffix: &str) -> fmt::Result {
        self.writer.write_str("_")?;
        self.writer.write_str(suffix)?;
        Ok(())
    }

    fn encode_label_set(&mut self, additional_labels: Option<&dyn EncodeLabelSet>) -> fmt::Result {
        if self.const_labels.is_empty()
            && self.family_labels.is_none()
            && additional_labels.is_none()
        {
            return Ok(());
        }

        self.writer.write_str("{")?;
        self.const_labels.encode(&mut LabelSetEncoder::new(self.writer))?;
        if let Some(family_labels) = self.family_labels {
            if !self.const_labels.is_empty() {
                self.writer.write_str(",")?;
            }
            family_labels.encode(&mut LabelSetEncoder::new(self.writer))?;
        }
        if let Some(additional_labels) = additional_labels {
            if !self.const_labels.is_empty() || self.family_labels.is_some() {
                self.writer.write_str(",")?;
            }
            additional_labels.encode(&mut LabelSetEncoder::new(self.writer))?;
        }
        self.writer.write_str("}")?;
        Ok(())
    }

    fn encode_sum(&mut self, sum: f64) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_suffix("sum")?;
        self.encode_label_set(None)?;
        self.writer.write_str(" ")?;
        self.writer.write_str(dtoa::Buffer::new().format(sum))?;
        Ok(())
    }

    fn encode_gsum(&mut self, gsum: f64) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_suffix("gsum")?;
        self.encode_label_set(None)?;
        self.writer.write_str(" ")?;
        self.writer.write_str(dtoa::Buffer::new().format(gsum))?;
        Ok(())
    }

    fn encode_count(&mut self, count: u64) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_suffix("count")?;
        self.encode_label_set(None)?;
        self.writer.write_str(" ")?;
        self.writer.write_str(itoa::Buffer::new().format(count))?;
        Ok(())
    }

    fn encode_gcount(&mut self, gcount: u64) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_suffix("gcount")?;
        self.encode_label_set(None)?;
        self.writer.write_str(" ")?;
        self.writer.write_str(itoa::Buffer::new().format(gcount))?;
        Ok(())
    }

    fn encode_created(&mut self, created: Duration) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_suffix("created")?;
        self.encode_label_set(None)?;
        self.writer.write_fmt(format_args!(
            " {}.{}",
            created.as_secs(),
            created.as_millis() % 1000
        ))?;
        Ok(())
    }

    fn encode_newline(&mut self) -> fmt::Result {
        self.writer.write_str("\n")?;
        Ok(())
    }
}

impl<W> encoder::MetricEncoder for MetricEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_label_set(None)?;
        self.writer.write_str(" ")?;
        value.encode(&mut UnknownValueEncoder { writer: self.writer } as _)?;
        self.encode_newline()?;
        Ok(())
    }

    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_label_set(None)?;
        self.writer.write_str(" ")?;
        value.encode(&mut GaugeValueEncoder { writer: self.writer } as _)?;
        self.encode_newline()?;
        Ok(())
    }

    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        created: Option<Duration>,
    ) -> fmt::Result {
        // encode `*_total` metric
        self.encode_metric_name()?;
        self.encode_suffix("total")?;
        self.encode_label_set(None)?;
        self.writer.write_str(" ")?;
        total.encode(&mut CounterValueEncoder { writer: self.writer } as _)?;
        self.encode_newline()?;

        // encode `*_created` metric if available
        if let Some(created) = created {
            self.encode_created(created)?;
            self.encode_newline()?;
        }

        Ok(())
    }

    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> fmt::Result {
        for (state, enabled) in states {
            self.encode_metric_name()?;
            self.encode_label_set(Some(&[(self.name, state)]))?;
            self.writer.write_str(" ")?;
            if enabled {
                self.writer.write_str(itoa::Buffer::new().format(1))?;
            } else {
                self.writer.write_str(itoa::Buffer::new().format(0))?;
            }
            self.encode_newline()?;
        }
        Ok(())
    }

    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> fmt::Result {
        self.encode_metric_name()?;
        self.encode_suffix("info")?;
        self.encode_label_set(Some(label_set))?;
        self.writer.write_str(" 1")?;
        self.encode_newline()?;
        Ok(())
    }

    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> fmt::Result {
        // encode bucket metrics
        let mut cumulative_count = 0;
        for bucket in buckets {
            let upper_bound = bucket.upper_bound();
            let bucket_count = bucket.count();
            self.encode_metric_name()?;
            self.encode_suffix("bucket")?;

            if upper_bound == f64::INFINITY {
                self.encode_label_set(Some(&[(BUCKET_LABEL, "+Inf")]))?;
            } else {
                self.encode_label_set(Some(&[(
                    BUCKET_LABEL,
                    ryu::Buffer::new().format(upper_bound),
                )]))?;
            }

            self.writer.write_str(" ")?;
            cumulative_count += bucket_count;
            self.writer.write_str(itoa::Buffer::new().format(cumulative_count))?;
            self.encode_newline()?;
        }

        // encode `*_sum` metric
        self.encode_sum(sum)?;
        self.encode_newline()?;

        // encode `*_count` metric
        self.encode_count(count)?;
        self.encode_newline()?;

        // encode `*_created` metric if available
        if let Some(created) = created {
            self.encode_created(created)?;
            self.encode_newline()?;
        }

        Ok(())
    }

    fn encode_gauge_histogram(&mut self, buckets: &[Bucket], sum: f64, count: u64) -> fmt::Result {
        // encode bucket metrics
        let mut cumulative_count = 0;
        for bucket in buckets {
            let upper_bound = bucket.upper_bound();
            let bucket_count = bucket.count();
            self.encode_metric_name()?;
            self.encode_suffix("bucket")?;

            if upper_bound == f64::INFINITY {
                self.encode_label_set(Some(&[(BUCKET_LABEL, "+Inf")]))?;
            } else {
                self.encode_label_set(Some(&[(
                    BUCKET_LABEL,
                    ryu::Buffer::new().format(upper_bound),
                )]))?;
            }

            self.writer.write_str(" ")?;
            cumulative_count += bucket_count;
            self.writer.write_str(itoa::Buffer::new().format(cumulative_count))?;
            self.encode_newline()?;
        }

        // encode `*_gsum` metric
        self.encode_gsum(sum)?;
        self.encode_newline()?;

        // encode `*_gcount` metric
        self.encode_gcount(count)?;
        self.encode_newline()?;

        Ok(())
    }

    fn encode_summary(
        &mut self,
        quantiles: &[Quantile],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> fmt::Result {
        // encode quantile metrics
        for quantile in quantiles {
            self.encode_metric_name()?;
            self.encode_label_set(Some(&[(
                QUANTILE_LABEL,
                dtoa::Buffer::new().format(quantile.quantile()),
            )]))?;
            self.writer.write_str(" ")?;
            self.writer.write_str(dtoa::Buffer::new().format(quantile.value()))?;
            self.encode_newline()?;
        }

        // encode `*_sum` metric
        self.encode_sum(sum)?;
        self.encode_newline()?;

        // encode `*_count` metric
        self.encode_count(count)?;
        self.encode_newline()?;

        // encode `*_created` metric if available
        if let Some(created) = created {
            self.encode_created(created)?;
            self.encode_newline()?;
        }

        Ok(())
    }

    fn encode_family<'s>(
        &'s mut self,
        label_set: &'s dyn EncodeLabelSet,
    ) -> Result<Box<dyn encoder::MetricEncoder + 's>, fmt::Error> {
        debug_assert!(self.family_labels.is_none());

        Ok(Box::new(MetricEncoder::<'s, W> {
            writer: self.writer,
            namespace: self.namespace,
            name: self.name,
            unit: self.unit,
            const_labels: self.const_labels,
            family_labels: Some(label_set),
        }))
    }
}

struct MetricNameEncoder<'a, W> {
    writer: &'a mut W,
    namespace: Option<&'a str>,
    name: &'a str,
    unit: Option<&'a Unit>,
}

impl<W> MetricNameEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode(&mut self) -> fmt::Result {
        if let Some(namespace) = self.namespace {
            self.writer.write_str(namespace)?;
            self.writer.write_str("_")?;
        }
        self.writer.write_str(self.name)?;
        if let Some(unit) = self.unit {
            self.writer.write_str("_")?;
            self.writer.write_str(unit.as_str())?;
        }
        Ok(())
    }
}

struct LabelSetEncoder<'a, W> {
    writer: &'a mut W,
    first: bool,
}

impl<'a, W> LabelSetEncoder<'a, W> {
    fn new(writer: &'a mut W) -> LabelSetEncoder<'a, W> {
        Self { writer, first: true }
    }
}

impl<W> encoder::LabelSetEncoder for LabelSetEncoder<'_, W>
where
    W: fmt::Write,
{
    fn label_encoder<'s>(&'s mut self) -> Box<dyn encoder::LabelEncoder + 's> {
        let first = self.first;
        self.first = false;
        Box::new(LabelEncoder { writer: self.writer, first })
    }
}

struct LabelEncoder<'a, W> {
    writer: &'a mut W,
    first: bool,
}

macro_rules! encode_integer_value_impls {
    ($($integer:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $integer _value>](&mut self, value: $integer) -> fmt::Result {
                self.writer.write_str("=\"")?;
                self.writer.write_str(itoa::Buffer::new().format(value))?;
                self.writer.write_str("\"")?;
                Ok(())
            }
        )* }
    )
}

macro_rules! encode_float_value_impls {
    ($($float:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $float _value>](&mut self, value: $float) -> fmt::Result {
                self.writer.write_str("=\"")?;
                self.writer.write_str(dtoa::Buffer::new().format(value))?;
                self.writer.write_str("\"")?;
                Ok(())
            }
        )* }
    )
}

impl<W> encoder::LabelEncoder for LabelEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode_label_name(&mut self, name: &str) -> fmt::Result {
        if !self.first {
            self.writer.write_str(",")?;
        }
        self.writer.write_str(name)?;
        Ok(())
    }

    fn encode_str_value(&mut self, value: &str) -> fmt::Result {
        self.writer.write_str("=\"")?;
        self.writer.write_str(value)?;
        self.writer.write_str("\"")?;
        Ok(())
    }

    fn encode_bool_value(&mut self, value: bool) -> fmt::Result {
        self.writer.write_str("=\"")?;
        self.writer.write_str(if value { "true" } else { "false" })?;
        self.writer.write_str("\"")?;
        Ok(())
    }

    encode_integer_value_impls! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize
    }

    encode_float_value_impls! { f32, f64 }

    fn encode_some_value(&mut self, value: &dyn EncodeLabelValue) -> fmt::Result {
        value.encode(self)
    }

    fn encode_none_value(&mut self) -> fmt::Result {
        self.writer.write_str("\"\"")
    }
}

macro_rules! encode_integer_number_impls {
    ($($integer:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $integer>](&mut self, value: $integer) -> fmt::Result {
                self.writer.write_str(itoa::Buffer::new().format(value))?;
                Ok(())
            }
        )* }
    )
}

macro_rules! encode_float_number_impls {
    ($($float:ty),*) => (
        paste::paste! { $(
            fn [<encode_ $float>](&mut self, value: $float) -> fmt::Result {
                self.writer.write_str(dtoa::Buffer::new().format(value))?;
                Ok(())
            }
        )* }
    )
}

struct UnknownValueEncoder<'a, W> {
    writer: &'a mut W,
}

impl<W> encoder::UnknownValueEncoder for UnknownValueEncoder<'_, W>
where
    W: fmt::Write,
{
    encode_integer_number_impls! {
        i32, i64, isize, u32
    }

    encode_float_number_impls! {
        f32, f64
    }
}

struct GaugeValueEncoder<'a, W> {
    writer: &'a mut W,
}

impl<W> encoder::GaugeValueEncoder for GaugeValueEncoder<'_, W>
where
    W: fmt::Write,
{
    encode_integer_number_impls! {
        i32, i64, isize, u32
    }

    encode_float_number_impls! {
        f32, f64
    }
}

struct CounterValueEncoder<'a, W> {
    writer: &'a mut W,
}

impl<W> encoder::CounterValueEncoder for CounterValueEncoder<'_, W>
where
    W: fmt::Write,
{
    encode_integer_number_impls! {
        u32, u64, usize
    }

    encode_float_number_impls! {
        f32, f64
    }
}
