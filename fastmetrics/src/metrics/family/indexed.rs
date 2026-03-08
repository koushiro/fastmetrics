//! Fixed-cardinality metric family implementation.
//!
//! Unlike [`super::Family`], [`IndexedFamily`] uses an index mapping provided by
//! [`LabelIndexMapping`] to avoid hashing during lookups.
//!
//! ## When to use
//!
//! [`IndexedFamily`] is a good fit when:
//! - the label domain is fixed and fully enumerable,
//! - cardinality is small enough to pre-allocate eagerly,
//! - the update path is very hot and should avoid hash map lookup costs.
//!
//! Typical examples are small enum-like labels such as HTTP method, role, or
//! network direction (`in`/`out`).
//!
//! ## Tradeoffs
//!
//! - Construction allocates slot storage for each label combination.
//! - Metrics are lazily initialized per slot on first indexed access.
//! - Export only includes labels whose metric slots were initialized through
//!   [`IndexedFamily::get`], [`IndexedFamily::get_by_index`], or [`IndexedFamily::with`].
//! - Label schemas with high or dynamic cardinality should use [`super::Family`].

use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    sync::{Arc, OnceLock},
};

use super::MetricFactory;
use crate::{
    encoder::{EncodeLabelSet, EncodeMetric, MetricEncoder},
    error::Result,
    raw::{LabelSetSchema, MetricLabelSet, MetricType, TypedMetric},
};

/// Labels (or label components) that can be mapped to and from stable indexes.
///
/// This trait powers indexed families by providing fixed-cardinality mapping.
pub trait LabelIndexMapping {
    /// Total number of valid values.
    const CARDINALITY: usize;

    /// Maps the current value to an index in `[0, Self::CARDINALITY)`.
    fn index(&self) -> usize;

    /// Reconstructs a value from a valid index.
    fn from_index(index: usize) -> Self;
}

impl LabelIndexMapping for () {
    const CARDINALITY: usize = 1;

    #[inline]
    fn index(&self) -> usize {
        0
    }

    #[inline]
    fn from_index(index: usize) -> Self {
        assert_eq!(index, 0);
    }
}

impl LabelIndexMapping for bool {
    const CARDINALITY: usize = 2;

    #[inline]
    fn index(&self) -> usize {
        match self {
            true => 1,
            false => 0,
        }
    }

    #[inline]
    fn from_index(index: usize) -> Self {
        match index {
            0 => false,
            1 => true,
            _ => panic!("invalid label index"),
        }
    }
}

/// A reusable index token derived from a fixed-cardinality label set.
///
/// Use [`LabelIndex::new`] to compute an index once, then reuse it across
/// multiple indexed families that share the same label schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LabelIndex<LS> {
    index: usize,
    _marker: PhantomData<fn() -> LS>,
}

impl<LS> LabelIndex<LS>
where
    LS: LabelIndexMapping,
{
    /// Creates a reusable index token from label values.
    #[inline]
    pub fn new(labels: &LS) -> Self {
        let index = labels.index();
        debug_assert!(index < LS::CARDINALITY, "label index out of bounds");
        Self { index, _marker: PhantomData }
    }

    #[inline]
    pub(crate) fn as_usize(&self) -> usize {
        self.index
    }
}

/// A fixed-cardinality metric family backed by indexed storage.
///
/// `LS` must provide a stable, total mapping between label values and indexes via
/// [`LabelIndexMapping`].
pub struct IndexedFamily<LS, M> {
    labels: Arc<[LS]>,
    metrics: Arc<[OnceLock<M>]>,
    metric_factory: Arc<MetricFactory<LS, M>>,
    _marker: PhantomData<fn() -> LS>,
}

impl<LS, M> Clone for IndexedFamily<LS, M> {
    fn clone(&self) -> Self {
        Self {
            labels: self.labels.clone(),
            metrics: self.metrics.clone(),
            metric_factory: self.metric_factory.clone(),
            _marker: PhantomData,
        }
    }
}

