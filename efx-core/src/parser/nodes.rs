use crate::parser::span_range::SpanRange;

/// AST Mini-XML Nodes
#[derive(Debug, Clone)]
pub enum Node {
    /// Element: <name attr="...">children...</name> или <name .../>
    Element(Element),
    /// Text node (after normalizing escapes {{ → {, }} → })
    Text(Text),
    /// Interpolation of a Rust expression from curly braces: { expr }
    I11n(Interpolation),
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attr>,
    pub children: Vec<Node>,
    pub span: SpanRange,
}

#[derive(Debug, Clone)]
pub struct Attr {
    pub name: String,
    pub value: String,
    pub span: SpanRange,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub value: String,
    pub span: SpanRange,
}

#[derive(Debug, Clone)]
pub struct Interpolation {
    /// Raw expression fragment
    pub expr_src: String,
    pub span: SpanRange,
}
