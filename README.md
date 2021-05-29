# build-timestamp

Simple proc-macros to generate build timestamp string literals.

Based on Jasen Borisov's [build_timestamp](https://crates.io/crates/build_timestamp) crate.

Two function like procedures are provided: `build_timestamp_utc` and `build_timestamp_local`. They take no input, and return a string literal in [RFC 3339 date and time format](https://en.wikipedia.org/wiki/ISO_8601#RFCs), e.g., `"2021-05-29T06:55:50.418437046+00:00"`.

Requires Rust 1.45+ because these macros are used in expression positions.

## Usage

```rust
use build_timestamp::build_timestamp_utc;

let build_timestamp = build_timestamp_utc!();
```