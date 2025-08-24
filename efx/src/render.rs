use efx_core::{Element, Node};
use quote::{quote, ToTokens};
use crate::tags::button::render_button;
use crate::tags::label::render_label_stmt;

pub(crate) fn render_nodes_as_stmts<UI: ToTokens>(ui: &UI, nodes: &[Node]) -> proc_macro2::TokenStream {
    let mut out = proc_macro2::TokenStream::new();
    for n in nodes {
        out.extend(render_node_stmt(ui, n));
    }
    out
}

fn render_node_stmt<UI: ToTokens>(ui: &UI, node: &Node) -> proc_macro2::TokenStream {
    use efx_core::Node::*;
    match node {
        Text(t) => {
            let s = &t.value;
            quote! { #ui.label(#s); }
        }
        I11n(i) => {
            let expr: syn::Expr = match syn::parse_str(&i.expr_src) {
                Ok(e) => e,
                Err(_) => {
                    let msg = format!("efx: invalid Rust expression in interpolation: {}", i.expr_src);
                    return quote! { compile_error!(#msg); };
                }
            };
            quote! { #ui.label(::std::format!("{}", (#expr))); }
        }
        Element(el) => render_element_stmt(ui, el),
    }
}

fn render_element_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    match el.name.as_str() {
        "Label" => render_label_stmt(ui, el),
        "Button" => {
            let btn_expr = render_button(ui, el);
            quote! { let _ = #btn_expr; }
        }
        "Row" => {
            let inner_ui = quote!(ui);
            let body = render_nodes_as_stmts(&inner_ui, &el.children);
            quote! {
                #ui.horizontal(|ui| {
                    #body
                });
            }
        }
        "Column" => {
            let inner_ui = quote!(ui);
            let body = render_nodes_as_stmts(&inner_ui, &el.children);
            quote! {
                #ui.vertical(|ui| {
                    #body
                });
            }
        }
        "Separator" => {
            if el.children.is_empty() {
                quote! { #ui.separator(); }
            } else {
                quote! { compile_error!("efx: <Separator/> must be self-closing without children"); }
            }
        }
        other => {
            let msg = format!("efx: unknown tag <{}>", other);
            quote! { compile_error!(#msg); }
        }
    }
}
