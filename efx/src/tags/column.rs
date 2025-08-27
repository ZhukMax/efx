use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::render::render_nodes_as_stmts;
use crate::tags::util::{attr_map, f32_opt};
use crate::tags::{TagAttributes, Tagged};

pub struct Column;

impl Tagged for Column {
    fn parse<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream {
        let attributes = match Attributes::new(el) {
            Ok(attr) => attr,
            Err(err) => return err,
        };

        let mut prolog = TokenStream::new();
        let mut epilog = TokenStream::new();

        if let Some(n) = attributes.gap {
            prolog.extend(quote! {
                let __efx_old_gap_y = #ui.spacing().item_spacing.y;
                #ui.spacing_mut().item_spacing.y = #n as f32;
            });
            epilog.extend(quote! {
                #ui.spacing_mut().item_spacing.y = __efx_old_gap_y;
            });
        }

        if let Some(p) = attributes.padding {
            prolog.extend(quote! { #ui.add_space(#p as f32); });
            epilog.extend(quote! { #ui.add_space(#p as f32); });
        }

        let content = Column::content(ui, el, attributes);

        quote! {
            {
                #prolog
                #content
                #epilog
            }
        }
    }
}

impl Column {
    fn content<UI: ToTokens>(ui: &UI, el: &Element, attributes: Attributes) -> TokenStream {
        let body = render_nodes_as_stmts(&quote!(ui), &el.children);

        // align: left|center|right → egui::Align::{Min,Center,Max} in Layout::top_down(...)
        if let Some(align) = attributes.align {
            let align_expr = match align.as_str() {
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
        }
    }
}

#[derive(Clone, Debug)]
struct Attributes {
    gap: Option<f32>,
    padding: Option<f32>,
    align: Option<String>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        const KNOWN: &[&str] = &["gap", "padding", "align"];
        let map = match attr_map(el, KNOWN, "Column") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            gap: f32_opt(&map, "gap").unwrap_or(None),
            padding: f32_opt(&map, "padding").unwrap_or(None),
            align: map.get("align").map(|s| (*s).to_string()),
        })
    }
}
