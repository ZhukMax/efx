use crate::ast::span_range::SpanRange;
use std::fmt;

/// Parser error with human-readable message and range
#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub span: SpanRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at bytes {}..{}",
            self.msg, self.span.start.0, self.span.end.0
        )
    }
}
