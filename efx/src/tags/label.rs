use crate::attr_adapters as A;
use crate::build_buffer_from_children;
use efx_core::Element;
use quote::{ToTokens, quote};

pub(crate) fn render_label_stmt<UI: ToTokens>(ui: &UI, el: &Element) -> proc_macro2::TokenStream {
    let (buf_init, buf_build) = build_buffer_from_children(&el.children);

    // Allowed attributes
    const KNOWN: &[&str] = &[
        "color",
        "size",
        "bold",
        "italic",
        "underline",
        "strike",
        "monospace",
        "wrap",
    ];

    // Collecting modifiers for RichText and the wrap flag
    let mut seen = std::collections::BTreeSet::<&str>::new();
    let mut mods = proc_macro2::TokenStream::new();
    let mut wrap: Option<bool> = None;
    let mut has_style_attrs = false;

    for a in &el.attrs {
        let name = a.name.as_str();
        let val = a.value.as_str();

        // unknown - compilation error
        if !KNOWN.iter().any(|k| *k == name) {
            let msg = format!("efx: <Label> unknown attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        // duplicates - error
        if !seen.insert(name) {
            let msg = format!("efx: <Label> duplicate attribute `{}`", name);
            return quote! { compile_error!(#msg); };
        }

        match name {
            "color" => {
                let ts = match A::parse_color_tokens("color", val) {
                    Ok(ts) => ts,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                mods.extend(quote! { .color(#ts) });
                has_style_attrs = true;
            }
            "size" => {
                let n = match A::parse_f32("size", val) {
                    Ok(n) => n,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                mods.extend(quote! { .size(#n as f32) });
                has_style_attrs = true;
            }
            "bold" => {
                let b = match A::parse_bool("bold", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                if b {
                    mods.extend(quote! { .strong() });
                    has_style_attrs = true;
                }
            }
            "italic" => {
                let b = match A::parse_bool("italic", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                if b {
                    mods.extend(quote! { .italics() });
                    has_style_attrs = true;
                }
            }
            "underline" => {
                let b = match A::parse_bool("underline", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                if b {
                    mods.extend(quote! { .underline() });
                    has_style_attrs = true;
                }
            }
            "strike" => {
                let b = match A::parse_bool("strike", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                if b {
                    mods.extend(quote! { .strikethrough() });
                    has_style_attrs = true;
                }
            }
            "monospace" => {
                let b = match A::parse_bool("monospace", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                if b {
                    mods.extend(quote! { .monospace() });
                    has_style_attrs = true;
                }
            }
            "wrap" => {
                let b = match A::parse_bool("wrap", val) {
                    Ok(b) => b,
                    Err(msg) => return quote! { compile_error!(#msg); },
                };
                wrap = Some(b);
            }
            _ => {}
        }
    }

    // Backward compatibility:
    // if there is NO style attribute and wrap != true — leave the old semantics:
    // ui.label(__efx_buf) -> suitable for test stubs with Into<String>.
    let use_plain_string = !has_style_attrs && wrap != Some(true);

    if use_plain_string {
        return quote! {
            #buf_init
            #buf_build
            #ui.label(__efx_buf);
        };
    }

    // Generation: RichText + ui.label(...) or Label::new(...).wrap(true)
    let rich_apply = if mods.is_empty() {
        quote!( let __efx_rich = egui::RichText::new(__efx_buf); )
    } else {
        quote!( let __efx_rich = egui::RichText::new(__efx_buf) #mods ; )
    };

    let call = match wrap {
        Some(true) => {
            quote! {
                let __efx_widget = egui::widgets::Label::new(__efx_rich).wrap(true);
                #ui.add(__efx_widget);
            }
        }
        _ => {
            quote! { #ui.label(__efx_rich); }
        }
    };

    quote! {
        #buf_init
        #buf_build
        #rich_apply
        #call
    }
}
