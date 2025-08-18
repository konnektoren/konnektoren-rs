# konnektoren-rs

Rust implementation of Konnektoren

[![codecov](https://codecov.io/gh/konnektoren/konnektoren-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/konnektoren/konnektoren-rs)

## Overview

This repository is a Rust workspace for the Konnektoren project, providing grammar learning and interactive exercises. The workspace consists of several packages:

- **konnektoren-core**:
  The main library containing core domain logic, challenge types, game state, achievements, and persistence.

- **konnektoren-platform**:
  Platform-specific utilities, internationalization (i18n), domain configuration, and asset management.

- **konnektoren-tests**:
  BDD and integration tests for the project, using Cucumber for feature-driven testing.

- **konnektoren-tui**:
  Terminal User Interface (TUI) application for interacting with Konnektoren challenges and games.

## Build

```bash
just build
```

## Documentation

Documentation is available online:

* [Project Docs](https://konnektoren.github.io/konnektoren-rs/docs/)
* [konnektoren_core API](https://konnektoren.github.io/konnektoren-rs/doc/konnektoren_core/)

## Run

Test Konnektoren Yew Components at https://konnektoren.github.io/konnektoren-rs/

```bash
cargo run --package konnektoren-tui
```

## Justfile Commands

This project uses a `Justfile` for simplified command execution. Here are some commonly used commands:

*   **`just build`**: Builds the entire workspace.
*   **`just test`**: Runs all tests in the workspace.
*   **`just lint`**: Runs the linter on the entire workspace.
*   **`just doc`**: Generates documentation for the entire workspace.
*   **`just coverage`**: Generates code coverage reports for the entire workspace.
*   **`just sbom`**: Generates SBOM.
*   **`just changelog`**: Generate changelog.
*   **`just clean`**: Cleans build artifacts.
*   **`just setup`**: Sets up the development environment.
*   **`just pre-commit`**: Runs pre-commit checks (lint and test).
*   **`just core build`**: Builds the konnektoren core package.
*   **`just platform build`**: Builds the konnektoren platform package.
*   **`just tests test`**: Run tests.
*   **`just bdd`**: Run BDD Tests

You can list all available commands by running:

```bash
just --list
```
