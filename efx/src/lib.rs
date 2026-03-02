#![doc = include_str!("../docs/intro.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/button.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/column.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/grid.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/heading.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/hyperlink.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/image.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/label.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/panel.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/resize.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/row.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/scroll_area.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/separator.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/table.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/tabs.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/text_field.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/tags/window.md")]
#![doc = "\n\n---\n\n"]
#![doc = include_str!("../docs/guide.md")]

mod attr_adapters;
mod input;
mod interfaces;
mod render;
mod tags;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::input::EfxInput;
use crate::render::render_nodes_as_stmts;
use crate::tags::Button;
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

    let ast = match get_ast(input.template.value()) {
        Ok(ast) => ast,
        Err(msg) => return msg,
    };

    let expanded = if ast.len() == 1 {
        if let Node::Element(el) = &ast[0] {
            if el.name == "Button" {
                return render::render_tag::<Button>(&ui, el).into();
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

#[proc_macro]
pub fn efx_ctx(input: TokenStream) -> TokenStream {
    use crate::input::EfxCtxInput;

    let args = parse_macro_input!(input as EfxCtxInput);
    let ctx_expr = args.ctx;

    let ast = match get_ast(args.template.value()) {
        Ok(ast) => ast,
        Err(msg) => return msg,
    };

    let roots: Vec<Node> = ast
        .into_iter()
        .filter(|n| match n {
            Node::Text(t) => !t.value.trim().is_empty(),
            _ => true,
        })
        .collect();

    let root = match roots.as_slice() {
        // exactly one root element - OK
        [Node::Element(el)] => el.clone(),

        // empty - expect a single root element
        [] => {
            let msg = "efx_ctx!: expected a single root element";
            return quote! { compile_error!(#msg); }.into();
        }

        // one node but not an Element (e.g. text/interpolation) - error
        [_] => {
            let msg = "efx_ctx!: root must be an element";
            return quote! { compile_error!(#msg); }.into();
        }

        // more than one root node is also an error
        [_, ..] => {
            let msg = "efx_ctx!: expected a single root element";
            return quote! { compile_error!(#msg); }.into();
        }
    };

    let allowed = [
        "Window",
        "CentralPanel",
        "TopPanel",
        "BottomPanel",
        "SidePanel",
    ];
    if !allowed.iter().any(|&n| n == root.name) {
        let msg = format!(
            "efx_ctx!: root <{}> is not context-root. Use efx!(ui, ...) instead.",
            root.name
        );
        return quote! { compile_error!(#msg); }.into();
    }

    // Generating a render for the root
    let ui_param = quote!(__efx_ui_ctx);
    let body = render::render_element_stmt(&ui_param, &root);

    quote! {{
        // local wrapper: send only .ctx()
        struct __EfxUiCtxShim<'a>(&'a egui::Context);
        impl<'a> __EfxUiCtxShim<'a> {
            #[inline]
            fn ctx(&self) -> &egui::Context { self.0 }
        }

        // clone the context and render the root
        let __efx_ctx_local = (#ctx_expr).clone();
        let __efx_ui_ctx = __EfxUiCtxShim(&__efx_ctx_local);
        #body
        ()
    }}
    .into()
}

fn get_ast(template_ref: String) -> Result<Vec<Node>, TokenStream> {
    let template_content = match load_template(&template_ref) {
        Ok(content) => content,
        Err(msg) => return Err(quote! { compile_error!(#msg); }.into()),
    };

    match parse_str(&template_content) {
        Ok(nodes) => Ok(nodes),
        Err(err) => {
            let msg = format!("efx parse error: {}", err);
            Err(quote! { compile_error!(#msg); }.into())
        }
    }
}

pub(crate) fn load_template(original_str: &str) -> Result<String, String> {
    let trimmed = original_str.trim();

    if trimmed.starts_with('<') {
        Ok(original_str.to_string())
    } else {
        match efx_core::component::resolve_path(trimmed) {
            Some(path) => std::fs::read_to_string(&path)
                .map_err(|e| format!("EFx: Found file {:?} but failed to read: {}", path, e)),
            None => {
                let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
                Err(format!(
                    "EFx: Template '{}' not found. Searched in {}/src/ui/ and recursive subdirectories.",
                    trimmed, root
                ))
            }
        }
    }
}
