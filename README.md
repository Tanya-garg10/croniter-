# croniter (Rust Port)

A Rust implementation of the original croniter Python project, created as part of the Code Resurrection 2026 – Port Mortem hackathon (Track D: Python → Rust).

## Original Repository

This project is a Rust port of the following open-source Python repository:

- Original Repository: <https://github.com/pallets-eco/croniter.git>
- Fork Used During Development (if applicable): <https://github.com/Tanya-garg10/croniter.git>

## Project Goal

The objective of this project is to recreate the original application's functionality in idiomatic Rust while preserving its behavior, improving memory safety, and following Rust best practices.

## Migration Objectives

- Port the Python codebase to Rust
- Preserve the original functionality and user experience
- Use idiomatic Rust architecture and error handling
- Build using Cargo
- Document architectural decisions in [DECISIONS.md](DECISIONS.md)

## Migration rationale

This port preserves the central workflow of the original library: parse a cron expression, check whether a timestamp matches, and iterate to the next matching timestamp. The implementation uses a small native Rust date type and standard Rust collections rather than Python runtime interop.

## Repository Structure

```text
README.md
DECISIONS.md
Dockerfile
Cargo.toml
src/
tests/
fuzz/
bench/
.port-mortem.toml
```

## Build

```bash
cargo build
```

## Run

```bash
cargo run -- "0 0 * * *" "2024-01-01 00:00:00"
```

The CLI prints the next matching timestamp after the supplied base time.

## Acknowledgements

Thanks to the original authors of the Python project for making the repository publicly available. This repository contains an independent Rust implementation created solely for the Port Mortem hackathon and is not an official replacement or affiliated version of the original project.

## Port-mortem layout

- [README.md](README.md) — migration rationale and build instructions
- [DECISIONS.md](DECISIONS.md) — major divergences and their rationale
- [Dockerfile](Dockerfile) — builds a runnable artifact
- [src](src) — idiomatic Rust implementation
- [tests/original](tests/original) — original test corpus location
- [tests/port](tests/port) — additional regression tests
- [fuzz](fuzz) — differential-fuzzing scaffold
- [bench](bench) — benchmark methodology and results
- [.port-mortem.toml](.port-mortem.toml) — project metadata
