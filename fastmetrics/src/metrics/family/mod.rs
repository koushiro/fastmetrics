//! Metric family implementations.
//!
//! This module provides two family implementations:
//! - [`Family`]: General-purpose dynamic label families backed by a hash map.
//! - [`IndexedFamily`]: Fixed-cardinality label families backed by indexed storage.
//! - [`LabelIndexMapping`]: Trait mapping labels to stable indexes.
//! - [`LabelIndex`]: Reusable index token for fast indexed lookups.
//!
//! ## Choosing Between `Family` and `IndexedFamily`
//!
//! Use [`Family`] when label values are dynamic, sparse, or potentially unbounded.
//! It allocates metrics lazily and only encodes label sets that were actually observed.
//!
//! Use [`IndexedFamily`] when label combinations are fixed and finite (for example,
//! enum-like dimensions such as `{direction=in|out}`), and update latency on the hot
//! path matters more than sparse storage. It avoids hash lookups by mapping labels to
//! indexes, pre-allocates fixed slots, and lazily initializes/encodes only labels
//! observed via indexed lookups.

mod dynamic;
pub mod indexed;

type MetricFactory<LS, M> = dyn Fn(&LS) -> M + Send + Sync + 'static;

pub use self::{
    dynamic::Family,
    indexed::{IndexedFamily, LabelIndex, LabelIndexMapping},
};
