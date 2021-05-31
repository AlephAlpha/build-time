# build-time

Simple proc-macros to generate build timestamp string literals.

Based on Jasen Borisov's [build_timestamp](https://crates.io/crates/build_timestamp) crate.

Two function like procedures are provided: `build_time_utc` and `build_time_local`.

They take an optional [`strftime`](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html) date and time format string as input, and return a string literal. If the input is empty, they will return a string literal in [RFC 3339 date and time format](https://en.wikipedia.org/wiki/ISO_8601#RFCs), e.g., `"2021-05-29T06:55:50.418437046+00:00"`.

Requires Rust 1.45+ because these macros are used in expression positions.

## Usage

```rust
use build_time::{build_time_utc, build_time_local};

// Returns the UTC build timestamp in RFC3339 date and time format.
let utc_build_time = build_time_utc!();

// Returns the local build timestamp in the specified format.
let local_build_time = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");
```