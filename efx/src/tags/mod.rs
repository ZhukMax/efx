pub mod button;
pub mod central_panel;
pub mod column;
pub mod hyperlink;
pub mod label;
pub mod row;
pub mod scroll_area;
pub mod separator;
pub mod text_field;
pub mod util;
pub mod window;

pub use button::render_button;
pub use central_panel::render_central_panel_stmt;
pub use column::Column;
use efx_core::Element;
pub use hyperlink::Hyperlink;
pub use label::render_label_stmt;
use proc_macro2::TokenStream;
use quote::ToTokens;
pub use row::Row;
pub use scroll_area::render_scroll_area_stmt;
pub use separator::Separator;
pub use text_field::render_text_field_stmt;

pub trait Tagged {
    fn parse<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream;
}

pub trait Tag {
    /// Constructor from Element (parses attributes and captures children inside self).
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized;
    /// Render contents
    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream;
    /// Full render
    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream;
}

pub(crate) trait TagAttributes {
    fn new(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized;
}
