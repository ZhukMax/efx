use efx_core::Element;
use quote::{ToTokens, quote};

use crate::attr_adapters as A;
use crate::render::render_node_stmt;

pub fn render_central_panel_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &[
        "frame",
        // fill / stroke
        "fill",
        "stroke_width",
        "stroke_color",
        // rounding
        "rounding",
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

    let mut seen = std::collections::BTreeSet::<&str>::new();

    // frame
    let mut frame_on: Option<bool> = None;

    // fill & stroke
    let mut fill_ts: Option<proc_macro2::TokenStream> = None;
    let mut stroke_w: Option<f32> = None;
    let mut stroke_color_ts: Option<proc_macro2::TokenStream> = None;

    // rounding (u8 → f32)
    let mut rounding_u8: Option<u8> = None;

    // padding (inner) & margin (outer)
    // uniform + per-side
    let (mut pad, mut pad_l, mut pad_r, mut pad_t, mut pad_b): (Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>) = (None, None, None, None, None);
    let (mut mar, mut mar_l, mut mar_r, mut mar_t, mut mar_b): (Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>) = (None, None, None, None, None);

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <CentralPanel> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }
        if !seen.insert(name) {
            let msg = format!("efx: <CentralPanel> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "frame" => {
                match A::parse_bool("frame", val) {
                    Ok(b) => frame_on = Some(b),
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            "fill" => {
                fill_ts = match A::parse_color_tokens("fill", val) {
                    Ok(ts) => Some(ts),
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            "stroke_width" => {
                match A::parse_f32("stroke_width", val) {
                    Ok(w) => stroke_w = Some(w),
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            "stroke_color" => {
                stroke_color_ts = match A::parse_color_tokens("stroke_color", val) {
                    Ok(ts) => Some(ts),
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            "rounding" => {
                match A::parse_u8("rounding", val) {
                    Ok(r) => rounding_u8 = Some(r),
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }

            // padding (inner_margin)
            "padding"    => { pad    = A::parse_f32("padding", val).ok(); }
            "padding_l"  => { pad_l  = A::parse_f32("padding_l", val).ok(); }
            "padding_r"  => { pad_r  = A::parse_f32("padding_r", val).ok(); }
            "padding_t"  => { pad_t  = A::parse_f32("padding_t", val).ok(); }
            "padding_b"  => { pad_b  = A::parse_f32("padding_b", val).ok(); }

            // margin (outer_margin)
            "margin"     => { mar    = A::parse_f32("margin", val).ok(); }
            "margin_l"   => { mar_l  = A::parse_f32("margin_l", val).ok(); }
            "margin_r"   => { mar_r  = A::parse_f32("margin_r", val).ok(); }
            "margin_t"   => { mar_t  = A::parse_f32("margin_t", val).ok(); }
            "margin_b"   => { mar_b  = A::parse_f32("margin_b", val).ok(); }

            _ => {}
        }
    }

    // Generate expressions for Margin if necessary
    let inner_margin_ts = margin_tokens(pad, pad_l, pad_r, pad_t, pad_b);
    let outer_margin_ts = margin_tokens(mar, mar_l, mar_r, mar_t, mar_b);

    // Generate children body
    let mut children_ts = proc_macro2::TokenStream::new();
    for ch in &el.children {
        let stmt = render_node_stmt(&quote!(ui), ch);
        children_ts.extend(quote! { #stmt });
    }

    // Assembling an expression for Frame
    let mut frame_build = proc_macro2::TokenStream::new();
    // main frame: true/default → default(); false → none();
    frame_build.extend(match frame_on {
        Some(false) => quote!( let mut __efx_frame = egui::Frame::none(); ),
        _ => quote!( let mut __efx_frame = egui::Frame::default(); ),
    });

    if let Some(ts) = fill_ts {
        frame_build.extend(quote!( __efx_frame = __efx_frame.fill(#ts); ));
    }
    if let Some(r) = rounding_u8 {
        // In egui/epaint 0.32 Rounding::same(u8)
        frame_build.extend(quote!( __efx_frame = __efx_frame.rounding(egui::Rounding::same(#r)); ));
    }
    if let Some(im) = inner_margin_ts {
        frame_build.extend(quote!( __efx_frame = __efx_frame.inner_margin(#im); ));
    }
    if let Some(om) = outer_margin_ts {
        frame_build.extend(quote!( __efx_frame = __efx_frame.outer_margin(#om); ));
    }
    if stroke_w.is_some() || stroke_color_ts.is_some() {
        let w = stroke_w.unwrap_or(1.0);
        let c = stroke_color_ts.unwrap_or_else(|| quote!( egui::Color32::BLACK ));
        frame_build.extend(quote! {
            __efx_frame = __efx_frame.stroke(egui::Stroke { width: #w as f32, color: #c });
        });
    }

    quote! {{
        #frame_build
        egui::CentralPanel::default().frame(__efx_frame).show(#ui.ctx(), |ui| {
            #children_ts
        });
    }}
}

/// Building egui::Margin from uniform/per-side options.
/// Returns Some(TokenStream) if something is given, None otherwise.
fn margin_tokens(
    uniform: Option<f32>,
    l: Option<f32>,
    r: Option<f32>,
    t: Option<f32>,
    b: Option<f32>,
) -> Option<proc_macro2::TokenStream> {
    if uniform.is_none() && l.is_none() && r.is_none() && t.is_none() && b.is_none() {
        return None;
    }

    let mk = |side: Option<f32>, uni: Option<f32>| -> proc_macro2::TokenStream {
        if let Some(v) = side {
            quote!( #v as f32 )
        } else if let Some(u) = uni {
            quote!( #u as f32 )
        } else {
            quote!( 0.0f32 )
        }
    };

    let l_ts = mk(l, uniform);
    let r_ts = mk(r, uniform);
    let t_ts = mk(t, uniform);
    let b_ts = mk(b, uniform);

    Some(quote!( egui::Margin { left: #l_ts, right: #r_ts, top: #t_ts, bottom: #b_ts } ))
}
