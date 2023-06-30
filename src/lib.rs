/*!
# build-time

Simple proc-macros to generate build timestamp string literals.

Based on Jasen Borisov's [build_timestamp](https://crates.io/crates/build_timestamp) crate.

Two function like procedures are provided: `build_time_utc` and `build_time_local`.

They take an optional [`strftime`](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html)
date and time format string as input, and return a string literal.
If the input is empty, they will return a string literal in
[RFC 3339 date and time format](https://en.wikipedia.org/wiki/ISO_8601#RFCs),
e.g., `"2021-05-29T06:55:50.418437046+00:00"`.

Requires Rust 1.45+ because these macros are used in expression positions.

## Usage

```rust
use build_time::{build_time_utc, build_time_local};

// Returns the UTC build timestamp in RFC3339 date and time format.
let utc_build_time = build_time_utc!();

// Returns the local build timestamp in the specified format.
let local_build_time = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");
```
*/

use chrono::{DateTime, Local, TimeZone, Utc};
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::env;
use syn::{parse_macro_input, LitStr};

static BUILD_TIME: Lazy<DateTime<Utc>> = Lazy::new(|| match env::var("SOURCE_DATE_EPOCH") {
    Ok(val) => Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap(),
    Err(_) => Utc::now(),
});

/// Build time in UTC.
///
/// It takes an optional [`strftime`](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html)
/// date and time format string as input, and returns a string literal.
/// If the input is empty, it will return a string literal in
/// [RFC 3339 date and time format](https://en.wikipedia.org/wiki/ISO_8601#RFCs),
/// e.g., `"2021-05-29T06:55:50.418437046+00:00"`.
///
/// # Example
///
/// ```rust
/// use build_time::build_time_utc;
///
/// // Returns the UTC build timestamp in RFC3339 date and time format.
/// let build_time_rfc3339 = build_time_utc!();
///
/// // Returns the UTC build timestamp in the specified format.
/// let build_time_formatted = build_time_utc!("%Y-%m-%dT%H:%M:%S%.f%:z");
/// ```
#[proc_macro]
pub fn build_time_utc(input: TokenStream) -> TokenStream {
    let time_str = if input.is_empty() {
        BUILD_TIME.to_rfc3339()
    } else {
        let format = parse_macro_input!(input as LitStr);
        BUILD_TIME.format(&format.value()).to_string()
    };

    let lit = LitStr::new(&time_str, Span::call_site());

    quote!(#lit).into()
}

/// Build time in the local timescale.
///
/// It takes an optional [`strftime`](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html)
/// date and time format string as input, and returns a string literal.
/// If the input is empty, it will return a string literal in
/// [RFC 3339 date and time format](https://en.wikipedia.org/wiki/ISO_8601#RFCs),
/// e.g., `"2021-05-29T06:55:50.418437046+00:00"`.
///
/// # Example
///
/// ```rust
/// use build_time::build_time_local;
///
/// // Returns the local build timestamp in RFC3339 date and time format.
/// let build_time_rfc3339 = build_time_local!();
///
/// // Returns the local build timestamp in the specified format.
/// let build_time_formatted = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");
/// ```
#[proc_macro]
pub fn build_time_local(input: TokenStream) -> TokenStream {
    let local_time = BUILD_TIME.with_timezone(&Local);
    let time_str = if input.is_empty() {
        local_time.to_rfc3339()
    } else {
        let format = parse_macro_input!(input as LitStr);
        local_time.format(&format.value()).to_string()
    };

    let lit = LitStr::new(&time_str, Span::call_site());

    quote!(#lit).into()
}
