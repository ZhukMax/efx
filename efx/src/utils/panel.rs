use crate::utils::attr::*;
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
