//! A metric family is a collection of metrics with the same name but different label values.
//!
//! Each metric within a family has the same metadata, but has a unique set of label values.
//!
//! See [`Family`] for more details.

use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::{self, Debug},
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    encoder::{EncodeLabelSet, EncodeMetric, MetricEncoder},
    error::Result,
    raw::{LabelSetSchema, MetricLabelSet, MetricType, TypedMetric},
};

type MetricFactory<LS, M> = dyn Fn(&LS) -> M + Send + Sync + 'static;

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
/// #     error::Result,
/// #     metrics::{counter::Counter, family::Family},
/// #     raw::LabelSetSchema,
/// #     registry::Registry,
/// # };
/// #
/// # fn main() -> Result<()> {
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
///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
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
pub struct Family<LS, M, S = RandomState> {
    // label set => metric points
    metrics: Arc<RwLock<HashMap<LS, M, S>>>,
    metric_factory: Arc<MetricFactory<LS, M>>,
}

impl<LS, M, S> Clone for Family<LS, M, S> {
    fn clone(&self) -> Self {
        Self { metrics: self.metrics.clone(), metric_factory: self.metric_factory.clone() }
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
    M: Default + 'static,
    S: Default,
{
    fn default() -> Self {
        Self::new(M::default)
    }
}

impl<LS, M, S> Family<LS, M, S> {
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, HashMap<LS, M, S>> {
        self.metrics.read()
    }

    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, HashMap<LS, M, S>> {
        self.metrics.write()
    }
}

impl<LS, M, S> Family<LS, M, S> {
    /// Creates a new metric family with a custom metric factory.
    ///
    /// The factory is used to create new metric instances when they are needed.
    ///
    /// # Parameters
    ///
    /// - `factory`: A factory function or closure that creates new metric instances
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
    /// #     error::Result,
    /// #     metrics::{
    /// #         gauge::Gauge,
    /// #         family::Family,
    /// #         histogram::{Histogram, exponential_buckets},
    /// #     },
    /// #     raw::LabelSetSchema,
    /// # };
    /// #
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
    ///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
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
    pub fn new(metric_factory: impl Fn() -> M + Send + Sync + 'static) -> Self
    where
        S: Default,
    {
        Self::new_with_labels(move |_| metric_factory())
    }

    /// Creates a new metric family with a label-aware factory.
    ///
    /// This is useful for metric types whose constructor needs label values.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fastmetrics::{
    /// #     encoder::{EncodeLabelSet, LabelSetEncoder},
    /// #     error::Result,
    /// #     metrics::{counter::LazyCounter, family::Family},
    /// #     raw::LabelSetSchema,
    /// # };
    /// #
    /// #[derive(Clone, Eq, PartialEq, Hash)]
    /// struct Labels {
    ///     method: &'static str,
    /// }
    ///
    /// impl LabelSetSchema for Labels {
    ///     fn names() -> Option<&'static [&'static str]> {
    ///         Some(&["method"])
    ///     }
    /// }
    ///
    /// impl EncodeLabelSet for Labels {
    ///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
    ///         encoder.encode(&("method", self.method))?;
    ///         Ok(())
    ///     }
    /// }
    ///
    /// # fn main() -> Result<()> {
    /// let family = Family::<Labels, LazyCounter<u64>>::new_with_labels(|labels| {
    ///     let method = labels.method;
    ///     LazyCounter::new(move || if method == "GET" { 1u64 } else { 2u64 })
    /// });
    ///
    /// let labels = Labels { method: "GET" };
    /// let value = family.with_or_new(&labels, |counter| counter.fetch());
    /// assert_eq!(value, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_with_labels(metric_factory: impl Fn(&LS) -> M + Send + Sync + 'static) -> Self
    where
        S: Default,
    {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::default())),
            metric_factory: Arc::new(metric_factory),
        }
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
    /// #     error::Result,
    /// #     raw::LabelSetSchema,
    /// #     metrics::{counter::Counter, family::Family},
    /// #     registry::Registry,
    /// # };
    /// #
    /// # fn main() -> Result<()> {
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
    ///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
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

    /// Gets a reference to an existing metric or creates a new one using this family's metric
    /// factory if it doesn't exist, then applies a function to it.
    ///
    /// This method will:
    /// 1. Check if a metric exists for the given labels
    /// 2. If it exists, apply the function to it
    /// 3. If it doesn't exist, create a metric using this family's metric factory and then apply the
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
    /// #     error::Result,
    /// #     raw::LabelSetSchema,
    /// #     metrics::{counter::Counter, family::Family},
    /// #     registry::Registry,
    /// # };
    /// #
    /// # fn main() -> Result<()> {
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
    ///     fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
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

        // Previously we constructed new metrics while holding the write lock, e.g.:
        // let mut write_guard = self.write();
        // let metric =
        // write_guard.entry(labels.clone()).or_insert((self.metric_factory)(labels));
        // func(metric)

        // That approach kept potentially expensive constructors inside the critical section,
        // blocking readers and other writers. We now stage the value in `new_metric` so heavy
        // work happens outside the lock and we can reuse the constructed metric if another
        // thread races to insert the same labels.
        let mut new_metric = None;
        loop {
            // Acquire the write lock only for entry inspection/insertion; construction happens
            // after dropping it.
            let mut write_guard = self.write();
            match write_guard.entry(labels.clone()) {
                Entry::Occupied(entry) => return func(entry.get()),
                Entry::Vacant(entry) => {
                    if let Some(metric) = new_metric.take() {
                        return func(entry.insert(metric));
                    } else {
                        drop(write_guard);
                        // Construct the metric outside the lock so expensive constructors cannot
                        // stall other threads.
                        new_metric = Some((self.metric_factory)(labels));
                    }
                },
            }
        }
    }
}

