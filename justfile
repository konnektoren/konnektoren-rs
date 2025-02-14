# Default recipe
default:
    @just --list

# Core package commands
core +COMMAND="build":
    @case "{{COMMAND}}" in \
        "build") cargo build -p konnektoren-core ;; \
        "test") cargo test -p konnektoren-core ;; \
        "doc") cargo doc -p konnektoren-core --no-deps ;; \
        "lint") cargo clippy -p konnektoren-core -- -D warnings ;; \
        *) echo "Invalid command. Use build, test, doc, or lint" ;; \
    esac

# Platform package commands
platform +COMMAND="build":
    @case "{{COMMAND}}" in \
        "build") cargo build -p konnektoren-platform ;; \
        "test") cargo test -p konnektoren-platform ;; \
        "doc") cargo doc -p konnektoren-platform --no-deps ;; \
        "lint") cargo clippy -p konnektoren-platform -- -D warnings ;; \
        *) echo "Invalid command. Use build, test, doc, or lint" ;; \
    esac

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
