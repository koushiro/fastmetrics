use std::sync::atomic::*;

use crate::raw::number::Number;

/// Atomic operations for the integer or float value.
///
/// # Note
///
/// `inc_by`/`dec_by` use the underlying atomic add/sub operations.
///
/// For integer atomics, this means they may wrap on overflow/underflow.
/// If you need a stronger guarantee (e.g. saturating arithmetic),
/// use [`Atomic::update`] to implement it via a CAS loop.
///
/// For floating-point atomics, `inc_by`/`dec_by` are implemented via [`Atomic::update`].
pub trait Atomic<N: Number>: Default + Send + Sync {
    /// Increase the value by `v`.
    fn inc_by(&self, v: N);

    /// Decrease the value by `v`.
    fn dec_by(&self, v: N);

    /// Atomically updates the current value.
    fn update<F>(&self, f: F)
    where
        F: FnMut(N) -> N;

    /// Set the value.
    fn set(&self, v: N);

    /// Get the value.
    fn get(&self) -> N;
}

macro_rules! impl_atomic_for_integer {
    ($($ty:ident => $atomic:ident);* $(;)?) => ($(
        impl Atomic<$ty> for $atomic {
            #[inline(always)]
            fn inc_by(&self, v: $ty) {
                self.fetch_add(v, Ordering::Relaxed);
            }

            #[inline(always)]
            fn dec_by(&self, v: $ty) {
                self.fetch_sub(v, Ordering::Relaxed);
            }

            #[inline]
            fn update<F>(&self, mut f: F)
            where
                F: FnMut($ty) -> $ty,
            {
                let _ = self.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| Some(f(old)));
            }

            #[inline(always)]
            fn set(&self, v: $ty) {
                self.store(v, Ordering::Relaxed);
            }

            #[inline(always)]
            fn get(&self) -> $ty {
                self.load(Ordering::Relaxed)
            }
        }
    )*);
}

impl_atomic_for_integer! {
    i32 => AtomicI32;
    i64 => AtomicI64;
    isize => AtomicIsize;

    u32 => AtomicU32;
    u64 => AtomicU64;
    usize => AtomicUsize;
}

macro_rules! impl_atomic_for_float  {
    ($($ty:ident => $atomic:ident);* $(;)?) => ($(
        impl Atomic<$ty> for $atomic {
            #[inline]
            fn inc_by(&self, v: $ty) {
                <Self as Atomic<$ty>>::update(self, |old| old + v);
            }

            #[inline(always)]
            fn dec_by(&self, v: $ty) {
                <Self as Atomic<$ty>>::update(self, |old| old - v);
            }

            #[inline]
            fn update<F>(&self, mut f: F)
            where
                F: FnMut($ty) -> $ty,
            {
                let _ = self.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old_bits| {
                    let old_value = $ty::from_bits(old_bits);
                    let new_value = f(old_value);
                    Some($ty::to_bits(new_value))
                });
            }

            #[inline]
            fn set(&self, v: $ty) {
                self.store($ty::to_bits(v), Ordering::Relaxed);
            }

            #[inline]
            fn get(&self) -> $ty {
                let value = self.load(Ordering::Relaxed);
                $ty::from_bits(value)
            }
        }
    )*);
}

impl_atomic_for_float! {
    f32 => AtomicU32;
    f64 => AtomicU64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_f32() {
        let value = AtomicU32::new(0);

        value.set(100f32);
        let new: f32 = value.get();
        assert_eq!(new, 100f32);

        value.inc_by(10f32);
        let new: f32 = value.get();
        assert_eq!(new, 110f32);

        value.dec_by(10f32);
        let new: f32 = value.get();
        assert_eq!(new, 100f32);
    }

    #[test]
    fn test_atomic_f64() {
        let value = AtomicU64::new(0);

        value.set(100f64);
        let new: f64 = value.get();
        assert_eq!(new, 100f64);

        value.inc_by(10f64);
        let new: f64 = value.get();
        assert_eq!(new, 110f64);

        value.dec_by(10f64);
        let new: f64 = value.get();
        assert_eq!(new, 100f64);
    }

    #[test]
    fn test_atomic_i32_update_saturating_overflow_underflow() {
        let value = AtomicI32::new(0);

        value.set(i32::MAX);
        crate::raw::Atomic::update(&value, |old| old.saturating_add(1));
        assert_eq!(value.get(), i32::MAX);

        value.set(i32::MIN);
        crate::raw::Atomic::update(&value, |old| old.saturating_sub(1));
        assert_eq!(value.get(), i32::MIN);
    }

    #[test]
    fn test_atomic_u64_update_saturating_overflow_underflow() {
        let value = AtomicU64::new(0);

        <AtomicU64 as Atomic<u64>>::set(&value, u64::MAX);
        <AtomicU64 as Atomic<u64>>::update(&value, |old| old.saturating_add(1));
        assert_eq!(<AtomicU64 as Atomic<u64>>::get(&value), u64::MAX);

        <AtomicU64 as Atomic<u64>>::set(&value, 0);
        <AtomicU64 as Atomic<u64>>::update(&value, |old| old.saturating_sub(1));
        assert_eq!(<AtomicU64 as Atomic<u64>>::get(&value), 0);
    }
}
