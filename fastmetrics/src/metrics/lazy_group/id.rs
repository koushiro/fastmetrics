//! Internal identifier for a [`LazyGroup`](crate::metrics::lazy_group::LazyGroup).
//!
//! This newtype exists to make scrape-scoped caching keys type-safe and self-documenting.
//!
//! It is crate-private because it is used by the encoder scrape context and by grouped lazy
//! metric sources, but it is not intended as a user-facing API.

use std::{
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};

/// A unique identifier for a `LazyGroup` instance.
///
/// `LazyGroupId` is guaranteed to be non-zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct LazyGroupId(NonZeroU64);

impl LazyGroupId {
    #[inline]
    pub(crate) fn new(value: NonZeroU64) -> Self {
        Self(value)
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Generates a new unique [`LazyGroupId`].
#[inline]
pub(crate) fn next_lazy_group_id() -> LazyGroupId {
    // We start at 1 and never set 0, so `NonZeroU64` is always valid.
    // Wraparound would require ~1.8e19 calls, which is not realistic.
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    LazyGroupId::new(NonZeroU64::new(id).expect("LazyGroupId must be non-zero"))
}
