use efx_core::Element;
use quote::{ToTokens, quote};

use crate::attr_adapters as A;

pub(crate) fn render_separator_stmt<UI: ToTokens>(
    ui: &UI,
    el: &Element,
) -> proc_macro2::TokenStream {
    // no children
    if !el.children.is_empty() {
        return quote! { compile_error!("efx: <Separator/> must be self-closing without children"); };
    }

    const KNOWN: &[&str] = &["space", "space_before", "space_after", "vertical"];

    let mut seen = std::collections::BTreeSet::<&str>::new();
    let mut space: Option<f32> = None;
    let mut space_before: Option<f32> = None;
    let mut space_after: Option<f32> = None;
    let mut _vertical: Option<bool> = None;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <Separator> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }
        if !seen.insert(name) {
            let msg = format!("efx: <Separator> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "space" => match A::parse_f32("space", val) {
                Ok(n) => space = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "space_before" => match A::parse_f32("space_before", val) {
                Ok(n) => space_before = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "space_after" => match A::parse_f32("space_after", val) {
                Ok(n) => space_after = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "vertical" => {
                match A::parse_bool("vertical", val) {
                    Ok(b) => _vertical = Some(b), // ignore for now: there is no vertical separator in the doc prelude
                    Err(msg) => return quote! { compile_error!(#msg); },
                }
            }
            _ => {}
        }
    }

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
