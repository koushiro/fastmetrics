//! A metric family is a collection of metrics with the same name but different label values.
//!
//! Each metric within a family has the same metadata, but has a unique set of label values.
//!
//! See [`Family`] for more details.

use std::{
    collections::HashMap,
    fmt::{self, Debug},
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::raw::{MetricType, TypedMetric};

/// A trait for creating new metric instances.
///
/// This trait is implemented by factory functions or objects that can create
/// new instances of metrics to be used in a metric family.
pub trait MetricFactory<M> {
    /// Creates a new metric instance.
    ///
    /// This method is called when a new metric needs to be created for a label set
    /// that doesn't have an associated metric yet.
    fn new_metric(&self) -> M;
}

impl<M, F: Fn() -> M> MetricFactory<M> for F {
    fn new_metric(&self) -> M {
        self()
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "foldhash")] {
        type RandomState = foldhash::fast::RandomState;
    } else {
        type RandomState = std::hash::RandomState;
    }
}

/// A collection of metrics that share the same name but have different label values.
///
/// The type parameters are:
/// - `LS`: The label set type that uniquely identifies a metric within the family
/// - `M`: The specific metric type (e.g., Counter, Gauge) stored in this family
/// - `MF`: The metric factory type that creates new metric instances
/// - `S`: The hash algorithm of internal HashMap type.
///
/// A metric family maintains a map of label sets to metric instances. Each combination
/// of label values maps to a unique metric instance. This allows tracking metrics
/// across different dimensions (e.g., request counts by method and status code).
///
/// # Example
///
/// A counter metric family named "http_requests_total" might contain multiple individual counters
/// for different HTTP methods (GET, POST) and status codes (200, 404, etc.).
///
/// ```rust
/// # use fastmetrics::{
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
/// http_requests.with_or_new(&labels, |metric| metric.inc());
///
/// # Ok(())
/// # }
/// ```
pub struct Family<LS, M, MF = fn() -> M, S = RandomState> {
    // label set => metric points
    metrics: Arc<RwLock<HashMap<LS, M, S>>>,
    metric_factory: MF,
}

impl<LS, M, MF, S> Clone for Family<LS, M, MF, S>
where
    MF: Clone,
{
    fn clone(&self) -> Self {
        Self { metrics: self.metrics.clone(), metric_factory: self.metric_factory.clone() }
    }
}

impl<LS, M> Debug for Family<LS, M>
where
    LS: Debug,
    M: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MetricFamily").field("metrics", &self.metrics).finish()
    }
}

impl<LS, M, S> Default for Family<LS, M, fn() -> M, S>
where
    M: Default,
    S: Default,
{
    fn default() -> Self {
        Self::new(M::default)
    }
}

impl<LS, M, MF, S> Family<LS, M, MF, S> {
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, HashMap<LS, M, S>> {
        self.metrics.read()
    }

    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, HashMap<LS, M, S>> {
        self.metrics.write()
    }
}

