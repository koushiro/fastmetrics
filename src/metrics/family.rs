//! A metric family is a collection of metrics with the same name but different label values.
//!
//! Each metric within a family has the same metadata, but has a unique set of label values.
//!
//! See [`Family`] for more details.

use std::{
    collections::hash_map::{HashMap, RandomState},
    fmt::{self, Debug},
    hash::{BuildHasher, Hash, Hasher},
    sync::Arc,
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::metrics::{MetricType, TypedMetric};

/// The metadata of a metric family.
///
/// There are four pieces of metadata: name, TYPE, UNIT and HELP.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    name: String,
    help: String,
    ty: MetricType,
    unit: Option<Unit>,
}

impl Hash for Metadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ty.hash(state);
        self.unit.hash(state);
    }
}

impl Metadata {
    /// Creates a new [`Metadata`] of metric family.
    pub fn new(
        name: impl Into<String>,
        help: impl Into<String>,
        ty: MetricType,
        unit: Option<Unit>,
    ) -> Self {
        Self { name: name.into(), help: help.into() + ".", ty, unit }
    }

    /// Returns the name of the metric family.
    ///
    /// The name uniquely identifies the metric family in the registry and
    /// is used when exposing metrics in the OpenMetrics format.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the help text of the metric family.
    ///
    /// The help text provides a description of what the metric measures and
    /// is included in the OpenMetrics output as a HELP comment.
    pub fn help(&self) -> &str {
        &self.help
    }

    /// Returns the type of the metric family.
    ///
    /// The type indicates what kind of metric this is (Counter, Gauge, etc.) and
    /// is included in the OpenMetrics output as a TYPE comment.
    pub fn metric_type(&self) -> MetricType {
        self.ty
    }

    /// Returns the optional unit of the metric family.
    ///
    /// The unit specifies the measurement unit for the metric values (e.g., seconds, bytes).
    /// If present, it is included in the OpenMetrics output as part of the metric name.
    pub fn unit(&self) -> Option<&Unit> {
        self.unit.as_ref()
    }
}

/// [Open Metrics units](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#units-and-base-units).
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
    /// Returns the string representation for the specified metric unit.
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
/// - `S`: The hash algorithm of internal HashMap type.
///
/// A metric family maintains a map of label sets to metric instances. Each combination
/// of label values maps to a unique metric instance. This allows tracking metrics
/// across different dimensions (e.g., request counts by method and status code).
///
/// # Example
///
/// A counter metric family named "http_requests_total" might contain multiple individual counters
/// for different HTTP methods (GET, POST) and status codes (200, 404, etc).
///
/// ```rust
/// # use openmetrics_client::{
/// #    metrics::{counter::Counter, family::Family},
/// #    registry::{Registry, RegistryError},
/// # };
/// #
/// # fn main() -> Result<(), RegistryError> {
/// let mut registry = Registry::default();
///
/// type LabelSet = Vec<(&'static str, &'static str)>;
/// let http_requests = Family::<LabelSet, Counter>::default();
///
/// registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
///
/// // Create metrics with different labels
/// let labels = vec![("method", "GET"), ("status", "200")];
/// http_requests.with_or_default(&labels, |metric| metric.inc());
///
/// # Ok(())
/// # }
/// ```
pub struct Family<LS, M, S = RandomState> {
    // label set => metric points
    metrics: Arc<RwLock<HashMap<LS, M, S>>>,
}

impl<LS, M, S> Clone for Family<LS, M, S> {
    fn clone(&self) -> Self {
        Self { metrics: self.metrics.clone() }
    }
}

impl<LS, M, S> Debug for Family<LS, M, S>
where
    LS: Debug,
    M: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MetricFamily").field("metrics", &self.metrics).finish()
    }
}

impl<LS, M, S> Default for Family<LS, M, S>
where
    S: Default,
{
    fn default() -> Self {
        Self { metrics: Arc::new(RwLock::new(Default::default())) }
    }
}

impl<LS, M, S> Family<LS, M, S> {
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, HashMap<LS, M, S>> {
        self.metrics.read()
    }

    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, HashMap<LS, M, S>> {
        self.metrics.write()
    }

    /// Gets a reference to the metric with the specified labels and applies a function to it.
    ///
    /// # Parameters
    ///
    /// - `labels`: The labels to identify the metric
    /// - `func`: Function to apply to the metric if it exists
    ///
    /// # Returns
    ///
    /// Returns `Some(R)` where R is the return value of `func` if the metric exists, or `None`
    /// if no metric exists for the given label set.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use openmetrics_client::{
    /// #    metrics::{counter::Counter, family::Family},
    /// #    registry::{Registry, RegistryError},
    /// # };
    /// #
    /// # fn main() -> Result<(), RegistryError> {
    /// let mut registry = Registry::default();
    ///
    /// type LabelSet = Vec<(&'static str, &'static str)>;
    /// let http_requests = Family::<LabelSet, Counter>::default();
    ///
    /// registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
    ///
    /// let labels = vec![("method", "GET"), ("status", "200")];
    /// assert_eq!(http_requests.with(&labels, |req| req.total()), None);
    ///
    /// http_requests.with_or_default(&labels, |req| req.inc());
    /// assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));
    ///
    /// http_requests.with(&labels, |req| req.inc());
    /// assert_eq!(http_requests.with(&labels, |req| req.total()), Some(2));
    /// # Ok(())
    /// # }
    /// ```
    pub fn with<R, F>(&self, labels: &LS, func: F) -> Option<R>
    where
        LS: Eq + Hash,
        F: FnOnce(&M) -> R,
        S: BuildHasher,
    {
        let guard = self.read();
        guard.get(labels).map(func)
    }

