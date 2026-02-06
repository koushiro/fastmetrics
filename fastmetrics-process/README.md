# fastmetrics-process

[![](https://github.com/koushiro/fastmetrics/actions/workflows/ci.yml/badge.svg)][actions]
[![](https://img.shields.io/docsrs/fastmetrics-process)][docs.rs]
[![](https://img.shields.io/crates/v/fastmetrics-process)][crates.io]
[![](https://img.shields.io/crates/l/fastmetrics-process)][crates.io]
[![](https://img.shields.io/crates/d/fastmetrics-process)][crates.io]
[![](https://img.shields.io/badge/MSRV-1.88.0-green?logo=rust)][whatrustisit]

[actions]: https://github.com/koushiro/fastmetrics/actions
[docs.rs]: https://docs.rs/fastmetrics-process
[crates.io]: https://crates.io/crates/fastmetrics-process
[whatrustisit]: https://www.whatrustisit.com

Prometheus-style process metrics built on top of `fastmetrics`.

This crate provides a small set of commonly used process metrics aligned with Prometheus naming
conventions (for example: `process_cpu_seconds_total`, `process_resident_memory_bytes`).

Metrics are implemented as **lazy (scrape-time) metrics** and grouped via [`fastmetrics::metrics::lazy_group::LazyGroup`]
so a single OS sampling operation is shared across all metrics per scrape.

## Usage

```rust,no_run
use fastmetrics::{error::Result, registry::{Register, Registry}};
use fastmetrics_process::ProcessMetrics;

fn main() -> Result<()> {
    let mut registry = Registry::default();
    let metrics = ProcessMetrics::default();

    // Standard Prometheus-style names: `process_*`
    let process = registry.subsystem("process")?;
    metrics.register(process)?;

    Ok(())
}
```

## Platform support and fallbacks

This crate uses [`sysinfo`](https://crates.io/crates/sysinfo) to collect process information.
Some values may be unavailable on certain platforms or blocked by permissions; unavailable values
fall back to `0`.

## Exposed metrics

This crate registers **base names** so you can choose your prefixing strategy
(for example, register into `registry.subsystem("process")?` to get `process_*` names).

- Register into `registry.subsystem("process")?` to get `process_*` names.
- Register at root to get unprefixed names (useful if you already have a namespace that should prefix everything).

Registered base names:

- `pid` — Process ID. (type: gauge)
- `cpu` — Total user and system CPU time spent in seconds. (type: counter, unit: seconds)
- `cpu_usage_percent` — CPU usage of the process in percent. (type: gauge)
- `resident_memory` — Resident memory size in bytes. (type: gauge, unit: bytes)
- `virtual_memory` — Virtual memory size in bytes. (type: gauge, unit: bytes)
- `start_time` — Start time of the process since Unix epoch in seconds. (type: gauge, unit: seconds)
- `run_time` — Process run time in seconds. (type: gauge, unit: seconds)
- `open_fds` — Number of open file descriptors. (type: gauge)
- `max_fds` — Maximum number of open file descriptors. (type: gauge)
- `threads` — Number of OS threads in the process. (type: gauge)

Standard names when registered into a `process` subsystem:

- `process_pid`
- `process_cpu_seconds_total`
- `process_cpu_usage_percent`
- `process_resident_memory_bytes`
- `process_virtual_memory_bytes`
- `process_start_time_seconds`
- `process_run_time_seconds`
- `process_open_fds`
- `process_max_fds`
- `process_threads`

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE] file for details.

[LICENSE]: https://github.com/koushiro/fastmetrics/blob/main/LICENSE
