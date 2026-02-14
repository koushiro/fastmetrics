//! Registry module provides functionality for metric collection and organization.
//!
//! The registry is the central component that holds all metrics in an application.
//! It supports organizing metrics hierarchically using namespaces and subsystems,
//! and allows attaching constant labels to groups of metrics.
//!
//! See [`Registry`] for more details.

mod global;
mod register;
mod validate;

use std::{
    borrow::Cow,
    collections::{
        HashSet,
        hash_map::{self, HashMap},
    },
};

pub(crate) use self::validate::{is_legacy_label_name, is_legacy_metric_name};
pub use self::{global::*, register::*, validate::NameRule};
pub use crate::raw::Unit;
use crate::{
    encoder::EncodeMetric,
    error::{Error, Result},
    raw::{
        LabelSetSchema, Metadata, MetricLabelSet, MetricType, TypedMetric, bucket::BUCKET_LABEL,
        quantile::QUANTILE_LABEL,
    },
    registry::validate::*,
};

/// Trait representing a metric that can be registered and encoded.
pub trait Metric: TypedMetric + MetricLabelSet + EncodeMetric + 'static {}
impl<T> Metric for T where T: TypedMetric + MetricLabelSet + EncodeMetric + 'static {}

/// A registry for collecting and organizing metrics.
///
/// The Registry type serves as a container for metrics and provides functionality to:
/// - Organize metrics using namespaces and subsystems
/// - Attach constant labels to groups of metrics
/// - Create hierarchical metric structures
/// - Register various types of metrics with optional units
///
/// # Namespaces and Subsystems
///
/// Metrics can be organized using:
/// - A namespace: top-level prefix for all metrics in the registry
/// - Subsystems: nested prefixes that create logical groupings
///
/// The final metric names will follow the pattern: `namespace_subsystem1_subsystem2_metric_name`.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::{
/// #    error::Result,
/// #    metrics::{counter::Counter, gauge::Gauge},
/// #    registry::Registry,
/// # };
/// #
/// # fn main() -> Result<()> {
/// // Create a registry with a `myapp` namespace
/// let mut registry = Registry::builder()
///     .with_namespace("myapp")
///     .with_const_labels([("env", "prod")])
///     .build()?;
/// assert_eq!(registry.namespace(), Some("myapp"));
/// assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);
///
/// // Register metrics into the registry
/// let uptime_seconds = <Gauge>::default();
/// registry.register("uptime_seconds", "Application uptime", uptime_seconds.clone())?;
///
/// // Create a subsystem for database metrics
/// let db = registry.subsystem("database")?;
/// assert_eq!(db.namespace(), Some("myapp_database"));
/// assert_eq!(db.constant_labels(), [("env".into(), "prod".into())]);
///
/// // Register metrics into the database subsystem
/// let db_connections = <Gauge>::default();
/// db.register("connections", "Active database connections", db_connections.clone())?;
///
/// // Create a nested subsystem with additional constant labels
/// let mysql = db.subsystem_builder("mysql").with_const_labels([("engine", "innodb")]).build()?;
/// assert_eq!(mysql.namespace(), Some("myapp_database_mysql"));
/// assert_eq!(
///     mysql.constant_labels(),
///     [("env".into(), "prod".into()), ("engine".into(), "innodb".into())],
/// );
///
/// // Register metrics into the mysql subsystem
/// let mysql_queries = <Counter>::default();
/// mysql.register("queries", "Total MySQL queries", mysql_queries.clone())?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct Registry {
    namespace: Option<Cow<'static, str>>,
    name_rule: NameRule,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    pub(crate) metrics: HashMap<Metadata, Box<dyn EncodeMetric + 'static>>,
    pub(crate) subsystems: HashMap<Cow<'static, str>, Registry>,
}

