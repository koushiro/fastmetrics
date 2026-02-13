use std::{borrow::Cow, collections::HashMap, fmt, time::Duration};

use super::{
    config::{NamePolicy, ProfileConfig, TimestampFormat},
    names::{escape_label_name, escape_metric_name},
};
use crate::{
    encoder::{
        self, EncodeCounterValue, EncodeExemplar, EncodeGaugeValue, EncodeLabel, EncodeLabelSet,
        EncodeMetric, EncodeUnknownValue, MetricFamilyEncoder as _,
    },
    error::{Error, Result},
    raw::{
        Metadata, MetricType, Unit,
        bucket::{BUCKET_LABEL, Bucket},
        quantile::{QUANTILE_LABEL, Quantile},
    },
    registry::{NameRule, Registry},
};

pub(super) fn encode(
    writer: &mut impl fmt::Write,
    registry: &Registry,
    config: ProfileConfig,
) -> Result<()> {
    Encoder::new(writer, registry, config).encode()
}

struct Encoder<'a, W> {
    writer: &'a mut W,
    registry: &'a Registry,
    config: ProfileConfig,
}

impl<'a, W> Encoder<'a, W>
where
    W: fmt::Write,
{
    fn new(writer: &'a mut W, registry: &'a Registry, config: ProfileConfig) -> Self {
        Self { writer, registry, config }
    }

    fn encode(&mut self) -> Result<()> {
        // Registration validates canonical names. We only need extra checks in
        // lossy profile escaping modes where different UTF-8 names can collapse
        // to the same escaped identifier.
        let check_escaped_name_collisions =
            self.registry.name_rule() == NameRule::Utf8 && self.config.name_policy.is_lossy();

        if check_escaped_name_collisions {
            // mapping: escaped metric name => canonical metric name
            let mut escaped_to_canonical = HashMap::new();
            self.check_family_name_collisions(self.registry, &mut escaped_to_canonical)?;
        }

        self.encode_registry(self.registry, check_escaped_name_collisions)?;
        if self.config.emit_eof {
            self.encode_eof()?;
        }
        Ok(())
    }

    fn encode_registry(
        &mut self,
        registry: &Registry,
        check_label_name_collisions: bool,
    ) -> Result<()> {
        for (metadata, metric) in &registry.metrics {
            MetricFamilyEncoder {
                writer: self.writer,
                namespace: registry.namespace(),
                const_labels: registry.constant_labels(),
                config: self.config,
                check_label_name_collisions,
            }
            .encode(metadata, metric)?;
        }
        for subsystem in registry.subsystems.values() {
            self.encode_registry(subsystem, check_label_name_collisions)?;
        }
        Ok(())
    }

    fn encode_eof(&mut self) -> Result<()> {
        self.writer.write_str("# EOF\n")?;
        Ok(())
    }

    fn check_family_name_collisions(
        &self,
        registry: &Registry,
        escaped_to_canonical: &mut HashMap<String, String>,
    ) -> Result<()> {
        for (metadata, metric) in &registry.metrics {
            if metric.is_empty() {
                continue;
            }

            let canonical_name = metric_name(
                registry.namespace(),
                metadata.name(),
                metadata.unit(),
                self.config.append_unit_suffix,
            )
            .into_owned();
            let escaped_name = escape_metric_name(
                Cow::Borrowed(canonical_name.as_str()),
                self.config.name_policy,
            )?
            .into_owned();

            if let Some(existing_name) = escaped_to_canonical.get(&escaped_name) {
                if existing_name != &canonical_name {
                    return Err(Error::duplicated("metric family names collide after escaping")
                        .with_context("escaped_metric", &escaped_name)
                        .with_context("existing_metric", existing_name)
                        .with_context("conflicting_metric", &canonical_name));
                }
            } else {
                escaped_to_canonical.insert(escaped_name, canonical_name);
            }
        }

        for subsystem in registry.subsystems.values() {
            self.check_family_name_collisions(subsystem, escaped_to_canonical)?;
        }

        Ok(())
    }
}

