use crate::buffer::build_buffer_from_children;
use efx_core::Element;
use quote::{ToTokens, quote};
use crate::attr_adapters as A;

pub(crate) fn render_button<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);

    const KNOWN: &[&str] = &[
        "fill","rounding","min_width","min_height","frame","enabled","tooltip",
    ];

    let mut seen = std::collections::BTreeSet::<&str>::new();

    let mut has_style_attrs = false;
    let mut fill_ts: Option<proc_macro2::TokenStream> = None;
    let mut rounding_u8: Option<u8> = None;
    let mut min_w: Option<f32> = None;
    let mut min_h: Option<f32> = None;
    let mut frame: Option<bool> = None;
    let mut enabled: Option<bool> = None;
    let mut tooltip: Option<String> = None;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <Button> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }
        if !seen.insert(name) {
            let msg = format!("efx: <Button> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "fill" => {
                let ts = match A::parse_color_tokens("fill", val) {
                    Ok(ts) => ts,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                fill_ts = Some(ts);
                has_style_attrs = true;
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
                has_style_attrs = true;
            }
            "min_width" => {
                let n = match A::parse_f32("min_width", val) {
                    Ok(n) => n,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                min_w = Some(n);
                has_style_attrs = true;
            }
            "min_height" => {
                let n = match A::parse_f32("min_height", val) {
                    Ok(n) => n,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                min_h = Some(n);
                has_style_attrs = true;
            }
            "frame" => {
                let b = match A::parse_bool("frame", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                frame = Some(b);
                has_style_attrs = true;
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
        btn_build.extend(quote!( __efx_btn = __efx_btn.min_size(egui::vec2(#w as f32, #h as f32)); ));
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
