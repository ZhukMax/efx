use efx_core::Node;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn render_children_stmt<UI: ToTokens>(ui_ident: &UI, children: &[Node]) -> TokenStream {
    let mut out = TokenStream::new();
    for ch in children {
        let stmt = crate::render::render_node_stmt(&quote!(#ui_ident), ch);
        out.extend(quote! { #stmt });
    }
    out
}