    /// Gets a reference to an existing metric or inserts a new one with the specified labels,
    /// then applies a function to it.
    ///
    /// This method will:
    /// 1. Check if a metric exists for the given labels
    /// 2. If it exists, apply the function to it
    /// 3. If it doesn't exist, insert the provided metric and then apply the function
    ///
    /// # Parameters
    ///
    /// - `labels`: The labels to identify the metric
    /// - `metric`: The new metric instance to insert if one doesn't exist
    /// - `func`: Function to apply to the metric
    ///
    /// # Returns
    ///
    /// Returns `Some(R)` where R is the return value of `func` after applying it to
    /// either the existing or newly inserted metric.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use openmetrics_client::{
    /// #    metrics::{counter::Counter, family::Family},
    /// #    registry::{Registry, RegistryError},
    /// # };
    /// #
    /// # fn main() -> Result<(), RegistryError> {
    /// let mut registry = Registry::default();
    ///
    /// type LabelSet = Vec<(&'static str, &'static str)>;
    /// let http_requests = Family::<LabelSet, Counter>::default();
    ///
    /// registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
    ///
    /// let labels = vec![("method", "GET"), ("status", "200")];
    /// http_requests.with_or_insert(&labels, Counter::with_created(), |req| req.inc());
    /// assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_or_insert<R, F>(&self, labels: &LS, metric: M, func: F) -> Option<R>
    where
        LS: Clone + Eq + Hash,
        F: FnOnce(&M) -> R,
        S: BuildHasher,
    {
        let guard = self.read();
        if let Some(metric) = guard.get(labels) {
            return Some(func(metric));
        }
        drop(guard);

        let mut write_guard = self.write();
        write_guard.entry(labels.clone()).or_insert(metric);

        let read_guard = RwLockWriteGuard::downgrade(write_guard);
        read_guard.get(labels).map(func)
    }

    /// Gets a reference to an existing metric or creates a default one with the specified labels,
    /// then applies a function to it.
    ///
    /// This is a convenience wrapper around `with_or_insert` that uses the `Default` implementation
    /// of the metric type to create new metrics. It's particularly useful when you don't need
    /// special initialization for new metrics.
    ///
    /// # Parameters
    ///
    /// - `labels`: The labels to identify the metric
    /// - `func`: Function to apply to the metric
    ///
    /// # Returns
    ///
    /// Returns `Some(R)` where R is the return value of `func` after applying it to
    /// either the existing or newly created default metric.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use openmetrics_client::{
    /// #    metrics::{counter::Counter, family::Family},
    /// #    registry::{Registry, RegistryError},
    /// # };
    /// #
    /// # fn main() -> Result<(), RegistryError> {
    /// let mut registry = Registry::default();
    ///
    /// type LabelSet = Vec<(&'static str, &'static str)>;
    /// let http_requests = Family::<LabelSet, Counter>::default();
    ///
    /// registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
    ///
    /// let labels = vec![("method", "GET"), ("status", "200")];
    /// http_requests.with_or_default(&labels, |req| req.inc());
    /// assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_or_default<R, F>(&self, labels: &LS, func: F) -> Option<R>
    where
        LS: Clone + Eq + Hash,
        M: Default,
        F: FnOnce(&M) -> R,
        S: BuildHasher,
    {
        self.with_or_insert(labels, M::default(), func)
    }
}

impl<LS, M: TypedMetric, S> TypedMetric for Family<LS, M, S> {
    const TYPE: MetricType = <M as TypedMetric>::TYPE;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        encoder::{EncodeLabel, EncodeLabelSet, EncodeLabelValue, LabelEncoder, LabelSetEncoder},
        format::text,
        metrics::counter::Counter,
        registry::Registry,
    };

    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Labels {
        method: Method,
        status: u32,
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    enum Method {
        Get,
        Put,
    }

    impl EncodeLabelSet for Labels {
        fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
            ("method", &self.method).encode(encoder.label_encoder().as_mut())?;
            ("status", self.status).encode(encoder.label_encoder().as_mut())?;
            Ok(())
        }
    }

    impl EncodeLabelValue for Method {
        fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
            match self {
                Self::Get => encoder.encode_str_value("Get"),
                Self::Put => encoder.encode_str_value("Put"),
            }
        }
    }

    #[test]
    fn test_metric_family() {
        let mut registry = Registry::default();

        let http_requests = Family::<Labels, Counter>::default();
        registry
            .register("http_requests", "Total HTTP requests", http_requests.clone())
            .unwrap();

        // Create metrics with different labels
        let labels = Labels { method: Method::Get, status: 200 };
        http_requests.with_or_default(&labels, |metric| metric.inc());

        let labels = Labels { method: Method::Put, status: 200 };
        http_requests.with_or_default(&labels, |metric| metric.inc());

        let mut output = String::new();
        text::encode(&mut output, &registry).unwrap();

        // println!("{}", out);
        assert!(output.contains(r#"http_requests_total{method="Get",status="200"} 1"#));
        assert!(output.contains(r#"http_requests_total{method="Put",status="200"} 1"#));
    }
}
