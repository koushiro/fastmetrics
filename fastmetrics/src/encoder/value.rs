use std::fmt;

/// Trait for encoding unknown numeric values in metrics.
pub trait UnknownValueEncoder {
    /// Encodes a 32-bit signed integer value.
    fn encode_i32(&mut self, value: i32) -> fmt::Result;
    /// Encodes a 64-bit signed integer value.
    fn encode_i64(&mut self, value: i64) -> fmt::Result;
    /// Encodes a platform-specific signed integer value.
    fn encode_isize(&mut self, value: isize) -> fmt::Result;
    /// Encodes a 32-bit unsigned integer value.
    fn encode_u32(&mut self, value: u32) -> fmt::Result;

    /// Encodes a 32-bit floating point value.
    fn encode_f32(&mut self, value: f32) -> fmt::Result;
    /// Encodes a 64-bit floating point value.
    fn encode_f64(&mut self, value: f64) -> fmt::Result;
}

/// Trait for encoding unknown metric values.
pub trait EncodeUnknownValue {
    /// Encodes the value using the provided [`UnknownValueEncoder`].
    fn encode(&self, encoder: &mut dyn UnknownValueEncoder) -> fmt::Result;
}

macro_rules! impl_encode_unknown_value {
    ($($value:ty),*) => (
        paste::paste! { $(
            impl EncodeUnknownValue for $value {
                fn encode(&self, encoder: &mut dyn UnknownValueEncoder) -> fmt::Result {
                    encoder.[<encode_ $value>](*self)
                }
            }
        )* }
    )
}

impl_encode_unknown_value! { i32, i64, isize, u32, f32, f64 }

/// Trait for encoding gauge numeric values in metrics.
pub trait GaugeValueEncoder {
    /// Encodes a 32-bit signed integer value.
    fn encode_i32(&mut self, value: i32) -> fmt::Result;
    /// Encodes a 64-bit signed integer value.
    fn encode_i64(&mut self, value: i64) -> fmt::Result;
    /// Encodes a platform-specific signed integer value.
    fn encode_isize(&mut self, value: isize) -> fmt::Result;
    /// Encodes a 32-bit unsigned integer value.
    fn encode_u32(&mut self, value: u32) -> fmt::Result;
    /// Encodes a 64-bit unsigned integer value.
    fn encode_u64(&mut self, value: u64) -> fmt::Result;

    /// Encodes a 32-bit floating point value.
    fn encode_f32(&mut self, value: f32) -> fmt::Result;
    /// Encodes a 64-bit floating point value.
    fn encode_f64(&mut self, value: f64) -> fmt::Result;
}

/// Trait for encoding gauge metric values.
pub trait EncodeGaugeValue {
    /// Encodes the gauge value using the provided [`GaugeValueEncoder`].
    fn encode(&self, encoder: &mut dyn GaugeValueEncoder) -> fmt::Result;
}

macro_rules! impl_encode_gauge_value {
    ($($value:ty),*) => (
        paste::paste! { $(
            impl EncodeGaugeValue for $value {
                fn encode(&self, encoder: &mut dyn GaugeValueEncoder) -> fmt::Result {
                    encoder.[<encode_ $value>](*self)
                }
            }
        )* }
    )
}

impl_encode_gauge_value! { i32, i64, isize, u32, u64, f32, f64 }

/// Trait for encoding counter numeric values in metrics.
pub trait CounterValueEncoder {
    /// Encodes a 32-bit unsigned integer value.
    fn encode_u32(&mut self, value: u32) -> fmt::Result;
    /// Encodes a 64-bit unsigned integer value.
    fn encode_u64(&mut self, value: u64) -> fmt::Result;
    /// Encodes a platform-specific unsigned integer value.
    fn encode_usize(&mut self, value: usize) -> fmt::Result;

    /// Encodes a 32-bit floating point value.
    fn encode_f32(&mut self, value: f32) -> fmt::Result;
    /// Encodes a 64-bit floating point value.
    fn encode_f64(&mut self, value: f64) -> fmt::Result;
}

/// Trait for encoding counter metric values.
pub trait EncodeCounterValue {
    /// Encodes the counter value using the provided [`CounterValueEncoder`].
    fn encode(&self, encoder: &mut dyn CounterValueEncoder) -> fmt::Result;
}

macro_rules! impl_encode_counter_value {
    ($($value:ty),*) => (
        paste::paste! { $(
            impl EncodeCounterValue for $value {
                fn encode(&self, encoder: &mut dyn CounterValueEncoder) -> fmt::Result {
                    encoder.[<encode_ $value>](*self)
                }
            }
        )* }
    )
}

impl_encode_counter_value! { u32, u64, usize, f32, f64 }
