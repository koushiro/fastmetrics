//! Grouped source implementations for [`LazyGroup`](crate::metrics::lazy_group::LazyGroup).
//!
//! These types power "sample once per scrape" behavior for lazy metrics derived from the same
//! [`LazyGroup`]. They are crate-private and constructed by crate-internal glue.
//
// NOTE: This module is intentionally *not* user-facing. Users should only interact with `LazyGroup`
// and `LazyGauge`/`LazyCounter`.

use std::{marker::PhantomData, sync::Arc, time::Duration};

use crate::{
    encoder::{EncodeCounterValue, EncodeGaugeValue},
    metrics::internal::lazy::LazySource,
    metrics::{counter::LazyCounter, gauge::LazyGauge, lazy_group::LazyGroup},
};

/// Constructs a `LazyCounter` derived from the shared `LazyGroup` sample.
///
/// `created` controls whether the counter should expose a `_created` timestamp in the OpenMetrics
/// output.
pub(crate) fn counter_from_group<S, N, M>(
    group: LazyGroup<S>,
    map: M,
    created: Option<Duration>,
) -> LazyCounter<N>
where
    S: Send + Sync + 'static,
    M: Fn(&S) -> N + Send + Sync + 'static,
    N: EncodeCounterValue + Send + Sync + 'static,
{
    LazyCounter::from_source(
        Arc::new(GroupedLazySource::<S, N, _>::new(group, Arc::new(map))),
        created,
    )
}

/// Constructs a `LazyGauge` derived from the shared `LazyGroup` sample.
///
/// This lives in the `lazy_group` module so the "grouped" wiring remains localized and the
/// `LazyGauge` type stays focused on encoding behavior.
pub(crate) fn gauge_from_group<S, N, M>(group: LazyGroup<S>, map: M) -> LazyGauge<N>
where
    S: Send + Sync + 'static,
    M: Fn(&S) -> N + Send + Sync + 'static,
    N: EncodeGaugeValue + Send + Sync + 'static,
{
    LazyGauge::from_source(Arc::new(GroupedLazySource::<S, N, _>::new(group, Arc::new(map))))
}

/// A lazy source whose value is derived from a shared per-scrape sample.
pub(crate) struct GroupedLazySource<S, N, M> {
    pub(crate) group: LazyGroup<S>,
    pub(crate) map: Arc<M>,
    pub(crate) _marker: PhantomData<N>,
}

impl<S, N, M> GroupedLazySource<S, N, M> {
    #[inline]
    pub(crate) fn new(group: LazyGroup<S>, map: Arc<M>) -> Self {
        Self { group, map, _marker: PhantomData }
    }
}

impl<S, N, M> LazySource<N> for GroupedLazySource<S, N, M>
where
    S: Send + Sync + 'static,
    M: Fn(&S) -> N + Send + Sync + 'static,
    N: Send + Sync + 'static,
{
    #[inline]
    fn load(&self) -> N {
        let map = self.map.as_ref();

        if let Some(r) = super::scrape_ctx::with_current(|ctx| {
            let sample = ctx.get_or_init::<S>(self.group.id, || (self.group.sample.as_ref())());
            map(sample)
        }) {
            r
        } else {
            let sample = (self.group.sample.as_ref())();
            map(&sample)
        }
    }
}
