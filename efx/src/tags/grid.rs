use crate::interfaces::{Tag, TagAttributes};
use crate::render::render_nodes_as_stmts;
use crate::utils::attr::*;
use efx_attrnames::AttrNames;
use efx_core::{Element, Node};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// <Grid columns="3" spacing-x="8" spacing-y="4" cell-align="left" cell-padding="4" striped="true" id="users">
///   <Label>A</Label>
///   <Label>B</Label>
///   <GridBreak/>
///   <Button>Ok</Button>
/// </Grid>
pub struct Grid {
    attributes: Attributes,
    items: Vec<Item>,
    element: Element,
}

impl Tag for Grid {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        let attributes = Attributes::new(el)?;

        // Only elements, the GridBreak special tag, and empty texts are allowed
        let mut items = Vec::<Item>::new();
        for ch in &el.children {
            match ch {
                Node::Element(e) if e.name == "GridBreak" => {
                    items.push(Item::Break);
                }
                Node::Element(e) => {
                    items.push(Item::Cell(vec![Node::Element(e.clone())]));
                }
                Node::Text(t) if t.value.trim().is_empty() => { /* ignore whitespace */ }
                _ => {
                    let msg = "efx: <Grid> allows only element children and <GridBreak/>";
                    return Err(quote! { compile_error!(#msg); });
                }
            }
        }

        Ok(Self {
            attributes,
            items,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let cols = self.attributes.columns;
        let spx = self.attributes.spacing_x.unwrap_or(8.0);
        let spy = self.attributes.spacing_y.unwrap_or(4.0);
        let striped = self.attributes.striped.unwrap_or(false);
        let pad = self.attributes.cell_padding.unwrap_or(0.0);

        let cell_align_ts = match self.attributes.cell_align.as_deref() {
            Some("center") => quote!(::egui::Align::Center),
            Some("right") => quote!(::egui::Align::Max),
            Some("left") | None => quote!(::egui::Align::Min),
            Some(other) => {
                let msg = format!(
                    "efx: <Grid> invalid cell-align '{}', expected left|center|right",
                    other
                );
                return quote! { compile_error!(#msg); };
            }
        };

        let mut body_ts = TokenStream::new();
        for it in &self.items {
            match it {
                Item::Break => {
                    body_ts.extend(quote! {{
                        if __efx_col != 0 { ui.end_row(); }
                        __efx_col = 0;
                    }});
                }
                Item::Cell(nodes) => {
                    let cell_body = render_nodes_as_stmts(&quote!(ui), nodes);
                    body_ts.extend(quote! {{
                        #ui.with_layout(::egui::Layout::left_to_right(#cell_align_ts), |ui| {
                            if #pad > 0.0 { ui.add_space(#pad as f32); }
                            #cell_body
                            if #pad > 0.0 { ui.add_space(#pad as f32); }
                        });
                        __efx_col += 1;
                        if __efx_col >= #cols {
                            ui.end_row();
                            __efx_col = 0;
                        }
                    }});
                }
            }
        }

        // Initialize the grid
        let id_decl = if let Some(id) = &self.attributes.id {
            quote!( let mut __efx_grid = ::egui::Grid::new(#id); )
        } else {
            quote!( let mut __efx_grid = ::egui::Grid::new(::egui::Id::new("efx.grid")); )
        };

        quote! {{
            #id_decl
            __efx_grid = __efx_grid
                .num_columns(#cols as usize)
                .striped(#striped)
                .spacing(::egui::vec2(#spx as f32, #spy as f32));

            __efx_grid.show(#ui, |ui| {
                let mut __efx_col: usize = 0;
                #body_ts
                if __efx_col != 0 { ui.end_row(); }
            });
        }}
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let ts = self.content(ui);
        quote! {{ #ts (); }}
    }
}

#[derive(Clone, Debug)]
enum Item {
    Break,
    Cell(Vec<Node>),
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    /// Mandatory number of columns
    columns: usize,
    /// Striped lines (background)
    striped: Option<bool>,
    /// Intercolumn/line spacing
    #[attr(name = "spacing-x")]
    spacing_x: Option<f32>,
    #[attr(name = "spacing-y")]
    spacing_y: Option<f32>,
    /// Internal cell padding
    #[attr(name = "cell-padding")]
    cell_padding: Option<f32>,
    /// Aligning cell contents
    #[attr(name = "cell-align")]
    cell_align: Option<String>,
    /// Stable grid id
    id: Option<String>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "Grid") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let columns = match map.get("columns") {
            Some(s) => match s.parse::<usize>() {
                Ok(n) if n > 0 => n,
                _ => {
                    let msg = "efx: <Grid> `columns` must be a positive integer";
                    return Err(quote! { compile_error!(#msg); });
                }
            },
            None => {
                let msg = "efx: <Grid> requires `columns` attribute";
                return Err(quote! { compile_error!(#msg); });
            }
        };

        Ok(Self {
            columns,
            striped: bool_opt(&map, "striped")?,
            spacing_x: f32_opt(&map, "spacing-x")?,
            spacing_y: f32_opt(&map, "spacing-y")?,
            cell_padding: f32_opt(&map, "cell-padding")?,
            cell_align: map.get("cell-align").map(|s| (*s).to_string()),
            id: map.get("id").map(|s| (*s).to_string()),
        })
    }
}
