//! [Open Metrics Counter](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#counter) metric type.
//!
//! See [`Counter`], [`ConstCounter`] and [`LazyCounter`] for more details.

use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::AddAssign,
    sync::{Arc, atomic::*},
    time::Duration,
};

use crate::{
    encoder::{EncodeCounterValue, EncodeMetric, MetricEncoder},
    raw::{Atomic, MetricLabelSet, MetricType, Number, TypedMetric},
};

/// A marker trait for **counter** metric value.
pub trait CounterValue<Rhs = Self>: Number + AddAssign<Rhs> {
    /// The atomic type corresponding to this counter value.
    type Atomic: Atomic<Self>;
}

macro_rules! impl_counter_value_for {
    ($($num:ident => $atomic:ident),*) => ($(
        impl CounterValue for $num {
            type Atomic = $atomic;
        }
    )*);
}

impl_counter_value_for! {
    u32 => AtomicU32,
    u64 => AtomicU64,
    usize => AtomicUsize,
    f32 => AtomicU32,
    f64 => AtomicU64
}

/// Open Metrics [`Counter`] metric, which is used to measure discrete events.
///
/// # Example
///
/// ```rust
/// # use std::time::SystemTime;
/// # use fastmetrics::metrics::counter::Counter;
/// // Create a default counter
/// let counter = <Counter>::default();
/// assert_eq!(counter.total(), 0);
/// assert!(counter.created().is_none());
///
/// // Increment by 1
/// counter.inc();
/// assert_eq!(counter.total(), 1);
///
/// // Increment by custom value
/// counter.inc_by(5);
/// assert_eq!(counter.total(), 6);
///
/// // Create a counter with created timestamp
/// let created = SystemTime::UNIX_EPOCH
///     .elapsed()
///     .expect("UNIX timestamp when the counter was created");
/// let counter = <Counter>::with_created(created);
/// assert!(counter.created().is_some());
/// ```
pub struct Counter<N: CounterValue = u64> {
    total: Arc<N::Atomic>,
    // UNIX timestamp
    created: Option<Duration>,
}

impl<N: CounterValue> Clone for Counter<N> {
    fn clone(&self) -> Self {
        Self { total: self.total.clone(), created: self.created }
    }
}

impl<N: CounterValue> Debug for Counter<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = self.total();
        let created = self.created();

        f.debug_struct("Counter")
            .field("total", &total)
            .field("created", &created)
            .finish()
    }
}

impl<N: CounterValue> Default for Counter<N> {
    fn default() -> Self {
        Self { total: Arc::new(Default::default()), created: None }
    }
}

impl<N: CounterValue> Counter<N> {
    /// Creates a [`Counter`] with a `created` timestamp.
    pub fn with_created(created: Duration) -> Self {
        Self { total: Default::default(), created: Some(created) }
    }

    /// Increases the [`Counter`] by 1, returning the previous value.
    #[inline]
    pub fn inc(&self) -> N {
        self.total.inc()
    }

    /// Increases the [`Counter`] by `v`, returning the previous value.
    #[inline]
    pub fn inc_by(&self, v: N) -> N {
        assert!(v >= N::ZERO);
        self.total.inc_by(v)
    }

    /// Sets the [`Counter`] to `v`.
    ///
    /// # Panics
    ///
    /// This function will panic if the new value `v` is less than the current counter value.
    /// This is because counters must be monotonically increasing.
    #[inline]
    pub fn set(&self, v: N) {
        assert!(v >= self.total.get(), "counter must be monotonically increasing");
        self.total.set(v)
    }

    /// Gets the current `total` value of the [`Counter`].
    #[inline]
    pub fn total(&self) -> N {
        self.total.get()
    }

    /// Gets the optional `created` value of the [`Counter`].
    pub const fn created(&self) -> Option<Duration> {
        self.created
    }
}

impl<N: CounterValue> TypedMetric for Counter<N> {
    const TYPE: MetricType = MetricType::Counter;
}

impl<N: CounterValue> MetricLabelSet for Counter<N> {
    type LabelSet = ();
}

impl<N: EncodeCounterValue + CounterValue> EncodeMetric for Counter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let total = self.total();
        let created = self.created();
        encoder.encode_counter(&total, None, created)
    }
}

/// A **constant** `Counter`, meaning it cannot be changed once created.
///
/// # Example
///
/// ```rust
/// # use std::time::SystemTime;
/// # use fastmetrics::metrics::counter::ConstCounter;
/// // Create a constant counter with initial value
/// let counter = ConstCounter::new(42_u64);
/// assert_eq!(counter.total(), 42);
/// assert!(counter.created().is_none());
///
/// // Create a constant counter with created timestamp
/// let created = SystemTime::UNIX_EPOCH
///     .elapsed()
///     .expect("UNIX timestamp when the counter was created");
/// let counter = ConstCounter::with_created(42_u64, created);
/// assert_eq!(counter.total(), 42);
/// assert!(counter.created().is_some());
/// ```
#[derive(Clone)]
pub struct ConstCounter<N = u64> {
    total: N,
    // UNIX timestamp
    created: Option<Duration>,
}

