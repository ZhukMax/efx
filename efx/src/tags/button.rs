use efx_core::Element;
use quote::{quote, ToTokens};
use crate::buffer::build_buffer_from_children;

pub(crate) fn render_button<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);
    quote! {{
        #buf_init
        #buf_build
        #ui.button(__efx_buf)
    }}
}
