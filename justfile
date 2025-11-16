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

# Run rust clippy with debug profile
clippy:
    @cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check code with debug profile
check:
	@cargo check --workspace --all-targets --all-features

# Build with debug profile
build:
    @cargo build --workspace --all-targets --all-features

# Run all tests with debug profile
test:
    @cargo test --workspace --all-features

# Generate docs
gen-docs:
	@cargo doc --no-deps --workspace --lib --all-features

# Run benchmarks: `just bench [-- <args...>]` or `just bench <name...> [-- <args...>]`
[working-directory: 'benchmarks']
[positional-arguments]
@bench *args:
    #!/usr/bin/env bash
    # Split args: names before `--`, extra args after
    names=""
    while [ $# -gt 0 ] && [ "$1" != "--" ]; do
        names="$names $1"
        shift
    done
    if [ "${1-}" = "--" ]; then
        shift
    fi
    extra_args="$@"

    if [ -z "$names" ]; then
        if [ -n "$extra_args" ]; then
            echo "Running all benchmarks with \"$extra_args\""
            cargo bench -- $extra_args
        else
            echo "Running all benchmarks"
            cargo bench -- --quiet
        fi
    else
        for name in $names; do
            if [ -n "$extra_args" ]; then
                echo "Running benchmark \"$name\" with \"$extra_args\""
                cargo bench --bench $name -- $extra_args
            else
                echo "Running benchmark \"$name\""
                cargo bench --bench $name -- --quiet
            fi
        done
    fi

# Run examples: `just example <name...> [-- <args...>]`
[working-directory: 'examples']
[positional-arguments]
[no-exit-message]
@example *args:
    #!/usr/bin/env bash
    # Split args: names before `--`, extra args after
    names=""
    while [ $# -gt 0 ] && [ "$1" != "--" ]; do
        names="$names $1"
        shift
    done
    if [ "${1-}" = "--" ]; then
        shift
    fi
    extra_args="$@"

    if [ -z "$names" ]; then
        echo "{{BOLD + RED}}Error{{NORMAL}}: must specify at least one example name."
        exit 1
    fi
    for name in $names; do
        if [ -n "$extra_args" ]; then
            echo "Running example \"$name\" with \"$extra_args\""
            cargo run --example $name -- $extra_args
        else
            echo "Running example \"$name\""
            cargo run --example $name
        fi
    done
