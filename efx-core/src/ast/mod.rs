pub use crate::ast::error::ParseError;
pub use crate::ast::nodes::{Attr, Element, Interpolation, Node, Text};
pub use crate::ast::parser::Parser;

pub mod error;
mod lexer;
pub mod nodes;
pub mod parser;
mod span_range;
mod tok;

/// Top-level utility: parse DSL source string into AST
pub fn parse_str(src: &str) -> PResult<Vec<Node>> {
    let mut p = Parser::new(src);
    p.parse_nodes()
}

pub type PResult<T> = Result<T, ParseError>;
