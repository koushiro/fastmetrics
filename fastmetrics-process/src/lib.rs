#![doc = include_str!("../README.md")]

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_crate_dependencies)]

use std::{process, sync::LazyLock};

use fastmetrics::{
    error::Result,
    metrics::{
        counter::LazyCounter,
        gauge::{ConstGauge, LazyGauge},
        lazy_group::LazyGroup,
    },
    registry::{Register, Registry, Unit},
};
use parking_lot::Mutex;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

/// A set of process metrics aligned with Prometheus' standard naming conventions.
///
/// This type implements [`fastmetrics::registry::Register`].
///
/// To get the standard Prometheus-style metric names (`process_*`), register into
/// `registry.subsystem("process")?`.
#[derive(Clone)]
pub struct ProcessMetrics {
    pid: ConstGauge<i64>,
    cpu_seconds_total: LazyCounter<f64>,
    cpu_usage_percent: LazyGauge<f32>,
    resident_memory_bytes: LazyGauge<i64>,
    virtual_memory_bytes: LazyGauge<i64>,
    start_time_seconds: LazyGauge<i64>,
    run_time_seconds: LazyGauge<i64>,
    open_fds: LazyGauge<i64>,
    max_fds: LazyGauge<i64>,
    threads: LazyGauge<i64>,
}

static PROCESS_SAMPLER: LazyLock<ProcessSampler> = LazyLock::new(ProcessSampler::new);

impl Default for ProcessMetrics {
    fn default() -> Self {
        let group: LazyGroup<ProcessSample> = LazyGroup::new(|| PROCESS_SAMPLER.sample());
        Self {
            pid: ConstGauge::new(PROCESS_SAMPLER.pid.as_u32() as i64),
            cpu_seconds_total: group.counter(|s| s.cpu_seconds_total),
            cpu_usage_percent: group.gauge(|s| s.cpu_usage_percent),
            resident_memory_bytes: group.gauge(|s| s.resident_memory_bytes),
            virtual_memory_bytes: group.gauge(|s| s.virtual_memory_bytes),
            start_time_seconds: group.gauge(|s| s.start_time_seconds),
            run_time_seconds: group.gauge(|s| s.run_time_seconds),
            open_fds: group.gauge(|s| s.open_fds),
            max_fds: group.gauge(|s| s.max_fds),
            threads: group.gauge(|s| s.threads),
        }
    }
}

impl Register for ProcessMetrics {
    fn register(&self, registry: &mut Registry) -> Result<()> {
        registry.register("pid", "Process ID.", self.pid.clone())?;
        registry.register_with_unit(
            "cpu",
            "Total user and system CPU time spent in seconds.",
            Unit::Seconds,
            self.cpu_seconds_total.clone(),
        )?;
        registry.register(
            "cpu_usage_percent",
            "CPU usage of the process in percent.",
            self.cpu_usage_percent.clone(),
        )?;
        registry.register_with_unit(
            "resident_memory",
            "Resident memory size in bytes.",
            Unit::Bytes,
            self.resident_memory_bytes.clone(),
        )?;
        registry.register_with_unit(
            "virtual_memory",
            "Virtual memory size in bytes.",
            Unit::Bytes,
            self.virtual_memory_bytes.clone(),
        )?;
        registry.register_with_unit(
            "start_time",
            "Start time of the process since unix epoch in seconds.",
            Unit::Seconds,
            self.start_time_seconds.clone(),
        )?;
        registry.register_with_unit(
            "run_time",
            "Process run time in seconds.",
            Unit::Seconds,
            self.run_time_seconds.clone(),
        )?;
        registry.register("open_fds", "Number of open file descriptors.", self.open_fds.clone())?;
        registry.register(
            "max_fds",
            "Maximum number of open file descriptors.",
            self.max_fds.clone(),
        )?;
        registry.register(
            "threads",
            "Number of OS threads in the process.",
            self.threads.clone(),
        )?;
        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
struct ProcessSample {
    cpu_seconds_total: f64,
    cpu_usage_percent: f32,
    resident_memory_bytes: i64,
    virtual_memory_bytes: i64,
    start_time_seconds: i64,
    run_time_seconds: i64,
    open_fds: i64,
    max_fds: i64,
    threads: i64,
}

struct ProcessSampler {
    pid: Pid,
    system: Mutex<System>,
}

impl ProcessSampler {
    fn new() -> Self {
        let pid = Pid::from_u32(process::id());
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

    let Some(process) = system.process(pid) else {
        return ProcessSample::default();
    };

    ProcessSample {
        cpu_seconds_total: process.accumulated_cpu_time() as f64 / 1_000.0,
        cpu_usage_percent: process.cpu_usage(),
        resident_memory_bytes: u64_to_i64_saturating(process.memory()),
        virtual_memory_bytes: u64_to_i64_saturating(process.virtual_memory()),
        start_time_seconds: u64_to_i64_saturating(process.start_time()),
        run_time_seconds: u64_to_i64_saturating(process.run_time()),
        open_fds: process.open_files().unwrap_or(0) as i64,
        max_fds: process.open_files_limit().unwrap_or(0) as i64,
        threads: process.tasks().map(|t| t.len()).unwrap_or(0) as i64,
    }
}

#[inline]
fn u64_to_i64_saturating(v: u64) -> i64 {
    if v > i64::MAX as u64 { i64::MAX } else { v as i64 }
}
