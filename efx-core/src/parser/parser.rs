use crate::parser::error::ParseError;
use crate::parser::lexer::Lexer;
use crate::parser::nodes::{Element, Interpolation, Node, Text};
use crate::parser::PResult;
use crate::parser::span_range::SpanRange;
use crate::parser::tok::Tok;

pub struct Parser<'a> {
    toks: Vec<(Tok, SpanRange)>,
    i: usize,
    _src: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { toks: Lexer::new(src).all(), i: 0, _src: src }
    }

    pub fn parse_nodes(&mut self) -> PResult<Vec<Node>> {
        let mut nodes = Vec::new();
        while !self.eof() {
            match self.cur() {
                Some((Tok::LAngle, _)) => nodes.push(self.parse_element()?),
                Some((Tok::LBrace, _)) => nodes.push(self.parse_i11n()?),
                Some((Tok::Text(_), _)) => nodes.push(self.parse_text()?),
                Some((tok, sp)) => return Err(ParseError { msg: format!("unexpected token: {:?}", tok), span: *sp }),
                None => break,
            }
        }

        Ok(nodes)
    }

    fn eof(&self) -> bool { self.i >= self.toks.len() }
    fn cur(&self) -> Option<&(Tok, SpanRange)> { self.toks.get(self.i) }
    fn bump(&mut self) { self.i += 1; }

    fn byte_end(&self) -> usize {
        self.toks.last().map(|t| (t.1.end).0).unwrap_or(0)
    }

    fn expect(&mut self, want: Tok) -> PResult<SpanRange> {
        if let Some((tok, sp)) = self.cur().cloned() {
            if tok == want {
                self.bump();
                return Ok(sp);
            }
        }

        let span = self.cur()
            .map(|t| t.1)
            .unwrap_or_else(|| SpanRange::new(self.byte_end(), self.byte_end()));
        Err(ParseError { msg: format!("expected {:?}", want), span })
    }

    fn parse_name(&mut self) -> PResult<(String, SpanRange)> {
        // Tag name - read as Tok::Text without spaces/special characters
        match self.cur().cloned() {
            Some((Tok::Text(s), sp)) => {
                if s.trim().is_empty() || s.chars().any(|c| c.is_whitespace() || matches!(c, '<'|'>'|'/'|'{'|'}')) {
                    return Err(ParseError { msg: "invalid tag name".into(), span: sp });
                }

                self.bump();
                Ok((s, sp))
            }
            Some((_tok, sp)) => Err(ParseError { msg: "expected tag name".into(), span: sp }),
            None => Err(ParseError {
                msg: "unexpected EOF when reading tag name".into(),
                span: SpanRange::new(self.byte_end(), self.byte_end()),
            }),
        }
    }

    fn parse_element(&mut self) -> PResult<Node> {
        let start = self.expect(Tok::LAngle)?.start;
        let (name, _nsp) = self.parse_name()?;

        match self.cur() {
            Some((Tok::Slash, _)) => {
                self.bump();
                let end_sp = self.expect(Tok::RAngle)?;
                Ok(Node::Element(Element { name, children: vec![], span: SpanRange { start, end: end_sp.end } }))
            }
            _ => {
                let _sp_gt = self.expect(Tok::RAngle)?; // close ">"
                let mut children = Vec::new();

                loop {
                    match self.cur() {
                        Some((Tok::LAngle, _)) => {
                            // Check for closing tag
                            if let Some((Tok::Slash, _)) = self.toks.get(self.i + 1) {
                                // </name>
                                self.bump(); // <
                                self.bump(); // /
                                let (close_name, sp_name) = self.parse_name()?;
                                if close_name != name {
                                    return Err(ParseError { msg: format!("unmatched closing tag: expected </{}>", name), span: sp_name });
                                }
                                let end_angle = self.expect(Tok::RAngle)?;
                                return Ok(Node::Element(Element { name, children, span: SpanRange { start, end: end_angle.end } }));
                            } else {
                                children.push(self.parse_element()?);
                            }
                        }
                        Some((Tok::LBrace, _)) => children.push(self.parse_i11n()?),
                        Some((Tok::Text(_), _)) => children.push(self.parse_text()?),
                        Some((tok, sp)) => return Err(ParseError { msg: format!("unexpected token in element body: {:?}", tok), span: *sp }),
                        None => return Err(ParseError { msg: "unexpected EOF in element body".into(), span: SpanRange::new(self.byte_end(), self.byte_end()) }),
                    }
                }
            }
        }
    }

    fn parse_text(&mut self) -> PResult<Node> {
        let mut start = None;
        let mut end = None;
        let mut buf = String::new();

        while let Some((Tok::Text(s), sp)) = self.cur().cloned() {
            if start.is_none() { start = Some(sp.start); }
            end = Some(sp.end);
            self.bump();
            buf.push_str(&s);
        }

        if let (Some(st), Some(en)) = (start, end) {
            Ok(Node::Text(Text { value: buf, span: SpanRange { start: st, end: en } }))
        } else {
            let span = self.cur()
                .map(|t| t.1)
                .unwrap_or_else(|| SpanRange::new(self.byte_end(), self.byte_end()));
            Err(ParseError { msg: "expected text".into(), span })
        }
    }

    fn parse_i11n(&mut self) -> PResult<Node> {
        let start = self.expect(Tok::LBrace)?.start;
        // Read everything up to a single RBrace (}} was already converted to Text("}"))
        let mut expr_src = String::new();

        loop {
            match self.cur().cloned() {
                Some((Tok::RBrace, sp)) => {
                    self.bump();
                    return Ok(Node::I11n(Interpolation { expr_src, span: SpanRange { start, end: sp.end } }));
                }
                Some((Tok::Text(s), _)) => { expr_src.push_str(&s); self.bump(); }
                Some((Tok::LBrace, sp)) => {
                    // Nested { } are not supported (let's simplify the first version)
                    return Err(ParseError { msg: "nested '{' in interpolation".into(), span: sp });
                }
                Some((_tok, sp)) => return Err(ParseError { msg: "unexpected token inside { }".into(), span: sp }),
                None => return Err(ParseError { msg: "unexpected EOF in { }".into(), span: SpanRange::new(self.byte_end(), self.byte_end()) }),
            }
        }
    }
}
