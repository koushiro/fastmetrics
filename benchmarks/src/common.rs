use std::sync::Arc;

use metrics::{Counter, Gauge, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit};
use metrics_util::registry::{AtomicStorage, Registry as StorageRegistry};

type RecorderRegistry = StorageRegistry<Key, AtomicStorage>;

#[derive(Clone)]
struct BenchRecorder {
    registry: Arc<RecorderRegistry>,
}

impl Default for BenchRecorder {
    fn default() -> Self {
        Self::new(Arc::new(RecorderRegistry::atomic()))
    }
}

impl BenchRecorder {
    fn new(registry: Arc<RecorderRegistry>) -> Self {
        Self { registry }
    }
}

impl Recorder for BenchRecorder {
    fn describe_counter(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}

    fn describe_gauge(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}

    fn describe_histogram(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}

    fn register_counter(&self, key: &Key, _: &Metadata<'_>) -> Counter {
        self.registry
            .get_or_create_counter(key, |counter| Counter::from_arc(counter.clone()))
    }

    fn register_gauge(&self, key: &Key, _: &Metadata<'_>) -> Gauge {
        self.registry.get_or_create_gauge(key, |gauge| Gauge::from_arc(gauge.clone()))
    }

    fn register_histogram(&self, key: &Key, _: &Metadata<'_>) -> Histogram {
        self.registry
            .get_or_create_histogram(key, |hist| Histogram::from_arc(hist.clone()))
    }
}

/// Runs the provided closure with the benchmark recorder installed as a local recorder.
pub fn with_metrics_recorder<R, F>(func: F) -> R
where
    F: FnOnce() -> R,
{
    let recorder = BenchRecorder::default();
    metrics::with_local_recorder(&recorder, func)
}