/// A builder for constructing [`Registry`] instances with custom configuration.
#[derive(Default)]
pub struct RegistryBuilder {
    namespace: Option<Cow<'static, str>>,
    name_rule: NameRule,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RegistryBuilder {
    /// Sets a `namespace` prefix for all metrics in the [`Registry`].
    ///
    /// # Note
    ///
    /// The namespace cannot be an empty string and must satisfy the OpenMetrics metric name rules.
    pub fn with_namespace(mut self, namespace: impl Into<Cow<'static, str>>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Sets the metric/label name rule at registration time.
    ///
    /// Defaults to [`NameRule::Legacy`].
    pub fn with_name_rule(mut self, rule: NameRule) -> Self {
        self.name_rule = rule;
        self
    }

    /// Sets the `constant labels` that apply to all metrics in the [`Registry`].
    ///
    /// **NOTE**: constant labels are rarely used.
    pub fn with_const_labels<N, V>(mut self, labels: impl IntoIterator<Item = (N, V)>) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.const_labels = labels
            .into_iter()
            .map(|(name, value)| (name.into(), value.into()))
            .collect::<Vec<_>>();
        self
    }

    /// Builds a [`Registry`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the namespace or constant labels are invalid.
    pub fn build(self) -> Result<Registry> {
        let namespace = if let Some(namespace) = self.namespace {
            if namespace.is_empty() {
                return Err(Error::invalid("namespace cannot be an empty string"));
            }
            match validate_metric_name_with_rule(namespace.as_ref(), true, self.name_rule) {
                Ok(()) => Some(namespace),
                Err(err) => {
                    return Err(
                        Error::invalid(err.to_string()).with_context("namespace", namespace)
                    );
                },
            }
        } else {
            None
        };

        validate_const_labels_config(&self.const_labels, self.name_rule)?;

        Ok(Registry {
            namespace,
            name_rule: self.name_rule,
            const_labels: self.const_labels,
            metrics: HashMap::default(),
            subsystems: HashMap::default(),
        })
    }
}

impl Registry {
    /// Creates a [`RegistryBuilder`] to build [`Registry`] instance.
    pub fn builder() -> RegistryBuilder {
        RegistryBuilder::default()
    }

    /// Returns the current `namespace` of [`Registry`].
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Returns the `constant labels` of [`Registry`].
    pub fn constant_labels(&self) -> &[(Cow<'static, str>, Cow<'static, str>)] {
        &self.const_labels
    }

    /// Returns the configured register-time name rule.
    pub fn name_rule(&self) -> NameRule {
        self.name_rule
    }
}

// register
impl Registry {
    /// Registers a metric without a unit into [`Registry`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #    error::Result,
    /// #    metrics::counter::Counter,
    /// #    registry::Registry,
    /// # };
    /// #
    /// # fn main() -> Result<()> {
    /// let mut registry = Registry::default();
    ///
    /// let http_request_total = <Counter>::default();
    /// registry.register(
    ///     "http_request",
    ///     "Total number of HTTP requests",
    ///     http_request_total.clone()
    /// )?;
    /// // update the metric
    /// // ...
    /// # Ok(())
    /// # }
    /// ```
    pub fn register(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        help: impl Into<Cow<'static, str>>,
        metric: impl Metric,
    ) -> Result<&mut Self> {
        self.register_metric(name, help, None::<Unit>, metric)
    }

    /// Registers a metric with the specified unit into [`Registry`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #    error::Result,
    /// #     metrics::histogram::Histogram,
    /// #     raw::metadata::Unit,
    /// #     registry::Registry,
    /// # };
    /// #
    /// # fn main() -> Result<()> {
    /// let mut registry = Registry::default();
    ///
    /// let http_request_duration_seconds = Histogram::default();
    /// registry.register_with_unit(
    ///     "http_request_duration",
    ///     "Histogram of time spent during HTTP requests",
    ///     Unit::Seconds,
    ///     http_request_duration_seconds.clone()
    /// )?;
    /// // update the metric
    /// // ...
    /// # Ok(())
    /// # }
    /// ```
    pub fn register_with_unit(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        help: impl Into<Cow<'static, str>>,
        unit: impl Into<Unit>,
        metric: impl Metric,
    ) -> Result<&mut Self> {
        self.register_metric(name, help, Some(unit), metric)
    }