impl<N: CounterValue> Debug for ConstCounter<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = self.total();
        let created = self.created();

        f.debug_struct("ConstCounter")
            .field("total", &total)
            .field("created", &created)
            .finish()
    }
}

impl<N: CounterValue> Default for ConstCounter<N> {
    fn default() -> Self {
        Self { total: N::ZERO, created: None }
    }
}

impl<N: CounterValue> ConstCounter<N> {
    /// Creates a new [`ConstCounter`] with a constant `total` value.
    pub const fn new(total: N) -> Self {
        Self { total, created: None }
    }

    /// Creates a [`ConstCounter`] with a constant `total` value and a `created` timestamp.
    pub fn with_created(total: N, created: Duration) -> Self {
        Self { total, created: Some(created) }
    }

    /// Gets the current `total` value of the [`ConstCounter`].
    #[inline]
    pub const fn total(&self) -> N {
        self.total
    }

    /// Gets the optional `created` value of the [`ConstCounter`].
    #[inline]
    pub const fn created(&self) -> Option<Duration> {
        self.created
    }
}

impl<N: CounterValue> TypedMetric for ConstCounter<N> {
    const TYPE: MetricType = MetricType::Counter;
}

impl<N: CounterValue> MetricLabelSet for ConstCounter<N> {
    type LabelSet = ();
}

impl<N: EncodeCounterValue + CounterValue> EncodeMetric for ConstCounter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let total = self.total();
        let created = self.created();
        encoder.encode_counter(&total, None, created)
    }
}

/// A `Counter` whose value is produced lazily every time it is encoded.
///
/// Ideal for process or system metrics that should only consult the OS
/// (e.g. `/proc`, cgroups) or other expensive sources when Prometheus scrapes
/// the registry.
///
/// # Example
/// ```rust
/// # use std::sync::atomic::{AtomicU64, Ordering};
/// # use fastmetrics::metrics::counter::LazyCounter;
/// let lazy = LazyCounter::new({
///     let total = AtomicU64::new(42);
///     move || total.load(Ordering::Relaxed)
/// });
/// assert_eq!(lazy.fetch(), 42);
/// ```
pub struct LazyCounter<F, N> {
    fetch: Arc<F>,
    created: Option<Duration>,
    _marker: PhantomData<N>,
}

impl<F, N> LazyCounter<F, N>
where
    F: Fn() -> N,
{
    /// Creates a `LazyCounter` without a creation timestamp.
    pub fn new(fetch: F) -> Self {
        Self { fetch: Arc::new(fetch), created: None, _marker: PhantomData }
    }

    /// Creates a `LazyCounter` with the provided `created` timestamp.
    pub fn with_created(fetch: F, created: Duration) -> Self {
        Self { fetch: Arc::new(fetch), created: Some(created), _marker: PhantomData }
    }

    /// Evaluates the underlying fetcher and returns the current total.
    ///
    /// Mainly intended for debugging or tests; regular metric collection should
    /// let the encoder trigger the fetch during scrapes.
    #[inline]
    pub fn fetch(&self) -> N {
        (self.fetch.as_ref())()
    }
}

impl<F, N> Clone for LazyCounter<F, N> {
    fn clone(&self) -> Self {
        Self { fetch: Arc::clone(&self.fetch), created: self.created, _marker: PhantomData }
    }
}

impl<F, N> TypedMetric for LazyCounter<F, N> {
    const TYPE: MetricType = MetricType::Counter;
}

impl<F, N> MetricLabelSet for LazyCounter<F, N> {
    type LabelSet = ();
}

