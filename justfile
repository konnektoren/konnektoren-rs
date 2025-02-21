# Default recipe
default:
    @just --list

# Core package commands
core +COMMAND="build":
    @just -f konnektoren-core/justfile {{COMMAND}}

# Platform package commands
platform +COMMAND="build":
    @just -f konnektoren-platform/justfile {{COMMAND}}

# Tests package commands
tests +COMMAND="test":
    @just -f konnektoren-tests/justfile {{COMMAND}}

# Add this to run BDD tests specifically
bdd:
    @just -f konnektoren-tests/justfile bdd

# Workspace-wide commands
build:
    cargo build --workspace

test:
    cargo test --workspace

lint:
    cargo clippy --workspace -- -D warnings

# Documentation
doc:
    cargo doc --workspace --no-deps

# Coverage
coverage:
    cargo tarpaulin --workspace --ignore-tests

# Generate SBOM
sbom:
    cargo cyclonedx

# Generate changelog
changelog:
    git-cliff -o CHANGELOG.md

# Clean build artifacts
clean:
    cargo clean

# Setup development environment
setup:
    rustup component add rustfmt
    rustup component add clippy
    cargo install cargo-tarpaulin
    cargo install cargo-cyclonedx
    cargo install git-cliff

# Pre-commit checks
pre-commit: lint test
