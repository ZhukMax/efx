pub mod button;
pub mod central_panel;
pub mod column;
pub mod heading;
pub mod hyperlink;
pub mod label;
pub mod row;
pub mod scroll_area;
pub mod separator;
pub mod side_panel;
pub mod text_field;
pub mod top_panel;
pub mod window;
pub mod bottom_panel;

pub use button::Button;
pub use central_panel::CentralPanel;
pub use column::Column;
use efx_core::Element;
pub use heading::Heading;
pub use hyperlink::Hyperlink;
pub use label::Label;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
pub use row::Row;
pub use scroll_area::ScrollArea;
pub use separator::Separator;
pub use side_panel::SidePanel;
pub use top_panel::TopPanel;
pub use text_field::TextField;
pub use bottom_panel::BottomPanel;

pub trait Tag: Sized {
    /// Constructor from Element (parses attributes and captures children inside self).
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized;
    /// Render contents
    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream;
    /// Full render
    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream;
}

pub(crate) trait Block: Sized {
    fn from_element(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized;
    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream;
    fn prolog_epilogue<UI: ToTokens>(&self, ui: &UI) -> (TokenStream, TokenStream);

    /// Full render: prologue → content → epilogue.
    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        let (prolog, epilogue) = self.prolog_epilogue(ui);
        let content = self.content(ui);
        quote! {{ #prolog #content #epilogue }}
    }
}

impl<T: Block> Tag for T {
    fn from_element(el: &Element) -> Result<Self, TokenStream> {
        <T as Block>::from_element(el)
    }

    fn content<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        <T as Block>::content(self, ui)
    }

    fn render<UI: ToTokens>(&self, ui: &UI) -> TokenStream {
        <T as Block>::render(self, ui)
    }
}

pub(crate) trait TagAttributes {
    fn new(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized;
}
