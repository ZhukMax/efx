use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::panel::FrameStyle;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct CentralPanel {
    attributes: Attributes,
    element: Element,
}

impl Tag for CentralPanel {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        Ok(Self {
            attributes: Attributes::new(el)?,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, _ui: &UI) -> TokenStream {
        FrameStyle {
            frame_on: self.attributes.frame,
            fill: self.attributes.fill.clone(),
            stroke_w: self.attributes.stroke_width,
            stroke_color: self.attributes.stroke_color.clone(),

            // padding (inner)
            pad: self.attributes.padding,
            pad_l: self.attributes.padding_l,
            pad_r: self.attributes.padding_r,
            pad_t: self.attributes.padding_t,
            pad_b: self.attributes.padding_b,

            // margin (outer)
            mar: self.attributes.margin,
            mar_l: self.attributes.margin_l,
            mar_r: self.attributes.margin_r,
            mar_t: self.attributes.margin_t,
            mar_b: self.attributes.margin_b,
        }
        .emit()
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let children = render_children_stmt(&quote!(ui), &self.element.children);
        let frame_ts = self.content(ui);

        quote! {{
            #frame_ts
            egui::CentralPanel::default()
                .frame(__efx_frame)
                .show(&#ui.ctx(), |ui| { #children });
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    frame: Option<bool>,
    fill: Option<TokenStream>,
    #[attr(name = "stroke-width")]
    stroke_width: Option<f32>,
    #[attr(name = "stroke-color")]
    stroke_color: Option<TokenStream>,

    // padding (inner_margin)
    padding: Option<f32>,
    #[attr(name = "padding-left")]
    padding_l: Option<f32>,
    #[attr(name = "padding-right")]
    padding_r: Option<f32>,
    #[attr(name = "padding-top")]
    padding_t: Option<f32>,
    #[attr(name = "padding-bottom")]
    padding_b: Option<f32>,

    // margin (outer_margin)
    margin: Option<f32>,
    #[attr(name = "margin-left")]
    margin_l: Option<f32>,
    #[attr(name = "margin-right")]
    margin_r: Option<f32>,
    #[attr(name = "margin-top")]
    margin_t: Option<f32>,
    #[attr(name = "margin-bottom")]
    margin_b: Option<f32>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "CentralPanel") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            frame: bool_opt(&map, "frame")?,
            fill: color_tokens_opt(&map, "fill")?,
            stroke_width: f32_opt(&map, "stroke-width")?,
            stroke_color: color_tokens_opt(&map, "stroke-color")?,

            // padding (inner_margin)
            padding: f32_opt(&map, "padding")?,
            padding_l: f32_opt(&map, "padding-left")?,
            padding_r: f32_opt(&map, "padding-right")?,
            padding_t: f32_opt(&map, "padding-top")?,
            padding_b: f32_opt(&map, "padding-bottom")?,

            // margin (outer_margin)
            margin: f32_opt(&map, "margin")?,
            margin_l: f32_opt(&map, "margin-left")?,
            margin_r: f32_opt(&map, "margin-right")?,
            margin_t: f32_opt(&map, "margin-top")?,
            margin_b: f32_opt(&map, "margin-bottom")?,
        })
    }
}