    /// Registers a metric with an optional unit into [`Registry`].
    ///
    /// This is the most flexible registration method that allows specifying an optional unit.
    /// Use [`Registry::register`] for metrics without units or [`Registry::register_with_unit`]
    /// for metrics with units unless you need the flexibility of optional units.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #     error::Result,
    /// #     metrics::{counter::Counter, histogram::Histogram},
    /// #     registry::{Registry, Unit},
    /// # };
    /// #
    /// # fn main() -> Result<()> {
    /// let mut registry = Registry::default();
    ///
    /// // Register without a unit
    /// let counter = <Counter>::default();
    /// registry.register_metric("requests", "Total requests", None::<Unit>, counter)?;
    ///
    /// // Register with the unit
    /// let histogram = Histogram::default();
    /// registry.register_metric("duration", "Request duration", Some(Unit::Seconds), histogram)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn register_metric<M: Metric>(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        help: impl Into<Cow<'static, str>>,
        unit: Option<impl Into<Unit>>,
        metric: M,
    ) -> Result<&mut Self> {
        // Check the metric name
        let name: Cow<'static, str> = name.into();
        validate_metric_name_with_rule(&name, self.namespace().is_none(), self.name_rule)
            .map_err(|err| Error::invalid(err.to_string()).with_context("metric", &name))?;

        // Check metric help text
        let help = help.into();
        validate_help_text(&help).map_err(|err| {
            Error::invalid(err.to_string())
                .with_context("metric", &name)
                .with_context("help", &help)
        })?;

        // Check the metric unit format
        let unit = unit.map(Into::into);
        let metric_type = <M as TypedMetric>::TYPE;
        if let Some(Unit::Other(unit)) = unit.as_ref() {
            validate_unit(unit.as_ref()).map_err(|err| {
                Error::invalid(err.to_string())
                    .with_context("metric", &name)
                    .with_context("unit", unit)
            })?;

            // Check if metric type requires empty unit
            match metric_type {
                MetricType::StateSet | MetricType::Info | MetricType::Unknown => {
                    return Err(Error::invalid("metric must have an empty unit string")
                        .with_context("metric", name)
                        .with_context("type", metric_type)
                        .with_context("unit", unit));
                },
                _ => {},
            }
        }

        let reserved_label_reason = |name: &str| -> Option<String> {
            match metric_type {
                MetricType::Histogram | MetricType::GaugeHistogram if name == BUCKET_LABEL => {
                    Some(format!("label name '{name}' is reserved for '{metric_type}' type"))
                },
                MetricType::Summary if name == QUANTILE_LABEL => {
                    Some(format!("label name '{name}' is reserved for '{metric_type}' type"))
                },
                _ => None,
            }
        };

        // Prepare the constant metric labels
        let mut const_label_names = HashSet::new();
        for (name, _) in self.const_labels.iter() {
            if let Some(reason) = reserved_label_reason(name.as_ref()) {
                return Err(Error::invalid(reason).with_context("label", name));
            }
            const_label_names.insert(name.as_ref());
        }

        // Check the variable metric labels
        let mut variable_label_names = HashSet::new();
        if let Some(names) = <M::LabelSet as LabelSetSchema>::names() {
            for name in names.iter().copied() {
                if let Err(err) = validate_label_name_with_rule(name, self.name_rule) {
                    return Err(Error::invalid(err.to_string()).with_context("label", name));
                }

                if let Some(reason) = reserved_label_reason(name) {
                    return Err(Error::invalid(reason).with_context("label", name));
                }

                if const_label_names.contains(name) {
                    return Err(Error::invalid("label name conflicts with a constant label")
                        .with_context("label", name));
                }

                if !variable_label_names.insert(name) {
                    return Err(Error::invalid("duplicate label name in variable labels")
                        .with_context("label", name));
                }
            }
        }

        let metadata = Metadata::new(name.clone(), help.clone(), metric_type, unit);
        match self.metrics.entry(metadata) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(Box::new(metric));
                Ok(self)
            },
            hash_map::Entry::Occupied(_) => {
                Err(Error::duplicated("metric already exists").with_context("metric", name))
            },
        }
    }
}

