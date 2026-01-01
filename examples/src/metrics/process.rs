use std::sync::LazyLock;

use fastmetrics::{
    derive::*,
    metrics::{
        gauge::{ConstGauge, LazyGauge},
        lazy_group::LazyGroup,
    },
};
use parking_lot::Mutex;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, get_current_pid};

/// Custom process metrics.
#[derive(Clone, Register)]
pub struct ProcessMetrics {
    /// Process ID.
    pid: ConstGauge<u32>,
    /// Total CPU time consumed by the current process in seconds.
    cpu_seconds: LazyGauge<f64>,
    /// CPU usage of the current process in percent.
    cpu_usage_percent: LazyGauge<f32>,
    /// Resident Set Size (RSS) of the current process in bytes.
    #[register(unit(Bytes))]
    resident_memory: LazyGauge<u64>,
    /// Virtual memory size of the current process in bytes.
    #[register(unit(Bytes))]
    virtual_memory: LazyGauge<u64>,
    /// Process start time in seconds since the Unix epoch.
    #[register(unit(Seconds))]
    start_time: LazyGauge<u64>,
    /// Process run time in seconds.
    #[register(unit(Seconds))]
    run_time: LazyGauge<u64>,
    /// Number of open file descriptors for the current process.
    open_fds: LazyGauge<u32>,
    /// Limit of open file descriptors for the current process.
    max_open_fds: LazyGauge<u32>,
    /// Number of threads for the current process.
    threads: LazyGauge<u32>,
}

impl Default for ProcessMetrics {
    fn default() -> Self {
        let group: LazyGroup<ProcessSample> = LazyGroup::new(|| PROCESS_SAMPLER.sample());
        Self {
            pid: ConstGauge::new(PROCESS_SAMPLER.pid.as_u32()),
            cpu_seconds: group.gauge(|s: &ProcessSample| s.cpu_seconds_total),
            cpu_usage_percent: group.gauge(|s: &ProcessSample| s.cpu_usage_percent),
            resident_memory: group.gauge(|s: &ProcessSample| s.resident_memory_bytes),
            virtual_memory: group.gauge(|s: &ProcessSample| s.virtual_memory_bytes),
            start_time: group.gauge(|s: &ProcessSample| s.start_time_seconds),
            run_time: group.gauge(|s: &ProcessSample| s.run_time_seconds),
            open_fds: group.gauge(|s: &ProcessSample| s.open_fds),
            max_open_fds: group.gauge(|s: &ProcessSample| s.max_open_fds),
            threads: group.gauge(|s: &ProcessSample| s.thread_count),
        }
    }
}

pub static PROCESS_SAMPLER: LazyLock<ProcessSampler> = LazyLock::new(|| {
    let pid = get_current_pid().expect("Unknown platform");
    ProcessSampler::new(pid)
});

#[derive(Clone, Copy, Default)]
struct ProcessSample {
    cpu_seconds_total: f64,
    cpu_usage_percent: f32,
    resident_memory_bytes: u64,
    virtual_memory_bytes: u64,
    start_time_seconds: u64,
    run_time_seconds: u64,
    open_fds: u32,
    max_open_fds: u32,
    thread_count: u32,
}

pub struct ProcessSampler {
    pid: Pid,
    system: Mutex<System>,
}

impl ProcessSampler {
    pub fn new(pid: Pid) -> Self {
        let mut system = System::new();
        sample(&mut system, pid);
        Self { pid, system: Mutex::new(system) }
    }

    fn sample(&self) -> ProcessSample {
        let mut system = self.system.lock();
        sample(&mut system, self.pid)
    }
}

fn sample(system: &mut System, pid: Pid) -> ProcessSample {
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        true,
        ProcessRefreshKind::everything(),
    );
    system
        .process(pid)
        .map(|p| ProcessSample {
            cpu_seconds_total: p.accumulated_cpu_time() as f64 / 1_000.0,
            cpu_usage_percent: p.cpu_usage(),
            resident_memory_bytes: p.memory(),
            virtual_memory_bytes: p.virtual_memory(),
            start_time_seconds: p.start_time(),
            run_time_seconds: p.run_time(),
            open_fds: p.open_files().unwrap_or(0) as u32,
            max_open_fds: p.open_files_limit().unwrap_or(0) as u32,
            thread_count: p.tasks().map(|t| t.len()).unwrap_or(0) as u32,
        })
        .unwrap_or_default()
}
