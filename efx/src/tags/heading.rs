use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::buffer::build_buffer_from_children;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::LitStr;

pub struct Heading {
    attributes: Attributes,
    element: Element,
}

impl Tag for Heading {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        let attributes = Attributes::new(el)?;
        Ok(Self {
            attributes,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let (buf_init, buf_build) = build_buffer_from_children(&self.element.children);

        let mods = self.get_mods();
        let has_mods = !mods.is_empty() || self.attributes.tooltip.is_some();

        // If no mods → plain ui.heading(...)
        if !has_mods {
            return quote! {
                #buf_init
                #buf_build
                let _ = #ui.heading(__efx_buf);
            };
        }

        let tooltip_ts = if let Some(tt) = &self.attributes.tooltip {
            let tt_lit = LitStr::new(tt, Span::call_site());
            quote! { __efx_resp = __efx_resp.on_hover_text(#tt_lit); }
        } else {
            quote! {}
        };

        // Otherwise → RichText + tooltip
        quote! {
            #buf_init
            #buf_build
            let __efx_rich = egui::RichText::new(__efx_buf) #mods;
            let mut __efx_resp = #ui.heading(__efx_rich);
            #tooltip_ts
            let _ = __efx_resp;
        }
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        self.content(ui)
    }
}

impl Heading {
    fn get_mods(&self) -> TokenStream {
        let mut ts = TokenStream::new();

        if let Some(color_ts) = &self.attributes.color {
            ts.extend(quote! { .color(#color_ts) });
        }

        if let Some(size) = self.attributes.size {
            ts.extend(quote! { .size(#size as f32) });
        } else if let Some(level) = self.attributes.level {
            let default_size = match level {
                1 => 24.0f32,
                2 => 20.0f32,
                3 => 18.0f32,
                4 => 16.0f32,
                5 => 14.0f32,
                6 => 12.0f32,
                _ => 16.0f32,
            };
            ts.extend(quote! { .size(#default_size) });
        }

        ts
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    level: Option<u8>,
    size: Option<f32>,
    color: Option<TokenStream>,
    tooltip: Option<String>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Self::ATTR_NAMES, "Heading") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let level = u8_opt(&map, "level")?;
        if let Some(lv) = level {
            if lv < 1 || lv > 6 {
                return Err(quote! { compile_error!("efx: <Heading> `level` must be in 1..=6"); });
            }
        }

        Ok(Attributes {
            level,
            size: f32_opt(&map, "size")?,
            color: color_tokens_opt(&map, "color")?,
            tooltip: map.get("tooltip").map(|s| (*s).to_string()),
        })
    }
}
