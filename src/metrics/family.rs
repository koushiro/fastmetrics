//! A metric family is a collection of metrics with the same name but different label values.
//!
//! Each metric within a family shares the same name and metadata, but has a unique set of label
//! values. For example, a counter metric family named "http_requests_total" might contain multiple
//! individual counters for different HTTP methods (GET, POST) and status codes (200, 404, etc).
//!
//! # Example
//!
//! ```rust
//! # use openmetrics_client::{
//! #    metrics::{counter::Counter, family::Family},
//! #    registry::{Registry, RegistryError},
//! # };
//! #
//! # fn main() -> Result<(), RegistryError> {
//! let mut registry = Registry::default();
//!
//! let http_requests = Family::<Vec<(String, String)>, Counter>::default();
//!
//! registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
//!
//! // Create metrics with different labels
//! http_requests.get_or_create(&vec![
//!     ("method".to_string(), "GET".to_string()),
//!     ("status".to_string(), "200".to_string()),
//! ]).inc();
//!
//! # Ok(())
//! # }
//! ```

use std::{
    collections::HashMap,
    fmt::{self, Debug},
    hash::Hash,
    sync::Arc,
};

use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::metrics::{MetricType, TypedMetric};

/// The metadata of a metric family.
///
/// There are four pieces of metadata: name, TYPE, UNIT and HELP.
#[derive(Clone, Debug)]
pub struct Metadata {
    /// Name of metric.
    pub(crate) name: String,
    /// Help of metric.
    pub(crate) help: String,
    /// Type of metric.
    pub(crate) ty: MetricType,
    /// Optional unit of metric.
    pub(crate) unit: Option<Unit>,
}

impl Metadata {
    /// Create a new [`Metadata`].
    pub fn new(
        name: impl Into<String>,
        help: impl Into<String>,
        ty: MetricType,
        unit: Option<Unit>,
    ) -> Self {
        Self { name: name.into(), help: help.into() + ".", ty, unit }
    }
}

/// [Open Metrics units](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#units-and-base-units).
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Unit {
    Seconds,
    Bytes,
    Joules,
    Grams,
    Meters,
    Ratios,
    Volts,
    Amperes,
    Celsius,
    Other(String),
}

impl Unit {
    /// Return the string representation for the specified metric unit.
    pub fn as_str(&self) -> &str {
        match self {
            Unit::Seconds => "seconds",
            Unit::Bytes => "bytes",
            Unit::Joules => "joules",
            Unit::Grams => "grams",
            Unit::Meters => "meters",
            Unit::Ratios => "ratios",
            Unit::Volts => "volts",
            Unit::Amperes => "amperes",
            Unit::Celsius => "celsius",
            Unit::Other(other) => other.as_str(),
        }
    }
}

/// A collection of metrics that share the same name but have different label values.
///
/// The type parameters are:
/// - `LS`: The label set type that uniquely identifies a metric within the family
/// - `M`: The specific metric type (e.g., Counter, Gauge) stored in this family
///
/// A metric family maintains a map of label sets to metric instances. Each combination
/// of label values maps to a unique metric instance. This allows tracking metrics
/// across different dimensions (e.g., request counts by method and status code).
pub struct Family<LS, M> {
    // label set => metric points
    metrics: Arc<RwLock<HashMap<LS, M>>>,
}

impl<LS, M> Clone for Family<LS, M> {
    fn clone(&self) -> Self {
        Self { metrics: self.metrics.clone() }
    }
}

impl<LS: Debug, M: Debug> Debug for Family<LS, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MetricFamily").field("metrics", &self.metrics).finish()
    }
}

impl<LS, M> Default for Family<LS, M> {
    fn default() -> Self {
        Self { metrics: Arc::new(RwLock::new(Default::default())) }
    }
}

impl<LS, M> Family<LS, M> {
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, HashMap<LS, M>> {
        self.metrics.read()
    }

    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, HashMap<LS, M>> {
        self.metrics.write()
    }

    /// Gets a reference to the metric with the specified label set.
    ///
    /// Return `None` if no metric exists for the given label set.
    pub fn get(&self, label_set: &LS) -> Option<MappedRwLockReadGuard<'_, M>>
    where
        LS: Eq + Hash,
    {
        let read_guard = self.read();
        RwLockReadGuard::try_map(read_guard, |metrics| metrics.get(label_set)).ok()
    }

    /// Gets a reference to the metric with the specified label set, creating it if it doesn't
    /// exist.
    ///
    /// If a metric with the given label set already exists, returns a reference to it.
    /// Otherwise, creates a new metric with default values and the given label set.
    pub fn get_or_create(&self, label_set: &LS) -> MappedRwLockReadGuard<'_, M>
    where
        LS: Clone + Eq + Hash,
        M: Default,
    {
        if let Some(metric) = self.get(label_set) {
            return metric;
        }

        let mut write_guard = self.write();
        write_guard.entry(label_set.clone()).or_default();

        let read_guard = RwLockWriteGuard::downgrade(write_guard);
        RwLockReadGuard::map(read_guard, |metrics| {
            metrics.get(label_set).expect("Metric must exist once it's created.")
        })
    }
}

impl<LS, M: TypedMetric> TypedMetric for Family<LS, M> {
    const TYPE: MetricType = <M as TypedMetric>::TYPE;
}
