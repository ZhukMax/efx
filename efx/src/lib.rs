#![doc = include_str!("../docs/intro.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/guide.md")]

mod attr_adapters;
mod buffer;
mod input;
mod render;
mod tags;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::buffer::build_buffer_from_children;
use crate::input::EfxInput;
use crate::render::render_nodes_as_stmts;
use crate::tags::button::render_button;
use efx_core::{parse_str, Node};

/// Functional procedural macro `efx!` - parses compact XML-like markup
/// and executes it against the passed UI context.
///
/// Takes two arguments:
/// 1) **ui** — UI context expression/identifier;
/// 2) **template** — a string literal with markup.
/// # Example
/// ```rust
/// use efx_core::doc_prelude::*;
/// use efx::*;
///
/// let mut ui = Ui::default();
/// efx!(ui, r#"
///   <Column>
///      <Label>Hello</Label>
///      <Separator/>
///      <Row><Label>Row</Label></Row>
///   </Column>
/// "#);
/// ```
///
/// # Errors
/// - Unknown tag → `compile_error!`.
/// - `Separator` with children → `compile_error!`.
/// - Invalid interpolation `{ expr }` → `compile_error!`.
///
/// Tag attributes are **parsed** (since 0.4), but are currently **ignored** by the renderer.
#[proc_macro]
pub fn efx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as EfxInput);
    let ui = input.ui;
    let template = input.template.value();

    let ast = match parse_str(&template) {
        Ok(nodes) => nodes,
        Err(err) => {
            let msg = format!("efx parse error: {}", err);
            return quote! { compile_error!(#msg); }.into();
        }
    };

    let expanded = if ast.len() == 1 {
        if let Node::Element(el) = &ast[0] {
            if el.name == "Button" {
                return render_button(&ui, el).into();
            } else {
                // Any other unit root is like a block with statements (return ())
                let body = render_nodes_as_stmts(&ui, &ast);
                quote! {{
                    #body
                }}
            }
        } else {
            // Text/interpolation on the root - just label
            let body = render_nodes_as_stmts(&ui, &ast);
            quote! {{
                #body
            }}
        }
    } else {
        let body = render_nodes_as_stmts(&ui, &ast);
        quote! {{
            #body
        }}
    };

    expanded.into()
}
