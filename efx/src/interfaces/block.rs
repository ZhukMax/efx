use crate::interfaces::Tag;
use efx_core::Element;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub trait Block: Sized {
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
