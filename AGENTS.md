# AGENTS guide

This file provides guidance to AI coding agents (e.g., Claude Code, ChatGPT) when working with code in this repository.

While working on this codebase, please remember:

- Always use English in code and comments.
- Only add meaningful comments when the code's behavior is difficult to understand.
- Only add meaningful tests when they actually verify internal behaviors; otherwise, don't create them unless requested.

## Goals

- Maintain a fast, correct OpenMetrics/Prometheus client in Rust.
- Preserve API stability and ergonomics while iterating.
- Keep CI clean (format, lint, tests, benches) and avoid regressions in performance-sensitive areas.

## Code guidelines

- Prefer clear, allocation-conscious code; avoid unnecessary clones.
- Favor `parking_lot` primitives already in use where locking is needed.
- Preserve zero-copy encoding paths in encoders; benchmark if touching hot loops.
- Keep public APIs stable; use deprecation over removal when possible.
- Derive macros (`fastmetrics-derive`) should emit clear errors; add trybuild tests for new cases.
- Registry/metrics: maintain thread safety and label correctness; avoid inconsistent schemas.
- Tests should be deterministic; avoid timing-based flakiness.

## Repository layout (high level)

- `fastmetrics/` – main library crate.
- `fastmetrics-derive/` – proc-macros for derives (EncodeLabelSet, LabelSetSchema, etc.).
- `examples/` – runnable examples (metrics, static metrics, process metrics).
- `benchmarks/` – criterion-style benches comparing implementations.
- `README.md` – user-facing overview and quickstart.
- `justfile` – common tasks (fmt, clippy, test, bench, docs).

## Toolchain & style

- Safety: `#![deny(unsafe_code)]` in crates. Avoid introducing `unsafe`; if unavoidable, document invariants and justification.
- Lints: keep code warning-free. CI runs `cargo clippy --all-targets --all-features -D warnings`.
- Formatting: use `taplo fmt` for TOML and `cargo fmt` for Rust. Prefer running `just fmt` (formats all) or `just fmt-check`.
- Docs: public items should have doc comments. Avoid missing_docs failures.

## Development commands

- Format: `just fmt` or `just fmt-check`
- Lint: `just clippy`
- Check/Build: `just check` or `just build`
- Test: `just test` (runs workspace tests)
- Docs: `just gen-docs` for local validation (no-deps, all-features)
- Example: `just example <name> [args]` from `examples/`
- Benchmarks: `just bench <name> [args]` or `just benches [args]` from `benchmarks/`. For perf-sensitive changes, run relevant benches and note results.

## Documentation pointers

- User-facing guide: `README.md`.
- Examples: `examples/src`.
- Benchmarks overview: `benchmarks/README.md`.
- Crate-level docs live in `fastmetrics/src/lib.rs`.

## Features & compatibility

- Default features: `foldhash`.
- Optional features:
  - `derive` – enables `fastmetrics-derive`.
  - `prost` – protobuf (openmetrics) via `prost`/`prost-build`.
  - `protobuf` – protobuf (openmetrics) via `protobuf`/`protobuf-codegen`.
- Avoid feature regressions: test with and without optional features when feasible.

## PR expectations

- Keep changes minimal and focused; include rationale.
- Update docs/examples if behavior or APIs change.
- Add/adjust tests to cover new behavior or bug fixes.
- Ensure formatting and clippy are clean.
- Mention feature combinations tested when relevant (e.g., `--no-default-features --features derive,prost`).
- For performance changes, attach before/after benchmark snippets when practical.

## Release Notes

Generate GitHub Release notes that cover these sections:

- Breaking Changes.
- New Features.
- Performance & Behavior Improvements.

## Security & licensing

- License: Apache-2.0 (see `LICENSE`).
- Do not add dependencies with incompatible licenses.
- Avoid embedding secrets or keys in code or tests.
