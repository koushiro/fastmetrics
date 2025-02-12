//! Registry module provides functionality for metric collection and organization.
//!
//! The registry is the central component that holds all metrics in an application.
//! It supports organizing metrics hierarchically using namespaces and subsystems,
//! and allows attaching constant labels to groups of metrics.
//!
//! # Examples
//!
//! ```rust
//! # use openmetrics_client::{
//! #     metrics::counter::Counter,
//! #     registry::{Registry, RegistryError},
//! # };
//! #
//! # fn main() -> Result<(), RegistryError> {
//!
//! let mut registry = Registry::default()
//!     .with_namespace("myapp")
//!     .with_const_labels([("env", "prod")]);
//!
//! // Create a subsystem for HTTP metrics
//! let http = registry.subsystem("http");
//! // Create a nested subsystem for HTTP server metrics
//! let server = http.subsystem("server");
//!
//! // Register metrics with automatic prefixing:
//! // This will create: myapp_http_server_requests_total
//! let http_requests = <Counter>::default();
//! server.register("requests", "Total HTTP requests", http_requests.clone())?;
//! # Ok(())
//! # }
//! ```

mod errors;

use std::borrow::Cow;

pub use self::errors::*;
use crate::{
    encoder::EncodeMetric,
    metrics::family::{Metadata, Unit},
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
/// # use openmetrics_client::{
/// #    metrics::{counter::Counter, gauge::Gauge},
/// #    registry::{Registry, RegistryError},
/// # };
/// #
/// # fn main() -> Result<(), RegistryError> {
/// let mut registry = Registry::default().with_namespace("myapp");
///
/// let uptime_seconds = <Gauge>::default();
/// // Create metrics in the main registry
/// registry.register("uptime_seconds", "Application uptime", uptime_seconds.clone())?;
///
/// let db_connections = <Gauge>::default();
/// // Create a subsystem for database metrics
/// let db = registry.subsystem("db");
/// db.register("connections", "Active database connections", db_connections.clone())?;
///
/// let mysql_queries = <Counter>::default();
/// // Create a nested subsystem with additional labels
/// let mysql = db.subsystem_with_labels(
///     "mysql",
///     [("engine", "innodb")]
/// );
/// mysql.register("queries", "Total MySQL queries", mysql_queries.clone())?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct Registry {
    namespace: Option<String>,
    pub(crate) const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    pub(crate) metrics: Vec<(Metadata, Box<dyn EncodeMetric + 'static>)>,
    pub(crate) subsystems: Vec<Registry>,
}

impl Registry {
    /// Sets the `namespace` of the [`Registry`].
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Sets the `constant labels` of the [`Registry`].
    ///
    /// **NOTE**:
    /// Constant labels are only used rarely.
    /// In particular, do not use them to attach the same labels to all your metrics.
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

    /// Registers a metric into [`Registry`].
    pub fn register(
        &mut self,
        name: impl Into<String>,
        help: impl Into<String>,
        metric: impl EncodeMetric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.do_register(name, help, None, metric)
    }

    /// Registers a metric with the specified unit into [`Registry`].
    pub fn register_with_unit(
        &mut self,
        name: impl Into<String>,
        help: impl Into<String>,
        unit: Unit,
        metric: impl EncodeMetric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.do_register(name, help, Some(unit), metric)
    }

    fn do_register(
        &mut self,
        name: impl Into<String>,
        help: impl Into<String>,
        unit: Option<Unit>,
        metric: impl EncodeMetric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        let metadata = Metadata::new(name, help, metric.metric_type(), unit);
        self.metrics.push((metadata, Box::new(metric)));
        Ok(self)
    }

    /// Creates a subsystem to register metrics with a given `subsystem` as a common prefix.
    pub fn subsystem(&mut self, subsystem: impl AsRef<str>) -> &mut Self {
        let subsystem_name = subsystem.as_ref();
        let namespace = match &self.namespace {
            Some(namespace) => format!("{}_{}", namespace, subsystem_name),
            None => subsystem_name.to_owned(),
        };

        let subsystem = Registry::default()
            .with_namespace(namespace)
            .with_const_labels(self.const_labels.clone());
        self.subsystems.push(subsystem);
        self.subsystems.last_mut().expect("subsystem must not be none")
    }

    /// Creates a subsystem to register metrics with a given `subsystem` as a common prefix and some
    /// additional `constant labels`.
    pub fn subsystem_with_labels<N, V>(
        &mut self,
        subsystem: impl AsRef<str>,
        additional_labels: impl IntoIterator<Item = (N, V)>,
    ) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let subsystem_name = subsystem.as_ref();
        let namespace = match &self.namespace {
            Some(namespace) => format!("{}_{}", namespace, subsystem_name),
            None => subsystem_name.to_owned(),
        };

        let additional_labels =
            additional_labels.into_iter().map(|(name, value)| (name.into(), value.into()));
        let mut new_const_labels = self.const_labels.clone();
        new_const_labels.extend(additional_labels);

        let subsystem = Registry::default()
            .with_namespace(namespace)
            .with_const_labels(new_const_labels);
        self.subsystems.push(subsystem);
        self.subsystems.last_mut().expect("subsystem must not be none")
    }

    /// Returns the current `namespace` of [`Registry`].
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }
}
