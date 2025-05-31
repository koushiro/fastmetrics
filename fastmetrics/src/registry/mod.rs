//! Registry module provides functionality for metric collection and organization.
//!
//! The registry is the central component that holds all metrics in an application.
//! It supports organizing metrics hierarchically using namespaces and subsystems,
//! and allows attaching constant labels to groups of metrics.
//!
//! See [`Registry`] for more details.

mod errors;
mod global;
mod register;

use std::{
    borrow::Cow,
    collections::hash_map::{self, HashMap},
};

pub use self::{errors::*, global::*, register::*};
pub use crate::raw::Unit;
use crate::{
    encoder::EncodeMetric,
    raw::{Metadata, MetricType},
};

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
/// #    registry::{Registry, RegistryError},
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
/// assert_eq!(db.namespace(), Some("myapp_database"));
/// assert_eq!(db.constant_labels(), [("env".into(), "prod".into())]);
///
/// // Register metrics into the database subsystem
/// let db_connections = <Gauge>::default();
/// db.register("connections", "Active database connections", db_connections.clone())?;
///
/// // Create a nested subsystem with additional constant labels
/// let mysql = db.subsystem_builder("mysql").with_const_labels([("engine", "innodb")]).build();
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
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    pub(crate) metrics: HashMap<Metadata, Box<dyn EncodeMetric + 'static>>,
    pub(crate) subsystems: HashMap<Cow<'static, str>, Registry>,
}

/// A builder for constructing [`Registry`] instances with custom configuration.
#[derive(Default)]
pub struct RegistryBuilder {
    namespace: Option<Cow<'static, str>>,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RegistryBuilder {
    /// Sets a `namespace` prefix for all metrics in the [`Registry`].
    ///
    /// # Note
    ///
    /// The namespace cannot be empty and must be in `snake_case` format,
    /// otherwise it will throw a panic.
    pub fn with_namespace(mut self, namespace: impl Into<Cow<'static, str>>) -> Self {
        let namespace = namespace.into();
        assert!(!namespace.is_empty(), "Namespace cannot be empty string");
        assert!(is_snake_case(&namespace), "Namespace must be in snake_case format");
        self.namespace = Some(namespace);
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
        metric: impl EncodeMetric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.register_metric(name, help, None, metric)
    }

