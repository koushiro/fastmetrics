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

use crate::{
    encoder::{EncodeLabelSet, EncodeMetric, MetricEncoder},
    raw::{LabelSetSchema, MetricLabelSet, MetricType, TypedMetric},
};

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
/// #     encoder::{EncodeLabelSet, LabelSetEncoder},
/// #     metrics::{counter::Counter, family::Family},
/// #     raw::LabelSetSchema,
/// #     registry::{Registry, RegistryError},
/// # };
/// #
/// # fn main() -> Result<(), RegistryError> {
/// let mut registry = Registry::default();
///
/// #[derive(Clone, Eq, PartialEq, Hash)]
/// struct HttpLabels {
///     method: &'static str,
///     status: &'static str,
/// }
///
/// impl LabelSetSchema for HttpLabels {
///     fn names() -> Option<&'static [&'static str]> {
///         Some(&["method", "status"])
///     }
/// }
///
/// impl EncodeLabelSet for HttpLabels {
///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> std::fmt::Result {
///         encoder.encode(&("method", self.method))?;
///         encoder.encode(&("status", self.status))?;
///         Ok(())
///     }
/// }
///
/// let http_requests = Family::<HttpLabels, Counter>::default();
///
/// registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
///
/// // Create metrics with different labels
/// let labels = HttpLabels { method: "GET", status: "200" };
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

impl<LS, M, MF, S> Debug for Family<LS, M, MF, S>
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
    /// # use fastmetrics::{
    /// #     encoder::{EncodeLabelSet, LabelSetEncoder},
    /// #     metrics::{
    /// #         gauge::Gauge,
    /// #         family::Family,
    /// #         histogram::{Histogram, exponential_buckets},
    /// #     },
    /// #     raw::LabelSetSchema,
    /// # };
    /// // Create a family with a custom factory function
    /// #[derive(Clone, Eq, PartialEq, Hash)]
    /// struct Labels {
    ///     region: &'static str,
    ///     status: &'static str,
    /// }
    ///
    /// impl LabelSetSchema for Labels {
    ///     fn names() -> Option<&'static [&'static str]> {
    ///         Some(&["region", "status"])
    ///     }
    /// }
    ///
    /// impl EncodeLabelSet for Labels {
    ///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> std::fmt::Result {
    ///         encoder.encode(&("region", self.region))?;
    ///         encoder.encode(&("status", self.status))?;
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let gauge_family: Family<Labels, Gauge> = Family::new(|| Gauge::new(100));
    /// let histogram_family: Family<Labels, Histogram> = Family::new(|| {
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
    /// #     encoder::{EncodeLabelSet, LabelSetEncoder},
    /// #     raw::LabelSetSchema,
    /// #     metrics::{counter::Counter, family::Family},
    /// #     registry::{Registry, RegistryError},
    /// # };
    /// #
    /// # fn main() -> Result<(), RegistryError> {
    /// let mut registry = Registry::default();
    ///
    /// #[derive(Clone, Eq, PartialEq, Hash)]
    /// struct Labels {
    ///     method: &'static str,
    ///     status: &'static str,
    /// }
    ///
    /// impl LabelSetSchema for Labels {
    ///     fn names() -> Option<&'static [&'static str]> {
    ///         Some(&["method", "status"])
    ///     }
    /// }
    ///
    /// impl EncodeLabelSet for Labels {
    ///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> std::fmt::Result {
    ///         encoder.encode(&("method", self.method))?;
    ///         encoder.encode(&("status", self.status))?;
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let http_requests = Family::<Labels, Counter>::default();
    ///
    /// registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
    ///
    /// let labels = Labels { method: "GET", status: "200" };
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
    /// #     encoder::{EncodeLabelSet, LabelSetEncoder},
    /// #     raw::LabelSetSchema,
    /// #     metrics::{counter::Counter, family::Family},
    /// #     registry::{Registry, RegistryError},
    /// # };
    /// #
    /// # fn main() -> Result<(), RegistryError> {
    /// let mut registry = Registry::default();
    ///
    /// #[derive(Clone, Eq, PartialEq, Hash)]
    /// struct Labels {
    ///     method: &'static str,
    ///     status: &'static str,
    /// }
    ///
    /// impl LabelSetSchema for Labels {
    ///     fn names() -> Option<&'static [&'static str]> {
    ///         Some(&["method", "status"])
    ///     }
    /// }
    ///
    /// impl EncodeLabelSet for Labels {
    ///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> std::fmt::Result {
    ///         encoder.encode(&("method", self.method))?;
    ///         encoder.encode(&("status", self.status))?;
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let http_requests = Family::<Labels, Counter>::default();
    ///
    /// registry.register("http_requests", "Total HTTP requests", http_requests.clone())?;
    ///
    /// let labels = Labels { method: "GET", status: "200" };
    /// http_requests.with_or_new(&labels, |req| req.inc());
    /// assert_eq!(http_requests.with(&labels, |req| req.total()), Some(1));
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_or_new<R, F>(&self, labels: &LS, func: F) -> R
    where
        LS: Clone + Eq + Hash,
        F: FnOnce(&M) -> R,
        S: BuildHasher,
    {
        let read_guard = self.read();
        if let Some(metric) = read_guard.get(labels) {
            return func(metric);
        }
        drop(read_guard);

        let mut write_guard = self.write();
        let metric = write_guard.entry(labels.clone()).or_insert(self.metric_factory.new_metric());
        func(metric)
    }
}