impl<LS, M> Debug for IndexedFamily<LS, M>
where
    M: Debug + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexedFamily")
            .field("cardinality", &self.labels.len())
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl<LS, M> Default for IndexedFamily<LS, M>
where
    LS: LabelIndexMapping,
    M: Default + 'static,
{
    fn default() -> Self {
        Self::new(M::default)
    }
}

impl<LS, M> IndexedFamily<LS, M>
where
    LS: LabelIndexMapping,
{
    /// Creates a fixed-cardinality family with a label-independent metric factory.
    pub fn new(metric_factory: impl Fn() -> M + Send + Sync + 'static) -> Self {
        Self::new_with_labels(move |_| metric_factory())
    }

    /// Creates a fixed-cardinality family with a label-aware metric factory.
    ///
    /// The factory is invoked lazily and at most once for each index in
    /// `0..LS::CARDINALITY` when that slot is first accessed.
    pub fn new_with_labels(metric_factory: impl Fn(&LS) -> M + Send + Sync + 'static) -> Self {
        let mut labels = Vec::with_capacity(LS::CARDINALITY);
        let mut metrics = Vec::with_capacity(LS::CARDINALITY);

        for index in 0..LS::CARDINALITY {
            labels.push(LS::from_index(index));
            metrics.push(OnceLock::new());
        }

        Self {
            labels: Arc::from(labels.into_boxed_slice()),
            metrics: Arc::from(metrics.into_boxed_slice()),
            metric_factory: Arc::new(metric_factory),
            _marker: PhantomData,
        }
    }

    /// Returns the metric for `index`.
    ///
    /// # Panics
    ///
    /// Panics if the index token does not belong to this label schema.
    #[inline]
    pub fn get_by_index(&self, index: LabelIndex<LS>) -> &M {
        let raw_index = index.as_usize();
        let labels = self.labels.get(raw_index).expect("label index out of bounds");
        let slot = self.metrics.get(raw_index).expect("label index out of bounds");
        slot.get_or_init(|| (self.metric_factory)(labels))
    }

    /// Returns the metric for `labels`.
    #[inline]
    pub fn get(&self, labels: &LS) -> &M {
        self.get_by_index(LabelIndex::new(labels))
    }

    /// Applies a function to the metric identified by `labels`.
    #[inline]
    pub fn with<R, F>(&self, labels: &LS, func: F) -> R
    where
        F: FnOnce(&M) -> R,
    {
        func(self.get(labels))
    }
}

impl<LS, M: TypedMetric> TypedMetric for IndexedFamily<LS, M> {
    const TYPE: MetricType = <M as TypedMetric>::TYPE;
}

impl<LS: LabelSetSchema, M> MetricLabelSet for IndexedFamily<LS, M> {
    type LabelSet = LS;
}

