use efx_core::Element;
use proc_macro2::TokenStream;
use quote::ToTokens;

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
