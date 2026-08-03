# croniter-rs

A Rust port of the core cron scheduling behavior from the original Python project.

## Features

- Parse cron expressions with minute/hour/day/month/day-of-week fields
- Support optional seconds and year fields
- Match specific timestamps and iterate to the next matching time
- Provide a small CLI for quick scheduling checks

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
