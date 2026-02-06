//! [Open Metrics Counter](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#counter) metric type.
//!
//! See [`Counter`], [`ConstCounter`] and [`LazyCounter`] for more details.
//!
//! ## Overflow/underflow behavior
//!
//! - Integer counters (`u32`, `u64`, `usize`) use **wrapping** arithmetic for `inc*` on overflow by
//!   default. If you need clamping behavior, use the `saturating_*` methods.
//! - Floating-point counters follow IEEE-754 semantics (they do not saturate; results may become
//!   `inf`/`NaN`).

use std::{
    fmt::{self, Debug},
    ops::AddAssign,
    sync::{Arc, atomic::*},
    time::Duration,
};

use crate::{
    encoder::{EncodeCounterValue, EncodeMetric, MetricEncoder},
    error::Result,
    metrics::internal::lazy::{LazySource, PlainLazySource},
    raw::{Atomic, MetricLabelSet, MetricType, Number, TypedMetric},
};

/// A marker trait for **counter** metric value.
pub trait CounterValue<Rhs = Self>: Number + AddAssign<Rhs> {
    /// The atomic type corresponding to this counter value.
    type Atomic: Atomic<Self>;
}

macro_rules! impl_counter_value_for {
    ($($num:ident => $atomic:ident);* $(;)?) => ($(
        impl CounterValue for $num {
            type Atomic = $atomic;
        }
    )*);
}

impl_counter_value_for! {
    u32 => AtomicU32;
    u64 => AtomicU64;
    usize => AtomicUsize;

    f32 => AtomicU32;
    f64 => AtomicU64;
}

/// Counter values that support `saturating_*` operations.
///
/// This is implemented for integer counter value types only.
pub trait SaturatingCounterValue: CounterValue {
    /// Saturating addition.
    ///
    /// This clamps at the numeric maximum instead of wrapping around on overflow.
    fn saturating_add(self, rhs: Self) -> Self;
}

macro_rules! impl_saturating_counter_value_for_uint {
    ($($ty:ty),* $(,)?) => {$(
        impl SaturatingCounterValue for $ty {
            #[inline]
            fn saturating_add(self, rhs: Self) -> Self {
                <$ty>::saturating_add(self, rhs)
            }
        }
    )*};
}

impl_saturating_counter_value_for_uint! { u32, u64, usize }

