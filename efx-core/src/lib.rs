pub mod ast;
pub mod attr;

#[cfg(feature = "doc-prelude")]
pub mod doc_prelude;

pub use ast::{Attr, Element, Interpolation, Node, ParseError, Parser, Text, parse_str};
