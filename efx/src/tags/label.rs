use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::buffer::build_buffer_from_children;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Label {
    attributes: Attributes,
    element: Element,
}

impl Tag for Label {
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
        match &self.attributes.wrap {
            Some(true) => {
                quote! {
                    let __efx_widget = egui::widgets::Label::new(__efx_rich).wrap(true);
                    #ui.add(__efx_widget);
                }
            }
            _ => {
                quote! { #ui.label(__efx_rich); }
            }
        }
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let mods = self.set_mods();
        let (buf_init, buf_build) = build_buffer_from_children(&self.element.children);

        let use_plain_string = mods.is_empty() && self.attributes.wrap != Some(true);

        if use_plain_string {
            return quote! {
                #buf_init
                #buf_build
                #ui.label(__efx_buf);
            };
        }

        // Generation: RichText + ui.label(...) or Label::new(...).wrap(true)
        let rich_apply = if mods.is_empty() {
            quote!( let __efx_rich = egui::RichText::new(__efx_buf); )
        } else {
            quote!( let __efx_rich = egui::RichText::new(__efx_buf) #mods ; )
        };

        let content = self.content(ui);

        quote! {
            #buf_init
            #buf_build
            #rich_apply
            #content
        }
    }
}

impl Label {
    fn set_mods(&self) -> TokenStream {
        let mut mods = TokenStream::new();

        if let Some(ts) = &self.attributes.color {
            mods.extend(quote! { .color(#ts) });
        }

        if let Some(n) = self.attributes.size {
            mods.extend(quote! { .size(#n as f32) });
        }

        if matches!(self.attributes.italic, Some(true)) {
            mods.extend(quote! { .italics() });
        }

        if matches!(self.attributes.bold, Some(true)) {
            mods.extend(quote! { .strong() });
        }

        if matches!(self.attributes.underline, Some(true)) {
            mods.extend(quote! { .underline() });
        }

        if matches!(self.attributes.strike, Some(true)) {
            mods.extend(quote! { .strikethrough() });
        }

        if matches!(self.attributes.monospace, Some(true)) {
            mods.extend(quote! { .monospace() });
        }

        mods
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    color: Option<TokenStream>,
    size: Option<f32>,
    italic: Option<bool>,
    bold: Option<bool>,
    underline: Option<bool>,
    strike: Option<bool>,
    monospace: Option<bool>,
    wrap: Option<bool>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Self::ATTR_NAMES, "Label") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            color: color_tokens_opt(&map, "color")?,
            size: f32_opt(&map, "size")?,
            italic: bool_opt(&map, "italic")?,
            bold: bool_opt(&map, "bold")?,
            underline: bool_opt(&map, "underline")?,
            strike: bool_opt(&map, "strike")?,
            monospace: bool_opt(&map, "monospace")?,
            wrap: bool_opt(&map, "wrap").unwrap_or(None),
        })
    }
}
