use crate::buffer::build_buffer_from_children;
use crate::tags::util::*;
use crate::tags::{TagAttributes, Tagged};
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

/// <Hyperlink url="..." [open_external=bool] [color=...] [underline=bool] [tooltip=...]>text?</Hyperlink>
pub struct Hyperlink;

impl Tagged for Hyperlink {
    fn parse<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream {
        let attributes = match Attributes::new(el) {
            Ok(attr) => attr,
            Err(err) => return err,
        };
        let url = attributes.url.clone();

        // Collect the caption (text) from children: allow only text/interpolations, like in Label/Button
        let (buf_init, buf_build) = build_buffer_from_children(&el.children);

        // Label: if the text is empty, we use the url itself
        let label_logic = quote! {{
            #buf_init
            #buf_build
            if __efx_buf.is_empty() { __efx_buf.push_str(#url); }
            __efx_buf
        }};

        // Simple link: no styles/hints/special behavior → just use ui.hyperlink*/
        if !attributes.clone().has_style_or_behavior() {
            return quote! {{
                let __efx_label = #label_logic;
                if __efx_label == #url {
                    // label == url
                    #ui.hyperlink(#url);
                } else {
                    #ui.hyperlink_to(__efx_label, #url);
                }
            }};
        }

        // Advanced link: constructing RichText and widgets::Hyperlink
        let mut rich_mods = TokenStream::new();
        if let Some(ts) = attributes.color_ts {
            rich_mods.extend(quote!( .color(#ts) ));
        }

        if let Some(b) = attributes.underline {
            // true → .underline(), false → .underline() not call (in egui RichText underline=true enables underlining)
            if b {
                rich_mods.extend(quote!( .underline() ));
            }
        }

        let open_tab_ts = match attributes.open_external {
            // egui: open_in_new_tab
            Some(b) => quote!( .open_in_new_tab(#b) ),
            None => quote!(),
        };

        let tooltip_ts = if let Some(t) = attributes.tooltip {
            quote!( __efx_resp = __efx_resp.on_hover_text(#t); )
        } else {
            quote!()
        };

        quote! {{
            let __efx_label = #label_logic;
            let __efx_rich = egui::RichText::new(__efx_label) #rich_mods ;
            let __efx_link = egui::widgets::Hyperlink::from_label_and_url(__efx_rich, #url) #open_tab_ts ;
            let mut __efx_resp = #ui.add(__efx_link);
            #tooltip_ts
            __efx_resp
        }}
    }
}

#[derive(Clone, Debug)]
struct Attributes {
    url: String,
    open_external: Option<bool>,
    underline: Option<bool>,
    color_ts: Option<TokenStream>,
    tooltip: Option<String>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        const KNOWN: &[&str] = &["url", "open_external", "color", "underline", "tooltip"];
        let map = match attr_map(el, KNOWN, "Hyperlink") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let url = match map.get("url") {
            Some(u) if !u.is_empty() => (*u).to_string(),
            _ => return Err(quote! { compile_error!("efx: <Hyperlink> requires `url=\"...\"`"); }),
        };

        Ok(Attributes {
            url,
            open_external: bool_opt(&map, "open_external").unwrap_or(None), // web: .open_in_new_tab
            underline: bool_opt(&map, "underline").unwrap_or(None),
            color_ts: color_tokens_opt(&map, "color").unwrap_or(None),
            tooltip: map.get("tooltip").map(|s| (*s).to_string()),
        })
    }
}

impl Attributes {
    pub(crate) fn has_style_or_behavior(self: Self) -> bool {
        self.open_external.is_some()
            || self.underline.is_some()
            || self.color_ts.is_some()
            || self.tooltip.is_some()
    }
}
