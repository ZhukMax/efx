use efx_core::Element;
use quote::{ToTokens, quote};

use crate::render::render_nodes_as_stmts;
use crate::tags::util::{attr_map, f32_opt};

pub fn render_column_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &["gap", "padding", "align"];
    let map = match attr_map(el, KNOWN, "Column") {
        Ok(m) => m,
        Err(err) => return err,
    };

    // vertical spacing between children
    let gap = f32_opt(&map, "gap").unwrap_or(None);
    let padding = f32_opt(&map, "padding").unwrap_or(None);
    // left|center|right (horizontal alignment of children)
    let align = map.get("align").map(|s| (*s).to_string());

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
