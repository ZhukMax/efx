#![deny(rust_2018_idioms)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr, Result as SynResult};

/// Derive to generate `ATTR_NAMES` constant (and optionally `ATTR_INFO`) for attribute structures.
/// Supports:
/// - `#[attr(name = "...")]` / `#[efx(name = "...")]`
/// - `#[attr(alias = "...")]` / `#[efx(alias = "...")]` (несколько раз)
/// - `#[attr(skip)]` / `#[efx(skip)]`
/// - `#[attr(prefix = "...")]` / `#[efx(prefix = "...")]` на контейнере (struct)
///
/// Дополнительно:
/// - Если в целевом пакете включена фича `attrnames-info`, генерируется:
///   `pub const ATTR_INFO: &'static [(&'static str, &'static [&'static str])]`.
#[proc_macro_derive(AttrNames, attributes(attr, efx))]
pub fn derive_attr_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_attr_names(&input).unwrap_or_else(|e| e.to_compile_error().into())
}

fn expand_attr_names(input: &DeriveInput) -> SynResult<TokenStream> {
    let ty_ident = &input.ident;

    // Supports only structures with named fields
    let named = match &input.data {
        Data::Struct(st) => match &st.fields {
            Fields::Named(n) => n,
            _ => {
                return Err(syn::Error::new_spanned(
                    &st.fields,
                    "AttrNames supports only structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "AttrNames supports only structs",
            ));
        }
    };

    // Prefix on container: #[attr(prefix="data-")] or #[efx(prefix="data-")]
    let mut container_prefix: Option<String> = None;
    for a in &input.attrs {
        if is_attr_or_efx(a) {
            a.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    let lit: LitStr = meta.value()?.parse()?;
                    container_prefix = Some(lit.value());
                }
                Ok(())
            })?;
        }
    }

    // Assembling names (primary + aliases) by fields
    struct FieldSpec {
        primary: (String, Span),
        aliases: Vec<(String, Span)>,
    }

    let mut specs: Vec<FieldSpec> = Vec::with_capacity(named.named.len());

    for f in &named.named {
        let ident = match &f.ident {
            Some(id) => id,
            None => {
                return Err(syn::Error::new_spanned(f, "expected named field"));
            }
        };

        let mut skip = false;
        let mut name: Option<(String, Span)> = None;
        let mut aliases: Vec<(String, Span)> = Vec::new();

        for a in &f.attrs {
            if is_attr_or_efx(a) {
                a.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        skip = true;
                    } else if meta.path.is_ident("name") {
                        let lit: LitStr = meta.value()?.parse()?;
                        name = Some((lit.value(), lit.span()));
                    } else if meta.path.is_ident("alias") {
                        let lit: LitStr = meta.value()?.parse()?;
                        aliases.push((lit.value(), lit.span()));
                    }
                    Ok(())
                })?;
            }
        }

        if skip {
            continue;
        }

        let (base_name, base_span) = match name {
            Some((n, sp)) => (n, sp),
            None => (ident.to_string(), ident.span()),
        };

        // Apply prefix to all names (primary + aliases) if specified
        let with_prefix = |s: &str| -> String {
            if let Some(pref) = &container_prefix {
                let mut out = String::with_capacity(pref.len() + s.len());
                out.push_str(pref);
                out.push_str(s);
                out
            } else {
                s.to_string()
            }
        };

        let prim = (with_prefix(&base_name), base_span);
        let ali = aliases
            .into_iter()
            .map(|(a, sp)| (with_prefix(&a), sp))
            .collect::<Vec<_>>();

        specs.push(FieldSpec {
            primary: prim,
            aliases: ali,
        });
    }

    // Check for duplicates after applying prefix
    let mut seen: HashSet<String> = HashSet::new();
    for spec in &specs {
        let (p, pspan) = &spec.primary;
        if !seen.insert(p.clone()) {
            return Err(syn::Error::new(
                *pspan,
                format!("duplicate attribute name '{p}'"),
            ));
        }
        for (a, aspan) in &spec.aliases {
            if !seen.insert(a.clone()) {
                return Err(syn::Error::new(
                    *aspan,
                    format!("duplicate attribute name '{a}'"),
                ));
            }
        }
    }

    // Preparing literals
    let mut name_lits: Vec<LitStr> = Vec::with_capacity(seen.len());
    for spec in &specs {
        let (p, sp) = &spec.primary;
        name_lits.push(LitStr::new(p, *sp));
        for (a, asp) in &spec.aliases {
            name_lits.push(LitStr::new(a, *asp));
        }
    }

    // For ATTR_INFO (`attrnames-info`)
    // &[("primary", &["alias1","alias2"]), ...]
    let info_pairs = specs.iter().map(|spec| {
        let (p, psp) = &spec.primary;
        let prim_lit = LitStr::new(p, *psp);
        let alias_lits: Vec<LitStr> = spec
            .aliases
            .iter()
            .map(|(a, sp)| LitStr::new(a, *sp))
            .collect();
        quote! {
            (#prim_lit, &[ #( #alias_lits ),* ])
        }
    });

    let output = quote! {
        impl #ty_ident {
            pub const ATTR_NAMES: &'static [&'static str] = &[ #( #name_lits ),* ];

            /// Additional information: pairs (primary, aliases), generated only if **target** packet has aliases enabled
            /// feature `attrnames-info`:
            ///
            /// ```toml
            /// [features]
            /// attrnames-info = []
            /// ```
            #[cfg(feature = "attrnames-info")]
            pub const ATTR_INFO: &'static [(&'static str, &'static [&'static str])] = &[
                #( #info_pairs ),*
            ];
        }
    };

    Ok(output.into())
}

#[inline]
fn is_attr_or_efx(attr: &syn::Attribute) -> bool {
    let p = attr.path();
    p.is_ident("attr") || p.is_ident("efx")
}
