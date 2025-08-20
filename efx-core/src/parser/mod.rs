pub use crate::parser::error::ParseError;
pub use crate::parser::nodes::{Node, Element, Text, Interpolation, Attr};
pub use crate::parser::parser::Parser;

pub mod error;
mod span_range;
pub mod nodes;
mod tok;
mod lexer;
pub mod parser;

/// Top-level utility: parse DSL source string into AST
pub fn parse_str(src: &str) -> PResult<Vec<Node>> {
    let mut p = Parser::new(src);
    p.parse_nodes()
}

pub type PResult<T> = Result<T, ParseError>;