impl<LS, M> EncodeMetric for IndexedFamily<LS, M>
where
    LS: EncodeLabelSet + Send + Sync,
    M: EncodeMetric,
{
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
        debug_assert_eq!(self.labels.len(), self.metrics.len(), "indexed family storage mismatch");

        for (labels, slot) in self.labels.iter().zip(self.metrics.iter()) {
            let Some(metric) = slot.get() else {
                continue;
            };
            if metric.is_empty() {
                continue;
            }
            encoder.encode(labels, metric)?;
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.metrics.iter().all(|slot| match slot.get() {
            None => true,
            Some(metric) => metric.is_empty(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use super::*;
    use crate::{
        encoder::{EncodeLabelSet, EncodeLabelValue, LabelEncoder, LabelSetEncoder},
        metrics::{check_text_encoding, counter::Counter},
        raw::LabelSetSchema,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct Labels {
        method: Method,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum Method {
        Get,
        Put,
    }

    impl LabelSetSchema for Labels {
        fn names() -> Option<&'static [&'static str]> {
            Some(&["method"])
        }
    }

    impl EncodeLabelSet for Labels {
        fn encode(&self, encoder: &mut dyn LabelSetEncoder) -> Result<()> {
            encoder.encode(&("method", self.method))
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

    impl LabelIndexMapping for Labels {
        const CARDINALITY: usize = 2;

        fn index(&self) -> usize {
            match self.method {
                Method::Get => 0,
                Method::Put => 1,
            }
        }

        fn from_index(index: usize) -> Self {
            let method = match index {
                0 => Method::Get,
                1 => Method::Put,
                _ => panic!("invalid label index"),
            };
            Self { method }
        }
    }

    #[test]
    fn test_indexed_family() {
        let family = IndexedFamily::<Labels, Counter>::default();

        let get = Labels { method: Method::Get };
        let put = Labels { method: Method::Put };

        family.with(&get, |metric| metric.inc());
        family.with(&put, |metric| metric.inc_by(2));

        assert_eq!(family.with(&get, |metric| metric.total()), 1);
        assert_eq!(family.with(&put, |metric| metric.total()), 2);
    }

    #[test]
    fn test_indexed_family_getters() {
        let family = IndexedFamily::<Labels, Counter>::default();

        let get = Labels { method: Method::Get };
        let put = Labels { method: Method::Put };

        family.get(&get).inc();
        family.get(&put).inc_by(2);

        assert_eq!(family.get(&get).total(), 1);
        assert_eq!(family.get(&put).total(), 2);
    }

    #[test]
    fn test_indexed_family_label_index() {
        let family = IndexedFamily::<Labels, Counter>::default();

        let get = Labels { method: Method::Get };
        let put = Labels { method: Method::Put };

        let get_index = LabelIndex::new(&get);
        let put_index = LabelIndex::new(&put);

        family.get_by_index(get_index).inc();
        family.get_by_index(put_index).inc_by(2);

        assert_eq!(family.get_by_index(get_index).total(), 1);
        assert_eq!(family.get_by_index(put_index).total(), 2);
    }

    #[test]
    fn test_indexed_family_text_encoding() {
        check_text_encoding(
            |registry| {
                let family = IndexedFamily::<Labels, Counter>::default();
                registry
                    .register("http_requests", "Total HTTP requests", family.clone())
                    .unwrap();

                let get = Labels { method: Method::Get };
                family.with(&get, |metric| metric.inc());
            },
            |output| {
                assert!(output.contains(r#"http_requests_total{method="GET"} 1"#));
                assert!(!output.contains(r#"http_requests_total{method="PUT"} 0"#));
            },
        );
    }

    #[test]
    fn test_indexed_family_text_encoding_initialized_but_not_updated() {
        check_text_encoding(
            |registry| {
                let family = IndexedFamily::<Labels, Counter>::default();
                registry
                    .register("http_requests", "Total HTTP requests", family.clone())
                    .unwrap();

                let put = Labels { method: Method::Put };
                family.get(&put);
            },
            |output| {
                assert!(output.contains(r#"http_requests_total{method="PUT"} 0"#));
                assert!(!output.contains(r#"http_requests_total{method="GET"}"#));
            },
        );
    }

    #[test]
    fn test_indexed_family_lazy_init_per_slot() {
        let init_calls = Arc::new(AtomicUsize::new(0));
        let init_calls_ref = init_calls.clone();
        let family = IndexedFamily::<Labels, Counter>::new_with_labels(move |_| {
            init_calls_ref.fetch_add(1, Ordering::Relaxed);
            Counter::default()
        });

        let get = Labels { method: Method::Get };
        let put = Labels { method: Method::Put };

        assert_eq!(init_calls.load(Ordering::Relaxed), 0);

        family.get(&get);
        family.get(&get);
        assert_eq!(init_calls.load(Ordering::Relaxed), 1);

        family.get(&put);
        assert_eq!(init_calls.load(Ordering::Relaxed), 2);
    }
}
