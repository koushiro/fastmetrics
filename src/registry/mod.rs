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
};

pub use self::{errors::*, subsystem::*};
use crate::{
    encoder::EncodeMetric,
    metrics::family::{Metadata, Unit},
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
/// # use openmetrics_client::{
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
///
/// // Register metrics into the registry
/// let uptime_seconds = <Gauge>::default();
/// registry.register("uptime_seconds", "Application uptime", uptime_seconds.clone())?;
///
/// // Create a subsystem for database metrics
/// let db = registry.subsystem("database");
/// assert_eq!(db.namespace(), "myapp_database");
///
/// // Register metrics into the database subsystem
/// let db_connections = <Gauge>::default();
/// db.register("connections", "Active database connections", db_connections.clone())?;
///
/// // Create a nested subsystem with additional constant labels
/// let mysql = db.subsystem("mysql").with_additional_const_labels([("engine", "innodb")]);
/// assert_eq!(mysql.namespace(), "myapp_database_mysql");
///
/// // Register metrics into the mysql subsystem
/// let mysql_queries = <Counter>::default();
/// mysql.register("queries", "Total MySQL queries", mysql_queries.clone())?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct Registry {
    namespace: Option<String>,
    pub(crate) const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    pub(crate) metrics: HashMap<Metadata, Box<dyn Metric + 'static>>,
    pub(crate) subsystems: HashMap<String, RegistrySystem>,
}

/// A builder for constructing [`Registry`] instances with custom configuration.
#[derive(Default)]
pub struct RegistryBuilder {
    namespace: Option<String>,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RegistryBuilder {
    /// Sets a `namespace` prefix for all metrics.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
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

    /// Registers a metric into [`Registry`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use openmetrics_client::{
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
        name: impl Into<String>,
        help: impl Into<String>,
        metric: impl Metric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.do_register(name, help, None, metric)
    }

    /// Registers a metric with the specified unit into [`Registry`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use openmetrics_client::{
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
        name: impl Into<String>,
        help: impl Into<String>,
        unit: Unit,
        metric: impl Metric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.do_register(name, help, Some(unit), metric)
    }

    fn do_register(
        &mut self,
        name: impl Into<String>,
        help: impl Into<String>,
        unit: Option<Unit>,
        metric: impl Metric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        let metadata = Metadata::new(name, help, metric.metric_type(), unit);
        match self.metrics.entry(metadata) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(Box::new(metric));
                Ok(self)
            },
            hash_map::Entry::Occupied(_) => Err(RegistryError::AlreadyExists),
        }
    }

    /// Creates a subsystem to register metrics with a subsystem `name`(as a part of prefix).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use openmetrics_client::registry::Registry;
    /// let mut registry = Registry::builder().with_namespace("myapp").build();
    ///
    /// let subsystem1 = registry.subsystem("subsystem1");
    /// assert_eq!(subsystem1.namespace(), "myapp_subsystem1");
    ///
    /// let subsystem2 = registry.subsystem("subsystem2");
    /// assert_eq!(subsystem2.namespace(), "myapp_subsystem2");
    ///
    /// let nested_subsystem = registry.subsystem("subsystem1").subsystem("subsystem2");
    /// assert_eq!(nested_subsystem.namespace(), "myapp_subsystem1_subsystem2");
    /// ```
    pub fn subsystem(&mut self, name: impl Into<String>) -> &mut RegistrySystem {
        let name = name.into();
        self.subsystems.entry(name).or_insert_with_key(|name| {
            RegistrySystem::builder(name)
                .with_prefix(self.namespace.clone())
                .with_const_labels(self.const_labels.clone())
                .build()
        })
    }

    /// Returns the current `namespace` of [`Registry`].
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }
}
