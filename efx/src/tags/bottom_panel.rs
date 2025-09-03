use crate::tags::{Tag, TagAttributes};
use crate::utils::panel::*;
use crate::utils::render::render_children_stmt;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct BottomPanel {
    attributes: Attributes,
    element: Element,
}

impl Tag for BottomPanel {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        Ok(Self {
            attributes: Attributes::new(el)?,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, _ui: &UI) -> TokenStream {
        FrameStyle::new(self.attributes.clone()).emit()
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let id = match &self.attributes.id {
            Some(s) if !s.is_empty() => s,
            _ => return quote! { compile_error!("efx: <BottomPanel> requires non-empty `id`"); },
        };

        let children = render_children_stmt(&quote!(ui), &self.element.children);
        let frame_ts = self.content(ui);

        let mut panel_ts =
            quote!( let mut __efx_panel = egui::TopBottomPanel::bottom(#id).frame(__efx_frame); );
        panel_ts.extend(emit_size_methods(
            Dim::Height,
            &SizeOpts {
                resizable: self.attributes.resizable,
                default: self.attributes.default_height,
                min: self.attributes.min_height,
                max: self.attributes.max_height,
            },
        ));

        quote! {{
            #frame_ts
            #panel_ts
            let __efx_ctx = #ui.ctx().clone();
            {
                let __efx_tmp = __efx_panel.show(&__efx_ctx, |ui| { #children });
                let _ = __efx_tmp;
            }
            ()
        }}
    }
}
