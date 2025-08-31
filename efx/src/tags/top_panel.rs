use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct TopPanel {
    attributes: Attributes,
    element: Element,
}

impl Tag for TopPanel {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        Ok(Self {
            attributes: Attributes::new(el)?,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, _ui: &UI) -> TokenStream {
        let mut ts = TokenStream::new();

        // default / none
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
        let id = match &self.attributes.id {
            Some(s) if !s.is_empty() => s,
            _ => {
                let msg = "efx: <TopPanel> requires non-empty `id` attribute";
                return quote! { compile_error!(#msg); };
            }
        };

        let children = render_children_stmt(&quote!(ui), &self.element.children);
        let frame_ts = self.content(ui);

        let mut panel_ts =
            quote!( let mut __efx_panel = egui::TopBottomPanel::top(#id).frame(__efx_frame); );
        if let Some(b) = self.attributes.resizable {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.resizable(#b); ));
        }
        if let Some(v) = self.attributes.default_height {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.default_height(#v as f32); ));
        }
        if let Some(v) = self.attributes.min_height {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.min_height(#v as f32); ));
        }
        if let Some(v) = self.attributes.max_height {
            panel_ts.extend(quote!( __efx_panel = __efx_panel.max_height(#v as f32); ));
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
    /// required: egui Id salt
    id: Option<String>,

    // frame + styling
    frame: Option<bool>,
    fill: Option<TokenStream>,
    #[attr(name = "stroke-width")]
    stroke_width: Option<f32>,
    #[attr(name = "stroke-color")]
    stroke_color: Option<TokenStream>,

    // sizing
    #[attr(name = "default-height")]
    default_height: Option<f32>,
    #[attr(name = "min-height")]
    min_height: Option<f32>,
    #[attr(name = "max-height")]
    max_height: Option<f32>,
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
        let map = attr_map(el, Attributes::ATTR_NAMES, "TopPanel")?;
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
