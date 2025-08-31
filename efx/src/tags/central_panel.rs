use crate::tags::{Tag, TagAttributes};
use crate::utils::panel::*;
use crate::utils::render::render_children_stmt;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct CentralPanel {
    attributes: Attributes,
    element: Element,
}

impl Tag for CentralPanel {
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
        let children = render_children_stmt(&quote!(ui), &self.element.children);
        let frame_ts = self.content(ui);

        quote! {{
            #frame_ts
            egui::CentralPanel::default()
                .frame(__efx_frame)
                .show(&#ui.ctx(), |ui| { #children });
        }}
    }
}
