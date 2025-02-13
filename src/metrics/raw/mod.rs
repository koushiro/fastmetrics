//! Raw metric types and traits.
//!
//! This module contains the low-level implementations of metric types, which serve as the
//! foundation for higher-level metric abstractions.

mod atomic;
pub(crate) mod bucket;
mod number;

pub use self::{atomic::Atomic, number::Number};
