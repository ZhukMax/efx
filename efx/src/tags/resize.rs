use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct Resize {
    attributes: Attributes,
    element: Element,
}

impl Tag for Resize {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        Ok(Self {
            attributes: Attributes::new(el)?,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, _ui: &UI) -> TokenStream {
        // id is required so that the state (size) is preserved between frames
        let id = match &self.attributes.id {
            Some(s) if !s.is_empty() => s,
            _ => {
                return quote! { compile_error!("efx: <Resize> requires non-empty `id` attribute"); };
            }
        };

        let mut build = quote!( let mut __efx_resize = egui::Resize::default().id_salt(#id); );

        if let Some(b) = self.attributes.resizable {
            build.extend(quote!( __efx_resize = __efx_resize.resizable(#b); ));
        }

        if let Some(b) = self.attributes.clip {
            build.extend(quote!( __efx_resize = __efx_resize.clip(#b); ));
        }

        // default-size / min-size / max-size
        if self.attributes.default_width.is_some() || self.attributes.default_height.is_some() {
            let w = self.attributes.default_width.unwrap_or(0.0);
            let h = self.attributes.default_height.unwrap_or(0.0);
            build.extend(quote!( __efx_resize = __efx_resize.default_size(egui::vec2(#w as f32, #h as f32)); ));
        }
        if self.attributes.min_width.is_some() || self.attributes.min_height.is_some() {
            let w = self.attributes.min_width.unwrap_or(0.0);
            let h = self.attributes.min_height.unwrap_or(0.0);
            build.extend(
                quote!( __efx_resize = __efx_resize.min_size(egui::vec2(#w as f32, #h as f32)); ),
            );
        }
        if self.attributes.max_width.is_some() || self.attributes.max_height.is_some() {
            let w = self.attributes.max_width.unwrap_or(f32::INFINITY);
            let h = self.attributes.max_height.unwrap_or(f32::INFINITY);
            build.extend(
                quote!( __efx_resize = __efx_resize.max_size(egui::vec2(#w as f32, #h as f32)); ),
            );
        }

        build
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let children = render_children_stmt(&quote!(ui), &self.element.children);
        let build = self.content(ui);

        quote! {{
            #build
            __efx_resize.show(#ui, |ui| { #children });
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    /// required: persist size across frames
    id: Option<String>,

    // behavior
    resizable: Option<bool>,
    clip: Option<bool>,

    #[attr(name = "default-width")]
    default_width: Option<f32>,
    #[attr(name = "default-height")]
    default_height: Option<f32>,

    #[attr(name = "min-width")]
    min_width: Option<f32>,
    #[attr(name = "min-height")]
    min_height: Option<f32>,

    #[attr(name = "max-width")]
    max_width: Option<f32>,
    #[attr(name = "max-height")]
    max_height: Option<f32>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = attr_map(el, Self::ATTR_NAMES, "Resize")?;
        Ok(Attributes {
            id: map.get("id").map(|s| (*s).to_string()),
            resizable: bool_opt(&map, "resizable")?,
            clip: bool_opt(&map, "clip")?,
            default_width: f32_opt(&map, "default-width")?,
            default_height: f32_opt(&map, "default-height")?,
            min_width: f32_opt(&map, "min-width")?,
            min_height: f32_opt(&map, "min-height")?,
            max_width: f32_opt(&map, "max-width")?,
            max_height: f32_opt(&map, "max-height")?,
        })
    }
}
