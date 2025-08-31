use efx_core::attr::{self as core_attr, Rgba};
use proc_macro2::TokenStream;
use quote::quote;

#[inline]
pub fn parse_bool(name: &str, s: &str) -> Result<bool, String> {
    core_attr::parse_bool(name, s)
}

/// Parse unsigned 8-bit integer attribute (0..=255).
#[inline]
pub fn parse_u8(name: &str, s: &str) -> Result<u8, String> {
    core_attr::parse_u8(name, s)
}

#[inline]
pub fn parse_f32(name: &str, s: &str) -> Result<f32, String> {
    core_attr::parse_f32(name, s)
}

#[inline]
pub fn parse_enum(name: &str, s: &str, allowed: &[&str]) -> Result<usize, String> {
    core_attr::parse_enum(name, s, allowed)
}

pub fn parse_color_tokens(name: &str, s: &str) -> Result<TokenStream, String> {
    let Rgba { r, g, b, a } = core_attr::parse_color_rgba(name, s)?;
    Ok(quote!(egui::Color32::from_rgba_unmultiplied(#r, #g, #b, #a)))
}