struct MetricFamilyEncoder<'a, W> {
    writer: &'a mut W,
    namespace: Option<&'a str>,
    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
    config: ProfileConfig,
    check_label_name_collisions: bool,
}

impl<W> MetricFamilyEncoder<'_, W>
where
    W: fmt::Write,
{
    #[inline]
    fn encode_type(&mut self, metric_name: &str, ty: &str) -> Result<()> {
        self.writer.write_fmt(format_args!("# TYPE {metric_name} {ty}"))?;
        self.encode_newline()
    }

    #[inline]
    fn encode_help(&mut self, metric_name: &str, help: &str) -> Result<()> {
        self.writer.write_fmt(format_args!("# HELP {metric_name} "))?;
        self.encode_escaped_help(help)?;
        self.encode_newline()
    }

    #[inline]
    fn encode_unit(&mut self, metric_name: &str, unit: Option<&Unit>) -> Result<()> {
        if self.config.emit_unit {
            if let Some(unit) = unit {
                let unit = unit.as_str();
                self.writer.write_fmt(format_args!("# UNIT {metric_name} {unit}"))?;
                self.encode_newline()?;
            }
        }
        Ok(())
    }

    #[inline]
    fn encode_newline(&mut self) -> Result<()> {
        self.writer.write_str("\n")?;
        Ok(())
    }

    fn encode_escaped_help(&mut self, help: &str) -> Result<()> {
        let mut chars = help.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '\\' => match chars.peek().copied() {
                    Some('\\') | Some('"') | Some('n') => {
                        self.writer.write_char('\\')?;
                        self.writer.write_char(chars.next().expect("peeked help escape char"))?;
                    },
                    _ => self.writer.write_str("\\\\")?,
                },
                '\n' => self.writer.write_str("\\n")?,
                '"' => self.writer.write_str("\\\"")?,
                _ => self.writer.write_char(ch)?,
            }
        }

        Ok(())
    }
}

fn metric_name<'a>(
    namespace: Option<&str>,
    name: &'a str,
    unit: Option<&Unit>,
    append_unit_suffix: bool,
) -> Cow<'a, str> {
    if !append_unit_suffix {
        return match namespace {
            Some(namespace) => Cow::Owned(format!("{namespace}_{name}")),
            None => Cow::Borrowed(name),
        };
    }

    match (namespace, unit) {
        (Some(namespace), Some(unit)) => {
            Cow::Owned(format!("{namespace}_{}_{}", name, unit.as_str()))
        },
        (Some(namespace), None) => Cow::Owned(format!("{namespace}_{name}")),
        (None, Some(unit)) => Cow::Owned(format!("{name}_{}", unit.as_str())),
        (None, None) => Cow::Borrowed(name),
    }
}

fn metric_type_name(metric_type: MetricType, prometheus_type_compat: bool) -> Result<&'static str> {
    if !prometheus_type_compat {
        return Ok(metric_type.as_str());
    }

    match metric_type {
        MetricType::Unknown => Ok("untyped"),
        MetricType::Counter => Ok("counter"),
        MetricType::Gauge => Ok("gauge"),
        MetricType::Histogram => Ok("histogram"),
        MetricType::Summary => Ok("summary"),
        MetricType::StateSet => {
            Err(Error::unsupported("stateset is unsupported in Prometheus text profile"))
        },
        MetricType::Info => {
            Err(Error::unsupported("info is unsupported in Prometheus text profile"))
        },
        MetricType::GaugeHistogram => {
            Err(Error::unsupported("gaugehistogram is unsupported in Prometheus text profile"))
        },
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

        let metric_name = metric_name(
            self.namespace,
            metadata.name(),
            metadata.unit(),
            self.config.append_unit_suffix,
        );
        let canonical_metric_name = metric_name.clone();
        let metric_name = escape_metric_name(metric_name, self.config.name_policy)?;
        let ty = metric_type_name(metadata.metric_type(), self.config.prometheus_type_compat)?;

        self.encode_type(metric_name.as_ref(), ty)?;
        self.encode_help(metric_name.as_ref(), metadata.help())?;
        self.encode_unit(metric_name.as_ref(), metadata.unit())?;

        metric.encode(&mut MetricEncoder {
            writer: self.writer,
            metric_name,
            canonical_metric_name,
            metric_type: metadata.metric_type(),
            timestamp: metric.timestamp(),
            const_labels: self.const_labels,
            family_labels: None,
            config: self.config,
            check_label_name_collisions: self.check_label_name_collisions,
        })
    }
}