impl<LS, M: TypedMetric, S> TypedMetric for Family<LS, M, S> {
    const TYPE: MetricType = <M as TypedMetric>::TYPE;
}

impl<LS: LabelSetSchema, M, S> MetricLabelSet for Family<LS, M, S> {
    type LabelSet = LS;
}

impl<LS, M, MF, S> EncodeMetric for Family<LS, M, MF, S>
where
    LS: EncodeLabelSet + Send + Sync,
    M: EncodeMetric,
    MF: Send + Sync,
    S: Send + Sync,
{
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let guard = self.read();
        for (labels, metric) in guard.iter() {
            encoder.encode(labels, metric)?;
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.read().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        encoder::{EncodeLabelSet, EncodeLabelValue, LabelEncoder, LabelSetEncoder},
        metrics::{
            check_text_encoding,
            counter::Counter,
            histogram::{Histogram, exponential_buckets},
        },
    };

    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Labels {
        method: Method,
        status: u16,
        error: Option<bool>,
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    enum Method {
        Get,
        Put,
    }

    impl LabelSetSchema for Labels {
        fn names() -> Option<&'static [&'static str]> {
            Some(&["method", "status", "error"])
        }
    }

    impl EncodeLabelSet for Labels {
        fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> fmt::Result {
            encoder.encode(&("method", &self.method))?;
            encoder.encode(&("status", self.status))?;
            encoder.encode(&("error", self.error))?;
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
        check_text_encoding(
            |registry| {
                let http_requests = Family::<Labels, Counter>::default();
                registry
                    .register("http_requests", "Total HTTP requests", http_requests.clone())
                    .unwrap();

                // Create metrics with different labels
                let labels = Labels { method: Method::Get, status: 200, error: None };
                http_requests.with_or_new(&labels, |metric| metric.inc());
                let labels = Labels { method: Method::Get, status: 404, error: Some(true) };
                http_requests.with_or_new(&labels, |metric| metric.inc());
                let labels = Labels { method: Method::Put, status: 200, error: None };
                http_requests.with_or_new(&labels, |metric| metric.inc());
            },
            |output| {
                // println!("{}", output);
                assert!(output.contains(r#"http_requests_total{method="GET",status="200"} 1"#));
                assert!(
                    output.contains(
                        r#"http_requests_total{method="GET",status="404",error="true"} 1"#
                    )
                );
                assert!(output.contains(r#"http_requests_total{method="PUT",status="200"} 1"#));
            },
        );

        check_text_encoding(
            |registry| {
                let http_requests_duration_seconds = Family::<Labels, Histogram>::new(|| {
                    Histogram::new(exponential_buckets(0.005, 2.0, 10))
                });

                registry
                    .register(
                        "http_requests_duration_seconds",
                        "Duration of HTTP requests",
                        http_requests_duration_seconds.clone(),
                    )
                    .unwrap();

                // Create metrics with different labels
                let labels = Labels { method: Method::Get, status: 200, error: None };
                http_requests_duration_seconds.with_or_new(&labels, |hist| hist.observe(0.1));
                let labels = Labels { method: Method::Get, status: 404, error: Some(true) };
                http_requests_duration_seconds.with_or_new(&labels, |hist| hist.observe(0.1));
                let labels = Labels { method: Method::Put, status: 200, error: None };
                http_requests_duration_seconds.with_or_new(&labels, |hist| hist.observe(2.0));
            },
            |output| {
                // println!("{}", output);
                assert!(output.contains(
                    r#"http_requests_duration_seconds_count{method="GET",status="200"} 1"#
                ));
                assert!(output.contains(
                    r#"http_requests_duration_seconds_sum{method="GET",status="200"} 0.1"#
                ));
                assert!(output.contains(
                    r#"http_requests_duration_seconds_count{method="GET",status="404",error="true"} 1"#
                ));
                assert!(output.contains(
                    r#"http_requests_duration_seconds_sum{method="GET",status="404",error="true"} 0.1"#
                ));
                assert!(output.contains(
                    r#"http_requests_duration_seconds_count{method="PUT",status="200"} 1"#
                ));
                assert!(output.contains(
                    r#"http_requests_duration_seconds_sum{method="PUT",status="200"} 2.0"#
                ));
            },
        );
    }

    #[test]
    fn test_empty_metric_family() {
        check_text_encoding(
            |registry| {
                let http_requests = Family::<Labels, Counter>::default();
                registry
                    .register("http_requests", "Total HTTP requests", http_requests)
                    .unwrap();
            },
            |output| {
                assert_eq!(output, "# EOF\n");
            },
        );

        check_text_encoding(
            |registry| {
                let http_requests = Family::<Labels, Counter>::default();
                registry
                    .register("http_requests", "Total HTTP requests", http_requests.clone())
                    .unwrap();

                let labels = Labels { method: Method::Get, status: 200, error: None };
                http_requests.with_or_new(&labels, |_| {});
            },
            |output| {
                assert!(output.contains("# TYPE http_requests counter"));
                assert!(output.contains("# HELP http_requests Total HTTP requests"));
                assert!(output.contains(r#"http_requests_total{method="GET",status="200"} 0"#));
            },
        );
    }
}
