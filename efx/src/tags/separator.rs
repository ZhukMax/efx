use crate::tags::util::{attr_map, f32_opt};
use crate::tags::{TagAttributes, Tagged};
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct Separator;

impl Tagged for Separator {
    fn parse<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream {
        // no children error
        if !el.children.is_empty() {
            return quote! { compile_error!("efx: <Separator/> must be self-closing without children"); };
        }

        let attributes = match Attributes::new(el) {
            Ok(attr) => attr,
            Err(err) => return err,
        };

        // Calculate the final indents:
        // if space_* is specified, they have priority; otherwise, we use space (the same before/after)
        let before = attributes
            .space_before
            .or(attributes.space)
            .unwrap_or(0.0f32);
        let after = attributes
            .space_after
            .or(attributes.space)
            .unwrap_or(0.0f32);

        let before_ts = if before > 0.0 {
            quote!( #ui.add_space(#before as f32); )
        } else {
            quote!()
        };

        let after_ts = if after > 0.0 {
            quote!( #ui.add_space(#after  as f32); )
        } else {
            quote!()
        };

        quote! {{
            #before_ts
            #ui.separator();
            #after_ts
        }}
    }
}

#[derive(Clone, Debug)]
struct Attributes {
    space: Option<f32>,
    space_before: Option<f32>,
    space_after: Option<f32>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        const KNOWN: &[&str] = &["space", "space_before", "space_after", "vertical"];
        let map = match attr_map(el, KNOWN, "Separator") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            space: f32_opt(&map, "space").unwrap_or(None),
            space_before: f32_opt(&map, "space_before").unwrap_or(None),
            space_after: f32_opt(&map, "space_after").unwrap_or(None),
        })
    }
}
