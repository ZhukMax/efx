use efx_core::Node;
use quote::quote;

pub fn build_buffer_from_children(children: &[Node]) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
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
