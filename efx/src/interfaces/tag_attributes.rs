use efx_core::Element;
use proc_macro2::TokenStream;

pub trait TagAttributes {
    fn new(el: &Element) -> Result<Self, TokenStream>
    where
        Self: Sized;
}
