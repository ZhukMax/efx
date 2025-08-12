pub(crate) mod label;
pub(crate) mod button;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

/// Check that the number of expressions matches the number of {}
pub(crate) fn check_expr_count(format_string: &str, expressions: Vec<&Expr>) -> bool {
    let expr_count = expressions.len();
    let placeholder_count = format_string.matches("{}").count();

    placeholder_count != expr_count
}

pub(crate) fn expr_count_error_msg(ui: &Expr) -> TokenStream {
    let error_msg = "Invalid number of expressions".to_string();
    quote! { #ui.label(::eframe::egui::RichText::new(#error_msg)) }
}

/// Generate code for format!
pub(crate) fn get_format_call(format_string: &str, expressions: Vec<&Expr>) -> TokenStream {
    let format_call = if expressions.is_empty() {
        quote! { #format_string }
    } else {
        let expr = expressions.iter();
        quote! { format!(#format_string, #(#expr),*) }
    };

    format_call
}
