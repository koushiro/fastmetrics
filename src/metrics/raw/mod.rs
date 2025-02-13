//! Raw metric types and implementations.
//!
//! This module contains the low-level implementations of metric types like numbers and atomic
//! values, which serve as the foundation for higher-level metric abstractions.

mod atomic;
pub(crate) mod bucket;
mod number;

pub use self::{atomic::Atomic, number::Number};
