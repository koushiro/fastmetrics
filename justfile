# Show all commands
[default]
help:
    @just --list --list-heading $'Available commands:\n'

# Cleanup compilation outputs
clean:
    @cargo clean

# Check the code format
fmt-check:
    @taplo fmt --check
    @cargo fmt -- --check

# Format the code
fmt:
    @taplo fmt
    @cargo +nightly fmt --all

# Run rust clippy
clippy *args='':
    @cargo clippy --workspace --all-targets --all-features {{ args }} -- -D warnings

# Check code
check *args='':
    @cargo check --workspace --all-targets --all-features {{ args }}

# Build workspace
build *args='':
    @cargo build --workspace --all-targets --all-features {{ args }}

# Run all tests
test *args='':
    @cargo test --workspace --all-features {{ args }}

# Generate docs
gen-docs *args='':
    @cargo doc --no-deps --workspace --lib --all-features {{ args }}

# Run a example: `just example [name] <args>`
[positional-arguments]
[working-directory('examples')]
example name *args:
    #!/usr/bin/env bash
    set -eo pipefail
    if [ -n "{{ args }}" ]; then
        echo "Running example \"{{ name }}\" with: {{ args }}"
        cargo run --example {{ name }} -- {{ args }}
    else
        echo "Running example \"{{ name }}\""
        cargo run --example {{ name }}
    fi

# Run a benchmark: `just bench [name] <args>`
[positional-arguments]
[working-directory('benchmarks')]
bench name *args:
    #!/usr/bin/env bash
    set -eo pipefail
    if [ -n "{{ args }}" ]; then
        echo "Running benchmark \"{{ name }}\" with: {{ args }}"
        cargo bench --bench {{ name }} -- {{ args }}
    else
        echo "Running benchmark \"{{ name }}\""
        cargo bench --bench {{ name }} -- --quiet
    fi

# Run benchmarks: `just benches <args>`
[positional-arguments]
[working-directory('benchmarks')]
benches *args:
    #!/usr/bin/env bash
    set -eo pipefail
    if [ -n "{{ args }}" ]; then
        echo "Running all benchmarks with: {{ args }}"
        cargo bench -- {{ args }}
    else
        echo "Running all benchmarks"
        cargo bench -- --quiet
    fi
