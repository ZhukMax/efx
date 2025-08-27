use crate::attr_adapters as A;
use crate::buffer::build_buffer_from_children;
use crate::tags::util::*;
use efx_core::Element;
use quote::{ToTokens, quote};

pub fn render_button<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &[
        "fill",
        "rounding",
        "min_width",
        "min_height",
        "frame",
        "enabled",
        "tooltip",
    ];

    let map = match attr_map(el, KNOWN, "Button") {
        Ok(m) => m,
        Err(err) => return err,
    };

    let min_w = f32_opt("Button", &map, "min_width").unwrap_or(None);
    let min_h = f32_opt("Button", &map, "min_height").unwrap_or(None);
    let frame = bool_opt("Button", &map, "frame").unwrap_or(None);
    let mut fill_ts: Option<proc_macro2::TokenStream> = None;
    let mut rounding_u8: Option<u8> = None;
    let mut enabled: Option<bool> = None;
    let mut tooltip: Option<String> = None;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        match name {
            "fill" => {
                let ts = match A::parse_color_tokens("fill", val) {
                    Ok(ts) => ts,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                fill_ts = Some(ts);
            }
            "rounding" => {
                let n = match A::parse_f32("rounding", val) {
                    Ok(n) => n,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                if !(0.0..=255.0).contains(&n) {
                    let msg = format!("efx: <Button rounding> must be in 0..=255, got {}", n);
                    return quote! { compile_error!(#msg); };
                }

                let r: u8 = n.round() as u8;
                rounding_u8 = Some(r);
            }
            "enabled" => {
                let b = match A::parse_bool("enabled", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                enabled = Some(b);
            }
            "tooltip" => {
                tooltip = Some(val.to_string());
            }
            _ => {}
        }
    }

    let has_style_attrs = min_w.is_some()
        || min_h.is_some()
        || frame.is_some()
        || fill_ts.is_some()
        || rounding_u8.is_some();

    let (buf_init, buf_build) = build_buffer_from_children(&el.children);

    let plain_mode = !has_style_attrs && enabled.is_none() && tooltip.is_none();

    if plain_mode {
        return quote! {{
            #buf_init
            #buf_build
            #ui.button(__efx_buf)
        }};
    }

    // Otherwise - egui::Button with RichText and modifiers
    let rich_decl = quote!( let __efx_rich = egui::RichText::new(__efx_buf); );

    // Assembling a button builder
    let mut btn_build = proc_macro2::TokenStream::new();
    btn_build.extend(quote!( let mut __efx_btn = egui::Button::new(__efx_rich); ));

    if let Some(ts) = fill_ts {
        btn_build.extend(quote!( __efx_btn = __efx_btn.fill(#ts); ));
    }

    if let Some(r) = rounding_u8 {
        btn_build.extend(quote!( __efx_btn = __efx_btn.rounding(egui::Rounding::same(#r)); ));
    }

    if min_w.is_some() || min_h.is_some() {
        let w = min_w.unwrap_or(0.0f32);
        let h = min_h.unwrap_or(0.0f32);
        btn_build
            .extend(quote!( __efx_btn = __efx_btn.min_size(egui::vec2(#w as f32, #h as f32)); ));
    }
    if let Some(b) = frame {
        btn_build.extend(quote!( __efx_btn = __efx_btn.frame(#b); ));
    }

    let add_btn = match enabled {
        Some(false) => quote!( let mut __efx_resp = #ui.add_enabled(false, __efx_btn); ),
        _ => quote!( let mut __efx_resp = #ui.add(__efx_btn); ),
    };

    // Tooltip
    let tooltip_apply = if let Some(text) = tooltip {
        quote!( __efx_resp = __efx_resp.on_hover_text(#text); )
    } else {
        quote!()
    };

    quote! {{
        #buf_init
        #buf_build
        #rich_decl
        #btn_build
        #add_btn
        #tooltip_apply
        __efx_resp
    }}
}
