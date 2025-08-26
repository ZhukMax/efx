use quote::{quote, ToTokens};
use efx_core::Element;
use crate::attr_adapters as A;
use crate::buffer::build_buffer_from_children;

/// <Hyperlink url="..." [open_external=bool] [color=..] [underline=bool] [tooltip=...]>text?</Hyperlink>
pub(crate) fn render_hyperlink_stmt<UI: ToTokens>(
    ui: &UI,
    el: &Element,
) -> proc_macro2::TokenStream {
    const KNOWN: &[&str] = &["url", "open_external", "color", "underline", "tooltip"];

    // Collect the caption (text) from children: allow only text/interpolations, like in Label/Button
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);

    let mut seen = std::collections::BTreeSet::<&str>::new();

    // Attributes
    let mut url: Option<String> = None;
    let mut open_external: Option<bool> = None; // web: .open_in_new_tab
    let mut color_ts: Option<proc_macro2::TokenStream> = None;
    let mut underline: Option<bool> = None;
    let mut tooltip: Option<String> = None;

    let mut has_style_or_behavior = false;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <Hyperlink> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }
        if !seen.insert(name) {
            let msg = format!("efx: <Hyperlink> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "url" => {
                url = Some(val.to_string());
            }
            "open_external" => {
                match A::parse_bool("open_external", val) {
                    Ok(b) => {
                        open_external = Some(b);
                        has_style_or_behavior = true;
                    }
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            "color" => {
                let ts = match A::parse_color_tokens("color", val) {
                    Ok(ts) => ts,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                color_ts = Some(ts);
                has_style_or_behavior = true;
            }
            "underline" => {
                match A::parse_bool("underline", val) {
                    Ok(b) => { underline = Some(b); has_style_or_behavior = true; }
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            "tooltip" => {
                tooltip = Some(val.to_string());
                has_style_or_behavior = true;
            }
            _ => {}
        }
    }

    // attribute url is required
    if url.is_none() {
        return quote! { compile_error!("efx: <Hyperlink> requires `url` attribute"); };
    }
    let url_lit = url.unwrap();

    // Label: if the text is empty, we use the url itself
    let label_logic = quote! {{
        #buf_init
        #buf_build
        if __efx_buf.is_empty() { __efx_buf.push_str(#url_lit); }
        __efx_buf
    }};

    // Simple link: no styles/hints/special behavior → just use ui.hyperlink*/
    if !has_style_or_behavior {
        return quote! {{
            let __efx_label = #label_logic;
            if __efx_label == #url_lit {
                // label == url
                #ui.hyperlink(#url_lit);
            } else {
                #ui.hyperlink_to(__efx_label, #url_lit);
            }
        }};
    }

    // Advanced link: constructing RichText and widgets::Hyperlink
    let mut rich_mods = proc_macro2::TokenStream::new();
    if let Some(ts) = color_ts {
        rich_mods.extend(quote!( .color(#ts) ));
    }

    if let Some(b) = underline {
        // true → .underline(), false → .underline() not call (in egui RichText underline=true enables underlining)
        if b { rich_mods.extend(quote!( .underline() )); }
    }

    let open_tab_ts = match open_external {
        // egui: open_in_new_tab
        Some(b) => quote!( .open_in_new_tab(#b) ),
        None => quote!(),
    };

    let tooltip_ts = if let Some(t) = tooltip {
        quote!( __efx_resp = __efx_resp.on_hover_text(#t); )
    } else {
        quote!()
    };

    quote! {{
        let __efx_label = #label_logic;
        let __efx_rich = egui::RichText::new(__efx_label) #rich_mods ;
        let __efx_link = egui::widgets::Hyperlink::from_label_and_url(__efx_rich, #url_lit) #open_tab_ts ;
        let mut __efx_resp = #ui.add(__efx_link);
        #tooltip_ts
        __efx_resp
    }}
}
