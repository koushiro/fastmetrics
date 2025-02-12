use std::sync::atomic::*;

use crate::metrics::raw::number::Number;

/// Atomic operations for the [Counter] or [Gauge] value.
///
/// [Counter]: super::counter::Counter
/// [Gauge]: super::gauge::Gauge
pub trait Atomic<N: Number>: Default {
    /// Increase the value by `1`.
    #[inline]
    fn inc(&self) -> N {
        self.inc_by(N::ONE)
    }

    /// Increase the value by `v`.
    fn inc_by(&self, v: N) -> N;

    /// Decrease the value by `1`.
    #[inline]
    fn dec(&self) -> N {
        self.dec_by(N::ONE)
    }

    /// Decrease the value.
    fn dec_by(&self, v: N) -> N;

    /// Set the value.
    fn set(&self, v: N) -> N;

    /// Get the value.
    fn get(&self) -> N;
}

macro_rules! impl_atomic_for_integer {
    ($($ty:ident, $atomic:ident, $size:expr)*) => ($(
        #[cfg(target_has_atomic = $size)]
        impl Atomic<$ty> for $atomic {
            fn inc_by(&self, v: $ty) -> $ty {
                self.fetch_add(v, Ordering::Relaxed)
            }

            fn dec_by(&self, v: $ty) -> $ty {
                self.fetch_sub(v, Ordering::Relaxed)
            }

            fn set(&self, v: $ty) -> $ty {
                self.swap(v, Ordering::Relaxed)
            }

            fn get(&self) -> $ty {
                self.load(Ordering::Relaxed)
            }
        }
    )*);
}

impl_atomic_for_integer! {
    i32, AtomicI32, "32"
    i64, AtomicI64, "64"
    isize, AtomicIsize, "ptr"
    u32, AtomicU32, "32"
    u64, AtomicU64, "64"
    usize, AtomicUsize, "ptr"
}

macro_rules! impl_atomic_for_float  {
    ($($ty:ident, $atomic:ident, $size:expr)*) => ($(
        #[cfg(target_has_atomic = $size)]
        impl Atomic<$ty> for $atomic {
            fn inc_by(&self, v: $ty) -> $ty {
                let mut old_u = self.load(Ordering::Relaxed);

                let mut old_f;
                loop {
                    old_f = $ty::from_bits(old_u);
                    let new = $ty::to_bits(old_f + v);
                    match self.compare_exchange_weak(old_u, new, Ordering::Relaxed, Ordering::Relaxed) {
                        Ok(_) => break,
                        Err(x) => old_u = x,
                    }
                }
                old_f
            }

            fn dec_by(&self, v: $ty) -> $ty {
                let mut old_u = self.load(Ordering::Relaxed);

                let mut old_f;
                loop {
                    old_f = $ty::from_bits(old_u);
                    let new = $ty::to_bits(old_f - v);
                    match self.compare_exchange_weak(old_u, new, Ordering::Relaxed, Ordering::Relaxed) {
                        Ok(_) => break,
                        Err(x) => old_u = x,
                    }
                }
                old_f
            }

            fn set(&self, v: $ty) -> $ty {
                let old_u = self.swap($ty::to_bits(v), Ordering::Relaxed);
                $ty::from_bits(old_u)
            }

            fn get(&self) -> $ty {
                let value = self.load(Ordering::Relaxed);
                $ty::from_bits(value)
            }
        }
    )*);
}

impl_atomic_for_float! {
    f32, AtomicU32, "32"
    f64, AtomicU64, "64"
}
