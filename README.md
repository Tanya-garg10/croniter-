# croniter-rs

Rust port of the core cron scheduling behavior from the original Python project.

## Migration rationale

This port preserves the central workflow of the original library: parse a cron expression, check whether a timestamp matches, and iterate to the next matching timestamp. The implementation uses a small native Rust date type and standard Rust collections rather than Python runtime interop.

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Run

```bash
cargo run -- "0 0 * * *" "2024-01-01 00:00:00"
```

The CLI prints the next matching timestamp after the supplied base time.

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
