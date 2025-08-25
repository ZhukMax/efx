pub mod attr;
pub mod parser;

pub use parser::{Attr, Element, Interpolation, Node, ParseError, Parser, Text, parse_str};