struct MetricEncoder<'a, W> {
    writer: &'a mut W,

    // Escaped [namespace_]name[_unit] used for rendering samples.
    metric_name: Cow<'a, str>,
    // Canonical [namespace_]name[_unit] before profile escaping.
    canonical_metric_name: Cow<'a, str>,
    metric_type: MetricType,
    timestamp: Option<Duration>,

    const_labels: &'a [(Cow<'static, str>, Cow<'static, str>)],
    family_labels: Option<&'a dyn EncodeLabelSet>,

    config: ProfileConfig,
    check_label_name_collisions: bool,
}

struct CommonLabels {
    encoded: String,
    // Escaped label names that were already emitted in `encoded`.
    // We keep this map only when lossy escaping collision checks are enabled,
    // so `encode_label_set_with_common` can perform incremental checks for
    // additional labels (for example `le`/`quantile`/stateset labels).
    escaped_to_canonical: Option<HashMap<String, String>>,
}

enum AdditionalLabelValue<'a> {
    Str(&'a str),
    F64(f64),
}

fn write_escaped_label_value(writer: &mut impl fmt::Write, value: &str) -> Result<()> {
    for ch in value.chars() {
        match ch {
            '\\' => writer.write_str("\\\\")?,
            '\n' => writer.write_str("\\n")?,
            '"' => writer.write_str("\\\"")?,
            _ => writer.write_char(ch)?,
        }
    }
    Ok(())
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

    fn encode_labels<T: fmt::Write>(
        writer: &mut T,
        labels: &dyn EncodeLabelSet,
        name_policy: NamePolicy,
        collision_existing: Option<&HashMap<String, String>>,
        collision_seen: Option<&mut HashMap<String, String>>,
    ) -> Result<()> {
        if let Some(collision_seen) = collision_seen {
            labels.encode(&mut LabelSetEncoder::new_with_collision_tracking(
                writer,
                name_policy,
                collision_existing,
                collision_seen,
            ))
        } else {
            labels.encode(&mut LabelSetEncoder::new(writer, name_policy))
        }
    }

    /// Pre-encode common labels (const_labels + family_labels) to a string buffer
    fn encode_common_labels_to_string(&self) -> Result<Option<CommonLabels>> {
        let has_const_labels = !self.const_labels.is_empty();
        let has_family_labels = matches!(self.family_labels, Some(labels) if !labels.is_empty());

        if !has_const_labels && !has_family_labels {
            return Ok(None);
        }

        let mut common_labels = String::new();
        // mapping: escaped label name => canonical label name
        let mut escaped_to_canonical = self.check_label_name_collisions.then(HashMap::new);

        if has_const_labels {
            Self::encode_labels(
                &mut common_labels,
                &self.const_labels,
                self.config.name_policy,
                None,
                escaped_to_canonical.as_mut(),
            )?;
        }

        if has_family_labels {
            if has_const_labels {
                common_labels.push(',');
            }

            Self::encode_labels(
                &mut common_labels,
                self.family_labels.expect("family_labels should be `Some` value"),
                self.config.name_policy,
                None,
                escaped_to_canonical.as_mut(),
            )?;
        }

        Ok(Some(CommonLabels { encoded: common_labels, escaped_to_canonical }))
    }

    /// Escape a single additional label name and check for collisions against
    /// already-encoded common labels when collision checking is enabled.
    fn prepare_additional_label_name<'n>(
        &self,
        common_labels: Option<&CommonLabels>,
        canonical_label_name: &'n str,
    ) -> Result<Cow<'n, str>> {
        let escaped_label_name = escape_label_name(canonical_label_name, self.config.name_policy)?;
        if self.check_label_name_collisions {
            if let Some(existing_name) = common_labels
                .and_then(|labels| labels.escaped_to_canonical.as_ref())
                .and_then(|existing| existing.get(escaped_label_name.as_ref()))
            {
                return Err(Error::duplicated("label names collide after escaping")
                    .with_context("escaped_label", escaped_label_name.as_ref())
                    .with_context("existing_label", existing_name)
                    .with_context("conflicting_label", canonical_label_name));
            }
        }
        Ok(escaped_label_name)
    }

    /// Encode label set with pre-computed common labels plus exactly one
    /// additional label, optimized for histogram/summary/stateset hot loops.
    fn encode_label_set_with_common(
        &mut self,
        common_labels: Option<&CommonLabels>,
        escaped_label_name: &str,
        value: AdditionalLabelValue<'_>,
    ) -> Result<()> {
        self.writer.write_str("{")?;

        if let Some(common_labels) = common_labels {
            self.writer.write_str(common_labels.encoded.as_str())?;
            self.writer.write_str(",")?;
        }

        self.writer.write_str(escaped_label_name)?;
        self.writer.write_str("=\"")?;
        match value {
            AdditionalLabelValue::Str(value) => write_escaped_label_value(self.writer, value)?,
            AdditionalLabelValue::F64(value) => {
                self.writer.write_str(zmij::Buffer::new().format(value))?;
            },
        }
        self.writer.write_str("\"} ")?;
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
        // mapping: escaped label name => canonical label name
        let mut collision_seen = self.check_label_name_collisions.then(HashMap::new);

        let mut wrote_any = false;
        if has_const_labels {
            Self::encode_labels(
                self.writer,
                &self.const_labels,
                self.config.name_policy,
                None,
                collision_seen.as_mut(),
            )?;
            wrote_any = true;
        }

        if has_family_labels {
            if wrote_any {
                self.writer.write_str(",")?;
            }

            Self::encode_labels(
                self.writer,
                self.family_labels.expect("family_labels should be `Some` value"),
                self.config.name_policy,
                None,
                collision_seen.as_mut(),
            )?;
            wrote_any = true;
        }

        if has_additional_labels {
            if wrote_any {
                self.writer.write_str(",")?;
            }

            Self::encode_labels(
                self.writer,
                additional_labels.expect("additional_labels should be `Some` value"),
                self.config.name_policy,
                None,
                collision_seen.as_mut(),
            )?;
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
        let escaped_bucket_label_name =
            self.prepare_additional_label_name(common_labels.as_ref(), BUCKET_LABEL)?;

        let mut cumulative_count = 0;
        for (idx, bucket) in buckets.iter().enumerate() {
            self.encode_metric_name()?;
            self.writer.write_str("_bucket")?;

            let upper_bound = bucket.upper_bound();
            let bucket_count = bucket.count();

            // use pre-computed common labels
            if upper_bound == f64::INFINITY {
                self.encode_label_set_with_common(
                    common_labels.as_ref(),
                    escaped_bucket_label_name.as_ref(),
                    AdditionalLabelValue::Str("+Inf"),
                )?;
            } else {
                self.encode_label_set_with_common(
                    common_labels.as_ref(),
                    escaped_bucket_label_name.as_ref(),
                    AdditionalLabelValue::F64(upper_bound),
                )?;
            }

            cumulative_count += bucket_count;
            self.writer.write_str(itoa::Buffer::new().format(cumulative_count))?;
            self.encode_timestamp()?;
            if self.config.emit_exemplars {
                if let Some(exemplars) = exemplars {
                    if let Some(exemplar) = exemplars[idx] {
                        exemplar.encode(&mut ExemplarEncoder {
                            writer: self.writer,
                            timestamp_format: self.config.timestamp_format,
                            name_policy: self.config.name_policy,
                            check_label_name_collisions: self.check_label_name_collisions,
                        })?;
                    }
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
            write_timestamp(self.writer, timestamp, self.config.timestamp_format)?;
        }
        Ok(())
    }

    #[inline]
    fn encode_newline(&mut self) -> Result<()> {
        self.writer.write_str("\n")?;
        Ok(())
    }
}

fn write_timestamp(
    writer: &mut impl fmt::Write,
    duration: Duration,
    format: TimestampFormat,
) -> Result<()> {
    match format {
        TimestampFormat::SecondsMillis => {
            writer.write_fmt(format_args!(
                " {}.{}",
                duration.as_secs(),
                duration.as_millis() % 1000
            ))?;
        },
        TimestampFormat::MillisecondsInteger => {
            writer.write_fmt(format_args!(" {}", duration.as_millis()))?;
        },
    }
    Ok(())
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
        self.encode_metric_name()?;
        if self.config.append_counter_total_suffix {
            self.writer.write_str("_total")?;
        }
        self.encode_label_set(None)?;
        total.encode(&mut CounterValueEncoder { writer: self.writer })?;
        self.encode_timestamp()?;
        if self.config.emit_exemplars {
            if let Some(exemplar) = exemplar {
                exemplar.encode(&mut ExemplarEncoder {
                    writer: self.writer,
                    timestamp_format: self.config.timestamp_format,
                    name_policy: self.config.name_policy,
                    check_label_name_collisions: self.check_label_name_collisions,
                })?;
            }
        }
        self.encode_newline()?;

        if self.config.emit_created_series {
            if let Some(created) = created {
                self.encode_created(created)?;
            }
        }

        Ok(())
    }

    fn encode_stateset(&mut self, states: Vec<(&str, bool)>) -> Result<()> {
        // pre-encode common labels once
        let common_labels = self.encode_common_labels_to_string()?;

        let canonical_state_label = self.canonical_metric_name.clone();
        let escaped_state_label = self.prepare_additional_label_name(
            common_labels.as_ref(),
            canonical_state_label.as_ref(),
        )?;

        for (state, enabled) in states {
            self.encode_metric_name()?;
            self.encode_label_set_with_common(
                common_labels.as_ref(),
                escaped_state_label.as_ref(),
                AdditionalLabelValue::Str(state),
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

        if self.config.emit_created_series {
            if let Some(created) = created {
                self.encode_created(created)?;
            }
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

        let escaped_quantile_label =
            self.prepare_additional_label_name(common_labels.as_ref(), QUANTILE_LABEL)?;

        // encode quantile metrics
        for quantile in quantiles {
            self.encode_metric_name()?;
            self.encode_label_set_with_common(
                common_labels.as_ref(),
                escaped_quantile_label.as_ref(),
                AdditionalLabelValue::F64(quantile.quantile()),
            )?;
            self.writer.write_str(zmij::Buffer::new().format(quantile.value()))?;
            self.encode_timestamp()?;
            self.encode_newline()?;
        }

        // encode `*_count` metric
        self.encode_count(count)?;
        // encode `*_sum` metric
        self.encode_sum(sum)?;

        if self.config.emit_created_series {
            if let Some(created) = created {
                self.encode_created(created)?;
            }
        }

        Ok(())
    }

    fn encode(&mut self, label_set: &dyn EncodeLabelSet, metric: &dyn EncodeMetric) -> Result<()> {
        debug_assert!(self.family_labels.is_none(), "family labels already set");
        metric.encode(&mut MetricEncoder {
            writer: self.writer,
            metric_name: self.metric_name.clone(),
            canonical_metric_name: self.canonical_metric_name.clone(),
            metric_type: self.metric_type,
            timestamp: self.timestamp,
            const_labels: self.const_labels,
            family_labels: Some(label_set),
            config: self.config,
            check_label_name_collisions: self.check_label_name_collisions,
        })
    }
}

struct LabelSetEncoder<'a, 'b, W> {
    writer: &'a mut W,
    first: bool,

    name_policy: NamePolicy,
    // `existing`: escaped names that were already encoded earlier
    // (used by the common-label fast path).
    collision_existing: Option<&'b HashMap<String, String>>,
    // `seen`: escaped names encoded in the current label segment.
    collision_seen: Option<&'b mut HashMap<String, String>>,
}

impl<'a, 'b, W> LabelSetEncoder<'a, 'b, W> {
    fn new(writer: &'a mut W, name_policy: NamePolicy) -> LabelSetEncoder<'a, 'b, W> {
        Self { writer, first: true, name_policy, collision_existing: None, collision_seen: None }
    }

    fn new_with_collision_tracking(
        writer: &'a mut W,
        name_policy: NamePolicy,
        collision_existing: Option<&'b HashMap<String, String>>,
        collision_seen: &'b mut HashMap<String, String>,
    ) -> LabelSetEncoder<'a, 'b, W> {
        Self {
            writer,
            first: true,
            name_policy,
            collision_existing,
            collision_seen: Some(collision_seen),
        }
    }
}

impl<W> encoder::LabelSetEncoder for LabelSetEncoder<'_, '_, W>
where
    W: fmt::Write,
{
    fn encode(&mut self, label: &dyn EncodeLabel) -> Result<()> {
        let first = self.first;
        self.first = false;
        let collision_guard =
            self.collision_seen
                .as_deref_mut()
                .map(|collision_seen| LabelNameCollisionGuard {
                    existing: self.collision_existing,
                    seen: collision_seen,
                });

        label.encode(&mut LabelEncoder {
            writer: self.writer,
            first,
            name_policy: self.name_policy,
            collision_guard,
        })
    }
}

struct LabelNameCollisionGuard<'a> {
    existing: Option<&'a HashMap<String, String>>,
    seen: &'a mut HashMap<String, String>,
}

impl LabelNameCollisionGuard<'_> {
    fn check_and_record(&mut self, canonical_name: &str, escaped_name: &str) -> Result<()> {
        if let Some(existing_name) = self.existing.and_then(|existing| existing.get(escaped_name)) {
            return Err(Error::duplicated("label names collide after escaping")
                .with_context("escaped_label", escaped_name)
                .with_context("existing_label", existing_name)
                .with_context("conflicting_label", canonical_name));
        }

        if let Some(existing_name) = self.seen.get(escaped_name) {
            return Err(Error::duplicated("label names collide after escaping")
                .with_context("escaped_label", escaped_name)
                .with_context("existing_label", existing_name)
                .with_context("conflicting_label", canonical_name));
        }

        self.seen.insert(escaped_name.to_owned(), canonical_name.to_owned());
        Ok(())
    }
}

struct LabelEncoder<'a, 'b, W> {
    writer: &'a mut W,
    first: bool,

    name_policy: NamePolicy,
    collision_guard: Option<LabelNameCollisionGuard<'b>>,
}

impl<W> LabelEncoder<'_, '_, W>
where
    W: fmt::Write,
{
    fn encode_escaped_label_value(&mut self, value: &str) -> Result<()> {
        write_escaped_label_value(self.writer, value)
    }
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

impl<W> encoder::LabelEncoder for LabelEncoder<'_, '_, W>
where
    W: fmt::Write,
{
    #[inline]
    fn encode_label_name(&mut self, name: &str) -> Result<()> {
        if !self.first {
            self.writer.write_str(",")?;
        }

        let escaped_name = escape_label_name(name, self.name_policy)?;

        if let Some(collision_guard) = self.collision_guard.as_mut() {
            collision_guard.check_and_record(name, escaped_name.as_ref())?;
        }

        self.writer.write_str(escaped_name.as_ref())?;
        Ok(())
    }

    #[inline]
    fn encode_str_value(&mut self, value: &str) -> Result<()> {
        self.writer.write_str("=\"")?;
        self.encode_escaped_label_value(value)?;
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
        i32, i64, isize
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
    timestamp_format: TimestampFormat,

    name_policy: NamePolicy,
    check_label_name_collisions: bool,
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

        if self.check_label_name_collisions {
            let mut collision_seen = HashMap::new();
            labels.encode(&mut LabelSetEncoder::new_with_collision_tracking(
                self.writer,
                self.name_policy,
                None,
                &mut collision_seen,
            ))?;
        } else {
            labels.encode(&mut LabelSetEncoder::new(self.writer, self.name_policy))?;
        }

        self.writer.write_str("} ")?;

        self.writer.write_str(zmij::Buffer::new().format(value))?;

        if let Some(timestamp) = timestamp {
            write_timestamp(self.writer, timestamp, self.timestamp_format)?;
        }

        Ok(())
    }
}
