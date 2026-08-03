# Port decisions

## 1. Replaced Python datetime handling with a Rust-native date type
The original project relies on Python's datetime library for calendar arithmetic and scheduling semantics. In Rust, that behavior was reconstructed with a small `DateTime` value type that tracks year, month, day, hour, minute, and second and performs simple date rollovers. This keeps the port self-contained and avoids Python interop.

## 2. Kept the public API intentionally small
The Rust crate exposes a lightweight API centered on `CronExpression`, `CronIterator`, and `DateTime`. This mirrors the core workflow of the Python version: parse an expression, test a timestamp, and move to the next match.

## 3. Used standard Rust data structures
Vectors and `BTreeSet` are used for field values instead of Python lists and sets, which makes the implementation idiomatic and deterministic. The parser stores each field as a `CronFieldValue` with wildcard metadata and explicit accepted values.

## 4. Prioritized correctness over full feature parity
The original library has a large surface area, including many edge cases and timezone handling. The initial Rust port focuses on the core cron matching and iteration behaviors that are exercised by the provided tests and the main documentation examples.

## 5. Added a simple CLI, Docker build path, and regression tests
A small binary is included so users can evaluate an expression directly from the command line, a Dockerfile allows the crate to be built into a runnable artifact, and a Rust test file captures the main parsing and iteration behaviors to prevent regressions.

## 6. Preserved the project layout expected for a port-mortem submission
The repository now includes the requested top-level directories for fuzzing, benchmarking, and project metadata so the result is ready to hand off as a structured migration artifact rather than a bare code dump.
