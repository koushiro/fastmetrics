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

# Run examples: `just example [NAME] <ARGS>`
[working-directory: 'examples']
@example NAME *ARGS:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -n "{{ARGS}}" ]; then
        echo "Running example \"{{NAME}}\" with: {{ARGS}}"
        cargo run --example {{NAME}} -- {{ARGS}}
    else
        echo "Running example \"{{NAME}}\""
        cargo run --example {{NAME}}
    fi

# Run benchmarks: `just bench [-- <args...>]` or `just bench <name...> [-- <args...>]`
[working-directory: 'benchmarks']
[positional-arguments]
@bench *ARGS:
    #!/usr/bin/env bash
    set -euo pipefail
    # Array-safe split: names before `--`, extra args after
    names=()
    extra_args=()
    parsing_names=1
    for arg in "$@"; do
        if [ "$parsing_names" -eq 1 ]; then
            if [ "$arg" = "--" ]; then
                parsing_names=0
            else
                names+=("$arg")
            fi
        else
            extra_args+=("$arg")
        fi
    done

    if [ ${#names[@]} -eq 0 ]; then
        if [ ${#extra[@]} -gt 0 ]; then
            echo "Running all benchmarks with: ${extra_args[*]}"
            cargo bench -- "${extra_args[@]}"
        else
            echo "Running all benchmarks"
            cargo bench -- --quiet
        fi
    else
        for name in "${names[@]}"; do
            if [ ${#extra[@]} -gt 0 ]; then
                echo "Running benchmark \"$name\" with: ${extra_args[*]}"
                cargo bench --bench "$name" -- "${extra_args[@]}"
            else
                echo "Running benchmark \"$name\""
                cargo bench --bench "$name" -- --quiet
            fi
        done
    fi