    /// Registers a metric with the specified unit into [`Registry`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #     metrics::histogram::Histogram,
    /// #     raw::metadata::Unit,
    /// #     registry::{Registry, RegistryError},
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
        metric: impl EncodeMetric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        let name = name.into();
        match metric.metric_type() {
            MetricType::StateSet | MetricType::Info | MetricType::Unknown => {
                return Err(RegistryError::MustHaveAnEmptyUnitString { name: name.clone() });
            },
            _ => {},
        }
        self.register_metric(name, help, Some(unit), metric)
    }

    fn register_metric(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        help: impl Into<Cow<'static, str>>,
        unit: Option<Unit>,
        metric: impl EncodeMetric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        let name = name.into();
        if !is_snake_case(&name) {
            return Err(RegistryError::InvalidNameFormat { name: name.clone() });
        }

        match unit {
            Some(Unit::Other(unit)) if !is_lowercase(unit.as_ref()) => {
                return Err(RegistryError::OtherUnitFormatMustBeLowercase { unit: unit.clone() });
            },
            _ => {},
        }

        let metadata = Metadata::new(name.clone(), help, metric.metric_type(), unit);
        match self.metrics.entry(metadata) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(Box::new(metric));
                Ok(self)
            },
            hash_map::Entry::Occupied(_) => Err(RegistryError::AlreadyExists { name }),
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
    /// The name of subsystem should be in `snake_case` format, otherwise it will throw a panic.
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
    /// assert_eq!(subsystem1.namespace(), Some("myapp_subsystem1"));
    /// assert_eq!(subsystem1.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let subsystem2 = registry.subsystem("subsystem2");
    /// assert_eq!(subsystem2.namespace(), Some("myapp_subsystem2"));
    /// assert_eq!(subsystem2.constant_labels(), [("env".into(), "prod".into())]);
    ///
    /// let nested_subsystem = registry.subsystem("subsystem1").subsystem("subsystem2");
    /// assert_eq!(nested_subsystem.namespace(), Some("myapp_subsystem1_subsystem2"));
    /// assert_eq!(nested_subsystem.constant_labels(), [("env".into(), "prod".into())]);
    /// ```
    pub fn subsystem(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Registry {
        self.subsystem_builder(name).build()
    }

    /// Creates a builder for constructing a subsystem with custom configuration.
    ///
    /// This method provides more flexibility than [`subsystem`](Registry::subsystem) by allowing
    /// you to configure additional properties like constant labels specific to the subsystem.
    ///
    /// # Note
    ///
    /// The name of subsystem should be in `snake_case` format, otherwise it will throw a panic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::registry::Registry;
    /// let mut registry = Registry::builder()
    ///     .with_namespace("myapp")
    ///     .with_const_labels([("env", "prod")])
    ///     .build();
    ///
    /// let db = registry.subsystem("database");
    ///
    /// let mysql = db
    ///     .subsystem_builder("mysql")
    ///     .with_const_labels([("engine", "innodb")])
    ///     .build();
    ///
    /// assert_eq!(mysql.namespace(), Some("myapp_database_mysql"));
    /// assert_eq!(
    ///     mysql.constant_labels(),
    ///     [("env".into(), "prod".into()), ("engine".into(), "innodb".into())]
    /// );
    /// ```
    pub fn subsystem_builder(
        &mut self,
        name: impl Into<Cow<'static, str>>,
    ) -> RegistrySubsystemBuilder<'_> {
        let name = name.into();
        assert!(is_snake_case(&name), "subsystem name must be in snake_case format");
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
    /// # use fastmetrics::registry::Registry;
    /// let mut registry = Registry::builder()
    ///     .with_namespace("myapp")
    ///     .with_const_labels([("env", "prod")])
    ///     .build();
    ///
    /// let subsystem = registry
    ///     .subsystem_builder("cache")
    ///     .with_const_labels([("cache_type", "redis"), ("instance", "primary")])
    ///     .build();
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
    pub fn build(self) -> &'a mut Registry {
        let const_labels = match self.const_labels {
            Some(subsystem_const_labels) => {
                let mut merged = self.parent.const_labels.clone();
                merged.extend(subsystem_const_labels);
                merged
            },
            None => self.parent.const_labels.clone(),
        };

        self.parent.subsystems.entry(self.name.clone()).or_insert_with(|| {
            let namespace = match &self.parent.namespace {
                Some(namespace) => Cow::Owned(format!("{}_{}", namespace, self.name)),
                None => self.name,
            };
            Registry::builder()
                .with_namespace(namespace)
                .with_const_labels(const_labels)
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
        assert_eq!(subsystem1.namespace(), Some("myapp_subsystem1"));
        assert_eq!(subsystem1.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem2 = registry.subsystem("subsystem2");
        assert_eq!(subsystem2.namespace(), Some("myapp_subsystem2"));
        assert_eq!(subsystem2.constant_labels(), [("env".into(), "prod".into())]);

        let nested_subsystem = registry.subsystem("subsystem1").subsystem("subsystem2");
        assert_eq!(nested_subsystem.namespace(), Some("myapp_subsystem1_subsystem2"));
        assert_eq!(nested_subsystem.constant_labels(), [("env".into(), "prod".into())]);
    }

    #[test]
    fn test_registry_subsystem_with_const_labels() {
        let mut registry = Registry::builder()
            .with_namespace("myapp")
            .with_const_labels([("env", "prod")])
            .build();
        assert_eq!(registry.namespace(), Some("myapp"));
        assert_eq!(registry.constant_labels(), [("env".into(), "prod".into())]);

        let subsystem1 = registry
            .subsystem_builder("subsystem1")
            .with_const_labels([("name", "value")])
            .build();
        assert_eq!(subsystem1.namespace(), Some("myapp_subsystem1"));
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
