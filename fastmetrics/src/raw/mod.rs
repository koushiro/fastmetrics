//! This module contains the low-level components of metric types, which serve as the
//! foundation for higher-level metric abstractions.
//!
//! These components are typically not used directly but rather through the higher-level
//! metric types provided by the crate.

mod atomic;
pub mod bucket;
mod label_set;
pub mod metadata;
mod number;
pub mod quantile;
mod types;

pub use self::{atomic::Atomic, label_set::*, metadata::*, number::Number, types::*};
