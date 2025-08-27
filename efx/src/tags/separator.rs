use efx_core::Element;
use quote::{ToTokens, quote};

use crate::tags::util::{attr_map, f32_opt};

pub fn render_separator_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    // no children
    if !el.children.is_empty() {
        return quote! { compile_error!("efx: <Separator/> must be self-closing without children"); };
    }

    const KNOWN: &[&str] = &["space", "space_before", "space_after", "vertical"];
    let map = match attr_map(el, KNOWN, "Separator") {
        Ok(m) => m,
        Err(err) => return err,
    };

    let space = f32_opt(&map, "space").unwrap_or(None);
    let space_before = f32_opt(&map, "space_before").unwrap_or(None);
    let space_after = f32_opt(&map, "space_after").unwrap_or(None);

    // Calculate the final indents:
    // if space_* is specified, they have priority; otherwise, we use space (the same before/after)
    let before = space_before.or(space).unwrap_or(0.0f32);
    let after = space_after.or(space).unwrap_or(0.0f32);

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
