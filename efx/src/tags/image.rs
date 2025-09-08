use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::expr::*;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;

pub struct Image {
    attributes: Attributes,
    element: Element,
}

impl Tag for Image {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        if !el.children.is_empty() {
            let msg = "efx: <Image> must not have children";
            return Err(quote! { compile_error!(#msg); });
        }

        let attributes = Attributes::new(el)?;
        // source required
        if attributes.texture_expr.is_none() && attributes.src.is_none() {
            let msg = "efx: <Image> requires either `texture` (expr) or `src` (string)";
            return Err(quote! { compile_error!(#msg); });
        }
        // conflicting combination
        if attributes.texture_expr.is_some() && attributes.src.is_some() {
            let msg = "efx: <Image> `texture` and `src` are mutually exclusive";
            return Err(quote! { compile_error!(#msg); });
        }

        Ok(Self {
            attributes,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let mut builder = TokenStream::new();

        let new_image = if let Some(expr) = &self.attributes.texture_expr {
            // texture: egui::TextureId / &TextureHandle / ImageSource
            quote!( let mut __efx_img = egui::Image::new(#expr); )
        } else if let Some(src) = &self.attributes.src {
            // src: string/uri
            // NB: используем ImageSource::uri (есть в egui 0.32)
            quote!( let mut __efx_img = egui::Image::new(egui::ImageSource::uri(#src)); )
        } else {
            quote!( compile_error!("efx: <Image> requires either `texture` or `src`"); )
        };
        builder.extend(new_image);

        // id_source
        if let Some(id) = &self.attributes.id {
            builder.extend(quote! {
                __efx_img = __efx_img.id_source(#id);
            });
        }

        // size
        let w = self.attributes.width;
        let h = self.attributes.height;

        if let (Some(w), Some(h)) = (w, h) {
            builder.extend(quote! {
                __efx_img = __efx_img.fit_to_exact_size(egui::vec2(#w as f32, #h as f32));
            });
        } else {
            let mw = self.attributes.max_width.or(w);
            let mh = self.attributes.max_height.or(h);
            if mw.is_some() || mh.is_some() {
                let mw_ts = mw
                    .map(|v| quote!(#v as f32))
                    .unwrap_or(quote!(f32::INFINITY));
                let mh_ts = mh
                    .map(|v| quote!(#v as f32))
                    .unwrap_or(quote!(f32::INFINITY));
                builder.extend(quote! {
                    __efx_img = __efx_img.max_size(egui::vec2(#mw_ts, #mh_ts));
                });
            }
        }

        // maintain-aspect
        if let Some(keep) = self.attributes.maintain_aspect {
            builder.extend(quote! {
                __efx_img = __efx_img.maintain_aspect_ratio(#keep);
            });
        }

        // tint/bg-fill/rounding
        if let Some(ts) = &self.attributes.tint {
            builder.extend(quote! {
                __efx_img = __efx_img.tint(#ts);
            });
        }
        if let Some(ts) = &self.attributes.bg_fill {
            builder.extend(quote! {
                __efx_img = __efx_img.bg_fill(#ts);
            });
        }
        if let Some(r) = self.attributes.rounding {
            builder.extend(quote! {
                #[allow(deprecated)]
                {
                    __efx_img = __efx_img.rounding(egui::Rounding::same(#r as _));
                }
            });
        }

        // interactivity
        if let Some(true) = self.attributes.clickable {
            builder.extend(quote! {
                __efx_img = __efx_img.sense(egui::Sense::click());
            });
        }

        // 7) insert in UI + tooltip
        let tooltip_apply = if let Some(t) = &self.attributes.tooltip {
            quote!( __efx_resp = __efx_resp.on_hover_text(#t); )
        } else {
            quote!()
        };

        quote! {{
            #builder
            let mut __efx_resp = #ui.add(__efx_img);
            #tooltip_apply
            __efx_resp
        }}
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        self.content(ui)
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    // источник
    src: Option<String>,
    texture_expr: Option<Expr>,

    // размеры
    width: Option<f32>,
    height: Option<f32>,
    #[attr(name = "max-width")]
    max_width: Option<f32>,
    #[attr(name = "max-height")]
    max_height: Option<f32>,
    #[attr(name = "maintain-aspect")]
    maintain_aspect: Option<bool>,

    // стили
    tint: Option<TokenStream>,
    #[attr(name = "bg-fill")]
    bg_fill: Option<TokenStream>,
    rounding: Option<u8>,

    // прочее
    id: Option<String>,
    clickable: Option<bool>,
    tooltip: Option<String>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "Image") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            src: map.get("src").map(|s| (*s).to_string()),
            texture_expr: expr_opt(&map, "texture")?,

            width: f32_opt(&map, "width")?,
            height: f32_opt(&map, "height")?,
            max_width: f32_opt(&map, "max-width")?,
            max_height: f32_opt(&map, "max-height")?,
            maintain_aspect: bool_opt(&map, "maintain-aspect")?,

            tint: color_tokens_opt(&map, "tint")?,
            bg_fill: color_tokens_opt(&map, "bg-fill")?,
            rounding: u8_opt(&map, "rounding")?,

            id: map.get("id").map(|s| (*s).to_string()),
            clickable: bool_opt(&map, "clickable")?,
            tooltip: map.get("tooltip").map(|s| (*s).to_string()),
        })
    }
}
