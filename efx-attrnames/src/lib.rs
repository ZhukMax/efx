#![deny(rust_2018_idioms)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

/// Derive for generating `ATTR_NAMES` constant for attribute structure.
/// Supports field renaming via `#[attr(name = "align")]`.
#[proc_macro_derive(AttrNames, attributes(attr))]
pub fn derive_attr_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty_ident = &input.ident;

    // Only allow structures with named fields
    let named = match &input.data {
        Data::Struct(st) => match &st.fields {
            Fields::Named(n) => n,
            _ => {
                return syn::Error::new_spanned(
                    &st.fields,
                    "AttrNames supports only structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input.ident, "AttrNames supports only structs")
                .to_compile_error()
                .into();
        }
    };

    // Collect attribute names as literals (LitStr) so that they are 'static
    let mut names: Vec<LitStr> = Vec::with_capacity(named.named.len());

    for f in &named.named {
        let ident = match &f.ident {
            Some(id) => id,
            None => {
                return syn::Error::new_spanned(f, "expected named field")
                    .to_compile_error()
                    .into();
            }
        };

        // Base name - field name
        let mut out = ident.to_string();

        // Process #[attr(name = "...")]
        for attr in &f.attrs {
            if attr.path().is_ident("attr") {
                // syn v2: using parse_nested_meta
                if let Err(e) = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let lit: LitStr = meta.value()?.parse()?;
                        out = lit.value();
                    }
                    Ok(())
                }) {
                    return e.to_compile_error().into();
                }
            }
        }

        // Literal with field span
        names.push(LitStr::new(&out, ident.span()));
    }

    let output_tokens = quote! {
        impl #ty_ident {
            pub const ATTR_NAMES: &'static [&'static str] = &[ #( #names ),* ];
        }
    };
    output_tokens.into()
}
