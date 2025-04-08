//! Registry module provides functionality for metric collection and organization.
//!
//! The registry is the central component that holds all metrics in an application.
//! It supports organizing metrics hierarchically using namespaces and subsystems,
//! and allows attaching constant labels to groups of metrics.
//!
//! See [`Registry`] for more details.

mod errors;
mod subsystem;

use std::{
    borrow::Cow,
    collections::hash_map::{self, HashMap},
    fmt,
};

pub use self::{errors::*, subsystem::*};
use crate::{
    encoder::{EncodeMetric, MetricEncoder},
    metrics::{
        family::{Metadata, Unit},
        MetricType,
    },
};

/// The `Metric` trait is a marker trait that combines the encoding capability with thread safety
/// requirements needed for metrics that can be stored in the registry.
/// Metrics implementing this trait can:
///
/// - Be encoded into OpenMetrics/Prometheus format
/// - Be safely accessed from multiple threads
/// - Be registered with the `Registry` or `RegistrySystem`
///
/// This trait is automatically implemented for any type that implements
/// `EncodeMetric`, `Send`, and `Sync`.
pub trait Metric: EncodeMetric + Send + Sync {}

impl<T> Metric for T where T: EncodeMetric + Send + Sync {}

// https://github.com/rust-lang/rust/pull/134367
impl EncodeMetric for Box<dyn Metric> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        self.as_ref().encode(encoder)
    }

    fn metric_type(&self) -> MetricType {
        self.as_ref().metric_type()
    }
}

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
/// #    metrics::{counter::Counter, gauge::Gauge},
/// #    registry::{Registry, RegistryError, RegistrySystem},
/// # };
/// #
/// # fn main() -> Result<(), RegistryError> {
/// // Create a registry with a `myapp` namespace
/// let mut registry = Registry::builder()
///     .with_namespace("myapp")
///     .with_const_labels([("env", "prod")])
///     .build();
/// assert_eq!(registry.namespace(), Some("myapp"));
/// assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);
///
/// // Register metrics into the registry
/// let uptime_seconds = <Gauge>::default();
/// registry.register("uptime_seconds", "Application uptime", uptime_seconds.clone())?;
///
/// // Create a subsystem for database metrics
/// let db = registry.subsystem("database");
/// assert_eq!(db.namespace(), "myapp_database");
/// assert_eq!(db.constant_labels(), [("env".into(), "prod".into())]);
///
/// // Register metrics into the database subsystem
/// let db_connections = <Gauge>::default();
/// db.register("connections", "Active database connections", db_connections.clone())?;
///
/// // Create a nested subsystem with additional constant labels
/// let mysql = db.attach_subsystem(
///     RegistrySystem::builder("mysql").with_const_labels([("engine", "innodb")])
/// );
/// assert_eq!(mysql.namespace(), "myapp_database_mysql");
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
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    pub(crate) metrics: HashMap<Metadata, Box<dyn Metric + 'static>>,
    pub(crate) subsystems: HashMap<Cow<'static, str>, RegistrySystem>,
}

/// A builder for constructing [`Registry`] instances with custom configuration.
#[derive(Default)]
pub struct RegistryBuilder {
    namespace: Option<Cow<'static, str>>,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RegistryBuilder {
    /// Sets a `namespace` prefix for all metrics.
    pub fn with_namespace(mut self, namespace: impl Into<Cow<'static, str>>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Sets the `constant labels` that apply to all metrics in the registry.
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
    pub fn build(self) -> Registry {
        Registry {
            namespace: self.namespace,
            const_labels: self.const_labels,
            metrics: HashMap::default(),
            subsystems: HashMap::default(),
        }
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
}

// register
impl Registry {
    /// Registers a metric into [`Registry`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #    metrics::counter::Counter,
    /// #    registry::{Registry, RegistryError},
    /// # };
    /// #
    /// # fn main() -> Result<(), RegistryError> {
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
        metric: impl Metric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.do_register(name, help, None, metric)
    }

    /// Registers a metric with the specified unit into [`Registry`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #     metrics::{
    /// #         histogram::Histogram,
    /// #         family::Unit,
    /// #    },
    /// #    registry::{Registry, RegistryError},
    /// # };
    /// # fn main() -> Result<(), RegistryError> {
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
        unit: Unit,
        metric: impl Metric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        match metric.metric_type() {
            MetricType::StateSet | MetricType::Info | MetricType::Unknown => {
                return Err(RegistryError::MustHaveAnEmptyUnitString)
            },
            _ => {},
        }
        self.do_register(name, help, Some(unit), metric)
    }

    fn do_register(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        help: impl Into<Cow<'static, str>>,
        unit: Option<Unit>,
        metric: impl Metric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        let name = name.into();
        if !is_snake_case(&name) {
            return Err(RegistryError::InvalidNameFormat);
        }

        let metadata = Metadata::new(name, help, metric.metric_type(), unit);
        match self.metrics.entry(metadata) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(Box::new(metric));
                Ok(self)
            },
            hash_map::Entry::Occupied(_) => Err(RegistryError::AlreadyExists),
        }
    }
}

