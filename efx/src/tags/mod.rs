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

use proc_macro2::TokenStream;
use quote::ToTokens;
pub use button::render_button;
pub use central_panel::render_central_panel_stmt;
pub use column::render_column_stmt;
use efx_core::Element;
pub use hyperlink::Hyperlink;
pub use label::render_label_stmt;
pub use row::render_row_stmt;
pub use scroll_area::render_scroll_area_stmt;
pub use separator::Separator;
pub use text_field::render_text_field_stmt;

pub trait Tagged {
    fn parse<UI: ToTokens>(ui: &UI, el: &Element) -> TokenStream;
}

pub(crate) trait TagAttributes {
    fn new(el: &Element) -> Result<Self, TokenStream> where Self: Sized;
}