impl<LS, M, MF, S> Family<LS, M, MF, S>
where
    MF: MetricFactory<M>,
{
    /// Creates a new metric family with a custom metric factory.
    ///
    /// The factory is used to create new metric instances when they are needed.
    ///
    /// # Parameters
    ///
    /// - `factory`: A factory function or object that creates new metric instances
    ///
    /// # Returns
    ///
    /// A new `Family` instance that uses the provided factory to create metrics.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::metrics::{
    /// #     counter::Counter,
    /// #     family::Family,
    /// #     histogram::{Histogram, exponential_buckets},
    /// # };
    /// // Create a family with a custom factory function
    /// type LabelSet = Vec<(&'static str, &'static str)>;
    /// let counter_family: Family<LabelSet, Counter> = Family::new(|| Counter::with_created());
    /// let histogram_family: Family<LabelSet, Histogram> = Family::new(|| {
    ///     Histogram::new(exponential_buckets(1.0, 2.0, 10))
    /// });
    /// ```
    pub fn new(metric_factory: MF) -> Self
    where
        S: Default,
    {
        Self { metrics: Arc::new(RwLock::new(HashMap::default())), metric_factory }
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
    /// # use fastmetrics::{
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
    /// http_requests.with_or_new(&labels, |req| req.inc());
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

    /// Gets a reference to an existing metric or creates a new one using given metric factory
    /// if it doesn't exist, then applies a function to it.
    ///
    /// This method will:
    /// 1. Check if a metric exists for the given labels
    /// 2. If it exists, apply the function to it
    /// 3. If it doesn't exist, create a metric using given metric factory and then apply the
    ///    function
    ///
    /// # Parameters
    ///
    /// - `labels`: The labels to identify the metric
    /// - `func`: Function to apply to the metric
    ///
    /// # Returns
    ///
    /// Returns `Some(R)` where R is the return value of `func` after applying it to
    /// either the existing or newly created metric.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
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
    /// http_requests.with_or_new(&labels, |req| req.inc());
    /// assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_or_new<R, F>(&self, labels: &LS, func: F) -> Option<R>
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
        write_guard.entry(labels.clone()).or_insert(self.metric_factory.new_metric());

        let read_guard = RwLockWriteGuard::downgrade(write_guard);
        read_guard.get(labels).map(func)
    }
}

impl<LS, M: TypedMetric, S> TypedMetric for Family<LS, M, S> {
    const TYPE: MetricType = <M as TypedMetric>::TYPE;
    const WITH_TIMESTAMP: bool = <M as TypedMetric>::WITH_TIMESTAMP;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        encoder::{EncodeLabelSet, EncodeLabelValue, LabelEncoder, LabelSetEncoder},
        format::text,
        metrics::{
            counter::Counter,
            histogram::{exponential_buckets, Histogram},
        },
        registry::Registry,
    };

    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Labels {
        method: Method,
        status: u16,
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    enum Method {
        Get,
        Put,
    }

    impl EncodeLabelSet for Labels {
        fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
            encoder.encode(&("method", &self.method))?;
            encoder.encode(&("status", self.status))?;
            Ok(())
        }
    }

    impl EncodeLabelValue for Method {
        fn encode(&self, encoder: &mut dyn LabelEncoder) -> fmt::Result {
            match self {
                Self::Get => encoder.encode_str_value("GET"),
                Self::Put => encoder.encode_str_value("PUT"),
            }
        }
    }

    #[test]
    fn test_metric_family() {
        let mut registry = Registry::default();

        let http_requests = Family::<Labels, Counter>::default();
        let http_requests_duration_seconds =
            Family::<(), Histogram>::new(|| Histogram::new(exponential_buckets(0.005, 2.0, 10)));
        registry
            .register("http_requests", "Total HTTP requests", http_requests.clone())
            .unwrap();
        registry
            .register(
                "http_requests_duration_seconds",
                "Duration of HTTP requests",
                http_requests_duration_seconds.clone(),
            )
            .unwrap();

        // Create metrics with different labels
        let labels = Labels { method: Method::Get, status: 200 };
        http_requests.with_or_new(&labels, |metric| metric.inc());
        http_requests_duration_seconds.with_or_new(&(), |hist| hist.observe(0.1));

        let labels = Labels { method: Method::Put, status: 200 };
        http_requests.with_or_new(&labels, |metric| metric.inc());
        http_requests_duration_seconds.with_or_new(&(), |hist| hist.observe(2.0));

        let mut output = String::new();
        text::encode(&mut output, &registry).unwrap();

        // println!("{}", output);
        assert!(output.contains(r#"http_requests_total{method="GET",status="200"} 1"#));
        assert!(output.contains(r#"http_requests_total{method="PUT",status="200"} 1"#));

        assert!(output.contains(r#"http_requests_duration_seconds_sum 2.1"#));
        assert!(output.contains(r#"http_requests_duration_seconds_count 2"#));
    }
}