impl<LS, M: TypedMetric, S> TypedMetric for Family<LS, M, S> {
    const TYPE: MetricType = <M as TypedMetric>::TYPE;
}

impl<LS: LabelSetSchema, M, S> MetricLabelSet for Family<LS, M, S> {
    type LabelSet = LS;
}

impl<LS, M, S> EncodeMetric for Family<LS, M, S>
where
    LS: EncodeLabelSet + Send + Sync,
    M: EncodeMetric,
    S: Send + Sync,
{
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
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
            counter::{Counter, LazyCounter},
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
        fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
            encoder.encode(&("method", &self.method))?;
            encoder.encode(&("status", self.status))?;
            encoder.encode(&("error", self.error))?;
            Ok(())
        }
    }

    impl EncodeLabelValue for Method {
        fn encode(&self, encoder: &mut dyn LabelEncoder) -> Result<()> {
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

    #[test]
    fn test_new_uses_label_aware_factory() {
        let family = Family::<Labels, LazyCounter<u64>>::new_with_labels(|labels| {
            let method = labels.method.clone();
            let status = u64::from(labels.status);
            let error = labels.error.unwrap_or(false);
            LazyCounter::new(move || {
                let method_value = match method {
                    Method::Get => 1_000_u64,
                    Method::Put => 2_000_u64,
                };
                let error_value = if error { 10_000_u64 } else { 0_u64 };
                method_value + error_value + status
            })
        });

        let labels_get = Labels { method: Method::Get, status: 200, error: None };
        let labels_put = Labels { method: Method::Put, status: 404, error: Some(true) };

        let get_total = family.with_or_new(&labels_get, |counter| counter.fetch());
        assert_eq!(get_total, 1_200_u64);

        let get_total_reused = family.with_or_new(&labels_get, |counter| counter.fetch());
        assert_eq!(get_total_reused, 1_200_u64);

        let put_total = family.with_or_new(&labels_put, |counter| counter.fetch());
        assert_eq!(put_total, 12_404_u64);

        assert_eq!(family.with(&labels_get, |counter| counter.fetch()), Some(1_200_u64));
    }
}
