use crate::ast::span_range::SpanRange;
use crate::ast::tok::Tok;

pub(crate) struct Lexer<'a> {
    src: &'a str,
    i: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(src: &'a str) -> Self {
        Self { src, i: 0 }
    }

    pub(crate) fn all(mut self) -> Vec<(Tok, SpanRange)> {
        let mut v = Vec::new();
        while let Some(t) = self.next_tok() {
            v.push(t);
        }

        v
    }

    fn eof(&self) -> bool {
        self.i >= self.src.len()
    }

    fn peek(&self) -> Option<char> {
        self.src[self.i..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        if self.eof() {
            return None;
        }

        let mut it = self.src[self.i..].char_indices();
        let (_, ch) = it.next().unwrap();
        let next_i = it.next().map(|(o, _)| self.i + o).unwrap_or(self.src.len());
        self.i = next_i;
        Some(ch)
    }

    /// Reads the sequence up to the nearest special character, processing escapes {{ and }}.
    /// NOTE: allow '/' inside attribute values; only treat '/>' as a boundary.
    fn read_text(&mut self) -> String {
        let mut out = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                '<' | '>' | '{' | '}' => break,
                '/' => {
                    // lookahead: only stop on `/>`
                    let mut it = self.src[self.i..].char_indices();
                    let _ = it.next(); // consume '/'
                    if let Some((_, next)) = it.next() {
                        if next == '>' { break; }
                    }
                    out.push(self.bump().unwrap());
                }
                _ => {
                    out.push(self.bump().unwrap());
                }
            }
        }
        out
    }

    fn next_tok(&mut self) -> Option<(Tok, SpanRange)> {
        if self.eof() {
            return None;
        }
        let start = self.i;

        match self.peek().unwrap() {
            '<' => {
                self.bump();
                Some((Tok::LAngle, SpanRange::new(start, self.i)))
            }
            '>' => {
                self.bump();
                Some((Tok::RAngle, SpanRange::new(start, self.i)))
            }
            '/' => {
                self.bump();
                Some((Tok::Slash, SpanRange::new(start, self.i)))
            }
            '{' => {
                self.bump();
                if self.peek() == Some('{') {
                    self.bump();
                    Some((Tok::Text("{".to_string()), SpanRange::new(start, self.i)))
                } else {
                    Some((Tok::LBrace, SpanRange::new(start, self.i)))
                }
            }
            '}' => {
                self.bump();
                if self.peek() == Some('}') {
                    self.bump();
                    Some((Tok::Text("}".to_string()), SpanRange::new(start, self.i)))
                } else {
                    Some((Tok::RBrace, SpanRange::new(start, self.i)))
                }
            }
            _ => {
                let s = self.read_text();
                Some((Tok::Text(s), SpanRange::new(start, self.i)))
            }
        }
    }
}
