//! Process metrics example showcasing lazy counters and gauges.

#[cfg(not(unix))]
fn main() -> anyhow::Result<()> {
    println!("process_metrics example currently only collects data on Unix-like targets.");
    Ok(())
}

#[cfg(unix)]
fn main() -> anyhow::Result<()> {
    unix::run()
}

#[cfg(unix)]
mod unix {
    use std::{io, mem::MaybeUninit};

    use fastmetrics::{
        derive::*,
        format::text,
        metrics::{counter::LazyCounter, gauge::LazyGauge},
        registry::{Register, Registry},
    };

    #[derive(Register)]
    pub struct ProcessMetrics<F1, F2>
    where
        F1: Fn() -> f64 + Send + Sync + 'static,
        F2: Fn() -> i64 + Send + Sync + 'static,
    {
        /// Total CPU time (user + system) consumed by the current process.
        cpu_seconds: LazyCounter<F1, f64>,
        /// Resident Set Size (RSS) of the current process in bytes.
        #[register(unit(Bytes))]
        resident_memory: LazyGauge<F2, i64>,
    }

    impl<F1, F2> ProcessMetrics<F1, F2>
    where
        F1: Fn() -> f64 + Send + Sync + 'static,
        F2: Fn() -> i64 + Send + Sync + 'static,
    {
        pub fn new(cpu_seconds_fn: F1, resident_memory_fn: F2) -> Self {
            Self {
                cpu_seconds: LazyCounter::new(cpu_seconds_fn),
                resident_memory: LazyGauge::new(resident_memory_fn),
            }
        }
    }

    pub fn run() -> anyhow::Result<()> {
        let mut registry = Registry::builder().with_namespace("process").build()?;

        let metrics = ProcessMetrics::new(read_cpu_seconds_total, read_resident_memory_bytes);
        metrics.register(&mut registry)?;

        // let cpu_seconds = LazyCounter::new(read_cpu_seconds_total);
        // let resident_memory = LazyGauge::new(read_resident_memory_bytes);
        // registry.register(
        //     "cpu_seconds",
        //     "Total CPU time (user + system) consumed by the current process.",
        //     cpu_seconds.clone(),
        // )?;
        // registry.register_with_unit(
        //     "resident_memory",
        //     "Resident Set Size (RSS) of the current process in bytes.",
        //     Unit::Bytes,
        //     resident_memory.clone(),
        // )?;

        // println!(
        //     "Debug snapshot → cpu={:.3}s rss={}B",
        //     cpu_seconds.fetch(),
        //     resident_memory.fetch()
        // );

        let mut encoded = String::new();
        text::encode(&mut encoded, &registry)?;
        println!("\n=== Exported Metrics ===\n{encoded}");

        Ok(())
    }

    fn read_cpu_seconds_total() -> f64 {
        read_rusage()
            .map(|usage| timeval_to_seconds(usage.ru_utime) + timeval_to_seconds(usage.ru_stime))
            .unwrap_or_else(|err| {
                eprintln!("failed to read CPU usage: {err}");
                0.0
            })
    }

    fn read_resident_memory_bytes() -> i64 {
        read_rusage()
            .map(|usage| usage.ru_maxrss * RSS_UNIT_BYTES as i64)
            .unwrap_or_else(|err| {
                eprintln!("failed to read RSS: {err}");
                0
            })
    }

    fn read_rusage() -> io::Result<libc::rusage> {
        let mut usage = MaybeUninit::<libc::rusage>::uninit();
        let rc = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) };
        if rc == 0 { Ok(unsafe { usage.assume_init() }) } else { Err(io::Error::last_os_error()) }
    }

    fn timeval_to_seconds(tv: libc::timeval) -> f64 {
        tv.tv_sec as f64 + tv.tv_usec as f64 / 1_000_000.0
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const RSS_UNIT_BYTES: u64 = 1;

    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    const RSS_UNIT_BYTES: u64 = 1024;
}
