use crate::ast::PResult;
use crate::ast::error::ParseError;
use crate::ast::lexer::Lexer;
use crate::ast::nodes::{Attr, Element, Interpolation, Node, Text};
use crate::ast::span_range::SpanRange;
use crate::ast::tok::Tok;

pub struct Parser<'a> {
    toks: Vec<(Tok, SpanRange)>,
    i: usize,
    _src: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            toks: Lexer::new(src).all(),
            i: 0,
            _src: src,
        }
    }

    pub fn parse_nodes(&mut self) -> PResult<Vec<Node>> {
        let mut nodes = Vec::new();
        while !self.eof() {
            match self.cur() {
                Some((Tok::LAngle, _)) => nodes.push(self.parse_element()?),
                Some((Tok::LBrace, _)) => nodes.push(self.parse_i11n()?),
                Some((Tok::Text(_), _)) => nodes.push(self.parse_text()?),
                Some((tok, sp)) => {
                    return Err(ParseError {
                        msg: format!("unexpected token: {:?}", tok),
                        span: *sp,
                    });
                }
                None => break,
            }
        }

        Ok(nodes)
    }

    fn eof(&self) -> bool {
        self.i >= self.toks.len()
    }
    fn cur(&self) -> Option<&(Tok, SpanRange)> {
        self.toks.get(self.i)
    }
    fn bump(&mut self) {
        self.i += 1;
    }

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

        let span = self
            .cur()
            .map(|t| t.1)
            .unwrap_or_else(|| SpanRange::new(self.byte_end(), self.byte_end()));
        Err(ParseError {
            msg: format!("expected {:?}", want),
            span,
        })
    }

    fn parse_name(&mut self) -> PResult<(String, SpanRange)> {
        match self.cur().cloned() {
            Some((Tok::Text(s), sp)) => {
                let mut name_end = s.len();
                for (idx, ch) in s.char_indices() {
                    if ch.is_whitespace() || matches!(ch, '<' | '>' | '/' | '{' | '}') {
                        name_end = idx;
                        break;
                    }
                }
                if name_end == 0 {
                    return Err(ParseError {
                        msg: "invalid tag name".into(),
                        span: sp,
                    });
                }

                let name = s[..name_end].to_string();

                self.bump();

                let remainder = &s[name_end..];
                if !remainder.is_empty() {
                    let rem_start = sp.start.0 + name_end;
                    // Insert directly at the current position of self.i - the next call to cur() will see it
                    self.toks.insert(
                        self.i,
                        (
                            Tok::Text(remainder.to_string()),
                            SpanRange::new(rem_start, sp.end.0),
                        ),
                    );
                }

                let name_span = SpanRange::new(sp.start.0, sp.start.0 + name_end);
                Ok((name, name_span))
            }
            Some((_tok, sp)) => Err(ParseError {
                msg: "expected tag name".into(),
                span: sp,
            }),
            None => Err(ParseError {
                msg: "unexpected EOF when reading tag name".into(),
                span: SpanRange::new(self.byte_end(), self.byte_end()),
            }),
        }
    }

    fn parse_element(&mut self) -> PResult<Node> {
        let start = self.expect(Tok::LAngle)?.start;
        let (name, _nsp) = self.parse_name()?;

        // Collect the "tail" after the name until '>' or '/>'
        let mut attrs_src = String::new();
        let mut attrs_span_start = None;
        let mut attrs_span_end = None;

        loop {
            match self.cur() {
                Some((Tok::Slash, _)) => {
                    return if let Some((Tok::RAngle, sp_gt)) = self.toks.get(self.i + 1).cloned() {
                        self.bump(); // '/'
                        self.bump(); // '>'

                        let end = sp_gt.end;
                        let attrs = parse_attrs_from_buffer(
                            &attrs_src,
                            SpanRange {
                                start: attrs_span_start.unwrap_or(start),
                                end: attrs_span_end.unwrap_or(start),
                            },
                        )?;
                        Ok(Node::Element(Element {
                            name,
                            attrs,
                            children: vec![],
                            span: SpanRange { start, end },
                        }))
                    } else {
                        let sp = self.cur().unwrap().1;
                        Err(ParseError {
                            msg: "unexpected '/' in tag head".into(),
                            span: sp,
                        })
                    };
                }
                Some((Tok::RAngle, sp_gt)) => {
                    // Closed the opening tag: now parse the children or end empty
                    let end_open = *sp_gt;
                    self.bump();

                    let attrs = parse_attrs_from_buffer(
                        &attrs_src,
                        SpanRange {
                            start: attrs_span_start.unwrap_or(start),
                            end: attrs_span_end.unwrap_or(end_open.end),
                        },
                    )?;

                    let mut children = Vec::new();
                    loop {
                        match self.cur() {
                            Some((Tok::LAngle, _)) => {
                                if let Some((Tok::Slash, _)) = self.toks.get(self.i + 1) {
                                    // </name>
                                    self.bump(); // '<'
                                    self.bump(); // '/'
                                    let (close_name, sp_name) = self.parse_name()?;
                                    if close_name != name {
                                        return Err(ParseError {
                                            msg: format!(
                                                "unmatched closing tag: expected </{}>",
                                                name
                                            ),
                                            span: sp_name,
                                        });
                                    }
                                    let end_angle = self.expect(Tok::RAngle)?;
                                    return Ok(Node::Element(Element {
                                        name,
                                        attrs,
                                        children,
                                        span: SpanRange {
                                            start,
                                            end: end_angle.end,
                                        },
                                    }));
                                } else {
                                    children.push(self.parse_element()?);
                                }
                            }
                            Some((Tok::LBrace, _)) => children.push(self.parse_i11n()?),
                            Some((Tok::Text(_), _)) => children.push(self.parse_text()?),
                            Some((tok, sp)) => {
                                return Err(ParseError {
                                    msg: format!("unexpected token in element body: {:?}", tok),
                                    span: *sp,
                                });
                            }
                            None => {
                                return Err(ParseError {
                                    msg: "unexpected EOF in element body".into(),
                                    span: SpanRange::new(self.byte_end(), self.byte_end()),
                                });
                            }
                        }
                    }
                }
                Some((Tok::Text(s), sp)) => {
                    // There can be spaces and attributes between the name and '>'
                    if attrs_span_start.is_none() {
                        attrs_span_start = Some(sp.start);
                    }
                    attrs_span_end = Some(sp.end);
                    attrs_src.push_str(s);
                    self.bump();
                }
                Some((tok, sp)) => {
                    return Err(ParseError {
                        msg: format!("unexpected token in tag head: {:?}", tok),
                        span: *sp,
                    });
                }
                None => {
                    return Err(ParseError {
                        msg: "unexpected EOF in tag head".into(),
                        span: SpanRange::new(self.byte_end(), self.byte_end()),
                    });
                }
            }
        }
    }

    fn parse_text(&mut self) -> PResult<Node> {
        let mut start = None;
        let mut end = None;
        let mut buf = String::new();

        while let Some((Tok::Text(s), sp)) = self.cur().cloned() {
            if start.is_none() {
                start = Some(sp.start);
            }
            end = Some(sp.end);
            self.bump();
            buf.push_str(&s);
        }

        if let (Some(st), Some(en)) = (start, end) {
            Ok(Node::Text(Text {
                value: buf,
                span: SpanRange { start: st, end: en },
            }))
        } else {
            let span = self
                .cur()
                .map(|t| t.1)
                .unwrap_or_else(|| SpanRange::new(self.byte_end(), self.byte_end()));
            Err(ParseError {
                msg: "expected text".into(),
                span,
            })
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
                    return Ok(Node::I11n(Interpolation {
                        expr_src,
                        span: SpanRange { start, end: sp.end },
                    }));
                }
                Some((Tok::Text(s), _)) => {
                    expr_src.push_str(&s);
                    self.bump();
                }
                Some((Tok::LBrace, sp)) => {
                    // Nested { } are not supported (let's simplify the first version)
                    return Err(ParseError {
                        msg: "nested '{' in interpolation".into(),
                        span: sp,
                    });
                }
                Some((_tok, sp)) => {
                    return Err(ParseError {
                        msg: "unexpected token inside { }".into(),
                        span: sp,
                    });
                }
                None => {
                    return Err(ParseError {
                        msg: "unexpected EOF in { }".into(),
                        span: SpanRange::new(self.byte_end(), self.byte_end()),
                    });
                }
            }
        }
    }
}

