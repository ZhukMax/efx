/// Byte position in DSL source
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(pub usize);

/// Byte range [start, end]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanRange { pub start: Pos, pub end: Pos }

impl SpanRange {
    pub fn new(start: usize, end: usize) -> Self { Self { start: Pos(start), end: Pos(end) } }
}
