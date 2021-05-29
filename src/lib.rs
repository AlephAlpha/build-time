/*!
Simple proc-macros to generate build timestamp string literals.

Based on Jasen Borisov's [build_timestamp](https://crates.io/crates/build_timestamp) crate.

Two function like procedures are provided: `build_timestamp_utc` and `build_timestamp_local`.
They take no input, and return a string literal in
[RFC 3339 date and time format](https://en.wikipedia.org/wiki/ISO_8601#RFCs),
e.g., `"2021-05-29T06:55:50.418437046+00:00"`.

Requires Rust 1.45+ because these two macros are used in expression positions.

# Usage

```
use build_timestamp::build_timestamp_utc;

let build_timestamp = build_timestamp_utc!();
```
*/

use chrono::{DateTime, Local, Utc};
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::LitStr;

static BUILD_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(Utc::now);

/// Build time in UTC, as an RFC 3339 date and time string.
#[proc_macro]
pub fn build_timestamp_utc(_input: TokenStream) -> TokenStream {
    let time_str = BUILD_TIMESTAMP.to_rfc3339();

    let lit = LitStr::new(&time_str, Span::call_site());

    quote!(#lit).into()
}

/// Build time in the local timescale, as an RFC 3339 date and time string.
#[proc_macro]
pub fn build_timestamp_local(_input: TokenStream) -> TokenStream {
    let local_time: DateTime<Local> = BUILD_TIMESTAMP.clone().into();
    let time_str = local_time.to_rfc3339();

    let lit = LitStr::new(&time_str, Span::call_site());

    quote!(#lit).into()
}