// subsystem
impl Registry {
    /// Creates a subsystem to register metrics with a subsystem `name` (as a part of prefix).
    /// If the subsystem `name` already exists, the previous created subsystem will be returned.
    ///
    /// # Note
    ///
    /// The subsystem name cannot be an empty string and must satisfy the OpenMetrics metric name
    /// rules.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #     error::Result,
    /// #     registry::Registry
    /// # };
    /// #
    /// # fn main() -> Result<()> {
    /// let mut registry = Registry::builder()
    ///     .with_namespace("myapp")
    ///     .with_const_labels([("env", "prod")])
    ///     .build()?;
    /// assert_eq!(registry.namespace(), Some("myapp"));
    /// assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let subsystem1 = registry.subsystem("subsystem1")?;
    /// assert_eq!(subsystem1.namespace(), Some("myapp_subsystem1"));
    /// assert_eq!(subsystem1.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let subsystem2 = registry.subsystem("subsystem2")?;
    /// assert_eq!(subsystem2.namespace(), Some("myapp_subsystem2"));
    /// assert_eq!(subsystem2.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let nested_subsystem =
    ///     registry.subsystem("subsystem1")?.subsystem("subsystem2")?;
    /// assert_eq!(nested_subsystem.namespace(), Some("myapp_subsystem1_subsystem2"));
    /// assert_eq!(nested_subsystem.constant_labels(), [("env".into(), "prod".into())]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn subsystem(&mut self, name: impl Into<Cow<'static, str>>) -> Result<&mut Registry> {
        self.subsystem_builder(name).build()
    }

    /// Creates a builder for constructing a subsystem with custom configuration.
    ///
    /// This method provides more flexibility than [`subsystem`](Registry::subsystem) by allowing
    /// you to configure additional properties like constant labels specific to the subsystem.
    ///
    /// # Note
    ///
    /// The subsystem name cannot be an empty string and must satisfy the OpenMetrics metric name
    /// rules.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #     error::Result,
    /// #     registry::Registry
    /// # };
    /// #
    /// # fn main() -> Result<()> {
    /// let mut registry = Registry::builder()
    ///     .with_namespace("myapp")
    ///     .with_const_labels([("env", "prod")])
    ///     .build()?;
    ///
    /// let db = registry.subsystem("database")?;
    ///
    /// let mysql = db
    ///     .subsystem_builder("mysql")
    ///     .with_const_labels([("engine", "innodb")])
    ///     .build()?;
    ///
    /// assert_eq!(mysql.namespace(), Some("myapp_database_mysql"));
    /// assert_eq!(
    ///     mysql.constant_labels(),
    ///     [("env".into(), "prod".into()), ("engine".into(), "innodb".into())]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn subsystem_builder(
        &mut self,
        name: impl Into<Cow<'static, str>>,
    ) -> RegistrySubsystemBuilder<'_> {
        let name = name.into();
        RegistrySubsystemBuilder::new(self, name)
    }
}

/// A builder for constructing subsystems with custom configuration.
///
/// This builder allows you to create subsystems with additional constant labels
/// beyond those inherited from the parent registry. The subsystem will inherit
/// the parent's namespace and constant labels, with any additional labels specified
/// through this builder being merged in.
pub struct RegistrySubsystemBuilder<'a> {
    parent: &'a mut Registry,
    name: Cow<'static, str>,
    const_labels: Option<Vec<(Cow<'static, str>, Cow<'static, str>)>>,
}

impl<'a> RegistrySubsystemBuilder<'a> {
    fn new(parent: &'a mut Registry, name: Cow<'static, str>) -> RegistrySubsystemBuilder<'a> {
        Self { parent, name, const_labels: None }
    }

    /// Sets additional constant labels for the subsystem.
    ///
    /// These labels will be merged with the parent registry's constant labels.
    /// If there are any label key conflicts, the subsystem's labels will take precedence.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #     error::Result,
    /// #     registry::Registry
    /// # };
    /// #
    /// # fn main() -> Result<()> {
    /// let mut registry = Registry::builder()
    ///     .with_namespace("myapp")
    ///     .with_const_labels([("env", "prod")])
    ///     .build()?;
    ///
    /// let subsystem = registry
    ///     .subsystem_builder("database")
    ///     .with_const_labels([("engine", "innodb"), ("instance", "primary")])
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_const_labels<N, V>(mut self, labels: impl IntoIterator<Item = (N, V)>) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let labels = labels
            .into_iter()
            .map(|(name, value)| (name.into(), value.into()))
            .collect::<Vec<_>>();
        self.const_labels = Some(labels);
        self
    }

    /// Builds and returns a mutable reference to the subsystem.
    ///
    /// If a subsystem with the same name already exists, this will return a reference
    /// to the existing subsystem. Otherwise, it creates a new subsystem with the
    /// configured properties.
    ///
    /// The resulting subsystem will have:
    /// - A namespace combining the parent's namespace with the subsystem name
    /// - Constant labels merged from parent and subsystem-specific labels
    pub fn build(self) -> Result<&'a mut Registry> {
        let RegistrySubsystemBuilder { parent, name, const_labels } = self;

        // Check if the subsystem name is valid
        if name.is_empty() {
            return Err(Error::invalid("subsystem name cannot be an empty string"));
        }
        validate_metric_name_with_rule(&name, parent.namespace.is_none(), parent.name_rule)
            .map_err(|err| Error::invalid(err.to_string()).with_context("subsystem", &name))?;

        match parent.subsystems.entry(name.clone()) {
            hash_map::Entry::Occupied(entry) => match const_labels {
                None => Ok(entry.into_mut()),
                Some(subsystem_const_labels) => {
                    validate_const_labels_config(&subsystem_const_labels, parent.name_rule)?;

                    let merged = merge_const_labels(&parent.const_labels, subsystem_const_labels);

                    let existing_const_labels = entry.get().constant_labels();
                    if merged.as_slice() != existing_const_labels {
                        return Err(Error::invalid(
                            "subsystem already exists with different constant labels",
                        )
                        .with_context("subsystem", &name)
                        .with_context("existing_const_labels", format!("{existing_const_labels:?}"))
                        .with_context("requested_const_labels", format!("{merged:?}")));
                    }

                    Ok(entry.into_mut())
                },
            },
            hash_map::Entry::Vacant(entry) => {
                // Handle namespace of subsystem
                let namespace = match &parent.namespace {
                    Some(namespace) => Cow::Owned(format!("{namespace}_{name}")),
                    None => name,
                };

                // Handle constant labels of this subsystem
                let const_labels = match const_labels {
                    Some(subsystem_const_labels) => {
                        validate_const_labels_config(&subsystem_const_labels, parent.name_rule)?;
                        merge_const_labels(&parent.const_labels, subsystem_const_labels)
                    },
                    None => parent.const_labels.clone(),
                };

                let registry = Registry::builder()
                    .with_namespace(namespace)
                    .with_name_rule(parent.name_rule)
                    .with_const_labels(const_labels)
                    .build()?;

                Ok(entry.insert(registry))
            },
        }
    }
}

