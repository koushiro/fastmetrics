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
    @cargo clippy --workspace --all-targets --all-features {{args}} -- -D warnings

# Check code
check *args='':
	@cargo check --workspace --all-targets --all-features {{args}}

# Build workspace
build *args='':
    @cargo build --workspace --all-targets --all-features {{args}}

# Run all tests
test *args='':
    @cargo test --workspace --all-features {{args}}

# Generate docs
gen-docs *args='':
	@cargo doc --no-deps --workspace --lib --all-features {{args}}

# Run examples: `just example [name] <args>`
[working-directory: 'examples']
[positional-arguments]
example name *args:
    #!/usr/bin/env bash
    set -eo pipefail
    if [ -n "{{args}}" ]; then
        echo "Running example \"{{name}}\" with: {{args}}"
        cargo run --example {{name}} -- {{args}}
    else
        echo "Running example \"{{name}}\""
        cargo run --example {{name}}
    fi

# Run benchmarks: `just bench <args>` or `just bench [name] <args>`
[working-directory: 'benchmarks']
[positional-arguments]
bench *args:
    #!/usr/bin/env bash
    set -eo pipefail
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
        if [ ${#extra_args[@]} -gt 0 ]; then
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
