use crate::tags::TagAttributes;
use crate::utils::attr::*;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Copy, Clone, Debug)]
pub enum Dim {
    Width,
    Height,
}

/// Set of general options for the panel frame.
#[derive(Clone, Debug)]
pub struct FrameStyle {
    pub frame_on: Option<bool>,    // true/None => default(), false => none()
    pub fill: Option<TokenStream>, // Color32 expression
    pub stroke_w: Option<f32>,     // points
    pub stroke_color: Option<TokenStream>, // Color32 expression

    // padding (inner_margin)
    pub pad: Option<f32>,
    pub pad_l: Option<f32>,
    pub pad_r: Option<f32>,
    pub pad_t: Option<f32>,
    pub pad_b: Option<f32>,

    // margin (outer_margin)
    pub mar: Option<f32>,
    pub mar_l: Option<f32>,
    pub mar_r: Option<f32>,
    pub mar_t: Option<f32>,
    pub mar_b: Option<f32>,
}

impl FrameStyle {
    pub fn new(attributes: Attributes) -> Self {
        FrameStyle {
            frame_on: attributes.frame,
            fill: attributes.fill.clone(),
            stroke_w: attributes.stroke_width,
            stroke_color: attributes.stroke_color.clone(),

            // padding (inner)
            pad: attributes.padding,
            pad_l: attributes.padding_l,
            pad_r: attributes.padding_r,
            pad_t: attributes.padding_t,
            pad_b: attributes.padding_b,

            // margin (outer)
            mar: attributes.margin,
            mar_l: attributes.margin_l,
            mar_r: attributes.margin_r,
            mar_t: attributes.margin_t,
            mar_b: attributes.margin_b,
        }
    }

    /// Generate tokens with `let mut __efx_frame = ...;` construction and all modifiers.
    pub fn emit(&self) -> TokenStream {
        let mut ts = TokenStream::new();

        // default/none
        ts.extend(match self.frame_on {
            Some(false) => quote!( let mut __efx_frame = egui::Frame::none(); ),
            _ => quote!( let mut __efx_frame = egui::Frame::default(); ),
        });

        if let Some(fill) = &self.fill {
            ts.extend(quote!( __efx_frame = __efx_frame.fill(#fill); ));
        }

        if let Some(im) = margin_tokens(self.pad, self.pad_l, self.pad_r, self.pad_t, self.pad_b) {
            ts.extend(quote!( __efx_frame = __efx_frame.inner_margin(#im); ));
        }
        if let Some(om) = margin_tokens(self.mar, self.mar_l, self.mar_r, self.mar_t, self.mar_b) {
            ts.extend(quote!( __efx_frame = __efx_frame.outer_margin(#om); ));
        }

        if let Some(st) = stroke_tokens(self.stroke_w, self.stroke_color.clone()) {
            ts.extend(quote!( __efx_frame = __efx_frame.stroke(#st); ));
        }

        ts
    }
}

#[derive(Clone, Debug, AttrNames)]
pub struct Attributes {
    /// required: egui Id salt
    pub(crate) id: Option<String>,

    // frame + styling
    pub(crate) frame: Option<bool>,
    pub(crate) fill: Option<TokenStream>,
    #[attr(name = "stroke-width")]
    pub(crate) stroke_width: Option<f32>,
    #[attr(name = "stroke-color")]
    pub(crate) stroke_color: Option<TokenStream>,

    // sizing
    #[attr(name = "default-height")]
    pub(crate) default_height: Option<f32>,
    #[attr(name = "min-height")]
    pub(crate) min_height: Option<f32>,
    #[attr(name = "max-height")]
    pub(crate) max_height: Option<f32>,
    pub(crate) resizable: Option<bool>,

    // padding (inner_margin)
    pub(crate) padding: Option<f32>,
    #[attr(name = "padding-left")]
    pub(crate) padding_l: Option<f32>,
    #[attr(name = "padding-right")]
    pub(crate) padding_r: Option<f32>,
    #[attr(name = "padding-top")]
    pub(crate) padding_t: Option<f32>,
    #[attr(name = "padding-bottom")]
    pub(crate) padding_b: Option<f32>,

    // margin (outer_margin)
    pub(crate) margin: Option<f32>,
    #[attr(name = "margin-left")]
    pub(crate) margin_l: Option<f32>,
    #[attr(name = "margin-right")]
    pub(crate) margin_r: Option<f32>,
    #[attr(name = "margin-top")]
    pub(crate) margin_t: Option<f32>,
    #[attr(name = "margin-bottom")]
    pub(crate) margin_b: Option<f32>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = attr_map(el, Attributes::ATTR_NAMES, el.name.clone().as_str())?;
        Ok(Attributes {
            id: map.get("id").map(|s| (*s).to_string()),

            frame: bool_opt(&map, "frame")?,
            fill: color_tokens_opt(&map, "fill")?,
            stroke_width: f32_opt(&map, "stroke-width")?,
            stroke_color: color_tokens_opt(&map, "stroke-color")?,

            default_height: f32_opt(&map, "default-height")?,
            min_height: f32_opt(&map, "min-height")?,
            max_height: f32_opt(&map, "max-height")?,
            resizable: bool_opt(&map, "resizable")?,

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

/// General options for panel size and resizing.
#[derive(Clone, Debug, Default)]
pub struct SizeOpts {
    pub resizable: Option<bool>,
    pub default: Option<f32>,
    pub min: Option<f32>,
    pub max: Option<f32>,
}

/// Apply default_/min_/max_ methods depending on the dimension.
/// Assumes the panel variable is named `__efx_panel`.
pub fn emit_size_methods(dim: Dim, s: &SizeOpts) -> TokenStream {
    let (def_m, min_m, max_m) = match dim {
        Dim::Width => (
            format_ident!("default_width"),
            format_ident!("min_width"),
            format_ident!("max_width"),
        ),
        Dim::Height => (
            format_ident!("default_height"),
            format_ident!("min_height"),
            format_ident!("max_height"),
        ),
    };

    let mut ts = TokenStream::new();
    if let Some(b) = s.resizable {
        ts.extend(quote!( __efx_panel = __efx_panel.resizable(#b); ));
    }
    if let Some(v) = s.default {
        ts.extend(quote!( __efx_panel = __efx_panel.#def_m(#v as f32); ));
    }
    if let Some(v) = s.min {
        ts.extend(quote!( __efx_panel = __efx_panel.#min_m(#v as f32); ));
    }
    if let Some(v) = s.max {
        ts.extend(quote!( __efx_panel = __efx_panel.#max_m(#v as f32); ));
    }
    ts
}
