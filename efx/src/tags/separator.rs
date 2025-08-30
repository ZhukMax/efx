use crate::tags::util::{attr_map, f32_opt};
use crate::tags::{Block, Tag, TagAttributes, Tagged};
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct Separator {
    attributes: Attributes,
    element: Element,
}

impl Block for Separator {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        // children error
        if !el.children.is_empty() {
            return Err(quote! {
                compile_error!("efx: <Separator/> must be self-closing without children");
            });
        }

        let attributes = Attributes::new(el)?;
        Ok(Self {
            attributes,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        quote! { #ui.separator(); }
    }

    fn prolog_epilogue<UI: ToTokens>(&self, ui: &UI) -> (TokenStream, TokenStream) {
        // Calculate the final indents:
        // if space_* is specified, they have priority; otherwise, we use space (the same before/after)
        let before = &self
            .attributes
            .space_before
            .or(self.attributes.space)
            .unwrap_or(0.0f32);
        let after = &self
            .attributes
            .space_after
            .or(self.attributes.space)
            .unwrap_or(0.0f32);

        let prolog = if before > &0.0 {
            quote!( #ui.add_space(#before as f32); )
        } else {
            quote!()
        };

        let epilogue = if after > &0.0 {
            quote!( #ui.add_space(#after  as f32); )
        } else {
            quote!()
        };

        (prolog, epilogue)
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    space: Option<f32>,
    space_before: Option<f32>,
    space_after: Option<f32>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Self::ATTR_NAMES, "Separator") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            space: f32_opt(&map, "space")?,
            space_before: f32_opt(&map, "space_before").unwrap_or(None),
            space_after: f32_opt(&map, "space_after").unwrap_or(None),
        })
    }
}
