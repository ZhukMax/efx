use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

/// Required expression attribute: parses into syn::Expr.
/// Returns `compile_error!` if the attribute is missing, empty, or not parsable.
pub fn expr_req(
    map: &BTreeMap<&str, &str>,
    key: &str,
    tag: &str,
) -> Result<syn::Expr, TokenStream> {
    let some = map.get(key).copied();
    let src = match some {
        Some(s) if !s.trim().is_empty() => s,
        _ => {
            let msg = format!("efx: <{}> requires `{}` attribute", tag, key);
            return Err(quote! { compile_error!(#msg); });
        }
    };

    match syn::parse_str::<syn::Expr>(src) {
        Ok(e) => Ok(e),
        Err(_) => {
            let msg = format!(
                "efx: attribute `{}` must be a valid Rust expression, got `{}`",
                key, src
            );
            Err(quote! { compile_error!(#msg); })
        }
    }
}

#[allow(dead_code)]
/// Optional expression attribute: `None` if absent; `compile_error!` if present but not parsed.
pub fn expr_opt(map: &BTreeMap<&str, &str>, key: &str) -> Result<Option<syn::Expr>, TokenStream> {
    match map.get(key) {
        None => Ok(None),
        Some(src) => {
            if src.trim().is_empty() {
                return Ok(None);
            }
            match syn::parse_str::<syn::Expr>(src) {
                Ok(e) => Ok(Some(e)),
                Err(_) => {
                    let msg = format!(
                        "efx: attribute `{}` must be a valid Rust expression, got `{}`",
                        key, src
                    );
                    Err(quote! { compile_error!(#msg); })
                }
            }
        }
    }
}
