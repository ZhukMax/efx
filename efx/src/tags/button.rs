use crate::interfaces::*;
use crate::utils::attr::*;
use crate::utils::buffer::build_buffer_from_children;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Button {
    attributes: Attributes,
    element: Element,
}

impl Tag for Button {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        let attributes = Attributes::new(el)?;
        Ok(Self {
            attributes,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        // Assembling a button builder
        let mut btn_build = TokenStream::new();
        btn_build.extend(quote!( let mut __efx_btn = egui::Button::new(__efx_rich); ));

        if let Some(ts) = self.attributes.fill.clone() {
            btn_build.extend(quote!( __efx_btn = __efx_btn.fill(#ts); ));
        }

        if let Some(r) = self.attributes.rounding {
            btn_build
                .extend(quote!( __efx_btn = __efx_btn.rounding(egui::Rounding::same(#r as _)); ));
        }

        if self.attributes.min_width.is_some() || self.attributes.min_height.is_some() {
            let w = self.attributes.min_width.unwrap_or(0.0);
            let h = self.attributes.min_height.unwrap_or(0.0);

            btn_build.extend(
                quote!( __efx_btn = __efx_btn.min_size(egui::vec2(#w as f32, #h as f32)); ),
            );
        }

        if let Some(b) = self.attributes.frame.clone() {
            btn_build.extend(quote!( __efx_btn = __efx_btn.frame(#b); ));
        }

        let add_btn = match self.attributes.enabled.clone() {
            Some(false) => quote!( let mut __efx_resp = #ui.add_enabled(false, __efx_btn); ),
            _ => quote!( let mut __efx_resp = #ui.add(__efx_btn); ),
        };

        quote!( #btn_build #add_btn )
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let (buf_init, buf_build) = build_buffer_from_children(&self.element.children);

        if self.attributes.is_plain_mode() {
            return quote! {{
                #buf_init
                #buf_build
                #ui.button(__efx_buf)
            }};
        }

        // Otherwise - egui::Button with RichText and modifiers
        let rich_decl = quote!( let __efx_rich = egui::RichText::new(__efx_buf); );

        let content = self.content(ui);

        // Tooltip
        let tooltip_apply = if let Some(text) = self.attributes.tooltip.clone() {
            quote!( __efx_resp = __efx_resp.on_hover_text(#text); )
        } else {
            quote!()
        };

        quote! {{
            #buf_init
            #buf_build
            #rich_decl
            #content
            #tooltip_apply
            __efx_resp
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    fill: Option<TokenStream>,
    min_width: Option<f32>,
    min_height: Option<f32>,
    frame: Option<bool>,
    enabled: Option<bool>,
    rounding: Option<u8>,
    tooltip: Option<String>,
}

impl Attributes {
    fn is_plain_mode(&self) -> bool {
        let has_style_attrs = self.min_width.is_some()
            || self.min_height.is_some()
            || self.frame.is_some()
            || self.fill.is_some()
            || self.rounding.is_some();

        !has_style_attrs && self.enabled.is_none() && self.tooltip.is_none()
    }
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "Button") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            fill: color_tokens_opt(&map, "fill")?,
            min_width: f32_opt(&map, "min_width")?,
            min_height: f32_opt(&map, "min_height")?,
            frame: bool_opt(&map, "frame")?,
            enabled: bool_opt(&map, "enabled")?,
            rounding: u8_opt(&map, "rounding")?,
            tooltip: map.get("tooltip").map(|s| (*s).to_string()),
        })
    }
}
