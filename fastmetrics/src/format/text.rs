//! Text exposition format.

use std::{borrow::Cow, fmt, time::Duration};

use crate::{
    encoder::{
        self, EncodeCounterValue, EncodeExemplar, EncodeGaugeValue, EncodeLabel, EncodeLabelSet,
        EncodeMetric, EncodeUnknownValue, MetricFamilyEncoder as _,
    },
    error::Result,
    raw::{
        Metadata, MetricType, Unit,
        bucket::{BUCKET_LABEL, Bucket},
        quantile::{QUANTILE_LABEL, Quantile},
    },
    registry::Registry,
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
/// Returns `Ok(())` if encoding was successful, or a [`Error`](crate::error::Error) if there was an
/// error writing to the output.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::{
/// #     error::Result,
/// #     format::text,
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
/// // Encode metrics in text format
/// let mut output = String::new();
/// text::encode(&mut output, &registry)?;
/// assert!(output.contains("http_requests_total"));
/// # Ok(())
/// # }
/// ```
pub fn encode(writer: &mut impl fmt::Write, registry: &Registry) -> Result<()> {
    encode_with(writer, registry, crate::metrics::lazy_group::scrape_ctx::enter)
}

/// Encodes metrics in text format, running a user-provided "scope/guard" for the duration of the
/// encoding pass.
///
/// This is the underlying primitive used by [`encode`]. The `before` closure is called once before
/// encoding starts, and the value it returns is kept alive for the duration of encoding and then
/// dropped.
///
/// ## Scrape-scoped caching (grouped lazy metrics)
///
/// Grouped lazy metrics created via [`crate::metrics::lazy_group::LazyGroup`] can share an expensive
/// sampling operation within a single scrape. To enable this, pass a closure that enters a scrape
/// scope (internally used by the crate's default [`encode`] implementation).
///
/// ## Example
///
/// ```rust
/// # use fastmetrics::{error::Result, format::text, registry::Registry};
/// # fn main() -> Result<()> {
/// let registry = Registry::default();
/// let mut out = String::new();
///
/// // Encode without adding any additional per-encode scope:
/// text::encode_with(&mut out, &registry, || ())?;
/// # Ok(())
/// # }
/// ```
pub fn encode_with<G>(
    writer: &mut impl fmt::Write,
    registry: &Registry,
    before: impl FnOnce() -> G,
) -> Result<()> {
    // The returned value is kept alive for the duration of encoding and then dropped.
    let _guard = before();

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

    fn encode(&mut self) -> Result<()> {
        self.encode_registry(self.registry)?;
        self.encode_eof()
    }

    fn encode_registry(&mut self, registry: &Registry) -> Result<()> {
        for (metadata, metric) in &registry.metrics {
            MetricFamilyEncoder {
                writer: self.writer,
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

    fn encode_eof(&mut self) -> Result<()> {
        self.writer.write_str("# EOF\n")?;
        Ok(())
    }
}

struct MetricFamilyEncoder<'a, W> {
    writer: &'a mut W,
    namespace: Option<&'a str>,
    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
}

impl<W> MetricFamilyEncoder<'_, W>
where
    W: fmt::Write,
{
    #[inline]
    fn encode_type(&mut self, metric_name: &str, ty: MetricType) -> Result<()> {
        let ty = ty.as_str();
        self.writer.write_fmt(format_args!("# TYPE {metric_name} {ty}"))?;
        self.encode_newline()
    }

    #[inline]
    fn encode_help(&mut self, metric_name: &str, help: &str) -> Result<()> {
        self.writer.write_fmt(format_args!("# HELP {metric_name} {help}"))?;
        self.encode_newline()
    }

    #[inline]
    fn encode_unit(&mut self, metric_name: &str, unit: Option<&Unit>) -> Result<()> {
        if let Some(unit) = unit {
            let unit = unit.as_str();
            self.writer.write_fmt(format_args!("# UNIT {metric_name} {unit}"))?;
            self.encode_newline()?;
        }
        Ok(())
    }

    #[inline]
    fn encode_newline(&mut self) -> Result<()> {
        self.writer.write_str("\n")?;
        Ok(())
    }
}

fn metric_name<'a>(namespace: Option<&str>, name: &'a str, unit: Option<&Unit>) -> Cow<'a, str> {
    match (namespace, unit) {
        (Some(namespace), Some(unit)) => {
            Cow::Owned(format!("{namespace}_{}_{}", name, unit.as_str()))
        },
        (Some(namespace), None) => Cow::Owned(format!("{namespace}_{name}")),
        (None, Some(unit)) => Cow::Owned(format!("{name}_{}", unit.as_str())),
        (None, None) => Cow::Borrowed(name),
    }
}

impl<W> encoder::MetricFamilyEncoder for MetricFamilyEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode(&mut self, metadata: &Metadata, metric: &dyn EncodeMetric) -> Result<()> {
        if metric.is_empty() {
            // skip empty metric family
            return Ok(());
        }

        let metric_name = metric_name(self.namespace, metadata.name(), metadata.unit());

        self.encode_type(metric_name.as_ref(), metadata.metric_type())?;
        self.encode_help(metric_name.as_ref(), metadata.help())?;
        self.encode_unit(metric_name.as_ref(), metadata.unit())?;

        metric.encode(&mut MetricEncoder {
            writer: self.writer,
            metric_name,
            metric_type: metadata.metric_type(),
            timestamp: metric.timestamp(),
            const_labels: self.const_labels,
            family_labels: None,
        })
    }
}

