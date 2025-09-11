#![cfg(feature = "extras")]

use crate::interfaces::{Tag, TagAttributes};
use crate::utils::attr::*;
use crate::utils::render::render_children_stmt;
use efx_attrnames::AttrNames;
use efx_core::{Element, Node};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// <DataTable>
///   <Columns>
///     <Column mode="initial" width="160" resizable="true" clip="true"/>
///     <Column mode="auto"/>
///     <Column mode="remainder" resizable="true"/>
///   </Columns>
///
///   <Header>
///     <Td>…</Td><Td>…</Td><Td>…</Td>
///   </Header>
///
///   <Tr>…</Tr>
///   <Tr>…</Tr>
/// </DataTable>
pub struct DataTable {
    attributes: Attributes,
    columns: Vec<ColDef>,
    header_cells: Option<Vec<Node>>,
    rows: Vec<Row>,
    element: Element,
}

impl Tag for DataTable {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized,
    {
        let attributes = Attributes::new(el)?;

        let mut columns: Vec<ColDef> = Vec::new();
        let mut header_cells: Option<Vec<Node>> = None;
        let mut rows: Vec<Row> = Vec::new();

        for ch in &el.children {
            match ch {
                Node::Element(e) if e.name == "Columns" => {
                    for cc in &e.children {
                        match cc {
                            Node::Element(col) if col.name == "Column" => {
                                columns
                                    .push(ColDef::from_element(col, attributes.default_resizable)?);
                            }
                            Node::Element(other) => {
                                let msg = format!(
                                    "efx: <Columns> only allows <Column>, got <{}>",
                                    other.name
                                );
                                return Err(quote! { compile_error!(#msg); });
                            }
                            Node::Text(t) if t.value.trim().is_empty() => {}
                            _ => {
                                let msg = "efx: <Columns> does not allow text/expressions";
                                return Err(quote! { compile_error!(#msg); });
                            }
                        }
                    }

                    if columns.is_empty() {
                        let msg = "efx: <Columns> must contain at least one <Column>";
                        return Err(quote! { compile_error!(#msg); });
                    }
                }

                Node::Element(e) if e.name == "Header" => {
                    if header_cells.is_some() {
                        let msg = "efx: <DataTable> only one <Header> is allowed";
                        return Err(quote! { compile_error!(#msg); });
                    }
                    // Header accepts only <Td> (single line)
                    let mut cells: Vec<Node> = Vec::new();
                    for cc in &e.children {
                        match cc {
                            Node::Element(td) if td.name == "Td" => {
                                cells.push(Node::Element(td.clone()))
                            }
                            Node::Element(other) => {
                                let msg =
                                    format!("efx: <Header> only allows <Td>, got <{}>", other.name);
                                return Err(quote! { compile_error!(#msg); });
                            }
                            Node::Text(t) if t.value.trim().is_empty() => {}
                            _ => {
                                let msg =
                                    "efx: <Header> does not allow text/expressions outside <Td>";
                                return Err(quote! { compile_error!(#msg); });
                            }
                        }
                    }
                    if cells.is_empty() {
                        let msg = "efx: <Header> must contain at least one <Td>";
                        return Err(quote! { compile_error!(#msg); });
                    }
                    header_cells = Some(cells);
                }

                Node::Element(e) if e.name == "Tr" => {
                    rows.push(Row::from_element(e)?);
                }

                Node::Element(other) => {
                    let msg = format!("efx: <DataTable> unknown child <{}>", other.name);
                    return Err(quote! { compile_error!(#msg); });
                }

                Node::Text(t) if t.value.trim().is_empty() => {}
                _ => {
                    let msg = "efx: <DataTable> does not allow text/expressions at top-level";
                    return Err(quote! { compile_error!(#msg); });
                }
            }
        }

        if columns.is_empty() {
            let msg = "efx: <DataTable> requires <Columns> with at least one <Column>";
            return Err(quote! { compile_error!(#msg); });
        }

        // If there is a header, check that the number of cells matches the number of columns.
        if let Some(hc) = &header_cells {
            if hc.len() != columns.len() {
                let msg = format!(
                    "efx: <Header> cells ({}) must match number of <Column> ({})",
                    hc.len(),
                    columns.len()
                );
                return Err(quote! { compile_error!(#msg); });
            }
        }

        Ok(Self {
            attributes,
            columns,
            header_cells,
            rows,
            element: el.clone(),
        })
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let ncols = self.columns.len();
        let striped = self.attributes.striped.unwrap_or(false);
        let resizable_global = self.attributes.table_resizable.unwrap_or(false);
        let row_h = self.attributes.row_height.unwrap_or(22.0);
        let header_h = self.attributes.header_height.unwrap_or(22.0);
        let pad = self.attributes.cell_padding.unwrap_or(0.0);

        // Alignment of content in cells (by X)
        let cell_align_ts = match self.attributes.cell_align.as_deref() {
            Some("center") => quote!(::egui::Align::Center),
            Some("right") => quote!(::egui::Align::Max),
            Some("left") | None => quote!(::egui::Align::Min),
            Some(other) => {
                let msg = format!(
                    "efx: <DataTable> invalid cell-align '{}', expected left|center|right",
                    other
                );
                return quote! { compile_error!(#msg); };
            }
        };

        let mut tbl_build = TokenStream::new();
        tbl_build.extend(quote! {
            let mut __efx_tbl = ::egui_extras::TableBuilder::new(ui)
                .striped(#striped)
                .resizable(#resizable_global);
        });

        for col in &self.columns {
            let mut cts = TokenStream::new();
            match col.mode {
                ColMode::Auto => {
                    cts.extend(quote!( let mut __efx_col = ::egui_extras::Column::auto(); ));
                }
                ColMode::Initial => {
                    let w = col.width.unwrap_or(120.0);
                    cts.extend(
                        quote!( let mut __efx_col = ::egui_extras::Column::initial(#w as f32); ),
                    );
                }
                ColMode::Exact => {
                    let w = col.width.unwrap_or(120.0);
                    cts.extend(
                        quote!( let mut __efx_col = ::egui_extras::Column::exact(#w as f32); ),
                    );
                }
                ColMode::Remainder => {
                    cts.extend(quote!( let mut __efx_col = ::egui_extras::Column::remainder(); ));
                }
            }
            if let Some(b) = col.resizable {
                cts.extend(quote!( __efx_col = __efx_col.resizable(#b); ));
            }
            if let Some(b) = col.clip {
                cts.extend(quote!( __efx_col = __efx_col.clip(#b); ));
            }
            tbl_build.extend(quote! {
                { #cts __efx_tbl = __efx_tbl.column(__efx_col); }
            });
        }

        let header_ts = if let Some(cells) = &self.header_cells {
            let mut cols_ts = TokenStream::new();
            for cell in cells {
                let body = render_children_stmt(
                    &quote!(ui),
                    &match cell {
                        Node::Element(e) => &e.children,
                        _ => &[],
                    },
                );
                cols_ts.extend(quote! {
                    header.col(|ui| {
                        #ui.with_layout(::egui::Layout::left_to_right(#cell_align_ts), |ui| {
                            if #pad > 0.0 { ui.add_space(#pad as f32); }
                            #body
                            if #pad > 0.0 { ui.add_space(#pad as f32); }
                        });
                    });
                });
            }
            quote! {
                __efx_tbl = __efx_tbl.header(#header_h as f32, |mut header| {
                    #cols_ts
                });
            }
        } else {
            quote!()
        };

        let mut body_rows_ts = TokenStream::new();
        for row in &self.rows {
            let mut row_cols_ts = TokenStream::new();

            let mut seen = 0usize;
            for cell in &row.cells {
                let body = render_children_stmt(&quote!(ui), &cell.children);
                row_cols_ts.extend(quote! {
                    row.col(|ui| {
                        #ui.with_layout(::egui::Layout::left_to_right(#cell_align_ts), |ui| {
                            if #pad > 0.0 { ui.add_space(#pad as f32); }
                            #body
                            if #pad > 0.0 { ui.add_space(#pad as f32); }
                        });
                    });
                });
                seen += 1;
            }
            // we fill in the number of columns with empty spaces
            if seen < ncols {
                let missing = ncols - seen;
                let fillers = (0..missing).map(|_| {
                    quote! { row.col(|ui| { let _ = ui; }); }
                });
                row_cols_ts.extend(quote! { #( #fillers )* });
            }

            let rh = row.height.unwrap_or(row_h);
            body_rows_ts.extend(quote! {
                body.row(#rh as f32, |mut row| { #row_cols_ts });
            });
        }

        let body_ts = quote! {
            __efx_tbl = __efx_tbl.body(|mut body| { #body_rows_ts });
        };

        // Wrapper by id (push_id). The inner block relies on the local `ui`.
        let table_block = quote! {{
            #tbl_build
            #header_ts
            #body_ts
        }};

        let out = if let Some(id) = &self.attributes.id {
            quote! {{
                let ui = #ui;
                ui.push_id(#id, |ui| { #table_block });
            }}
        } else {
            quote! {{
                let ui = #ui;
                #table_block
            }}
        };

        out.into()
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        self.content(ui)
    }
}

#[derive(Clone, Debug)]
struct Row {
    height: Option<f32>,
    cells: Vec<Cell>,
}

impl Row {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        const KNOWN: &[&str] = &["height"];
        let map = match attr_map(el, KNOWN, "Tr") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let mut cells = Vec::<Cell>::new();
        for ch in &el.children {
            match ch {
                Node::Element(td) if td.name == "Td" => cells.push(Cell::from_element(td)?),
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

        Ok(Self {
            height: f32_opt(&map, "height")?,
            cells,
        })
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
    id: Option<String>,

    striped: Option<bool>,
    /// Глобальный флаг resizable для таблицы (колонки всё равно можно переопределять индивидуально)
    #[attr(name = "resizable")]
    table_resizable: Option<bool>,

    /// Дефолт для <Column resizable=...> если не указан
    #[attr(name = "default-resizable")]
    default_resizable: Option<bool>,

    #[attr(name = "row-height")]
    row_height: Option<f32>,
    #[attr(name = "header-height")]
    header_height: Option<f32>,

    #[attr(name = "cell-padding")]
    cell_padding: Option<f32>,
    #[attr(name = "cell-align")]
    cell_align: Option<String>,
}

impl TagAttributes for Attributes {
    fn new(el: &Element) -> Result<Self, TokenStream> {
        let map = match attr_map(el, Attributes::ATTR_NAMES, "DataTable") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        Ok(Self {
            id: map.get("id").map(|s| (*s).to_string()),
            striped: bool_opt(&map, "striped")?,
            table_resizable: bool_opt(&map, "resizable")?,
            default_resizable: bool_opt(&map, "default-resizable")?,
            row_height: f32_opt(&map, "row-height")?,
            header_height: f32_opt(&map, "header-height")?,
            cell_padding: f32_opt(&map, "cell-padding")?,
            cell_align: map.get("cell-align").map(|s| (*s).to_string()),
        })
    }
}

#[derive(Clone, Debug)]
struct ColDef {
    mode: ColMode,
    width: Option<f32>,      // для initial/exact
    resizable: Option<bool>, // если None — берём default_resizable из таблицы
    clip: Option<bool>,
}

#[derive(Clone, Copy, Debug)]
enum ColMode {
    Auto,
    Initial,
    Exact,
    Remainder,
}

impl ColDef {
    fn from_element(el: &Element, inherit_resizable: Option<bool>) -> Result<Self, TokenStream> {
        const KNOWN: &[&str] = &["mode", "width", "resizable", "clip"];
        let map = match attr_map(el, KNOWN, "Column") {
            Ok(m) => m,
            Err(err) => return Err(err),
        };

        let mode = match map.get("mode").map(|s| (*s).to_string()) {
            Some(m) => match m.as_str() {
                "auto" => ColMode::Auto,
                "initial" => ColMode::Initial,
                "exact" => ColMode::Exact,
                "remainder" => ColMode::Remainder,
                other => {
                    let msg = format!(
                        "efx: <Column mode=\"{}\"> must be one of auto|initial|exact|remainder",
                        other
                    );
                    return Err(quote! { compile_error!(#msg); });
                }
            },
            None => {
                let msg = "efx: <Column> requires `mode` (auto|initial|exact|remainder)";
                return Err(quote! { compile_error!(#msg); });
            }
        };

        let width = f32_opt(&map, "width")?;
        // требуем width для initial/exact
        if matches!(mode, ColMode::Initial | ColMode::Exact) && width.is_none() {
            let msg = "efx: <Column mode=\"initial|exact\"> requires `width`";
            return Err(quote! { compile_error!(#msg); });
        }

        Ok(Self {
            mode,
            width,
            resizable: bool_opt(&map, "resizable")?.or(inherit_resizable),
            clip: bool_opt(&map, "clip")?,
        })
    }
}
