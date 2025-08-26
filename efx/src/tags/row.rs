use efx_core::Element;
use quote::{ToTokens, quote};

use crate::attr_adapters as A;
use crate::render::render_nodes_as_stmts;

pub(crate) fn render_row_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &["gap", "padding", "align", "wrap"];

    let mut seen = std::collections::BTreeSet::<&str>::new();
    let mut gap: Option<f32> = None;
    let mut padding: Option<f32> = None;
    let mut align: Option<String> = None;
    let mut wrap: bool = false;

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
            "gap" => match A::parse_f32("gap", val) {
                Ok(n) => gap = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "padding" => match A::parse_f32("padding", val) {
                Ok(n) => padding = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "align" => {
                align = Some(val.to_string());
            }
            "wrap" => {
                // "true"/"false"
                match A::parse_bool("wrap", val) {
                    Ok(b) => wrap = b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            _ => {}
        }
    }

    // Settings
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

    let body = render_nodes_as_stmts(&quote!(ui), &el.children);

    // align / wrap
    let content = if wrap {
        // horizontal_wrapped
        quote! {
            #ui.horizontal_wrapped(|ui| {
                #body
            });
        }
    } else if let Some(al) = align {
        // map string → egui::Align
        let align_expr = match al.as_str() {
            "top" => quote!(::egui::Align::Min),
            "bottom" => quote!(::egui::Align::Max),
            "center" => quote!(::egui::Align::Center),
            other => {
                let msg = format!("efx: invalid align '{}', expected top|bottom|center", other);
                return quote! { compile_error!(#msg); };
            }
        };
        quote! {
            #ui.with_layout(::egui::Layout::left_to_right(#align_expr), |ui| {
                #body
            });
        }
    } else {
        // default horizontal
        quote! {
            #ui.horizontal(|ui| {
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
