use crate::interfaces::*;
use crate::tags::*;
use efx_core::{Element, Node};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) fn render_nodes_as_stmts<UI: ToTokens>(ui: &UI, nodes: &[Node]) -> TokenStream {
    let mut out = TokenStream::new();
    for n in nodes {
        out.extend(render_node_stmt(ui, n));
    }
    out
}

pub(crate) fn render_node_stmt<UI: ToTokens>(ui: &UI, node: &Node) -> TokenStream {
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

pub(crate) fn render_element_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream {
    match el.name.as_str() {
        "Window" => render_tag::<Window>(ui, el),
        "Heading" => render_tag::<Heading>(ui, el),
        "Panel" => render_tag::<Panel>(ui, el),
        "CentralPanel" => render_tag::<CentralPanel>(ui, el),
        "SidePanel" => render_tag::<SidePanel>(ui, el),
        "TopPanel" => render_tag::<TopPanel>(ui, el),
        "BottomPanel" => render_tag::<BottomPanel>(ui, el),
        "ScrollArea" => render_tag::<ScrollArea>(ui, el),
        "Row" => render_tag::<Row>(ui, el),
        "Column" => render_tag::<Column>(ui, el),
        "Resize" => render_tag::<Resize>(ui, el),
        "Tabs" => render_tag::<Tabs>(ui, el),
        "Tab" => {
            let msg = "efx: <Tab> is only allowed as a child of <Tabs>";
            quote! { compile_error!(#msg); }
        }
        "Table" => render_tag::<Table>(ui, el),
        "Tr" | "Td" => {
            let msg = "efx: <Tr>/<Td> are only allowed inside <Table>";
            quote! { compile_error!(#msg); }
        }
        "Image" => render_tag::<Image>(ui, el),
        "Label" => render_tag::<Label>(ui, el),
        "Button" => {
            let btn_expr = render_tag::<Button>(ui, el);
            quote! { #btn_expr; }
        }
        "Separator" => render_tag::<Separator>(ui, el),
        "Hyperlink" => {
            let ts = render_tag::<Hyperlink>(ui, el);
            quote! { #ts; }
        }
        "TextField" => render_tag::<TextField>(ui, el),
        other => {
            let msg = format!("efx: unknown tag <{}>", other);
            quote! { compile_error!(#msg); }
        }
    }
}

pub fn render_tag<T: Tag>(ui: &impl ToTokens, el: &Element) -> TokenStream {
    match T::from_element(el) {
        Ok(tag) => tag.render(ui),
        Err(err) => err,
    }
}
