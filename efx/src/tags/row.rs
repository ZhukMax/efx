use quote::{quote, ToTokens};
use efx_core::Element;
use crate::buffer::build_buffer_from_children;

use crate::attr_adapters as A;

pub(crate) fn render_row_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);

    const KNOWN: &[&str] = &["gap", "padding", "align", "wrap"];

    let mut seen = std::collections::BTreeSet::<&str>::new();
    let mut gap: Option<f32> = None;
    let mut padding: Option<f32> = None;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <Row> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }
        if !seen.insert(name) {
            let msg = format!("efx: <Row> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "gap" => {
                let n = match A::parse_f32("gap", val) {
                    Ok(n) => n,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                gap = Some(n);
            }
            "padding" => {
                let n = match A::parse_f32("padding", val) {
                    Ok(n) => n,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                padding = Some(n);
            }
            "align" | "wrap" => { /* TODO(0.5): Layout + horizontal_wrapped */ }
            _ => {}
        }
    }

    let mut prolog = proc_macro2::TokenStream::new();
    let mut epilog = proc_macro2::TokenStream::new();

    if let Some(n) = gap {
        prolog.extend(quote! {
            let __efx_old_gap_x = #ui.spacing().item_spacing.x;
            #ui.spacing_mut().item_spacing.x = #n as f32;
        });
        epilog.extend(quote! {
            #ui.spacing_mut().item_spacing.x = __efx_old_gap_x;
        });
    }

    if let Some(p) = padding {
        prolog.extend(quote! { #ui.add_space(#p as f32); });
        epilog.extend(quote! { #ui.add_space(#p as f32); });
    }

    let inner_ui = quote!(ui);
    let body = crate::render::render_nodes_as_stmts(&inner_ui, &el.children);

    quote! {
        {
            #prolog
            #ui.horizontal(|ui| {
                #buf_init
                #buf_build
                #body
            });
            #epilog
        }
    }
}