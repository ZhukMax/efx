use crate::interfaces::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::{Element, Node};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// <Table columns="3" striped="true" spacing-x="8" spacing-y="4">
///   <Tr>
///     <Td><Label>R1C1</Label></Td>
///     <Td><Label>R1C2</Label></Td>
///     <Td><Label>R1C3</Label></Td>
///   </Tr>
///   ...
/// </Table>
pub struct Table {
    attributes: Attributes,
    rows: Vec<Row>,
    element: Element,
}

impl Tag for Table {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        let attributes = Attributes::new(el)?;

        // We collect only <Tr> and validate the structure
        let mut rows = Vec::<Row>::new();
        for ch in &el.children {
            match ch {
                Node::Element(e) if e.name == "Tr" => rows.push(Row::from_element(e)?),
                Node::Element(other) => {
                    let msg = format!(
                        "efx: <Table> only allows <Tr> children, got <{}>",
                        other.name
                    );
                    return Err(quote! { compile_error!(#msg); });
                }
                Node::Text(t) if t.value.trim().is_empty() => { /*ignore ws*/ }
                _ => {
                    let msg = "efx: <Table> does not allow text/expressions outside <Tr>";
                    return Err(quote! { compile_error!(#msg); });
                }
            }
        }

        if rows.is_empty() {
            let msg = "efx: <Table> requires at least one <Tr>";
            return Err(quote! { compile_error!(#msg); });
        }

        Ok(Self {
            attributes,
            rows,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let ncols = self.attributes.columns;
        let spx = self.attributes.spacing_x.unwrap_or(8.0);
        let spy = self.attributes.spacing_y.unwrap_or(4.0);
        let striped = self.attributes.striped.unwrap_or(false);

        // General layout for cells (left|center|right)
        let cell_align_ts = match self.attributes.cell_align.as_deref() {
            Some("center") => quote!(::egui::Align::Center),
            Some("right") => quote!(::egui::Align::Max),
            Some("left") | None => quote!(::egui::Align::Min),
            Some(other) => {
                let msg = format!(
                    "efx: <Table> invalid cell-align '{}', expected left|center|right",
                    other
                );
                return quote! { compile_error!(#msg); };
            }
        };

        let pad = self.attributes.cell_padding.unwrap_or(0.0);

        // Build the body: grid.show(ui, |ui| { ... })
        let mut body_ts = TokenStream::new();

        for row in &self.rows {
            let mut cols_ts = TokenStream::new();
            let mut seen = 0usize;

            for cell in &row.cells {
                let cell_body = render_children_stmt(&quote!(ui), &cell.children);
                cols_ts.extend(quote! {
                    #ui.with_layout(::egui::Layout::left_to_right(#cell_align_ts), |ui| {
                        if #pad > 0.0 { ui.add_space(#pad as f32); }
                        #cell_body
                        if #pad > 0.0 { ui.add_space(#pad as f32); }
                    });
                });
                seen += 1;
            }

            // If there are fewer cells in a row, we add empty spaces to columns
            if seen < ncols as usize {
                let missing = ncols as usize - seen;
                let fillers = (0..missing).map(|_| {
                    quote! {
                        // empty cell
                        #ui.add_space(0.0);
                    }
                });
                cols_ts.extend(quote! { #( #fillers )* });
            }

            body_ts.extend(quote! {
                { #cols_ts #ui.end_row(); }
            });
        }

        // Wrap in Grid::new(...)
        let id_apply = if let Some(id) = &self.attributes.id {
            quote!( let mut __efx_grid = ::egui::Grid::new(#id); )
        } else {
            quote!( let mut __efx_grid = ::egui::Grid::new(::egui::Id::new("efx.table")); )
        };

        quote! {{
            #id_apply
            __efx_grid = __efx_grid
                .num_columns(#ncols as usize)
                .striped(#striped)
                .spacing(::egui::vec2(#spx as f32, #spy as f32));

            __efx_grid.show(#ui, |ui| {
                #body_ts
            });
        }}
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let ts = self.content(ui);
        quote! {{ #ts (); }}
    }
}

#[derive(Clone, Debug)]
struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        let mut cells = Vec::<Cell>::new();
        for ch in &el.children {
            match ch {
                Node::Element(e) if e.name == "Td" => cells.push(Cell::from_element(e)?),
                Node::Element(other) => {
                    let msg = format!("efx: <Tr> only allows <Td>, got <{}>", other.name);
                    return Err(quote! { compile_error!(#msg); });
                }
                Node::Text(t) if t.value.trim().is_empty() => {}
                _ => {
                    let msg = "efx: <Tr> does not allow text/expressions outside <Td>";
                    return Err(quote! { compile_error!(#msg); });
                }
            }
        }
        if cells.is_empty() {
            let msg = "efx: <Tr> must contain at least one <Td>";
            return Err(quote! { compile_error!(#msg); });
        }
        Ok(Self { cells })
    }
}

#[derive(Clone, Debug)]
struct Cell {
    children: Vec<Node>,
}

impl Cell {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        if el
            .attrs
            .iter()
            .any(|a| a.name == "colspan" || a.name == "rowspan")
        {
            let msg = "efx: <Td> attributes colspan/rowspan are not supported yet";
            return Err(quote! { compile_error!(#msg); });
        }
        Ok(Self {
            children: el.children.clone(),
        })
    }
}

#[derive(Clone, Debug, AttrNames)]
struct Attributes {
    /// Mandatory number of columns
    columns: usize,
    /// Line striping
    striped: Option<bool>,
    /// Indents between columns/rows
    #[attr(name = "spacing-x")]
    spacing_x: Option<f32>,
    #[attr(name = "spacing-y")]
    spacing_y: Option<f32>,
    /// Internal cell padding
    #[attr(name = "cell-padding")]
    cell_padding: Option<f32>,
    /// Alignment of cell contents: left|center|right
    #[attr(name = "cell-align")]
    cell_align: Option<String>,
    /// Stable grid id
    id: Option<String>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "Table") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        // columns — required, integer, >0
        let columns = match map.get("columns") {
            Some(s) => match s.parse::<usize>() {
                Ok(n) if n > 0 => n,
                _ => {
                    let msg = "efx: <Table> `columns` must be a positive integer";
                    return Err(quote! { compile_error!(#msg); });
                }
            },
            None => {
                let msg = "efx: <Table> requires `columns` attribute";
                return Err(quote! { compile_error!(#msg); });
            }
        };

        Ok(Attributes {
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
