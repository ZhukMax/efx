use proc_macro2::TokenStream;
use efx_core::Element;
use quote::{ToTokens, quote};

use crate::tags::util::*;

pub fn render_central_panel_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream {
    const KNOWN: &[&str] = &[
        "frame",
        // fill / stroke
        "fill",
        "stroke_width",
        "stroke_color",
        // padding (inner_margin)
        "padding",
        "padding_l",
        "padding_r",
        "padding_t",
        "padding_b",
        // margin (outer_margin)
        "margin",
        "margin_l",
        "margin_r",
        "margin_t",
        "margin_b",
    ];

    let map = match attr_map(el, KNOWN, "CentralPanel") {
        Ok(m) => m,
        Err(err) => return err,
    };

    let frame_on = bool_opt(&map, "frame").unwrap_or(None);

    // fill & stroke
    let fill_ts = color_tokens_opt(&map, "fill").unwrap_or(None);
    let stroke_w = f32_opt(&map, "stroke_width").unwrap_or(None);
    let stroke_col = color_tokens_opt(&map, "stroke_color").unwrap_or(None);

    // padding (inner) & margin (outer)
    let pad = f32_opt(&map, "padding").unwrap_or(None);
    let pad_l = f32_opt(&map, "padding_l").unwrap_or(None);
    let pad_r = f32_opt(&map, "padding_r").unwrap_or(None);
    let pad_t = f32_opt(&map, "padding_t").unwrap_or(None);
    let pad_b = f32_opt(&map, "padding_b").unwrap_or(None);

    let mar = f32_opt(&map, "margin").unwrap_or(None);
    let mar_l = f32_opt(&map, "margin_l").unwrap_or(None);
    let mar_r = f32_opt(&map, "margin_r").unwrap_or(None);
    let mar_t = f32_opt(&map, "margin_t").unwrap_or(None);
    let mar_b = f32_opt(&map, "margin_b").unwrap_or(None);

    // Generate expressions for Margin if necessary
    let inner_margin_ts = margin_tokens(pad, pad_l, pad_r, pad_t, pad_b);
    let outer_margin_ts = margin_tokens(mar, mar_l, mar_r, mar_t, mar_b);

    // Generate children
    let children_rt = render_children_stmt(&quote!(ui), &el.children);
    let children_doc = render_children_stmt(&quote!(__efx_doc_ui), &el.children);

    // Assembling an expression for Frame
    let mut frame_build = TokenStream::new();
    // main frame: true/default → default(); false → none();
    frame_build.extend(match frame_on {
        Some(false) => quote!( let mut __efx_frame = egui::Frame::none(); ),
        _ => quote!( let mut __efx_frame = egui::Frame::default(); ),
    });

    if let Some(ts) = fill_ts {
        frame_build.extend(quote!( __efx_frame = __efx_frame.fill(#ts); ));
    }
    if let Some(im) = inner_margin_ts {
        frame_build.extend(quote!( __efx_frame = __efx_frame.inner_margin(#im); ));
    }
    if let Some(om) = outer_margin_ts {
        frame_build.extend(quote!( __efx_frame = __efx_frame.outer_margin(#om); ));
    }
    if let Some(st) = stroke_tokens(stroke_w, stroke_col) {
        frame_build.extend(quote!( __efx_frame = __efx_frame.stroke(#st); ));
    }

    quote! {{
        #frame_build
        egui::CentralPanel::default()
                .frame(__efx_frame)
                .show(&#ui.ctx(), |ui| { #children_rt });
    }}
}
