use std::{sync::LazyLock, time::Instant};

use fastmetrics::{
    derive::*,
    metrics::{counter::LazyCounter, gauge::LazyGauge},
};
use parking_lot::Mutex;
use sysinfo::{
    MINIMUM_CPU_UPDATE_INTERVAL, Pid, ProcessRefreshKind, ProcessesToUpdate, System,
    get_current_pid,
};

#[derive(Clone, Register)]
pub struct ProcessMetrics {
    /// Process ID.
    pid: LazyGauge<fn() -> u32, u32>,
    /// Total CPU time consumed by the current process in seconds.
    cpu_seconds: LazyCounter<fn() -> f64, f64>,
    /// CPU usage of the current process in percent.
    cpu_usage_percent: LazyGauge<fn() -> f32, f32>,
    /// Resident Set Size (RSS) of the current process in bytes.
    #[register(unit(Bytes))]
    resident_memory: LazyGauge<fn() -> u64, u64>,
    /// Virtual memory size of the current process in bytes.
    #[register(unit(Bytes))]
    virtual_memory: LazyGauge<fn() -> u64, u64>,
    /// Process start time in seconds since the Unix epoch.
    #[register(unit(Seconds))]
    start_time: LazyGauge<fn() -> u64, u64>,
    /// Process run time in seconds.
    #[register(unit(Seconds))]
    run_time: LazyGauge<fn() -> u64, u64>,
    /// Number of open file descriptors for the current process.
    open_fds: LazyCounter<fn() -> usize, usize>,
    /// Limit of open file descriptors for the current process.
    max_open_fds: LazyCounter<fn() -> usize, usize>,
    /// Number of threads for the current process.
    threads: LazyCounter<fn() -> usize, usize>,
}

impl Default for ProcessMetrics {
    fn default() -> Self {
        Self {
            pid: LazyGauge::new(|| PROCESS_SAMPLER.sample().pid),
            cpu_seconds: LazyCounter::new(|| PROCESS_SAMPLER.sample().cpu_seconds_total),
            cpu_usage_percent: LazyGauge::new(|| PROCESS_SAMPLER.sample().cpu_usage_percent),
            resident_memory: LazyGauge::new(|| PROCESS_SAMPLER.sample().resident_memory_bytes),
            virtual_memory: LazyGauge::new(|| PROCESS_SAMPLER.sample().virtual_memory_bytes),
            start_time: LazyGauge::new(|| PROCESS_SAMPLER.sample().start_time_seconds),
            run_time: LazyGauge::new(|| PROCESS_SAMPLER.sample().run_time_seconds),
            open_fds: LazyCounter::new(|| PROCESS_SAMPLER.sample().open_fds),
            max_open_fds: LazyCounter::new(|| PROCESS_SAMPLER.sample().max_open_fds),
            threads: LazyCounter::new(|| PROCESS_SAMPLER.sample().thread_count),
        }
    }
}

pub static PROCESS_SAMPLER: LazyLock<ProcessSampler> = LazyLock::new(|| ProcessSampler::new());

#[derive(Clone, Copy, Default)]
struct ProcessSample {
    pid: u32,
    cpu_seconds_total: f64,
    cpu_usage_percent: f32,
    resident_memory_bytes: u64,
    virtual_memory_bytes: u64,
    start_time_seconds: u64,
    run_time_seconds: u64,
    open_fds: usize,
    max_open_fds: usize,
    thread_count: usize,
}

pub struct ProcessSampler {
    pid: Pid,
    system: Mutex<System>,
    sample: Mutex<Option<ProcessSample>>,
    last_sample_at: Mutex<Option<Instant>>,
}

impl ProcessSampler {
    pub fn new() -> Self {
        let pid = get_current_pid().expect("Unknown platform");
        let mut system = System::new();
        system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[pid]),
            true,
            ProcessRefreshKind::nothing().with_cpu(),
        );

        Self {
            pid,
            system: Mutex::new(system),
            sample: Mutex::new(None),
            last_sample_at: Mutex::new(None),
        }
    }

    fn sample(&self) -> ProcessSample {
        let mut sample_lock = self.sample.lock();
        let mut ts_lock = self.last_sample_at.lock();
        if let (Some(cached), Some(at)) = (*sample_lock, *ts_lock) {
            // elapsed < 200ms
            if at.elapsed() < MINIMUM_CPU_UPDATE_INTERVAL {
                return cached;
            }
        }

        let mut system = self.system.lock();
        system.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[self.pid]),
            false,
            ProcessRefreshKind::everything(),
        );
        let sample = system
            .process(self.pid)
            .map(|p| ProcessSample {
                pid: self.pid.as_u32(),
                cpu_seconds_total: p.accumulated_cpu_time() as f64 / 1_000.0,
                cpu_usage_percent: p.cpu_usage(),
                resident_memory_bytes: p.memory(),
                virtual_memory_bytes: p.virtual_memory(),
                start_time_seconds: p.start_time(),
                run_time_seconds: p.run_time(),
                open_fds: p.open_files().unwrap_or(0),
                max_open_fds: p.open_files_limit().unwrap_or(0),
                thread_count: p.tasks().map(|t| t.len()).unwrap_or(0),
            })
            .unwrap_or_default();
        *sample_lock = Some(sample);
        *ts_lock = Some(Instant::now());
        sample
    }
}
