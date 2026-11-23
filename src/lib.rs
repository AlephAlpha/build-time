/*!
# build-time

Simple proc-macros to generate build timestamp string literals.

Based on Jasen Borisov's [build_timestamp](https://crates.io/crates/build_timestamp) crate.

Two function like procedures are provided: `build_time_utc` and `build_time_local`.

They take an optional [Jiff `strftime`](https://docs.rs/jiff/1.0/jiff/fmt/strtime/index.html)
date and time format string as input, and return a string literal.
If the input is empty, they will return a string literal in
[RFC 3339 date and time format](https://en.wikipedia.org/wiki/ISO_8601#RFCs),
e.g., `"2021-05-29T06:55:50.418437046+00:00"`.

Requires Rust 1.85+.

## Usage

```rust
use build_time::{build_time_utc, build_time_local};

// Returns the UTC build timestamp in RFC3339 date and time format.
let utc_build_time = build_time_utc!();

// Returns the local build timestamp in the specified format.
let local_build_time = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");
```
*/

use jiff::{tz::TimeZone, Timestamp};
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::env;
use syn::{parse_macro_input, LitStr};

static BUILD_TIME: Lazy<Timestamp> = Lazy::new(|| {
    env::var("SOURCE_DATE_EPOCH").map_or_else(
        |_| Timestamp::now(),
        |val| {
            let seconds = val
                .parse::<i64>()
                .expect("SOURCE_DATE_EPOCH must be an integer number of seconds");
            Timestamp::from_second(seconds)
                .expect("SOURCE_DATE_EPOCH is outside the supported timestamp range")
        },
    )
});

fn build_time(input: TokenStream, is_utc: bool) -> TokenStream {
    let format = if input.is_empty() {
        None
    } else {
        Some(parse_macro_input!(input as LitStr))
    };
    let zone = if is_utc {
        TimeZone::UTC
    } else {
        TimeZone::system()
    };
    let time = BUILD_TIME.to_zoned_datetime(zone);
    let time_str = format.map_or_else(
        || BUILD_TIME.display_with_offset(time.offset()).to_string(),
        |format| time.strftime(&format.value()).to_string(),
    );
    let lit = LitStr::new(&time_str, Span::call_site());
    quote!(#lit).into()
}

/// Build time in UTC.
///
/// It takes an optional [Jiff `strftime`](https://docs.rs/jiff/1.0/jiff/fmt/strtime/index.html)
/// date and time format string as input, and returns a string literal.
/// With no argument, it returns an RFC 3339-style timestamp, e.g.,
/// `"2021-05-29T06:55:50.418437046+00:00"`.
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
    build_time(input, true)
}

/// Build time in the local timescale.
///
/// It takes an optional [Jiff `strftime`](https://docs.rs/jiff/1.0/jiff/fmt/strtime/index.html)
/// date and time format string as input, and returns a string literal.
/// With no argument, it returns an RFC 3339-style timestamp, e.g.,
/// `"2021-05-29T06:55:50.418437046+00:00"`.
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
    build_time(input, false)
}
