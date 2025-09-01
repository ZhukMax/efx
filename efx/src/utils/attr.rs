use crate::attr_adapters as A;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use syn::Expr;

#[inline]
pub fn attr_map<'a>(
    el: &'a Element,
    known: &[&str],
    tag: &str,
) -> Result<BTreeMap<&'a str, &'a str>, TokenStream> {
    let mut seen = BTreeMap::<&str, usize>::new();
    let mut out = BTreeMap::<&str, &str>::new();

    for a in &el.attrs {
        let name = a.name.as_str();
        if !known.iter().any(|k| *k == name) {
            let msg = format!("efx: <{}> unknown attribute `{}`", tag, name);
            return Err(quote! { compile_error!(#msg); });
        }
        if seen.insert(name, 1).is_some() {
            let msg = format!("efx: <{}> duplicate attribute `{}`", tag, name);
            return Err(quote! { compile_error!(#msg); });
        }
        out.insert(name, a.value.as_str());
    }

    Ok(out)
}

#[inline]
pub fn bool_opt(map: &BTreeMap<&str, &str>, key: &str) -> Result<Option<bool>, TokenStream> {
    Ok(match map.get(key) {
        Some(v) => Some(A::parse_bool(key, v).map_err(|m| quote! { compile_error!(#m); })?),
        None => None,
    })
}

#[inline]
pub fn bool_or(map: &BTreeMap<&str, &str>, key: &str, default: bool) -> Result<bool, TokenStream> {
    match map.get(key) {
        None => Ok(default),
        Some(v) => crate::attr_adapters::parse_bool(key, v)
            .map_err(|m| quote::quote! { compile_error!(#m); }),
    }
}

#[inline]
pub fn f32_opt(map: &BTreeMap<&str, &str>, key: &str) -> Result<Option<f32>, TokenStream> {
    Ok(match map.get(key) {
        Some(v) => Some(A::parse_f32(key, v).map_err(|m| quote! { compile_error!(#m); })?),
        None => None,
    })
}

#[inline]
pub fn u8_opt(map: &BTreeMap<&str, &str>, key: &str) -> Result<Option<u8>, TokenStream> {
    Ok(match map.get(key) {
        Some(v) => Some(A::parse_u8(key, v).map_err(|m| quote! { compile_error!(#m); })?),
        None => None,
    })
}

pub fn color_tokens_opt(
    map: &BTreeMap<&str, &str>,
    key: &str,
) -> Result<Option<TokenStream>, TokenStream> {
    Ok(match map.get(key) {
        Some(v) => Some(A::parse_color_tokens(key, v).map_err(|m| quote! { compile_error!(#m); })?),
        None => None,
    })
}

/// Building egui::Margin from uniform/per-side options.
/// Use `as _` to avoid being limited to a specific numeric field type.
/// Returns Some(TokenStream) if something is given, None otherwise.
pub fn margin_tokens(
    uniform: Option<f32>,
    l: Option<f32>,
    r: Option<f32>,
    t: Option<f32>,
    b: Option<f32>,
) -> Option<TokenStream> {
    if uniform.is_none() && l.is_none() && r.is_none() && t.is_none() && b.is_none() {
        return None;
    }
    let mk = |side: Option<f32>, uni: Option<f32>| -> TokenStream {
        if let Some(v) = side {
            quote!( #v as _ )
        } else if let Some(u) = uni {
            quote!( #u as _ )
        } else {
            quote!(0 as _)
        }
    };
    let l_ts = mk(l, uniform);
    let r_ts = mk(r, uniform);
    let t_ts = mk(t, uniform);
    let b_ts = mk(b, uniform);

    Some(quote!( egui::Margin { left: #l_ts, right: #r_ts, top: #t_ts, bottom: #b_ts } ))
}

/// Build `egui::Stroke` from optional width and color.
/// Returns `None` if both parameters are missing.
/// Numeric casts are done via `as _`, so as not to be limited to a specific type.
pub fn stroke_tokens(width: Option<f32>, color: Option<TokenStream>) -> Option<TokenStream> {
    if width.is_none() && color.is_none() {
        return None;
    }
    let w = width.unwrap_or(1.0);
    let c = color.unwrap_or_else(|| quote!(egui::Color32::BLACK));

    Some(quote!( egui::Stroke { width: #w as _, color: #c } ))
}

pub fn is_assignable_expr(e: &Expr) -> bool {
    use syn::{
        Expr::{Field, Index, Paren, Path},
        ExprParen,
    };
    match e {
        Path(_) | Field(_) | Index(_) => true,
        Paren(ExprParen { expr, .. }) => is_assignable_expr(expr),
        _ => false,
    }
}