// subsystem
impl Registry {
    /// Creates a subsystem to register metrics with a subsystem `name`(as a part of prefix).
    /// If the subsystem `name` already exists, the previous created subsystem will be returned.
    ///
    /// # Note
    ///
    /// The name of subsystem should be `snake_case`, otherwise it will throw a panic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::registry::Registry;
    /// let mut registry = Registry::builder()
    ///     .with_namespace("myapp")
    ///     .with_const_labels([("env", "prod")])
    ///     .build();
    /// assert_eq!(registry.namespace(), Some("myapp"));
    /// assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let subsystem1 = registry.subsystem("subsystem1");
    /// assert_eq!(subsystem1.namespace(), "myapp_subsystem1");
    /// assert_eq!(subsystem1.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let subsystem2 = registry.subsystem("subsystem2");
    /// assert_eq!(subsystem2.namespace(), "myapp_subsystem2");
    /// assert_eq!(subsystem2.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let nested_subsystem = registry.subsystem("subsystem1").subsystem("subsystem2");
    /// assert_eq!(nested_subsystem.namespace(), "myapp_subsystem1_subsystem2");
    /// assert_eq!(nested_subsystem.constant_labels(), [("env".into(), "prod".into())]);
    /// ```
    pub fn subsystem(&mut self, name: impl Into<Cow<'static, str>>) -> &mut RegistrySystem {
        let name = name.into();
        self.subsystems.entry(name).or_insert_with_key(|name| {
            RegistrySystem::builder(name.clone())
                // inherit prefix from the registry
                .with_prefix(self.namespace.clone())
                // inherit constant labels from the registry
                .with_inherited_const_labels(self.const_labels.clone())
                .build()
        })
    }

    /// Attach a configurable subsystem.
    /// If the subsystem `name` already exists, the previous created subsystem will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::registry::{Registry, RegistrySystem};
    /// let mut registry = Registry::builder().with_namespace("myapp").build();
    /// assert_eq!(registry.namespace(), Some("myapp"));
    ///
    /// let subsystem1 = registry.attach_subsystem(
    ///     RegistrySystem::builder("subsystem1").with_const_labels([("name", "value")])
    /// );
    /// assert_eq!(subsystem1.namespace(), "myapp_subsystem1");
    /// assert_eq!(subsystem1.constant_labels(), [("name".into(), "value".into())]);
    /// ```
    pub fn attach_subsystem(&mut self, builder: RegistrySystemBuilder) -> &mut RegistrySystem {
        let name = builder.system_name.clone();
        self.subsystems.entry(name).or_insert_with(|| {
            builder
                // inherit prefix from the registry
                .with_prefix(self.namespace.clone())
                // inherit constant labels from the registry
                .with_inherited_const_labels(self.const_labels.clone())
                .build()
        })
    }
}

fn is_snake_case(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    match name.chars().next() {
        // first char shouldn't be ascii digit or '_'
        Some(first) if first.is_ascii_digit() || first == '_' => return false,
        _ => {},
    }

    // name shouldn't contain "__" and the suffix of name shouldn't be '_'
    if name.contains("__") || name.ends_with('_') {
        return false;
    }

    // all chars of name should match 'a'..='z' | '0'..='9' | '_'
    name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_lowercase(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // all chars of name should match 'a'..='z' | '0'..='9'
    name.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_subsystem() {
        let mut registry = Registry::builder()
            .with_namespace("myapp")
            .with_const_labels([("env", "prod")])
            .build();
        assert_eq!(registry.namespace(), Some("myapp"));
        assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem1 = registry.subsystem("subsystem1");
        assert_eq!(subsystem1.namespace(), "myapp_subsystem1");
        assert_eq!(subsystem1.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem2 = registry.subsystem("subsystem2");
        assert_eq!(subsystem2.namespace(), "myapp_subsystem2");
        assert_eq!(subsystem2.constant_labels(), [("env".into(), "prod".into())]);

        let nested_subsystem = registry.subsystem("subsystem1").subsystem("subsystem2");
        assert_eq!(nested_subsystem.namespace(), "myapp_subsystem1_subsystem2");
        assert_eq!(nested_subsystem.constant_labels(), [("env".into(), "prod".into())]);
    }

    #[test]
    fn test_registry_attach_subsystem() {
        let mut registry = Registry::builder()
            .with_namespace("myapp")
            .with_const_labels([("env", "prod")])
            .build();
        assert_eq!(registry.namespace(), Some("myapp"));
        assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem1 = registry.attach_subsystem(
            RegistrySystem::builder("subsystem1").with_const_labels([("name", "value")]),
        );
        assert_eq!(subsystem1.namespace(), "myapp_subsystem1");
        assert_eq!(
            subsystem1.constant_labels(),
            [("env".into(), "prod".into()), ("name".into(), "value".into())]
        );
    }

    #[test]
    fn test_is_snake_case() {
        let cases = vec!["name1", "name_1", "name_1_2"];
        for case in cases {
            assert!(is_snake_case(case));
        }

        let invalid_cases = vec!["_", "1name", "name__1", "name_", "name!"];
        for invalid_case in invalid_cases {
            assert!(!is_snake_case(invalid_case));
        }
    }

    #[test]
    fn test_is_lowercase() {
        let cases = vec!["name1", "1name", "na1me"];
        for case in cases {
            assert!(is_lowercase(case));
        }

        let invalid_cases = vec!["_", "name_", "name!"];
        for invalid_case in invalid_cases {
            assert!(!is_lowercase(invalid_case));
        }
    }
}
