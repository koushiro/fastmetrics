use std::fmt::Debug;

/// A marker trait for number types.
pub trait Number: Copy + PartialOrd + Debug + Send + Sync {
    /// The multiplicative identity element of Self, 0.
    const ZERO: Self;
    /// The multiplicative identity element of Self, 1.
    const ONE: Self;
}

macro_rules! impl_number {
    ($($num:ty => $zero:expr, $one:expr);* $(;)?) => ($(
        impl Number for $num {
            const ZERO: Self = $zero;
            const ONE: Self = $one;
        }
    )*)
}

impl_number! {
    i32 => 0i32, 1i32;
    i64 => 0i64, 1i64;
    isize => 0isize, 1isize;
    u32 => 0u32, 1u32;
    u64 => 0u64, 1u64;
    usize => 0usize, 1usize;
    f32 => 0.0f32, 1.0f32;
    f64 => 0.0f64, 1.0f64;
}
