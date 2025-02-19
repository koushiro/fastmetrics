use std::borrow::Cow;

use crate::{
    encoder::EncodeMetric,
    metrics::family::{Metadata, Unit},
    registry::RegistryError,
};

/// A subsystem within a registry that provides metric organization and labeling.
///
/// RegistrySystem represents a logical grouping of metrics within a registry. It allows:
///
/// - Hierarchical organization of metrics using nested subsystems
/// - Adding constant labels specific to the subsystem
/// - Automatic prefix handling for metric names
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
/// assert_eq!(registry.namespace(), None);
///
/// // Create a subsystem for database metrics
/// let db = registry.subsystem("database");
/// assert_eq!(db.namespace(), "database");
///
/// // Register metrics into the subsystem
/// let queries = <Counter>::default();
/// db.register("queries_total", "Total database queries", queries.clone())?;
///
/// // Update metrics
/// queries.inc();
/// # Ok(())
/// # }
/// ```
pub struct RegistrySystem {
    // namespace: prefix + system_name
    pub(crate) namespace: String,
    pub(crate) const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,

    pub(crate) metrics: Vec<(Metadata, Box<dyn EncodeMetric + 'static>)>,
    pub(crate) subsystems: Vec<RegistrySystem>,
}

/// A builder for constructing [`RegistrySystem`] instances with custom configuration.
pub(crate) struct RegistrySystemBuilder {
    prefix: Option<String>,
    system_name: String,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl RegistrySystemBuilder {
    fn new(system_name: impl Into<String>) -> Self {
        Self { prefix: None, system_name: system_name.into(), const_labels: vec![] }
    }

    pub(crate) fn with_prefix(mut self, prefix: Option<impl Into<String>>) -> Self {
        self.prefix = prefix.map(|prefix| prefix.into());
        self
    }

    pub(crate) fn with_const_labels<N, V>(
        mut self,
        labels: impl IntoIterator<Item = (N, V)>,
    ) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.const_labels =
            labels.into_iter().map(|(name, value)| (name.into(), value.into())).collect();
        self
    }

    pub(crate) fn build(self) -> RegistrySystem {
        let namespace = match self.prefix {
            Some(prefix) => format!("{}_{}", prefix, self.system_name),
            None => self.system_name,
        };
        RegistrySystem {
            namespace,
            const_labels: self.const_labels,
            metrics: vec![],
            subsystems: vec![],
        }
    }
}

impl RegistrySystem {
    pub(crate) fn builder(system_name: impl Into<String>) -> RegistrySystemBuilder {
        RegistrySystemBuilder::new(system_name)
    }

    /// Adds additional `constant labels` into this subsystem.
    ///
    /// This method allows you to add constant labels specific to this subsystem,
    /// which will be included with all metrics registered in this subsystem and
    /// its child subsystems.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use openmetrics_client::registry::Registry;
    /// let mut registry = Registry::default();
    /// let db = registry.subsystem("database")
    ///     .with_additional_const_labels([
    ///         ("engine", "mysql"),
    ///         ("version", "8.0")
    ///     ]);
    /// ```
    pub fn with_additional_const_labels<N, V>(
        &mut self,
        additional_labels: impl IntoIterator<Item = (N, V)>,
    ) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let additional_labels = additional_labels
            .into_iter()
            .map(|(name, value)| (name.into(), value.into()))
            .collect::<Vec<_>>();
        self.const_labels.extend(additional_labels);
        self
    }

    /// Registers a metric into [`RegistrySystem`].
    pub fn register(
        &mut self,
        name: impl Into<String>,
        help: impl Into<String>,
        metric: impl EncodeMetric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.do_register(name, help, None, metric)
    }

    /// Registers a metric with the specified unit into [`RegistrySystem`].
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

    /// Creates a subsystem to register metrics with a given `name` as a part of prefix.
    pub fn subsystem(&mut self, name: impl Into<String>) -> &mut Self {
        let subsystem = RegistrySystem::builder(name)
            .with_prefix(Some(self.namespace.clone()))
            .with_const_labels(self.const_labels.clone())
            .build();
        self.subsystems.push(subsystem);
        self.subsystems.last_mut().expect("subsystem must not be none")
    }

    /// Returns the current `namespace` of [`RegistrySystem`].
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
}
