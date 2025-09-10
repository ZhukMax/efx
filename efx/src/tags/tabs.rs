use crate::interfaces::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::expr::expr_opt;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::{Element, Node};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Tabs {
    attributes: Attributes,
    tabs: Vec<TabItem>,
    element: Element,
}

impl Tag for Tabs {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        let attributes = Attributes::new(el)?;

        let mut tabs: Vec<TabItem> = Vec::new();
        for ch in &el.children {
            match ch {
                Node::Element(t) if t.name == "Tab" => {
                    tabs.push(TabItem::from_element(t)?);
                }
                Node::Element(other) => {
                    let msg = format!(
                        "efx: <Tabs> only allows <Tab> children, got <{}>",
                        other.name
                    );
                    return Err(quote! { compile_error!(#msg); });
                }
                Node::Text(txt) if txt.value.trim().is_empty() => { /* skip whitespace */ }
                _ => {
                    let msg = "efx: <Tabs> does not allow text or expressions outside <Tab>";
                    return Err(quote! { compile_error!(#msg); });
                }
            }
        }

        if tabs.is_empty() {
            let msg = "efx: <Tabs> requires at least one <Tab>";
            return Err(quote! { compile_error!(#msg); });
        }

        {
            use std::collections::BTreeSet;
            let mut seen = BTreeSet::new();

            for t in &tabs {
                if !seen.insert(&t.id) {
                    let msg = format!("efx: <Tabs> duplicate <Tab id=\"{}\">", t.id);
                    return Err(quote! { compile_error!(#msg); });
                }
            }
        }

        Ok(Self {
            attributes,
            tabs,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let default_active = &self.tabs[0].id;
        let active_init = if let Some(expr) = &self.attributes.active_expr {
            quote!({
                // приводим к String (требуем совместимый тип у пользователя)
                let __v = (#expr);
                ::std::string::ToString::to_string(&__v)
            })
        } else {
            quote!(::std::string::ToString::to_string(#default_active))
        };

        let mut header_ts = TokenStream::new();
        for t in &self.tabs {
            let id = &t.id;
            let title = t.title_ts();
            let clickable = if t.enabled.unwrap_or(true) {
                quote! {
                    let __resp = ui.selectable_label(__efx_active == #id, #title);
                    if __resp.clicked() {
                        __efx_active = ::std::string::ToString::to_string(#id);
                    }
                }
            } else {
                quote! {
                    let __resp = ui.add_enabled(false, egui::SelectableLabel::new(__efx_active == #id, #title));
                    let _ = __resp;
                }
            };

            header_ts.extend(clickable);
        }

        let mut content_ts = TokenStream::new();
        for t in &self.tabs {
            let id = &t.id;
            let body = render_children_stmt(&quote!(ui), &t.children);
            content_ts.extend(quote! {
                #id => { #body }
            });
        }

        let (prolog, epilog) = if let Some(g) = self.attributes.gap {
            (
                quote! {
                    let __efx_old_gap_x = #ui.spacing().item_spacing.x;
                    #ui.spacing_mut().item_spacing.x = #g as f32;
                },
                quote! {
                    #ui.spacing_mut().item_spacing.x = __efx_old_gap_x;
                },
            )
        } else {
            (quote!(), quote!())
        };

        let write_back = if let Some(expr) = &self.attributes.active_expr {
            quote!( #expr = __efx_active.clone(); )
        } else {
            quote!()
        };

        quote! {{
            // initialize the active tab
            let mut __efx_active: ::std::string::String = #active_init;

            // header
            #prolog
            #ui.horizontal(|ui| {
                #header_ts
            });
            #epilog

            #ui.add(egui::widgets::Separator::default());

            // content
            match __efx_active.as_str() {
                #content_ts
                _ => { /* unknown id - don't draw anything */ }
            }

            // sync back (if controlled mode)
            #write_back
            ()
        }}
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        self.content(ui)
    }
}

#[derive(Clone, Debug)]
struct TabItem {
    id: String,
    title: Option<String>,
    enabled: Option<bool>,
    children: Vec<Node>,
}

impl TabItem {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        const KNOWN: &[&str] = &["id", "title", "enabled"];
        let map = match attr_map(el, KNOWN, "Tab") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let id = match map.get("id") {
            Some(s) if !s.is_empty() => (*s).to_string(),
            _ => {
                let msg = "efx: <Tab> requires non-empty `id`";
                return Err(quote! { compile_error!(#msg); });
            }
        };

        Ok(Self {
            id,
            title: map.get("title").map(|s| (*s).to_string()),
            enabled: bool_opt(&map, "enabled")?,
            children: el.children.clone(),
        })
    }

    fn title_ts(&self) -> TokenStream {
        if let Some(t) = &self.title {
            quote!(#t)
        } else {
            // By default, use id as the title
            let id = &self.id;
            quote!(#id)
        }
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    /// Controlled mode: String/&str type expression with the id of the active tab.
    active_expr: Option<syn::Expr>,
    /// Indentation between headings, px
    gap: Option<f32>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "Tabs") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Self {
            active_expr: expr_opt(&map, "active")?,
            gap: f32_opt(&map, "gap")?,
        })
    }
}
