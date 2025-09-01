use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::panel::*;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use crate::utils::expr::expr_opt;

pub struct Window {
    attributes: Attributes,
    element: Element,
}

impl Tag for Window {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        Ok(Self {
            attributes: Attributes::new(el)?,
            element: el.clone(),
        })
    }

    /// Build __efx_frame (fill/padding/margin/stroke)
    fn content<UI: ToTokens>(&self, _ui: &UI) -> TokenStream {
        FrameStyle {
            frame_on: self.attributes.frame,
            fill: self.attributes.fill.clone(),
            stroke_w: self.attributes.stroke_width,
            stroke_color: self.attributes.stroke_color.clone(),

            // padding
            pad: self.attributes.padding,
            pad_l: self.attributes.padding_l,
            pad_r: self.attributes.padding_r,
            pad_t: self.attributes.padding_t,
            pad_b: self.attributes.padding_b,

            // margin
            mar: self.attributes.margin,
            mar_l: self.attributes.margin_l,
            mar_r: self.attributes.margin_r,
            mar_t: self.attributes.margin_t,
            mar_b: self.attributes.margin_b,
        }
        .emit()
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let title = match &self.attributes.title {
            Some(s) if !s.is_empty() => s,
            _ => {
                return quote! { compile_error!("efx: <Window> requires non-empty `title` attribute"); };
            }
        };

        let children = render_children_stmt(&quote!(ui), &self.element.children);
        let frame_ts = self.content(ui);

        let mut win =
            quote!( let mut __efx_window = egui::Window::new(#title).frame(__efx_frame); );

        // id
        if let Some(id) = &self.attributes.id {
            win.extend(quote!( __efx_window = __efx_window.id(egui::Id::new(#id)); ));
        }

        // behavior
        if let Some(b) = self.attributes.movable {
            win.extend(quote!( __efx_window = __efx_window.movable(#b); ));
        }
        if let Some(b) = self.attributes.resizable {
            win.extend(quote!( __efx_window = __efx_window.resizable(#b); ));
        }
        if let Some(b) = self.attributes.collapsible {
            win.extend(quote!( __efx_window = __efx_window.collapsible(#b); ));
        }
        if let Some(b) = self.attributes.title_bar {
            win.extend(quote!( __efx_window = __efx_window.title_bar(#b); ));
        }
        if let Some(b) = self.attributes.enabled {
            win.extend(quote!( __efx_window = __efx_window.enabled(#b); ));
        }
        if let Some(b) = self.attributes.constrain {
            win.extend(quote!( __efx_window = __efx_window.constrain(#b); ));
        }
        if self.attributes.auto_sized == Some(true) {
            win.extend(quote!( __efx_window = __efx_window.auto_sized(); ));
        }

        // geometry: default_pos / current_pos
        if self.attributes.default_x.is_some() || self.attributes.default_y.is_some() {
            let x = self.attributes.default_x.unwrap_or(0.0);
            let y = self.attributes.default_y.unwrap_or(0.0);
            win.extend(quote!( __efx_window = __efx_window.default_pos(egui::pos2(#x as f32, #y as f32)); ));
        }
        if self.attributes.pos_x.is_some() || self.attributes.pos_y.is_some() {
            let x = self.attributes.pos_x.unwrap_or(0.0);
            let y = self.attributes.pos_y.unwrap_or(0.0);
            win.extend(quote!( __efx_window = __efx_window.current_pos(egui::pos2(#x as f32, #y as f32)); ));
        }

        // size: default/min/max size
        if self.attributes.default_width.is_some() || self.attributes.default_height.is_some() {
            let w = self.attributes.default_width.unwrap_or(0.0);
            let h = self.attributes.default_height.unwrap_or(0.0);
            win.extend(quote!( __efx_window = __efx_window.default_size(egui::vec2(#w as f32, #h as f32)); ));
        }
        if self.attributes.min_width.is_some() || self.attributes.min_height.is_some() {
            let w = self.attributes.min_width.unwrap_or(0.0);
            let h = self.attributes.min_height.unwrap_or(0.0);
            win.extend(
                quote!( __efx_window = __efx_window.min_size(egui::vec2(#w as f32, #h as f32)); ),
            );
        }
        if self.attributes.max_width.is_some() || self.attributes.max_height.is_some() {
            let w = self.attributes.max_width.unwrap_or(f32::INFINITY);
            let h = self.attributes.max_height.unwrap_or(f32::INFINITY);
            win.extend(
                quote!( __efx_window = __efx_window.max_size(egui::vec2(#w as f32, #h as f32)); ),
            );
        }

        // anchor (Align2 + offset)
        if self.attributes.anchor_h.is_some() || self.attributes.anchor_v.is_some() {
            let h = self.attributes.anchor_h.clone().unwrap_or_else(|| "center".to_string());
            let v = self.attributes.anchor_v.clone().unwrap_or_else(|| "center".to_string());

            let h_align = match h.as_str() {
                "left" => quote!( egui::Align::Min ),
                "center" => quote!( egui::Align::Center ),
                "right" => quote!( egui::Align::Max ),
                other => {
                    let msg = format!("efx: <Window> `anchor-h` expected left|center|right, got `{}`", other);
                    return quote! { compile_error!(#msg); };
                }
            };
            let v_align = match v.as_str() {
                "top"    => quote!( egui::Align::Min ),
                "center" => quote!( egui::Align::Center ),
                "bottom" => quote!( egui::Align::Max ),
                other => {
                    let msg = format!("efx: <Window> `anchor-v` expected top|center|bottom, got `{}`", other);
                    return quote! { compile_error!(#msg); };
                }
            };

            let ax = self.attributes.anchor_x.unwrap_or(0.0);
            let ay = self.attributes.anchor_y.unwrap_or(0.0);

            win.extend(quote!(
                __efx_window = __efx_window.anchor(egui::Align2(#h_align, #v_align), egui::vec2(#ax as f32, #ay as f32));
            ));
        }

        let open_bind = if let Some(expr) = &self.attributes.open_expr {
            quote! {
                let mut __efx_open = (#expr);
                __efx_window = __efx_window.open(&mut __efx_open);
                #expr = __efx_open;
            }
        } else {
            quote!()
        };

        quote! {{
            #frame_ts
            #win
            #open_bind
            __efx_window.show(&#ui.ctx(), |ui| { #children });
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    // required
    title: Option<String>,

    // optional id
    id: Option<String>,

    // behavior
    movable: Option<bool>,
    resizable: Option<bool>,
    collapsible: Option<bool>,
    title_bar: Option<bool>,
    enabled: Option<bool>,
    constrain: Option<bool>,
    auto_sized: Option<bool>,

    // opening state binding (expression)
    #[attr(name = "open")]
    open_expr: Option<TokenStream>,

    // geometry: positions
    #[attr(name = "default-x")]
    default_x: Option<f32>,
    #[attr(name = "default-y")]
    default_y: Option<f32>,
    #[attr(name = "pos-x")]
    pos_x: Option<f32>,
    #[attr(name = "pos-y")]
    pos_y: Option<f32>,

    // geometry: sizes
    #[attr(name = "default-width")]
    default_width: Option<f32>,
    #[attr(name = "default-height")]
    default_height: Option<f32>,
    #[attr(name = "min-width")]
    min_width: Option<f32>,
    #[attr(name = "min-height")]
    min_height: Option<f32>,
    #[attr(name = "max-width")]
    max_width: Option<f32>,
    #[attr(name = "max-height")]
    max_height: Option<f32>,

    // anchor
    #[attr(name = "anchor-h")]
    anchor_h: Option<String>, // left|center|right
    #[attr(name = "anchor-v")]
    anchor_v: Option<String>, // top|center|bottom
    #[attr(name = "anchor-x")]
    anchor_x: Option<f32>, // offset
    #[attr(name = "anchor-y")]
    anchor_y: Option<f32>,

    // frame + style
    frame: Option<bool>,
    fill: Option<TokenStream>,
    #[attr(name = "stroke-width")]
    stroke_width: Option<f32>,
    #[attr(name = "stroke-color")]
    stroke_color: Option<TokenStream>,

    // padding
    padding: Option<f32>,
    #[attr(name = "padding-left")]
    padding_l: Option<f32>,
    #[attr(name = "padding-right")]
    padding_r: Option<f32>,
    #[attr(name = "padding-top")]
    padding_t: Option<f32>,
    #[attr(name = "padding-bottom")]
    padding_b: Option<f32>,

    // margin
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

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = attr_map(el, Self::ATTR_NAMES, "Window")?;

        Ok(Attributes {
            title: map.get("title").map(|s| (*s).to_string()),
            id: map.get("id").map(|s| (*s).to_string()),

            movable: bool_opt(&map, "movable")?,
            resizable: bool_opt(&map, "resizable")?,
            collapsible: bool_opt(&map, "collapsible")?,
            title_bar: bool_opt(&map, "title-bar")?,
            enabled: bool_opt(&map, "enabled")?,
            constrain: bool_opt(&map, "constrain")?,
            auto_sized: bool_opt(&map, "auto-sized")?,

            open_expr: expr_opt(&map, "open")?,

            default_x: f32_opt(&map, "default-x")?,
            default_y: f32_opt(&map, "default-y")?,
            pos_x: f32_opt(&map, "pos-x")?,
            pos_y: f32_opt(&map, "pos-y")?,

            default_width: f32_opt(&map, "default-width")?,
            default_height: f32_opt(&map, "default-height")?,
            min_width: f32_opt(&map, "min-width")?,
            min_height: f32_opt(&map, "min-height")?,
            max_width: f32_opt(&map, "max-width")?,
            max_height: f32_opt(&map, "max-height")?,

            anchor_h: map.get("anchor-h").map(|s| (*s).to_string()),
            anchor_v: map.get("anchor-v").map(|s| (*s).to_string()),
            anchor_x: f32_opt(&map, "anchor-x")?,
            anchor_y: f32_opt(&map, "anchor-y")?,

            frame: bool_opt(&map, "frame")?,
            fill: color_tokens_opt(&map, "fill")?,
            stroke_width: f32_opt(&map, "stroke-width")?,
            stroke_color: color_tokens_opt(&map, "stroke-color")?,

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
