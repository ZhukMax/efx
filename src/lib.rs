use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, LitStr};

mod components;

fn parse_template_content(content: &str) -> (String, Vec<Expr>) {
    let mut expressions = Vec::new();
    let mut format_string = String::new();
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() != Some(&'{') {
            let mut expr = String::new();
            while let Some(next) = chars.next() {
                if next == '}' {
                    break;
                }
                expr.push(next);
            }
            let expr_tokens: proc_macro2::TokenStream = expr.parse().unwrap();
            let expr: Expr = syn::parse2(expr_tokens).unwrap();
            expressions.push(expr);
            format_string.push_str("{}");
        } else if c == '{' && chars.peek() == Some(&'{') {
            chars.next();
            format_string.push('{');
        } else if c == '}' && chars.peek() == Some(&'}') {
            chars.next();
            format_string.push('}');
        } else {
            format_string.push(c);
        }
    }

    (format_string, expressions)
}

#[proc_macro]
pub fn efx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as EfxInput);

    let ui = input.ui;
    let template = input.template.value();

    let re = regex::Regex::new(r"^<(\w+)>(.*?)</(\w+)>$").unwrap();
    let expanded = if let Some(captures) = re.captures(&template) {
        let open_tag = captures.get(1).unwrap().as_str();
        let content = captures.get(2).unwrap().as_str();
        let close_tag = captures.get(3).unwrap().as_str();
        let (format_string, expressions) = parse_template_content(content);

        if open_tag == close_tag {
            match open_tag {
                "Label" => {
                    let content_lit = LitStr::new(&format_string, proc_macro2::Span::call_site());
                    components::label::render_label(&ui, &content_lit.value(), expressions.iter().collect())
                }
                "Button" => {
                    let content_lit = LitStr::new(&format_string, proc_macro2::Span::call_site());
                    components::button::render_button(&ui, &content_lit.value(), expressions.iter().collect())
                }
                _ => quote! {
                    #ui.label(::eframe::egui::RichText::new(format!("Unknown tag: {}", #open_tag)));
                },
            }
        } else {
            quote! {
                #ui.label(::eframe::egui::RichText::new(format!(
                    "Mismatched tags: <{}> and </{}>",
                    #open_tag, #close_tag
                )));
            }
        }
    } else {
        quote! {
            #ui.label(::eframe::egui::RichText::new(format!("Invalid template: {}", #template)));
        }
    };

    TokenStream::from(expanded)
}

/// Structure for parsing input arguments efx
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
