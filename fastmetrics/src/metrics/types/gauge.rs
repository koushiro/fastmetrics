//! [Open Metrics Gauge](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#gauge) metric type.
//!
//! See [`Gauge`], [`ConstGauge`] and [`LazyGauge`] for more details.

use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{AddAssign, SubAssign},
    sync::{Arc, atomic::*},
};

use crate::{
    encoder::{EncodeGaugeValue, EncodeMetric, MetricEncoder},
    raw::{Atomic, MetricLabelSet, MetricType, Number, TypedMetric},
};

/// A marker trait for **gauge** metric value.
pub trait GaugeValue<Rhs = Self>: Number + AddAssign<Rhs> + SubAssign<Rhs> {
    /// The atomic type corresponding to this gauge value.
    type Atomic: Atomic<Self>;
}

macro_rules! impl_gauge_value_for {
    ($($num:ident => $atomic:ident),*) => ($(
        impl GaugeValue for $num {
            type Atomic = $atomic;
        }
    )*);
}

impl_gauge_value_for! {
    i32 => AtomicI32,
    i64 => AtomicI64,
    isize => AtomicIsize,
    u32 => AtomicU32,
    u64 => AtomicU64,
    f32 => AtomicU32,
    f64 => AtomicU64
}

/// Open Metrics [`Gauge`] metric, which is used to record current measurements,
/// such as bytes of memory currently used or the number of items in a queue.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::gauge::Gauge;
/// // Create a default gauge
/// let gauge = <Gauge>::default();
/// assert_eq!(gauge.get(), 0);
///
/// // Create a gauge with initial value
/// let gauge = <Gauge>::new(42);
/// assert_eq!(gauge.get(), 42);
///
/// // Increment and decrement
/// gauge.inc();
/// assert_eq!(gauge.get(), 43);
/// gauge.dec();
/// assert_eq!(gauge.get(), 42);
///
/// // Increment and decrement by custom values
/// gauge.inc_by(10);
/// assert_eq!(gauge.get(), 52);
/// gauge.dec_by(5);
/// assert_eq!(gauge.get(), 47);
///
/// // Set to specific value
/// gauge.set(-10);
/// assert_eq!(gauge.get(), -10);
/// ```
pub struct Gauge<N: GaugeValue = i64> {
    value: Arc<N::Atomic>,
}

impl<N: GaugeValue> Clone for Gauge<N> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl<N: GaugeValue> Debug for Gauge<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Gauge").field("value", &self.get()).finish()
    }
}

impl<N: GaugeValue> Default for Gauge<N> {
    fn default() -> Self {
        Self { value: Arc::new(Default::default()) }
    }
}

impl<N: GaugeValue> Gauge<N> {
    /// Creates a new [`Gauge`] with an initial value.
    pub fn new(value: N) -> Self {
        let this = Self::default();
        this.set(value);
        this
    }

    /// Increases the [`Gauge`] by 1.
    #[inline]
    pub fn inc(&self) {
        self.value.inc();
    }

    /// Increases the [`Gauge`] by `v`.
    #[inline]
    pub fn inc_by(&self, v: N) {
        self.value.inc_by(v);
    }

    /// Decreases the [`Gauge`] by 1.
    #[inline]
    pub fn dec(&self) {
        self.value.dec();
    }

    /// Decreases the [`Gauge`] by `v`.
    #[inline]
    pub fn dec_by(&self, v: N) {
        self.value.dec_by(v);
    }

    /// Sets the [`Gauge`] to `v`.
    #[inline]
    pub fn set(&self, v: N) {
        self.value.set(v);
    }

    /// Gets the current value of the [`Gauge`].
    #[inline]
    pub fn get(&self) -> N {
        self.value.get()
    }
}

impl<N: GaugeValue> TypedMetric for Gauge<N> {
    const TYPE: MetricType = MetricType::Gauge;
}

impl<N: GaugeValue> MetricLabelSet for Gauge<N> {
    type LabelSet = ();
}

impl<N: EncodeGaugeValue + GaugeValue> EncodeMetric for Gauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_gauge(&self.get())
    }
}

/// A **constant** `Gauge`, meaning it cannot be changed once created.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::gauge::ConstGauge;
/// // Create a constant gauge with initial value
/// let gauge = ConstGauge::new(42);
/// assert_eq!(gauge.get(), 42);
/// ```
#[derive(Clone, Debug, Default)]
pub struct ConstGauge<N = i64> {
    value: N,
}

impl<N: GaugeValue> ConstGauge<N> {
    /// Creates a new [`ConstGauge`] with a constant value.
    pub const fn new(value: N) -> Self {
        Self { value }
    }

    /// Gets the current value of the [`ConstGauge`].
    #[inline]
    pub const fn get(&self) -> N {
        self.value
    }
}

impl<N: GaugeValue> TypedMetric for ConstGauge<N> {
    const TYPE: MetricType = MetricType::Gauge;
}

impl<N: GaugeValue> MetricLabelSet for ConstGauge<N> {
    type LabelSet = ();
}

impl<N: EncodeGaugeValue + GaugeValue> EncodeMetric for ConstGauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_gauge(&self.get())
    }
}

