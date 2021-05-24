# build-timestamp

Simple proc-macros to generate build timestamp string literals.

Based on Jasen Borisov's [build-timestamp](https://crates.io/crates/build_timestamp) crate.

Two function like procedures are provided: `build_timestamp_utc` and `build_timestamp_local`. They take no input, and return a string literal in RFC 3339 date and time format.

Requires Rust 1.45+ because these two macros are used in expression positions.