/// Open Metrics [`Counter`] metric, which is used to measure discrete events.
///
/// # Example
///
/// ```rust
/// # use std::time::SystemTime;
/// #
/// # use fastmetrics::metrics::counter::Counter;
/// #
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

    /// Increases the [`Counter`] by 1.
    ///
    /// For integer counters, this uses wrapping arithmetic on overflow.
    /// Use [`Counter::saturating_inc`] if you need clamping semantics.
    #[inline]
    pub fn inc(&self) {
        self.total.inc_by(N::ONE);
    }

    /// Increases the [`Counter`] by `v`.
    ///
    /// For integer counters, this uses wrapping arithmetic on overflow.
    /// Use [`Counter::saturating_inc_by`] if you need clamping semantics.
    ///
    /// # Panics
    ///
    /// This function will panic if the increment `v` is negative (i.e, not zero or positive).
    #[inline]
    pub fn inc_by(&self, v: N) {
        assert!(v >= N::ZERO, "increment must be zero or positive");
        self.total.inc_by(v);
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
        self.total.set(v);
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

impl<N: SaturatingCounterValue> Counter<N> {
    /// Increases the [`Counter`] by 1, saturating at the numeric maximum.
    ///
    /// This is slower than [`Counter::inc`] because it uses a CAS loop.
    #[inline]
    pub fn saturating_inc(&self) {
        self.saturating_inc_by(N::ONE);
    }

    /// Increases the [`Counter`] by `v`, saturating at the numeric maximum.
    ///
    /// This is slower than [`Counter::inc_by`] because it uses a CAS loop.
    ///
    /// # Panics
    ///
    /// This function will panic if the increment `v` is negative (i.e, not zero or positive).
    #[inline]
    pub fn saturating_inc_by(&self, v: N) {
        assert!(v >= N::ZERO, "increment must be zero or positive");
        self.total.update(|old| old.saturating_add(v));
    }
}

impl<N: CounterValue> TypedMetric for Counter<N> {
    const TYPE: MetricType = MetricType::Counter;
}

impl<N: CounterValue> MetricLabelSet for Counter<N> {
    type LabelSet = ();
}

impl<N: EncodeCounterValue + CounterValue> EncodeMetric for Counter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
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
/// #
/// # use fastmetrics::metrics::counter::ConstCounter;
/// #
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
    pub const fn with_created(total: N, created: Duration) -> Self {
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

impl<N> TypedMetric for ConstCounter<N> {
    const TYPE: MetricType = MetricType::Counter;
}

impl<N> MetricLabelSet for ConstCounter<N> {
    type LabelSet = ();
}

impl<N: EncodeCounterValue + CounterValue> EncodeMetric for ConstCounter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
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
///
/// ```rust
/// # use std::sync::atomic::{AtomicU64, Ordering};
/// #
/// # use fastmetrics::metrics::counter::LazyCounter;
/// #
/// let lazy = LazyCounter::new({
///     let total = AtomicU64::new(42);
///     move || total.load(Ordering::Relaxed)
/// });
/// assert_eq!(lazy.fetch(), 42);
/// ```
///
/// # Grouped sampling
///
/// When constructed via [`crate::metrics::lazy_group::LazyGroup`], multiple lazy counters can share
/// a single expensive sample per scrape.
pub struct LazyCounter<N> {
    source: Arc<dyn LazySource<N>>,
    created: Option<Duration>,
}

impl<N> Clone for LazyCounter<N> {
    fn clone(&self) -> Self {
        Self { source: self.source.clone(), created: self.created }
    }
}

impl<N: CounterValue + 'static> LazyCounter<N> {
    /// Internal: constructs a `LazyCounter` from an implementation of [`CounterSource`].
    ///
    /// This is used by crate-internal glue (e.g., `metrics::lazy_group`) to build a `LazyCounter`
    /// without exposing additional public types.
    pub(crate) fn from_source(source: Arc<dyn LazySource<N>>, created: Option<Duration>) -> Self {
        Self { source, created }
    }

    /// Creates a `LazyCounter` without a creation timestamp.
    pub fn new(fetch: impl Fn() -> N + Send + Sync + 'static) -> Self {
        Self::from_source(Arc::new(PlainLazySource::new(Arc::new(fetch))), None)
    }

    /// Creates a `LazyCounter` with the provided `created` timestamp.
    pub fn with_created(fetch: impl Fn() -> N + Send + Sync + 'static, created: Duration) -> Self {
        Self::from_source(Arc::new(PlainLazySource::new(Arc::new(fetch))), Some(created))
    }

    /// Evaluates the underlying fetcher and returns the current total.
    ///
    /// Mainly intended for debugging or tests; regular metric collection should
    /// let the encoder trigger the fetch during scrapes.
    #[inline]
    pub fn fetch(&self) -> N {
        self.source.load()
    }

    /// Gets the optional `created` value of the [`LazyCounter`].
    pub fn created(&self) -> Option<Duration> {
        self.created
    }
}

impl<N> TypedMetric for LazyCounter<N> {
    const TYPE: MetricType = MetricType::Counter;
}

impl<N> MetricLabelSet for LazyCounter<N> {
    type LabelSet = ();
}

impl<N: EncodeCounterValue + CounterValue + 'static> EncodeMetric for LazyCounter<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
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
    fn test_counter_saturating_inc() {
        // clamps at max
        let counter = <Counter<u64>>::default();
        counter.set(u64::MAX);

        counter.saturating_inc();
        assert_eq!(counter.total(), u64::MAX);

        counter.saturating_inc_by(1);
        assert_eq!(counter.total(), u64::MAX);

        counter.saturating_inc_by(123);
        assert_eq!(counter.total(), u64::MAX);

        // behaves normally in range
        let counter = <Counter<u64>>::default();

        counter.saturating_inc_by(5);
        assert_eq!(counter.total(), 5);

        counter.saturating_inc();
        assert_eq!(counter.total(), 6);

        counter.saturating_inc_by(3);
        assert_eq!(counter.total(), 9);
    }

    #[test]
    fn test_const_counter() {
        let counter = ConstCounter::new(42u64);
        assert_eq!(counter.total(), 42);
        assert!(counter.created.is_none());

        let created = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("UNIX timestamp when the counter was created");
        let counter = ConstCounter::with_created(42u64, created);
        assert_eq!(counter.total(), 42);
        assert!(counter.created.is_some());

        let clone = counter.clone();
        assert_eq!(clone.total(), 42);
    }

    #[test]
    fn test_lazy_counter() {
        let total = Arc::new(AtomicU64::new(42));
        let counter = LazyCounter::new({
            let total = total.clone();
            move || total.load(Ordering::Relaxed)
        });
        assert_eq!(counter.fetch(), 42);

        let created = Duration::from_secs(123);
        let counter = LazyCounter::with_created(|| 42u64, created);
        assert_eq!(counter.fetch(), 42);
        assert_eq!(counter.created(), Some(created));
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
}
