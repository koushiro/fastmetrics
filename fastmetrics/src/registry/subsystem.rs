use std::{
    borrow::Cow,
    collections::hash_map::{self, HashMap},
};

use crate::{
    raw::{Metadata, MetricType, Unit},
    registry::{is_lowercase, is_snake_case, Metric, RegistryError},
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
/// # use fastmetrics::{
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
/// db.register(
///     "queries_total",
///     "Total number of database query operation",
///     queries.clone(),
/// )?;
///
/// // Update metrics
/// queries.inc();
/// # Ok(())
/// # }
/// ```
pub struct RegistrySystem {
    // namespace: prefix + system_name
    namespace: Cow<'static, str>,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,

    pub(crate) metrics: HashMap<Metadata, Box<dyn Metric + 'static>>,
    pub(crate) subsystems: HashMap<Cow<'static, str>, RegistrySystem>,
}

/// A builder for constructing [`RegistrySystem`] instances with custom configuration.
pub struct RegistrySystemBuilder {
    prefix: Option<Cow<'static, str>>,
    pub(crate) system_name: Cow<'static, str>,
    const_labels: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

// Public methods
impl RegistrySystemBuilder {
    /// Sets `constant labels` for this subsystem.
    ///
    /// This method allows you to add constant labels specific to this subsystem,
    /// which will be included with all metrics registered in this subsystem and
    /// its child subsystems.
    pub fn with_const_labels<N, V>(mut self, labels: impl IntoIterator<Item = (N, V)>) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.const_labels =
            labels.into_iter().map(|(name, value)| (name.into(), value.into())).collect();
        self
    }
}

// Private methods
impl RegistrySystemBuilder {
    fn new(system_name: impl Into<Cow<'static, str>>) -> Self {
        Self { prefix: None, system_name: system_name.into(), const_labels: vec![] }
    }

    pub(crate) fn with_prefix(mut self, prefix: Option<impl Into<Cow<'static, str>>>) -> Self {
        self.prefix = prefix.map(|prefix| prefix.into());
        self
    }

    pub(crate) fn with_inherited_const_labels<N, V>(
        mut self,
        inherited_labels: impl IntoIterator<Item = (N, V)>,
    ) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let labels = inherited_labels
            .into_iter()
            .map(|(name, value)| (name.into(), value.into()))
            .chain(self.const_labels)
            .collect::<Vec<_>>();
        self.const_labels = labels;
        self
    }

    pub(crate) fn build(self) -> RegistrySystem {
        let namespace = match self.prefix {
            Some(prefix) => Cow::Owned(format!("{}_{}", prefix, self.system_name)),
            None => self.system_name,
        };
        RegistrySystem {
            namespace,
            const_labels: self.const_labels,
            metrics: HashMap::default(),
            subsystems: HashMap::default(),
        }
    }
}

impl RegistrySystem {
    /// Creates a [`RegistrySystemBuilder`] to build [`RegistrySystem`] instance.
    pub fn builder(system_name: impl Into<Cow<'static, str>>) -> RegistrySystemBuilder {
        let system_name = system_name.into();
        assert!(is_lowercase(&system_name), "invalid subsystem name, must be lowercase");
        RegistrySystemBuilder::new(system_name)
    }

    /// Returns the current `namespace` of [`RegistrySystem`].
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the `constant labels` of [`RegistrySystem`].
    pub fn constant_labels(&self) -> &[(Cow<'static, str>, Cow<'static, str>)] {
        &self.const_labels
    }
}

// register
impl RegistrySystem {
    /// Registers a metric into [`RegistrySystem`], similar to [Registry::register] method.
    ///
    /// [Registry::register]: crate::registry::Registry::register
    pub fn register(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        help: impl Into<Cow<'static, str>>,
        metric: impl Metric + 'static,
    ) -> Result<&mut Self, RegistryError> {
        self.do_register(name, help, None, metric)
    }

    /// Registers a metric with the specified unit into [`RegistrySystem`], similar to
    /// [Registry::register_with_unit] method.
    ///
    /// [Registry::register_with_unit]: crate::registry::Registry::register_with_unit
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
impl RegistrySystem {
    /// Creates a subsystem to register metrics with a subsystem `name` (as a part of prefix).
    /// If the subsystem `name` already exists, the previous created subsystem will be returned.
    ///
    /// Similar to [Registry::subsystem] method.
    ///
    /// [Registry::subsystem]: crate::registry::Registry::subsystem
    pub fn subsystem(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Self {
        let name = name.into();
        self.subsystems.entry(name).or_insert_with_key(|name| {
            RegistrySystem::builder(name.clone())
                // inherit prefix from this subsystem
                .with_prefix(Some(self.namespace.clone()))
                // inherit constant labels from this subsystem
                .with_inherited_const_labels(self.const_labels.clone())
                .build()
        })
    }

    /// Attach a configurable subsystem.
    /// If the subsystem `name` already exists, the previous created subsystem will be returned.
    ///
    /// Similar to [Registry::attach_subsystem] method.
    ///
    /// [Registry::attach_subsystem]: crate::registry::Registry::attach_subsystem
    pub fn attach_subsystem(&mut self, builder: RegistrySystemBuilder) -> &mut RegistrySystem {
        let name = builder.system_name.clone();
        self.subsystems.entry(name).or_insert_with(|| {
            builder
                // inherit prefix from this subsystem
                .with_prefix(Some(self.namespace.clone()))
                // inherit constant labels from this subsystem
                .with_inherited_const_labels(self.const_labels.clone())
                .build()
        })
    }
}
