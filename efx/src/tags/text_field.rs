use efx_core::Element;
use quote::{ToTokens, quote};

use crate::tags::util::{attr_map, bool_opt, expr_req, f32_opt};

pub fn render_text_field_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    // Disallow children (<TextField>...</TextField>) is a widget, not a container
    if !el.children.is_empty() {
        return quote! { compile_error!("efx: <TextField> must be self-closing and have no children"); };
    }

    const KNOWN: &[&str] = &["value", "hint", "password", "width", "multiline"];
    let map = match attr_map(el, KNOWN, "TextField") {
        Ok(m) => m,
        Err(err) => return err,
    };

    // value — required (Rust expression without curly braces)
    let value_expr = match expr_req(&map, "value", "TextField") {
        Ok(e) => e,
        Err(err) => return err,
    };

    let hint = map.get("hint").map(|s| (*s).to_string());
    let password = match bool_opt(&map, "password") {
        Ok(v) => v,
        Err(err) => return err,
    };
    let width = match f32_opt(&map, "width") {
        Ok(v) => v,
        Err(err) => return err,
    };
    let multiline = match bool_opt(&map, "multiline") {
        Ok(v) => v,
        Err(err) => return err,
    };

    let base = if matches!(multiline, Some(true)) {
        quote!( egui::TextEdit::multiline(&mut (#value_expr)) )
    } else {
        quote!( egui::TextEdit::singleline(&mut (#value_expr)) )
    };

    let mut build = quote!( let mut __efx_te = #base; );

    if let Some(h) = hint {
        build.extend(quote!( __efx_te = __efx_te.hint_text(#h); ));
    }
    if let Some(pw) = password {
        if pw {
            build.extend(quote!( __efx_te = __efx_te.password(true); ));
        }
    }
    if let Some(w) = width {
        build.extend(quote!( __efx_te = __efx_te.desired_width(#w as f32); ));
    }

    quote! {{
        #build
        let _ = #ui.add(__efx_te);
    }}
}
