use chrono::{Local, Utc};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::LitStr;

/// Build time in UTC.
#[proc_macro]
pub fn build_timestamp_utc(_input: TokenStream) -> TokenStream {
    let time = Utc::now().to_rfc3339();

    let lit = LitStr::new(&time, Span::call_site());

    quote!(#lit).into()
}

/// Build time in the local timescale.
#[proc_macro]
pub fn build_timestamp_local(_input: TokenStream) -> TokenStream {
    let time = Local::now().to_rfc3339();

    let lit = LitStr::new(&time, Span::call_site());

    quote!(#lit).into()
}
