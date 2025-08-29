use crate::render::render_nodes_as_stmts;
use crate::tags::util::{attr_map, bool_or, f32_opt};
use crate::tags::{Block, TagAttributes};
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct Row {
    attributes: Attributes,
    element: Element,
}

impl Block for Row {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        let attributes = Attributes::new(el)?;
        Ok(Self {
            attributes,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let body = render_nodes_as_stmts(&quote!(ui), &self.element.children);

        // align / wrap
        if self.attributes.wrap {
            // horizontal_wrapped
            quote! {
                #ui.horizontal_wrapped(|ui| {
                    #body
                });
            }
        } else if let Some(al) = &self.attributes.align {
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

    fn prolog_epilogue<UI: ToTokens>(&self, ui: &UI) -> (TokenStream, TokenStream) {
        let mut prolog = TokenStream::new();
        let mut epilogue = TokenStream::new();

        if let Some(n) = &self.attributes.gap {
            prolog.extend(quote! {
                let __efx_old_gap_x = #ui.spacing().item_spacing.x;
                #ui.spacing_mut().item_spacing.x = #n as _;
            });
            epilogue.extend(quote! {
                #ui.spacing_mut().item_spacing.x = __efx_old_gap_x;
            });
        }

        if let Some(p) = self.attributes.padding {
            prolog.extend(quote! { #ui.add_space(#p as _); });
            epilogue.extend(quote! { #ui.add_space(#p as _); });
        }

        (prolog, epilogue)
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    gap: Option<f32>,
    padding: Option<f32>,
    align: Option<String>,
    wrap: bool,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "Row") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let wrap = bool_or(&map, "wrap", false)?;

        Ok(Attributes {
            gap: f32_opt(&map, "gap").unwrap_or(None),
            padding: f32_opt(&map, "padding").unwrap_or(None),
            align: map.get("align").map(|s| (*s).to_string()),
            wrap,
        })
    }
}
