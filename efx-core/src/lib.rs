pub mod parser;

pub use parser::{
    parse_str, Parser, Node, Element, Text, Interpolation, ParseError,
};
