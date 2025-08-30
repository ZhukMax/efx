use crate::tags::{Tag, TagAttributes};
use efx_attrnames::AttrNames;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Expr;
use crate::utils::attr::*;
use crate::utils::expr::expr_req;

pub struct TextField {
    attributes: Attributes,
    element: Element,
}

impl Tag for TextField {
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

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let value = self.attributes.value.clone();

        let base = if matches!(self.attributes.multiline, Some(true)) {
            quote!( egui::TextEdit::multiline(&mut (#value)) )
        } else {
            quote!( egui::TextEdit::singleline(&mut (#value)) )
        };

        let mut build = quote!( let mut __efx_te = #base; );

        if let Some(h) = self.attributes.hint.clone() {
            build.extend(quote!( __efx_te = __efx_te.hint_text(#h); ));
        }
        if let Some(pw) = self.attributes.password.clone() {
            if pw {
                build.extend(quote!( __efx_te = __efx_te.password(true); ));
            }
        }
        if let Some(w) = self.attributes.width.clone() {
            build.extend(quote!( __efx_te = __efx_te.desired_width(#w as f32); ));
        }

        build
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        // Disallow children (<TextField>...</TextField>) is a widget, not a container
        if !self.element.children.is_empty() {
            return quote! { compile_error!("efx: <TextField> must be self-closing and have no children"); };
        }

        let build = self.content(ui);

        quote! {{
            #build
            let _ = #ui.add(__efx_te);
        }}
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    value: Expr,
    hint: Option<String>,
    width: Option<f32>,
    multiline: Option<bool>,
    password: Option<bool>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "TextField") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        // value — required (Rust expression without curly braces)
        let value_expr = match expr_req(&map, "value", "TextField") {
            Ok(e) => e,
            Err(err) => return Err(err),
        };

        Ok(Attributes {
            value: value_expr,
            hint: map.get("hint").map(|s| (*s).to_string()),
            width: f32_opt(&map, "width")?,
            multiline: bool_opt(&map, "multiline")?,
            password: bool_opt(&map, "password")?,
        })
    }
}
