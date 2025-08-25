use crate::build_buffer_from_children;
use efx_core::Element;
use quote::{ToTokens, quote};

pub(crate) fn render_label_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);
    quote! {
        #buf_init
        #buf_build
        #ui.label(__efx_buf);
    }
}
