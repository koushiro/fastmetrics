//! Scrape-scoped context for [`LazyGroup`](crate::metrics::lazy_group::LazyGroup).
//!
//! This module provides a thread-local "scrape scope" that lives for the duration of a single
//! encode call (e.g. `format::text::encode`). It enables lazy metrics derived from the same
//! `LazyGroup` to share a single expensive sampling operation per scrape.
//!
//! The scope is intentionally crate-private. Users should interact with this capability via
//! `metrics::LazyGroup`, `metrics::gauge::LazyGauge` and `metrics::counter::LazyCounter`.

use std::{any::Any, cell::RefCell, collections::HashMap};

use crate::metrics::lazy_group::LazyGroupId;

thread_local! {
    static STACK: RefCell<Vec<ScrapeContext>> = const { RefCell::new(Vec::new()) };
}

/// Enters a new scrape scope on the current thread.
///
/// The returned guard will exit the scope when dropped.
///
/// This is designed to be invoked by encoder entrypoints (e.g. `format::text::encode`),
/// so that all metrics encoded during that call can share scrape-scoped caches.
#[inline]
pub(crate) fn enter() -> ScrapeGuard {
    STACK.with(|stack| stack.borrow_mut().push(ScrapeContext::default()));
    ScrapeGuard { _private: () }
}

/// Executes `f` with access to the current scrape context, if one exists.
///
/// Returns `None` if no scrape scope is active on the current thread.
#[inline]
pub(crate) fn with_current<R>(f: impl FnOnce(&mut ScrapeContext) -> R) -> Option<R> {
    STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let ctx = stack.last_mut()?;
        Some(f(ctx))
    })
}

/// A guard object that exits a scrape scope when dropped.
pub(crate) struct ScrapeGuard {
    _private: (),
}

impl Drop for ScrapeGuard {
    fn drop(&mut self) {
        STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            let _ = stack.pop();
        });
    }
}

#[derive(Default)]
pub(crate) struct ScrapeContext {
    // Keyed by LazyGroup id. Values are type-erased samples.
    samples: HashMap<LazyGroupId, Box<dyn Any + Send + Sync>>,
}

impl ScrapeContext {
    /// Gets the cached sample for `key`, initializing it with `init` if absent.
    ///
    /// # Panics
    ///
    /// Panics if a value for `key` exists but is of a different type than `T`.
    #[inline]
    pub(crate) fn get_or_init<T>(&mut self, key: LazyGroupId, init: impl FnOnce() -> T) -> &T
    where
        T: Any + Send + Sync,
    {
        self.samples.entry(key).or_insert_with(|| Box::new(init()));

        // Correctness: a given `LazyGroupId` must only ever be used with a single sample type `T`.
        //
        // If this panics, the scrape context's internal cache has become inconsistent, most likely
        // because the same `LazyGroupId` was used with different sample types in a prior call.
        self.samples
            .get(&key)
            .and_then(|v| v.downcast_ref::<T>())
            .expect("lazy_group::scrape_ctx: cached sample type mismatch")
    }
}
