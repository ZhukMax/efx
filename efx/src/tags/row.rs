use efx_core::Element;
use quote::{ToTokens, quote};

use crate::attr_adapters as A;
use crate::render::render_nodes_as_stmts;
use crate::tags::util::{attr_map, f32_opt};

pub fn render_row_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &["gap", "padding", "align", "wrap"];
    let map = match attr_map(el, KNOWN, "Row") {
        Ok(m) => m,
        Err(err) => return err,
    };

    let gap = f32_opt(&map, "gap").unwrap_or(None);
    let padding = f32_opt(&map, "padding").unwrap_or(None);
    let align = map.get("align").map(|s| (*s).to_string());
    let mut wrap: bool = false;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        match name {
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