impl<F, N> EncodeMetric for LazyCounter<F, N>
where
    F: Fn() -> N + Send + Sync,
    N: EncodeCounterValue + Send + Sync,
{
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let total = self.fetch();
        encoder.encode_counter(&total, None, self.created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{metrics::check_text_encoding, registry::Unit};

    #[test]
    fn test_counter_initialization() {
        let counter = <Counter>::default();
        assert_eq!(counter.total(), 0);
        assert!(counter.created().is_none());

        let created = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("UNIX timestamp when the counter was created");
        let counter = <Counter>::with_created(created);
        assert_eq!(counter.total(), 0);
        assert!(counter.created().is_some());
    }

    #[test]
    fn test_counter_inc() {
        let counter = <Counter>::default();
        let clone = counter.clone();

        assert_eq!(counter.total(), 0);
        counter.inc();
        assert_eq!(counter.total(), 1);
        counter.inc();
        assert_eq!(counter.total(), 2);

        assert_eq!(clone.total(), 2);
        clone.inc();
        assert_eq!(counter.total(), 3);
    }

    #[test]
    fn test_counter_inc_by() {
        let counter = <Counter>::default();

        assert_eq!(counter.total(), 0);
        counter.inc_by(5);
        assert_eq!(counter.total(), 5);
        counter.inc_by(3);
        assert_eq!(counter.total(), 8);
    }

    #[test]
    fn test_counter_set() {
        let counter = <Counter>::default();
        let clone = counter.clone();

        counter.set(42);
        assert_eq!(counter.total(), 42);
        assert_eq!(counter.total(), 42);

        clone.set(100);
        assert_eq!(clone.total(), 100);
        assert_eq!(counter.total(), 100);
    }

    #[test]
    #[should_panic(expected = "counter must be monotonically increasing")]
    fn test_counter_set_panic() {
        let counter = <Counter>::default();
        let clone = counter.clone();

        counter.set(42);
        assert_eq!(counter.total(), 42);
        assert_eq!(counter.total(), 42);

        clone.set(10);
    }

    #[test]
    fn test_counter_thread_safe() {
        let counter = <Counter>::default();
        let clone = counter.clone();

        let handle = std::thread::spawn(move || {
            for _ in 0..1000 {
                clone.inc();
            }
        });

        for _ in 0..1000 {
            counter.inc();
        }

        handle.join().unwrap();
        assert_eq!(counter.total(), 2000);
    }

    #[test]
    fn test_const_counter() {
        let counter = ConstCounter::new(42_u64);
        assert_eq!(counter.total(), 42);
        assert!(counter.created.is_none());

        let created = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("UNIX timestamp when the counter was created");
        let counter = ConstCounter::with_created(42_u64, created);
        assert_eq!(counter.total(), 42);
        assert!(counter.created.is_some());

        let clone = counter.clone();
        assert_eq!(clone.total(), 42);
    }

    #[test]
    fn test_text_encoding() {
        check_text_encoding(
            |registry| {
                let counter = <Counter>::default();
                registry.register("my_counter", "My counter help", counter.clone()).unwrap();
                counter.inc_by(100);
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_counter counter
                    # HELP my_counter My counter help
                    my_counter_total 100
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );

        let created = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("SystemTime when the counter was created");
        check_text_encoding(
            |registry| {
                let counter = <Counter>::with_created(created);
                registry.register("my_counter", "My counter help", counter.clone()).unwrap();
                counter.inc_by(100);
            },
            |output| {
                let expected = indoc::formatdoc! {r#"
                    # TYPE my_counter counter
                    # HELP my_counter My counter help
                    my_counter_total 100
                    my_counter_created {}.{}
                    # EOF
                    "#,
                    created.as_secs(),
                    created.as_millis() % 1000
                };
                assert_eq!(expected, output);
            },
        );

        check_text_encoding(
            |registry| {
                let counter = <Counter>::default();
                registry
                    .register_with_unit(
                        "my_counter",
                        "My counter help",
                        Unit::Bytes,
                        counter.clone(),
                    )
                    .unwrap();
                counter.inc_by(100);
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_counter_bytes counter
                    # HELP my_counter_bytes My counter help
                    # UNIT my_counter_bytes bytes
                    my_counter_bytes_total 100
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );

        check_text_encoding(
            |registry| {
                let counter = <ConstCounter>::new(42u64);
                registry.register("my_counter", "My counter help", counter.clone()).unwrap();
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_counter counter
                    # HELP my_counter My counter help
                    my_counter_total 42
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );
    }

    #[test]
    fn test_lazy_counter() {
        check_text_encoding(
            |registry| {
                let total = Arc::new(AtomicU64::new(0));
                let lazy = LazyCounter::new({
                    let total = total.clone();
                    move || total.load(Ordering::Relaxed)
                });
                registry.register("lazy_counter", "Lazy counter help", lazy).unwrap();
                total.store(123, Ordering::Relaxed);
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE lazy_counter counter
                    # HELP lazy_counter Lazy counter help
                    lazy_counter_total 123
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );

        let created = Duration::from_secs(123);
        check_text_encoding(
            |registry| {
                let lazy = LazyCounter::with_created(|| 42_u64, created);
                registry
                    .register("lazy_counter_created", "Lazy counter with created help", lazy)
                    .unwrap();
            },
            |output| {
                let expected = indoc::formatdoc! {r#"
                    # TYPE lazy_counter_created counter
                    # HELP lazy_counter_created Lazy counter with created help
                    lazy_counter_created_total 42
                    lazy_counter_created_created {}.{}
                    # EOF
                    "#,
                    created.as_secs(),
                    created.as_millis() % 1000
                };
                assert_eq!(expected, output);
            },
        );
    }
}
