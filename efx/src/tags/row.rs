use efx_core::{Element, Node};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::render::render_nodes_as_stmts;
use crate::tags::util::{attr_map, bool_or, f32_opt};
use crate::tags::{TagAttributes, Tagged};

pub struct Row;

impl Tagged for Row {
    fn parse<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream {
        let attributes = match Attributes::new(el) {
            Ok(attr) => attr,
            Err(err) => return err,
        };

        let mut prolog = TokenStream::new();
        let mut epilog = TokenStream::new();

        if let Some(n) = attributes.gap {
            prolog.extend(quote! {
                let __efx_old_gap_x = #ui.spacing().item_spacing.x;
                #ui.spacing_mut().item_spacing.x = #n as f32;
            });

            epilog.extend(quote! {
                #ui.spacing_mut().item_spacing.x = __efx_old_gap_x;
            });
        }

        if let Some(p) = attributes.padding {
            prolog.extend(quote! { #ui.add_space(#p as f32); });
            epilog.extend(quote! { #ui.add_space(#p as f32); });
        }

        let content = Row::content(ui, &*el.children, attributes.clone());

        quote! {
            {
                #prolog
                #content
                #epilog
            }
        }
    }
}

impl Row {
    fn content<UI: ToTokens>(ui: &UI, children: &[Node], attributes: Attributes) -> TokenStream {
        let body = render_nodes_as_stmts(&quote!(ui), children);

        // align / wrap
        if attributes.wrap {
            // horizontal_wrapped
            quote! {
                #ui.horizontal_wrapped(|ui| {
                    #body
                });
            }
        } else if let Some(al) = attributes.align {
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
        }
    }
}

#[derive(Clone, Debug)]
struct Attributes {
    gap: Option<f32>,
    padding: Option<f32>,
    align: Option<String>,
    wrap: bool,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        const KNOWN: &[&str] = &["gap", "padding", "align", "wrap"];
        let map = match attr_map(el, KNOWN, "Row") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let wrap: bool = match bool_or(&map, "wrap", false) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            gap: f32_opt(&map, "gap").unwrap_or(None),
            padding: f32_opt(&map, "padding").unwrap_or(None),
            align: map.get("align").map(|s| (*s).to_string()),
            wrap,
        })
    }
}
