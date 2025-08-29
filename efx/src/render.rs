use proc_macro2::TokenStream;
use crate::tags::*;
use efx_core::{Element, Node};
use quote::{ToTokens, quote};

pub(crate) fn render_nodes_as_stmts<UI: ToTokens>(
    ui: &UI,
    nodes: &[Node],
) -> proc_macro2::TokenStream {
    let mut out = proc_macro2::TokenStream::new();
    for n in nodes {
        out.extend(render_node_stmt(ui, n));
    }
    out
}

pub(crate) fn render_node_stmt<UI: ToTokens>(ui: &UI, node: &Node) -> proc_macro2::TokenStream {
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
                    let msg = format!(
                        "efx: invalid Rust expression in interpolation: {}",
                        i.expr_src
                    );
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
        "CentralPanel" => render_central_panel_stmt(ui, el),
        "Label" => render_label_stmt(ui, el),
        "Button" => {
            let btn_expr = render_button(ui, el);
            quote! { #btn_expr; }
        }
        "Row" => render_tag::<Row>(ui, el),
        "Column" => render_tag::<Column>(ui, el),
        "Separator" => Separator::parse(ui, el),
        "ScrollArea" => render_scroll_area_stmt(ui, el),
        "Hyperlink" => {
            let ts = Hyperlink::parse(ui, el);
            quote! { #ts; }
        }
        "TextField" => render_text_field_stmt(ui, el),
        other => {
            let msg = format!("efx: unknown tag <{}>", other);
            quote! { compile_error!(#msg); }
        }
    }
}

fn render_tag<T: Tag>(ui: &impl ToTokens, el: &Element) -> TokenStream {
    match T::from_element(el) {
        Ok(tag)  => tag.render(ui),
        Err(err) => err,
    }
}
