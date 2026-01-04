//! [Open Metrics Gauge](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#gauge) metric type.
//!
//! See [`Gauge`], [`ConstGauge`] and [`LazyGauge`] for more details.
//!
//! ## Overflow/underflow behavior
//!
//! - Integer gauges (`i32`, `i64`, `isize`) use **wrapping** arithmetic for `inc*`/`dec*` on overflow/underflow by default.
//!   If you need clamping behavior, use the `saturating_*` methods.
//! - Floating-point gauges follow IEEE-754 semantics (they do not saturate; results may become `inf`/`-inf`/`NaN`).

use std::{
    fmt::{self, Debug},
    ops::{AddAssign, SubAssign},
    sync::{Arc, atomic::*},
};

use crate::{
    encoder::{EncodeGaugeValue, EncodeMetric, MetricEncoder},
    error::Result,
    metrics::internal::lazy::{LazySource, PlainLazySource},
    raw::{Atomic, MetricLabelSet, MetricType, Number, TypedMetric},
};

/// A marker trait for **gauge** metric value.
pub trait GaugeValue<Rhs = Self>: Number + AddAssign<Rhs> + SubAssign<Rhs> {
    /// The atomic type corresponding to this gauge value.
    type Atomic: Atomic<Self>;
}

macro_rules! impl_gauge_value_for {
    ($($num:ident => $atomic:ident);* $(;)?) => ($(
        impl GaugeValue for $num {
            type Atomic = $atomic;
        }
    )*);
}

impl_gauge_value_for! {
    i32 => AtomicI32;
    i64 => AtomicI64;
    isize => AtomicIsize;

    f32 => AtomicU32;
    f64 => AtomicU64;
}

/// Gauge values that support saturating arithmetic helpers.
///
/// This is intentionally limited to integer types where saturating add/sub are well-defined.
pub trait SaturatingGaugeValue: GaugeValue {
    /// Saturating addition.
    ///
    /// Implementations must clamp at the numeric maximum instead of wrapping on overflow.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating subtraction.
    ///
    /// Implementations must clamp at the numeric minimum instead of wrapping on underflow.
    fn saturating_sub(self, rhs: Self) -> Self;
}

macro_rules! impl_saturating_gauge_value_for_integer {
    ($($ty:ty),* $(,)?) => {
        $(
            impl SaturatingGaugeValue for $ty {
                #[inline]
                fn saturating_add(self, rhs: Self) -> Self {
                    <$ty>::saturating_add(self, rhs)
                }

                #[inline]
                fn saturating_sub(self, rhs: Self) -> Self {
                    <$ty>::saturating_sub(self, rhs)
                }
            }
        )*
    };
}

impl_saturating_gauge_value_for_integer! { i32, i64, isize }

/// Open Metrics [`Gauge`] metric, which is used to record current measurements,
/// such as bytes of memory currently used or the number of items in a queue.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::gauge::Gauge;
/// #
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
    ///
    /// For integer gauges, this uses wrapping arithmetic on overflow.
    /// Use [`Gauge::saturating_inc`] if you need clamping semantics.
    #[inline]
    pub fn inc(&self) {
        self.value.inc_by(N::ONE);
    }

    /// Increases the [`Gauge`] by `v`.
    ///
    /// For integer gauges, this uses wrapping arithmetic on overflow.
    /// Use [`Gauge::saturating_inc_by`] if you need clamping semantics.
    #[inline]
    pub fn inc_by(&self, v: N) {
        self.value.inc_by(v);
    }

    /// Decreases the [`Gauge`] by 1.
    ///
    /// For integer gauges, this uses wrapping arithmetic on underflow.
    /// Use [`Gauge::saturating_dec`] if you need clamping semantics.
    #[inline]
    pub fn dec(&self) {
        self.value.dec_by(N::ONE);
    }

    /// Decreases the [`Gauge`] by `v`.
    ///
    /// For integer gauges, this uses wrapping arithmetic on underflow.
    /// Use [`Gauge::saturating_dec_by`] if you need clamping semantics.
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

