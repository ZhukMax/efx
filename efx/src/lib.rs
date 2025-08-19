use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, LitStr};

use efx_core::{parse_str, Element, Node};

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
                let render_btn = render_button(&ui, el);
                quote! { #render_btn }
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

struct EfxInput {
    ui: Expr,
    template: LitStr,
}

impl syn::parse::Parse for EfxInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ui = input.parse::<Expr>()?;
        input.parse::<syn::Token![,]>()?;
        let template = input.parse::<LitStr>()?;
        Ok(EfxInput { ui, template })
    }
}

fn render_nodes_as_stmts<UI: ToTokens>(ui: &UI, nodes: &[Node]) -> proc_macro2::TokenStream {
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

fn render_label_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);
    quote! {
        #buf_init
        #buf_build
        #ui.label(__efx_buf);
    }
}

fn render_button<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);
    quote! {{
        #buf_init
        #buf_build
        #ui.button(__efx_buf)
    }}
}

fn build_buffer_from_children(children: &[Node]) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    use efx_core::Node::*;
    let init = quote! { let mut __efx_buf = ::std::string::String::new(); };
    let mut build = proc_macro2::TokenStream::new();

    for ch in children {
        match ch {
            Text(t) => {
                let s = &t.value;
                build.extend(quote! { __efx_buf.push_str(#s); });
            }
            I11n(i) => {
                let expr: syn::Expr = match syn::parse_str(&i.expr_src) {
                    Ok(e) => e,
                    Err(_) => {
                        let msg = format!("efx: invalid Rust expression in interpolation: {}", i.expr_src);
                        build.extend(quote! { compile_error!(#msg); });
                        continue;
                    }
                };
                build.extend(quote! { ::std::fmt::Write::write_fmt(&mut __efx_buf, format_args!("{}", (#expr))).ok(); });
            }
            Element(_) => {
                // For Label/Button we expect only text/interpolations
                build.extend(quote! { compile_error!("efx: nested elements are not allowed inside <Label>/<Button> in this version"); });
            }
        }
    }

    (init, build)
}
