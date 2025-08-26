use efx_core::Element;
use quote::{ToTokens, quote};

use crate::attr_adapters as A;
use crate::render::render_nodes_as_stmts;

pub fn render_column_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &["gap", "padding", "align"];

    let mut seen = std::collections::BTreeSet::<&str>::new();

    let mut gap: Option<f32> = None; // vertical spacing between children
    let mut padding: Option<f32> = None; // top/bottom
    let mut align: Option<String> = None; // left|center|right (horizontal alignment of children)

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <Column> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }
        if !seen.insert(name) {
            let msg = format!("efx: <Column> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "gap" => match A::parse_f32("gap", val) {
                Ok(n) => gap = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "padding" => match A::parse_f32("padding", val) {
                Ok(n) => padding = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "align" => {
                // parse the line, check below
                align = Some(val.to_string());
            }
            _ => {}
        }
    }

    let mut prolog = proc_macro2::TokenStream::new();
    let mut epilog = proc_macro2::TokenStream::new();

    if let Some(n) = gap {
        prolog.extend(quote! {
            let __efx_old_gap_y = #ui.spacing().item_spacing.y;
            #ui.spacing_mut().item_spacing.y = #n as f32;
        });
        epilog.extend(quote! {
            #ui.spacing_mut().item_spacing.y = __efx_old_gap_y;
        });
    }

    if let Some(p) = padding {
        prolog.extend(quote! { #ui.add_space(#p as f32); });
        epilog.extend(quote! { #ui.add_space(#p as f32); });
    }

    let body = render_nodes_as_stmts(&quote!(ui), &el.children);

    // align: left|center|right → egui::Align::{Min,Center,Max} in Layout::top_down(...)
    let content = if let Some(al) = align {
        let align_expr = match al.as_str() {
            "left" => quote!(::egui::Align::Min),
            "right" => quote!(::egui::Align::Max),
            "center" => quote!(::egui::Align::Center),
            other => {
                let msg = format!("efx: invalid align '{}', expected left|center|right", other);
                return quote! { compile_error!(#msg); };
            }
        };
        quote! {
            #ui.with_layout(::egui::Layout::top_down(#align_expr), |ui| {
                #body
            });
        }
    } else {
        // default: vertical
        quote! {
            #ui.vertical(|ui| {
                #body
            });
        }
    };

    quote! {
        {
            #prolog
            #content
            #epilog
        }
    }
}