struct MetricEncoder<'a, W> {
    writer: &'a mut W,
    // [namespace_]name[_unit]
    metric_name: Cow<'a, str>,
    metric_type: MetricType,
    timestamp: Option<Duration>,
    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
    family_labels: Option<&'a dyn EncodeLabelSet>,
}

impl<W> MetricEncoder<'_, W>
where
    W: fmt::Write,
{
    #[inline]
    fn encode_metric_name(&mut self) -> Result<()> {
        let metric_name = self.metric_name.as_ref();
        self.writer.write_str(metric_name)?;
        Ok(())
    }

    /// Pre-encode common labels (const_labels + family_labels) to a string buffer
    fn encode_common_labels_to_string(&self) -> Result<Option<String>> {
        let has_const_labels = !self.const_labels.is_empty();
        let has_family_labels = matches!(self.family_labels, Some(labels) if !labels.is_empty());

        if !has_const_labels && !has_family_labels {
            return Ok(None);
        }

        let mut common_labels = String::new();

        if has_const_labels {
            self.const_labels.encode(&mut LabelSetEncoder::new(&mut common_labels))?;
        }

        if has_family_labels {
            if has_const_labels {
                common_labels.push(',');
            }
            self.family_labels
                .expect("family_labels should be `Some` value")
                .encode(&mut LabelSetEncoder::new(&mut common_labels))?;
        }

        Ok(Some(common_labels))
    }

    /// Encode label set with pre-computed common labels for better performance
    fn encode_label_set_with_common(
        &mut self,
        common_labels: Option<&str>,
        additional_labels: Option<&dyn EncodeLabelSet>,
    ) -> Result<()> {
        let has_common_labels = common_labels.is_some();
        let has_additional_labels = matches!(additional_labels, Some(labels) if !labels.is_empty());

        if !has_common_labels && !has_additional_labels {
            self.writer.write_str(" ")?;
            return Ok(());
        }

        self.writer.write_str("{")?;

        let mut wrote_any = false;
        if has_common_labels {
            let common_labels = common_labels.expect("common_labels should be `Some` value");
            self.writer.write_str(common_labels)?;
            wrote_any = true;
        }

        if has_additional_labels {
            if wrote_any {
                self.writer.write_str(",")?;
            }
            additional_labels
                .expect("additional_labels should be `Some` value")
                .encode(&mut LabelSetEncoder::new(self.writer))?;
        }

        self.writer.write_str("} ")?;
        Ok(())
    }

    fn encode_label_set(&mut self, additional_labels: Option<&dyn EncodeLabelSet>) -> Result<()> {
        let has_const_labels = !self.const_labels.is_empty();
        let has_family_labels = matches!(self.family_labels, Some(labels) if !labels.is_empty());
        let has_additional_labels = matches!(additional_labels, Some(labels) if !labels.is_empty());

        if !has_const_labels && !has_family_labels && !has_additional_labels {
            self.writer.write_str(" ")?;
            return Ok(());
        }

        self.writer.write_str("{")?;

        let mut wrote_any = false;
        if has_const_labels {
            self.const_labels.encode(&mut LabelSetEncoder::new(self.writer))?;
            wrote_any = true;
        }

        if has_family_labels {
            if wrote_any {
                self.writer.write_str(",")?;
            }
            self.family_labels
                .expect("family_labels should be `Some` value")
                .encode(&mut LabelSetEncoder::new(self.writer))?;
            wrote_any = true;
        }

        if has_additional_labels {
            if wrote_any {
                self.writer.write_str(",")?;
            }
            additional_labels
                .expect("additional_labels should be `Some` value")
                .encode(&mut LabelSetEncoder::new(self.writer))?;
        }

        self.writer.write_str("} ")?;
        Ok(())
    }

    fn encode_buckets(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
    ) -> Result<()> {
        let exemplars = exemplars.inspect(|exemplars| {
            assert_eq!(buckets.len(), exemplars.len(), "buckets and exemplars count mismatch");
        });

        // pre-encode common labels once
        let common_labels = self.encode_common_labels_to_string()?;

        let mut cumulative_count = 0;
        for (idx, bucket) in buckets.iter().enumerate() {
            self.encode_metric_name()?;
            self.writer.write_str("_bucket")?;

            let upper_bound = bucket.upper_bound();
            let bucket_count = bucket.count();

            // use pre-computed common labels
            if upper_bound == f64::INFINITY {
                self.encode_label_set_with_common(
                    common_labels.as_deref(),
                    Some(&[(BUCKET_LABEL, "+Inf")]),
                )?;
            } else {
                self.encode_label_set_with_common(
                    common_labels.as_deref(),
                    Some(&[(BUCKET_LABEL, upper_bound)]),
                )?;
            }

            cumulative_count += bucket_count;
            self.writer.write_str(itoa::Buffer::new().format(cumulative_count))?;
            self.encode_timestamp()?;
            if let Some(exemplars) = exemplars {
                if let Some(exemplar) = exemplars[idx] {
                    exemplar.encode(&mut ExemplarEncoder { writer: self.writer })?;
                }
            }
            self.encode_newline()?;
        }
        Ok(())
    }

    fn encode_count(&mut self, count: u64) -> Result<()> {
        self.encode_metric_name()?;
        self.writer.write_str("_count")?;
        self.encode_label_set(None)?;
        self.writer.write_str(itoa::Buffer::new().format(count))?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    fn encode_sum(&mut self, sum: f64) -> Result<()> {
        self.encode_metric_name()?;
        self.writer.write_str("_sum")?;
        self.encode_label_set(None)?;
        self.writer.write_str(zmij::Buffer::new().format(sum))?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    fn encode_gcount(&mut self, gcount: u64) -> Result<()> {
        self.encode_metric_name()?;
        self.writer.write_str("_gcount")?;
        self.encode_label_set(None)?;
        self.writer.write_str(itoa::Buffer::new().format(gcount))?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    fn encode_gsum(&mut self, gsum: f64) -> Result<()> {
        self.encode_metric_name()?;
        self.writer.write_str("_gsum")?;
        self.encode_label_set(None)?;
        self.writer.write_str(zmij::Buffer::new().format(gsum))?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    fn encode_created(&mut self, created: Duration) -> Result<()> {
        self.encode_metric_name()?;
        self.writer.write_str("_created")?;
        self.encode_label_set(None)?;
        self.writer.write_fmt(format_args!(
            "{}.{}",
            created.as_secs(),
            created.as_millis() % 1000
        ))?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    #[inline]
    fn encode_timestamp(&mut self) -> Result<()> {
        if let Some(timestamp) = self.timestamp {
            self.writer.write_fmt(format_args!(
                " {}.{}",
                timestamp.as_secs(),
                timestamp.as_millis() % 1000
            ))?;
        }
        Ok(())
    }

    #[inline]
    fn encode_newline(&mut self) -> Result<()> {
        self.writer.write_str("\n")?;
        Ok(())
    }
}

impl<W> encoder::MetricEncoder for MetricEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode_unknown(&mut self, value: &dyn EncodeUnknownValue) -> Result<()> {
        self.encode_metric_name()?;
        self.encode_label_set(None)?;
        value.encode(&mut UnknownValueEncoder { writer: self.writer })?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    fn encode_gauge(&mut self, value: &dyn EncodeGaugeValue) -> Result<()> {
        self.encode_metric_name()?;
        self.encode_label_set(None)?;
        value.encode(&mut GaugeValueEncoder { writer: self.writer })?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    fn encode_counter(
        &mut self,
        total: &dyn EncodeCounterValue,
        exemplar: Option<&dyn EncodeExemplar>,
        created: Option<Duration>,
    ) -> Result<()> {
        // encode `*_total` metric
        self.encode_metric_name()?;
        self.writer.write_str("_total")?;
        self.encode_label_set(None)?;
        total.encode(&mut CounterValueEncoder { writer: self.writer })?;
        self.encode_timestamp()?;
        if let Some(exemplar) = exemplar {
            exemplar.encode(&mut ExemplarEncoder { writer: self.writer })?;
        }
        self.encode_newline()?;

        // encode `*_created` metric if available
        if let Some(created) = created {
            self.encode_created(created)?;
        }

        Ok(())
    }

    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> Result<()> {
        // pre-encode common labels once
        let common_labels = self.encode_common_labels_to_string()?;

        // encode state metrics
        for (state, enabled) in states {
            self.encode_metric_name()?;
            self.encode_label_set_with_common(
                common_labels.as_deref(),
                Some(&[(self.metric_name.clone(), state)]),
            )?;
            if enabled {
                self.writer.write_str("1")?;
            } else {
                self.writer.write_str("0")?;
            }
            self.encode_timestamp()?;
            self.encode_newline()?;
        }
        Ok(())
    }

    fn encode_info(&mut self, label_set: &dyn EncodeLabelSet) -> Result<()> {
        self.encode_metric_name()?;
        self.writer.write_str("_info")?;
        self.encode_label_set(Some(label_set))?;
        self.writer.write_str("1")?;
        self.encode_timestamp()?;
        self.encode_newline()
    }

    fn encode_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
        count: u64,
        sum: f64,
        created: Option<Duration>,
    ) -> Result<()> {
        // encode `*_bucket` metrics
        self.encode_buckets(buckets, exemplars)?;
        // encode `*_count` metric
        self.encode_count(count)?;
        // encode `*_sum` metric
        self.encode_sum(sum)?;

        // encode `*_created` metric if available
        if let Some(created) = created {
            self.encode_created(created)?;
        }

        Ok(())
    }

    fn encode_gauge_histogram(
        &mut self,
        buckets: &[Bucket],
        exemplars: Option<&[Option<&dyn EncodeExemplar>]>,
        count: u64,
        sum: f64,
    ) -> Result<()> {
        // encode `*_bucket` metrics
        self.encode_buckets(buckets, exemplars)?;
        // encode `*_gcount` metric
        self.encode_gcount(count)?;
        // encode `*_gsum` metric
        self.encode_gsum(sum)
    }

    fn encode_summary(
        &mut self,
        quantiles: &[Quantile],
        sum: f64,
        count: u64,
        created: Option<Duration>,
    ) -> Result<()> {
        // pre-encode common labels once
        let common_labels = self.encode_common_labels_to_string()?;

        // encode quantile metrics
        for quantile in quantiles {
            self.encode_metric_name()?;
            self.encode_label_set_with_common(
                common_labels.as_deref(),
                Some(&[(QUANTILE_LABEL, quantile.quantile())]),
            )?;
            self.writer.write_str(zmij::Buffer::new().format(quantile.value()))?;
            self.encode_timestamp()?;
            self.encode_newline()?;
        }

        // encode `*_count` metric
        self.encode_count(count)?;
        // encode `*_sum` metric
        self.encode_sum(sum)?;

        // encode `*_created` metric if available
        if let Some(created) = created {
            self.encode_created(created)?;
        }

        Ok(())
    }

    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> Result<()> {
        debug_assert!(self.family_labels.is_none(), "family labels already set");
        metric.encode(&mut MetricEncoder {
            writer: self.writer,
            metric_name: self.metric_name.clone(),
            metric_type: self.metric_type,
            timestamp: self.timestamp,
            const_labels: self.const_labels,
            family_labels: Some(label_set),
        })
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
    fn encode(&mut self, label: &dyn EncodeLabel) -> Result<()> {
        let first = self.first;
        self.first = false;
        label.encode(&mut LabelEncoder { writer: self.writer, first })
    }
}

struct LabelEncoder<'a, W> {
    writer: &'a mut W,
    first: bool,
}

macro_rules! encode_integer_value_impls {
    ($($integer:ty),*) => (
        paste::paste! { $(
            #[inline]
            fn [<encode_ $integer _value>](&mut self, value: $integer) -> Result<()> {
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
            #[inline]
            fn [<encode_ $float _value>](&mut self, value: $float) -> Result<()> {
                self.writer.write_str("=\"")?;
                self.writer.write_str(zmij::Buffer::new().format(value))?;
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
    #[inline]
    fn encode_label_name(&mut self, name: &str) -> Result<()> {
        if !self.first {
            self.writer.write_str(",")?;
        }
        self.writer.write_str(name)?;
        Ok(())
    }

    #[inline]
    fn encode_str_value(&mut self, value: &str) -> Result<()> {
        self.writer.write_str("=\"")?;
        self.writer.write_str(value)?;
        self.writer.write_str("\"")?;
        Ok(())
    }

    #[inline]
    fn encode_bool_value(&mut self, value: bool) -> Result<()> {
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
}

macro_rules! encode_integer_number_impls {
    ($($integer:ty),*) => (
        paste::paste! { $(
            #[inline]
            fn [<encode_ $integer>](&mut self, value: $integer) -> Result<()> {
                self.writer.write_str(itoa::Buffer::new().format(value))?;
                Ok(())
            }
        )* }
    )
}

macro_rules! encode_float_number_impls {
    ($($float:ty),*) => (
        paste::paste! { $(
            #[inline]
            fn [<encode_ $float>](&mut self, value: $float) -> Result<()> {
                self.writer.write_str(zmij::Buffer::new().format(value))?;
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
        i32, i64, isize, u32, u64
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

struct ExemplarEncoder<'a, W> {
    writer: &'a mut W,
}

impl<W> encoder::ExemplarEncoder for ExemplarEncoder<'_, W>
where
    W: fmt::Write,
{
    fn encode(
        &mut self,
        labels: &dyn EncodeLabelSet,
        value: f64,
        timestamp: Option<Duration>,
    ) -> Result<()> {
        // # { labels } value [timestamp]
        self.writer.write_str(" # {")?;
        labels.encode(&mut LabelSetEncoder::new(self.writer))?;
        self.writer.write_str("} ")?;

        self.writer.write_str(zmij::Buffer::new().format(value))?;

        if let Some(timestamp) = timestamp {
            self.writer.write_fmt(format_args!(
                " {}.{}",
                timestamp.as_secs(),
                timestamp.as_millis() % 1000
            ))?;
        }

        Ok(())
    }
}
