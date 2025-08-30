use crate::tags::{Tag, TagAttributes};
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use crate::utils::attr::*;
use crate::utils::render::render_children_stmt;

pub struct CentralPanel {
    attributes: Attributes,
    element: Element,
}

impl Tag for CentralPanel {
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

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        // Assembling an expression for Frame
        let mut frame_build = TokenStream::new();

        // main frame: true/default → default(); false → none();
        frame_build.extend(match self.attributes.frame {
            Some(false) => quote!( let mut __efx_frame = egui::Frame::none(); ),
            _ => quote!( let mut __efx_frame = egui::Frame::default(); ),
        });

        if let Some(ts) = self.attributes.fill.clone() {
            frame_build.extend(quote!( __efx_frame = __efx_frame.fill(#ts); ));
        }
        if let Some(im) = self.attributes.padding_ts() {
            frame_build.extend(quote!( __efx_frame = __efx_frame.inner_margin(#im); ));
        }
        if let Some(om) = self.attributes.margin_ts() {
            frame_build.extend(quote!( __efx_frame = __efx_frame.outer_margin(#om); ));
        }
        if let Some(st) = stroke_tokens(
            self.attributes.stroke_width.clone(),
            self.attributes.stroke_color.clone(),
        ) {
            frame_build.extend(quote!( __efx_frame = __efx_frame.stroke(#st); ));
        }

        frame_build
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        // Generate children
        let children = render_children_stmt(&quote!(ui), &self.element.children);
        // Building Frame
        let frame_build = self.content(ui);

        quote! {{
            #frame_build
            egui::CentralPanel::default()
                    .frame(__efx_frame)
                    .show(&#ui.ctx(), |ui| { #children });
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    frame: Option<bool>,
    fill: Option<TokenStream>,
    #[attr(name = "stroke-width")]
    stroke_width: Option<f32>,
    #[attr(name = "stroke-color")]
    stroke_color: Option<TokenStream>,

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
    // Generate expressions for Padding
    fn padding_ts(&self) -> Option<TokenStream> {
        margin_tokens(
            self.padding,
            self.padding_l,
            self.padding_r,
            self.padding_t,
            self.padding_b,
        )
    }

    // Generate expressions for Margin
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
        let map = match attr_map(el, Attributes::ATTR_NAMES, "CentralPanel") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            frame: bool_opt(&map, "frame")?,
            fill: color_tokens_opt(&map, "fill")?,
            stroke_width: f32_opt(&map, "stroke-width")?,
            stroke_color: color_tokens_opt(&map, "stroke-color")?,

            // padding (inner_margin)
            padding: f32_opt(&map, "padding")?,
            padding_l: f32_opt(&map, "padding-left")?,
            padding_r: f32_opt(&map, "padding-right")?,
            padding_t: f32_opt(&map, "padding-top")?,
            padding_b: f32_opt(&map, "padding-bottom")?,

            // margin (outer_margin)
            margin: f32_opt(&map, "margin")?,
            margin_l: f32_opt(&map, "margin-left")?,
            margin_r: f32_opt(&map, "margin-right")?,
            margin_t: f32_opt(&map, "margin-top")?,
            margin_b: f32_opt(&map, "margin-bottom")?,
        })
    }
}