fn parse_attrs_from_buffer(buf: &str, span: SpanRange) -> PResult<Vec<Attr>> {
    let mut attrs = Vec::new();
    let bytes = buf.as_bytes();
    let mut i = 0usize;

    let skip_ws = |b: &[u8], mut j: usize| {
        while j < b.len() && b[j].is_ascii_whitespace() {
            j += 1;
        }
        j
    };
    let is_name_char =
        |c: u8| -> bool { c.is_ascii_alphanumeric() || c == b'_' || c == b'-' || c == b':' };

    i = skip_ws(bytes, i);
    while i < bytes.len() {
        let name_start = i;
        while i < bytes.len() && is_name_char(bytes[i]) {
            i += 1;
        }
        if i == name_start {
            // no name - maybe only spaces are left
            i = skip_ws(bytes, i);
            if i >= bytes.len() {
                break;
            }

            return Err(ParseError {
                msg: "invalid attribute name".into(),
                span,
            });
        }

        let name = &buf[name_start..i];
        i = skip_ws(bytes, i);
        if i >= bytes.len() || bytes[i] != b'=' {
            return Err(ParseError {
                msg: format!("expected '=' after attribute '{}'", name),
                span,
            });
        }
        i += 1;
        i = skip_ws(bytes, i);

        if i >= bytes.len() || (bytes[i] != b'"' && bytes[i] != b'\'') {
            return Err(ParseError {
                msg: format!("expected quote after '=' in attribute '{}'", name),
                span,
            });
        }
        let quote = bytes[i];
        i += 1;
        let val_start = i;
        while i < bytes.len() && bytes[i] != quote {
            i += 1;
        }
        if i >= bytes.len() {
            return Err(ParseError {
                msg: format!("unterminated quoted value for attribute '{}'", name),
                span,
            });
        }
        let value = &buf[val_start..i];
        i += 1; // closing quote

        attrs.push(Attr {
            name: name.to_string(),
            value: value.to_string(),
            span,
        });

        i = skip_ws(bytes, i);
    }

    Ok(attrs)
}
