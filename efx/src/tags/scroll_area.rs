use crate::tags::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct ScrollArea {
    attributes: Attributes,
    element: Element,
}

impl Tag for ScrollArea {
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
        let axis = self.attributes.axis.clone();
        let mut build = quote!( let mut __efx_sa = #axis; );

        if let Some(b) = self.attributes.always_show {
            if b {
                build.extend(quote!(
                    __efx_sa = __efx_sa.scroll_bar_visibility(
                        egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible
                    );
                ));
            } else {
                build.extend(quote!(
                    __efx_sa = __efx_sa.scroll_bar_visibility(
                        egui::containers::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
                    );
                ));
            }
        }
        if let Some(b) = self.attributes.bottom {
            build.extend(quote!( __efx_sa = __efx_sa.stick_to_bottom(#b); ));
        }
        if let Some(b) = self.attributes.right {
            build.extend(quote!( __efx_sa = __efx_sa.stick_to_right(#b); ));
        }
        if let Some(h) = self.attributes.max_height {
            build.extend(quote!( __efx_sa = __efx_sa.max_height(#h as _); ));
        }
        if let Some(w) = self.attributes.max_width {
            build.extend(quote!( __efx_sa = __efx_sa.max_width(#w as _); ));
        }
        if let Some(id) = self.attributes.id.clone() {
            build.extend(quote!( __efx_sa = __efx_sa.id_salt(#id); ));
        }

        build
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let children_ts = render_children_stmt(&quote!(ui), &self.element.children);
        let build = self.content(ui);

        quote! {{
            #build
            let _ = __efx_sa.show(#ui, |ui| { #children_ts });
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    // source of state identifier (optional)
    id: Option<String>,
    #[attr(name = "always-show")]
    always_show: Option<bool>,
    #[attr(name = "max-width")]
    max_width: Option<f32>,
    #[attr(name = "max-height")]
    max_height: Option<f32>,
    bottom: Option<bool>,
    right: Option<bool>,
    axis: TokenStream,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "Button") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let axis_src = map.get("axis").copied().unwrap_or("vertical");
        let axis_ctor = match axis_src {
            "vertical" => quote!(egui::ScrollArea::vertical()),
            "horizontal" => quote!(egui::ScrollArea::horizontal()),
            "both" => quote!(egui::ScrollArea::both()),
            other => {
                let msg = format!(
                    "efx: <ScrollArea> attribute `axis` must be one of vertical|horizontal|both, got `{}`",
                    other
                );
                return Err(quote! { compile_error!(#msg); });
            }
        };

        Ok(Attributes {
            id: map.get("id").map(|s| (*s).to_string()),
            always_show: bool_opt(&map, "always-show")?,
            max_width: f32_opt(&map, "max-width")?,
            max_height: f32_opt(&map, "max-height")?,
            bottom: bool_opt(&map, "bottom")?,
            right: bool_opt(&map, "right")?,
            axis: axis_ctor,
        })
    }
}