/// A `Gauge` whose value is produced lazily every time it is encoded.
///
/// This is ideal for process or system metrics that should only consult the OS
/// (e.g. `/proc`, cgroups) or other expensive sources at scrape time.
///
/// # Example
/// ```rust
/// # use std::sync::atomic::{AtomicI64, Ordering};
/// # use fastmetrics::metrics::gauge::LazyGauge;
/// let lazy = LazyGauge::new({
///     let value = AtomicI64::new(42);
///     move || value.load(Ordering::Relaxed)
/// });
/// assert_eq!(lazy.fetch(), 42);
/// ```
pub struct LazyGauge<F, N> {
    fetch: Arc<F>,
    _marker: PhantomData<N>,
}

impl<F, N> LazyGauge<F, N>
where
    F: Fn() -> N,
{
    /// Creates a new [`LazyGauge`] from the provided fetcher function or closure.
    pub fn new(fetch: F) -> Self {
        Self { fetch: Arc::new(fetch), _marker: PhantomData }
    }

    /// Evaluates the underlying fetcher and returns the current value.
    ///
    /// Mainly intended for debugging or tests; regular metric collection should
    /// let the encoder trigger the fetch during scrapes.
    #[inline]
    pub fn fetch(&self) -> N {
        (self.fetch.as_ref())()
    }
}

impl<F, N> Clone for LazyGauge<F, N> {
    fn clone(&self) -> Self {
        Self { fetch: Arc::clone(&self.fetch), _marker: PhantomData }
    }
}

impl<F, N> TypedMetric for LazyGauge<F, N> {
    const TYPE: MetricType = MetricType::Gauge;
}

impl<F, N> MetricLabelSet for LazyGauge<F, N> {
    type LabelSet = ();
}

impl<F, N> EncodeMetric for LazyGauge<F, N>
where
    F: Fn() -> N + Send + Sync,
    N: EncodeGaugeValue + Send + Sync,
{
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        let value = self.fetch();
        encoder.encode_gauge(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{metrics::check_text_encoding, registry::Unit};

    #[test]
    fn test_gauge_initialization() {
        let gauge = <Gauge>::default();
        assert_eq!(gauge.get(), 0);

        let gauge = Gauge::new(42_i64);
        assert_eq!(gauge.get(), 42);
    }

    #[test]
    fn test_gauge_inc_dec() {
        let gauge = <Gauge>::default();

        assert_eq!(gauge.get(), 0);
        gauge.inc();
        assert_eq!(gauge.get(), 1);
        gauge.dec();
        assert_eq!(gauge.get(), 0);
    }

    #[test]
    fn test_gauge_inc_dec_by() {
        let gauge = <Gauge>::default();

        assert_eq!(gauge.get(), 0);
        gauge.inc_by(5);
        assert_eq!(gauge.get(), 5);
        gauge.inc_by(-1);
        assert_eq!(gauge.get(), 4);

        gauge.dec_by(3);
        assert_eq!(gauge.get(), 1);
        gauge.dec_by(-1);
        assert_eq!(gauge.get(), 2);
    }

    #[test]
    fn test_gauge_set() {
        let gauge = <Gauge>::default();
        let clone = gauge.clone();

        gauge.set(42);
        assert_eq!(gauge.get(), 42);
        assert_eq!(clone.get(), 42);

        clone.set(-10);
        assert_eq!(clone.get(), -10);
        assert_eq!(gauge.get(), -10);
    }

    #[test]
    fn test_gauge_thread_safe() {
        let gauge = <Gauge>::default();
        let clone = gauge.clone();

        let handle = std::thread::spawn(move || {
            for _ in 0..1000 {
                clone.inc();
            }
        });

        for _ in 0..1000 {
            gauge.inc();
        }

        handle.join().unwrap();
        assert_eq!(gauge.get(), 2000);
    }

    #[test]
    fn test_const_gauge() {
        let gauge = ConstGauge::new(42_i64);
        assert_eq!(gauge.get(), 42);

        let clone = gauge.clone();
        assert_eq!(clone.get(), 42);
    }

    #[test]
    fn test_text_encoding() {
        check_text_encoding(
            |registry| {
                let gauge = <Gauge>::default();
                registry.register("my_gauge", "My gauge help", gauge.clone()).unwrap();
                gauge.set(100);
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_gauge gauge
                    # HELP my_gauge My gauge help
                    my_gauge 100
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );

        check_text_encoding(
            |registry| {
                let gauge = <Gauge>::default();
                registry
                    .register_with_unit("my_gauge", "My gauge help", Unit::Bytes, gauge.clone())
                    .unwrap();
                gauge.set(100);
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_gauge_bytes gauge
                    # HELP my_gauge_bytes My gauge help
                    # UNIT my_gauge_bytes bytes
                    my_gauge_bytes 100
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );

        check_text_encoding(
            |registry| {
                let gauge = <ConstGauge>::new(42i64);
                registry.register("my_gauge", "My gauge help", gauge.clone()).unwrap();
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_gauge gauge
                    # HELP my_gauge My gauge help
                    my_gauge 42
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );
    }

    #[test]
    fn test_lazy_gauge() {
        check_text_encoding(
            |registry| {
                let value = Arc::new(AtomicI64::new(0));
                let lazy = LazyGauge::new({
                    let value = value.clone();
                    move || value.load(Ordering::Relaxed)
                });
                registry.register("lazy_gauge", "Lazy gauge help", lazy).unwrap();
                value.store(99, Ordering::Relaxed);
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE lazy_gauge gauge
                    # HELP lazy_gauge Lazy gauge help
                    lazy_gauge 99
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );
    }
}
