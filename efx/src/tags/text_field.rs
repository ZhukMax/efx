use efx_core::Element;
use quote::{ToTokens, quote};

use crate::attr_adapters as A;

pub(crate) fn render_text_field_stmt<UI: ToTokens>(
    ui: &UI,
    el: &Element,
) -> proc_macro2::TokenStream {
    // Disallow children (<TextField>...</TextField>) is a widget, not a container
    if !el.children.is_empty() {
        return quote! { compile_error!("efx: <TextField> must be self-closing and have no children"); };
    }

    const KNOWN: &[&str] = &["value", "hint", "password", "width", "multiline"];

    let mut seen = std::collections::BTreeSet::<&str>::new();

    // value — required (Rust expression without curly braces)
    let mut value_expr_src: Option<String> = None;

    let mut hint: Option<String> = None;
    let mut password: Option<bool> = None;
    let mut width: Option<f32> = None;
    let mut multiline: Option<bool> = None;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <TextField> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }
        if !seen.insert(name) {
            let msg = format!("efx: <TextField> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "value" => {
                // Здесь ожидаем валидное Rust-выражение, например: state.name
                value_expr_src = Some(a.value.clone());
            }
            "hint" => {
                hint = Some(a.value.clone());
            }
            "password" => match A::parse_bool("password", val) {
                Ok(b) => password = Some(b),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "width" => match A::parse_f32("width", val) {
                Ok(n) => width = Some(n),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            "multiline" => match A::parse_bool("multiline", val) {
                Ok(b) => multiline = Some(b),
                Err(msg) => return quote! { compile_error!(#msg); },
            },
            _ => {}
        }
    }

    let Some(expr_src) = value_expr_src else {
        return quote! { compile_error!("efx: <TextField> requires `value=\"<expr>\"` attribute"); };
    };

    // Parse the string into syn::Expr - this will give a type-safe code generator
    let value_expr: syn::Expr = match syn::parse_str(&expr_src) {
        Ok(e) => e,
        Err(_) => {
            let msg = format!(
                "efx: attribute `value` must be a valid Rust expression, got `{}`",
                expr_src
            );
            return quote! { compile_error!(#msg); };
        }
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
