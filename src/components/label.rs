use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;
use crate::components::{check_expr_count, expr_count_error_msg, get_format_call};

pub fn render_label(ui: &Expr, format_string: &str, expressions: Vec<&Expr>) -> TokenStream {
    if check_expr_count(format_string, expressions.clone()) {
        return expr_count_error_msg(ui)
    }

    let format_call = get_format_call(format_string, expressions.clone());

    quote! {
        #ui.label(::eframe::egui::RichText::new(#format_call));
    }
}
