use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct SidePanel {
    attributes: Attributes,
    element: Element,
}

impl Tag for SidePanel {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        let attributes = Attributes::new(el)?;
        Ok(Self {
            attributes,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, _ui: &UI) -> TokenStream {
        let mut ts = TokenStream::new();

        // default frame or none
        ts.extend(match self.attributes.frame {
            Some(false) => quote!( let mut __efx_frame = egui::Frame::none(); ),
            _ => quote!( let mut __efx_frame = egui::Frame::default(); ),
        });

        if let Some(fill) = &self.attributes.fill {
            ts.extend(quote!( __efx_frame = __efx_frame.fill(#fill); ));
        }
        if let Some(im) = self.attributes.padding_ts() {
            ts.extend(quote!( __efx_frame = __efx_frame.inner_margin(#im); ));
        }
        if let Some(om) = self.attributes.margin_ts() {
            ts.extend(quote!( __efx_frame = __efx_frame.outer_margin(#om); ));
        }
        if let Some(st) = stroke_tokens(
            self.attributes.stroke_width,
            self.attributes.stroke_color.clone(),
        ) {
            ts.extend(quote!( __efx_frame = __efx_frame.stroke(#st); ));
        }

        ts
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let children = render_children_stmt(&quote!(ui), &self.element.children);
        let frame_ts = self.content(ui);

        let side = match self.attributes.side.as_deref() {
            Some(s @ ("left" | "right")) => s,
            Some(other) => {
                let msg = format!(
                    "efx: <SidePanel> `side` must be `left` or `right`, got `{}`",
                    other
                );
                return quote! { compile_error!(#msg); };
            }
            None => {
                let msg = "efx: <SidePanel> requires `side` attribute (`left` | `right`)";
                return quote! { compile_error!(#msg); };
            }
        };
        let id = match &self.attributes.id {
            Some(s) if !s.is_empty() => s,
            _ => {
                let msg = "efx: <SidePanel> requires non-empty `id` attribute";
                return quote! { compile_error!(#msg); };
            }
        };

        let mut panel_ts = TokenStream::new();
        match side {
            "left" => panel_ts.extend(quote!( let mut __efx_panel = egui::SidePanel::left(#id); )),
            "right" => {
                panel_ts.extend(quote!( let mut __efx_panel = egui::SidePanel::right(#id); ))
            }
            _ => {}
        }
        panel_ts.extend(quote!( __efx_panel = __efx_panel.frame(__efx_frame); ));

        if let Some(b) = self.attributes.resizable {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.resizable(#b); ));
        }
        if let Some(v) = self.attributes.default_width {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.default_width(#v as f32); ));
        }
        if let Some(v) = self.attributes.min_width {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.min_width(#v as f32); ));
        }
        if let Some(v) = self.attributes.max_width {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.max_width(#v as f32); ));
        }

        quote! {{
            #frame_ts
            #panel_ts
            __efx_panel.show(&#ui.ctx(), |ui| { #children });
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    /// required: left | right
    side: Option<String>,
    /// required: egui Id salt
    id: Option<String>,

    // frame on/off + styling
    frame: Option<bool>,
    fill: Option<TokenStream>,
    #[attr(name = "stroke-width")]
    stroke_width: Option<f32>,
    #[attr(name = "stroke-color")]
    stroke_color: Option<TokenStream>,

    // sizing
    #[attr(name = "default-width")]
    default_width: Option<f32>,
    #[attr(name = "min-width")]
    min_width: Option<f32>,
    #[attr(name = "max-width")]
    max_width: Option<f32>,
    resizable: Option<bool>,

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

impl Attributes {
    fn padding_ts(&self) -> Option<TokenStream> {
        margin_tokens(
            self.padding,
            self.padding_l,
            self.padding_r,
            self.padding_t,
            self.padding_b,
        )
    }
    fn margin_ts(&self) -> Option<TokenStream> {
        margin_tokens(
            self.margin,
            self.margin_l,
            self.margin_r,
            self.margin_t,
            self.margin_b,
        )
    }
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = attr_map(el, Attributes::ATTR_NAMES, "SidePanel")?;
        Ok(Attributes {
            side: map.get("side").map(|s| (*s).to_string()),
            id: map.get("id").map(|s| (*s).to_string()),

            frame: bool_opt(&map, "frame")?,
            fill: color_tokens_opt(&map, "fill")?,
            stroke_width: f32_opt(&map, "stroke-width")?,
            stroke_color: color_tokens_opt(&map, "stroke-color")?,

            default_width: f32_opt(&map, "default-width")?,
            min_width: f32_opt(&map, "min-width")?,
            max_width: f32_opt(&map, "max-width")?,
            resizable: bool_opt(&map, "resizable")?,

            padding: f32_opt(&map, "padding")?,
            padding_l: f32_opt(&map, "padding-left")?,
            padding_r: f32_opt(&map, "padding-right")?,
            padding_t: f32_opt(&map, "padding-top")?,
            padding_b: f32_opt(&map, "padding-bottom")?,

            margin: f32_opt(&map, "margin")?,
            margin_l: f32_opt(&map, "margin-left")?,
            margin_r: f32_opt(&map, "margin-right")?,
            margin_t: f32_opt(&map, "margin-top")?,
            margin_b: f32_opt(&map, "margin-bottom")?,
        })
    }
}