impl<N: SaturatingGaugeValue> Gauge<N> {
    /// Saturating variant of [`Gauge::inc`].
    ///
    /// For integer gauges, this increment clamps at the numeric maximum instead of wrapping around.
    #[inline]
    pub fn saturating_inc(&self) {
        self.saturating_inc_by(N::ONE);
    }

    /// Saturating variant of [`Gauge::inc_by`].
    ///
    /// For integer gauges, this increment clamps at the numeric maximum instead of wrapping around.
    #[inline]
    pub fn saturating_inc_by(&self, v: N) {
        self.value.update(|old| old.saturating_add(v));
    }

    /// Saturating variant of [`Gauge::dec`].
    ///
    /// For integer gauges, this decrement clamps at the numeric minimum instead of wrapping around.
    #[inline]
    pub fn saturating_dec(&self) {
        self.saturating_dec_by(N::ONE);
    }

    /// Saturating variant of [`Gauge::dec_by`].
    ///
    /// For integer gauges, this decrement clamps at the numeric minimum instead of wrapping around.
    #[inline]
    pub fn saturating_dec_by(&self, v: N) {
        self.value.update(|old| old.saturating_sub(v));
    }
}

impl<N: GaugeValue> MetricLabelSet for Gauge<N> {
    type LabelSet = ();
}

impl<N: EncodeGaugeValue + GaugeValue> EncodeMetric for Gauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
        encoder.encode_gauge(&self.get())
    }
}

/// A **constant** `Gauge`, meaning it cannot be changed once created.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::gauge::ConstGauge;
/// #
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

impl<N> TypedMetric for ConstGauge<N> {
    const TYPE: MetricType = MetricType::Gauge;
}

impl<N> MetricLabelSet for ConstGauge<N> {
    type LabelSet = ();
}

impl<N: EncodeGaugeValue + GaugeValue> EncodeMetric for ConstGauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
        encoder.encode_gauge(&self.get())
    }
}

/// A `Gauge` whose value is produced lazily every time it is encoded.
///
/// This is ideal for process or system metrics that should only consult the OS
/// (e.g. `/proc`, cgroups) or other expensive sources at scrape time.
///
/// # Example
///
/// ```rust
/// # use std::sync::atomic::{AtomicI64, Ordering};
/// #
/// # use fastmetrics::metrics::gauge::LazyGauge;
/// #
/// let lazy = LazyGauge::new({
///     let value = AtomicI64::new(42);
///     move || value.load(Ordering::Relaxed)
/// });
/// assert_eq!(lazy.fetch(), 42);
/// ```
///
/// # Grouped sampling
///
/// When constructed via [`crate::metrics::lazy_group::LazyGroup`], multiple lazy gauges can share a
/// single expensive sample per scrape.
pub struct LazyGauge<N> {
    source: Arc<dyn LazySource<N>>,
}

impl<N> Clone for LazyGauge<N> {
    fn clone(&self) -> Self {
        Self { source: self.source.clone() }
    }
}

impl<N: GaugeValue + 'static> LazyGauge<N> {
    /// Internal: constructs a lazy gauge from a source.
    ///
    /// This is used by crate-internal glue (e.g. `metrics::lazy_group`) to build a `LazyGauge`
    /// without exposing additional public types.
    pub(crate) fn from_source(source: Arc<dyn LazySource<N>>) -> Self {
        Self { source }
    }

    /// Creates a new [`LazyGauge`] from the provided fetcher function or closure.
    pub fn new(fetch: impl Fn() -> N + Send + Sync + 'static) -> Self {
        Self::from_source(Arc::new(PlainLazySource::new(Arc::new(fetch))))
    }

    /// Evaluates the underlying fetcher and returns the current value.
    ///
    /// Mainly intended for debugging or tests; regular metric collection should
    /// let the encoder trigger the fetch during scrapes.
    #[inline]
    pub fn fetch(&self) -> N {
        self.source.load()
    }
}

impl<N> TypedMetric for LazyGauge<N> {
    const TYPE: MetricType = MetricType::Gauge;
}

impl<N> MetricLabelSet for LazyGauge<N> {
    type LabelSet = ();
}

impl<N: EncodeGaugeValue + GaugeValue + 'static> EncodeMetric for LazyGauge<N> {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
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