fn merge_const_labels(
    parent_labels: &[(Cow<'static, str>, Cow<'static, str>)],
    subsystem_const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
    // If there are any label key conflicts,
    // the subsystem's labels will take precedence.
    let mut merged = parent_labels.to_vec();
    for (new_key, new_value) in subsystem_const_labels {
        if let Some(pos) = merged.iter().position(|(key, _)| key == &new_key) {
            merged[pos] = (new_key, new_value);
        } else {
            merged.push((new_key, new_value));
        }
    }
    merged
}

fn validate_const_labels_config(
    const_labels: &[(Cow<'static, str>, Cow<'static, str>)],
    name_rule: NameRule,
) -> Result<()> {
    let mut names = HashSet::new();

    for (name, _) in const_labels.iter() {
        validate_label_name_with_rule(name.as_ref(), name_rule)
            .map_err(|err| Error::invalid(err.to_string()).with_context("label", name))?;

        if !names.insert(name.clone()) {
            return Err(Error::invalid("duplicate label name in constant labels")
                .with_context("label", name));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::{encoder::MetricEncoder, error::ErrorKind};

    #[test]
    fn test_registry_subsystem() -> Result<()> {
        let mut registry = Registry::builder()
            .with_namespace("myapp")
            .with_const_labels([("env", "prod")])
            .build()?;
        assert_eq!(registry.namespace(), Some("myapp"));
        assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem1 = registry.subsystem("subsystem1")?;
        assert_eq!(subsystem1.namespace(), Some("myapp_subsystem1"));
        assert_eq!(subsystem1.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem2 = registry.subsystem("subsystem2")?;
        assert_eq!(subsystem2.namespace(), Some("myapp_subsystem2"));
        assert_eq!(subsystem2.constant_labels(), [("env".into(), "prod".into())]);

        let nested_subsystem = registry.subsystem("subsystem1").unwrap().subsystem("subsystem2")?;
        assert_eq!(nested_subsystem.namespace(), Some("myapp_subsystem1_subsystem2"));
        assert_eq!(nested_subsystem.constant_labels(), [("env".into(), "prod".into())]);

        Ok(())
    }

    #[test]
    fn test_name_rule_default_is_legacy() {
        let registry = Registry::default();
        assert_eq!(registry.name_rule(), NameRule::Legacy);
    }

    #[test]
    fn test_legacy_mode_rejects_utf8_metric_name() {
        let mut registry = Registry::default();
        let result = registry.register("指标", "help", DummyCounter);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::Invalid);
        }
    }

    #[test]
    fn test_utf8_mode_accepts_utf8_names() -> Result<()> {
        let mut registry = Registry::builder()
            .with_name_rule(NameRule::Utf8)
            .with_const_labels([("标签", "值")])
            .build()?;

        registry.register("指标", "help", DummyCounter)?;
        Ok(())
    }

    #[test]
    fn test_registry_subsystem_with_const_labels() -> Result<()> {
        let mut registry = Registry::builder()
            .with_namespace("myapp")
            .with_const_labels([("env", "prod")])
            .build()?;
        assert_eq!(registry.namespace(), Some("myapp"));
        assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem1 = registry
            .subsystem_builder("subsystem1")
            .with_const_labels([("name", "value")])
            .build()?;
        assert_eq!(subsystem1.namespace(), Some("myapp_subsystem1"));
        assert_eq!(
            subsystem1.constant_labels(),
            [("env".into(), "prod".into()), ("name".into(), "value".into())]
        );

        Ok(())
    }

    #[test]
    fn test_subsystem_const_labels_override() -> Result<()> {
        let mut registry = Registry::builder()
            .with_namespace("myapp")
            .with_const_labels([("env", "dev"), ("region", "us-west")])
            .build()?;

        let subsystem = registry
            .subsystem_builder("cache")
            .with_const_labels([("env", "prod"), ("type", "redis")])
            .build()?;

        let labels = subsystem.constant_labels();

        assert_eq!(labels.iter().filter(|(k, _)| k == "env").count(), 1);
        assert_eq!(labels.len(), 3);

        assert!(labels.iter().any(|(k, v)| k == "env" && v == "prod"));
        assert!(labels.iter().any(|(k, v)| k == "region" && v == "us-west"));
        assert!(labels.iter().any(|(k, v)| k == "type" && v == "redis"));

        Ok(())
    }

    #[test]
    fn test_subsystem_const_labels_validation() -> Result<()> {
        let mut registry = Registry::builder().with_namespace("myapp").build()?;

        let result = registry
            .subsystem_builder("cache")
            .with_const_labels([("1invalid", "value")])
            .build();

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_subsystem_const_labels_existing_behaviors() -> Result<()> {
        let mut registry = Registry::builder().with_namespace("myapp").build()?;
        registry
            .subsystem_builder("cache")
            .with_const_labels([("role", "primary")])
            .build()?;

        // Existing subsystem with invalid constant labels should error
        let result_invalid = registry
            .subsystem_builder("cache")
            .with_const_labels([("1invalid", "value")])
            .build();
        assert!(result_invalid.is_err());

        // Existing subsystem with different constant labels should error
        let result_mismatch = registry
            .subsystem_builder("cache")
            .with_const_labels([("role", "secondary")])
            .build();
        assert!(result_mismatch.is_err());

        // Same subsystem with identical constant labels should be reused
        let result_same = registry
            .subsystem_builder("cache")
            .with_const_labels([("role", "primary")])
            .build();
        assert!(result_same.is_ok());

        Ok(())
    }

    #[test]
    fn test_subsystem_accepts_numeric_segment() -> Result<()> {
        let mut registry = Registry::builder().with_namespace("myapp").build()?;

        let subsystem = registry.subsystem("123cache")?;
        assert_eq!(subsystem.namespace(), Some("myapp_123cache"));

        assert!(subsystem.register("hits_total", "Total hits", DummyCounter).is_ok());
        Ok(())
    }

    #[test]
    fn test_root_subsystem_requires_initial_char() {
        let mut registry = Registry::default();
        assert!(registry.subsystem("123cache").is_err());
    }

    pub(crate) struct DummyCounter;

    impl TypedMetric for DummyCounter {
        const TYPE: MetricType = MetricType::Counter;
    }

    impl MetricLabelSet for DummyCounter {
        type LabelSet = ();
    }

    impl EncodeMetric for DummyCounter {
        fn encode(&self, _encoder: &mut dyn MetricEncoder) -> Result<()> {
            Ok(())
        }

        fn timestamp(&self) -> Option<Duration> {
            None
        }
    }

    #[test]
    fn test_register_same_metric() -> Result<()> {
        let mut registry = Registry::default();

        // Register first counter
        registry.register("my_dummy_counter", "", DummyCounter)?;

        // Try to register another counter with the same name and type - this will fail
        let result = registry.register("my_dummy_counter", "Another dummy counter", DummyCounter);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::Duplicated);
            assert_eq!(err.message(), "metric already exists");
        }

        Ok(())
    }

    #[test]
    fn test_custom_unit_accepts_metricname_chars() {
        let mut registry = Registry::default();

        assert!(
            registry
                .register_metric(
                    "custom_unit_metricname_chars",
                    "help",
                    Some(Unit::Other("foo:bar_123".into())),
                    DummyCounter,
                )
                .is_ok()
        );
    }
}
