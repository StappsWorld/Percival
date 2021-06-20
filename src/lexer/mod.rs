lazy_static::lazy_static! {
    pub static ref FLOAT_RE: Regex = Regex::new(r"[-+]?[0-9]*\.[0-9]+([eE][-+]?[0-9]+)?").unwrap();
    pub static ref STRING_RE: Regex = Regex::new(r#""(?:[^"\\]|\\.)*""#).unwrap();
    pub static ref CHARS_RE: Regex = Regex::new(r#"'(?:[^"\\]|\\.)*'"#).unwrap();
}

use regex::Regex;

pub use self::{
    err::LexicalError,
    pos::{Location, Spanned},
    tok::Token,
};

pub mod err;
pub mod pos;
pub mod tok;

fn is_ident_start(ch: &char) -> bool {
    match ch {
        '_' | 'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}
fn is_ident_continue(ch: &char) -> bool {
    match ch {
        '0'..='9' => true,
        ch => is_ident_start(ch),
    }
}

fn is_digit(ch: &char) -> bool {
    ch.is_digit(10)
}

fn is_hex(ch: &char) -> bool {
    ch.is_digit(16)
}

fn is_float(s: &String) -> bool {
    FLOAT_RE.is_match(s)
}

fn is_operator_char(ch: &char) -> bool {
    "`-%|+/&^*=!><".contains(&ch.to_string())
}

fn is_kw_char(ch: &char) -> bool {
    "dplwnubhsfeatxric".contains(&ch.to_string())
}

fn is_punctuation(ch: &char) -> bool {
    ",;{}()[]".contains(&ch.to_string())
}

pub struct Lexer {
    text: String,
    location: Location,
    seen_first: bool,
    last: Option<Token>,
    types: Vec<String>,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            text: input.to_string(),
            location: Location {
                line: 0.into(),
                column: 0.into(),
                absolute: 0.into(),
            },
            seen_first: false,
            last: None,
            types: vec![],
        }
    }

    fn current(&self) -> Option<char> {
        self.lookahead_offset(0)
    }

    fn lookahead(&self) -> Option<char> {
        self.lookahead_offset(1)
    }
    fn lookahead_offset(&self, offset: usize) -> Option<char> {
        self.text
            .chars()
            .nth(self.location.absolute.to_usize() + offset)
    }

    fn lookahead_eq(&self, ch: char, offset: usize) -> bool {
        self.lookahead_offset(offset)
            .map(|next| next == ch)
            .unwrap_or_default()
    }

    fn operator(&self, start: Location, ch: char) -> Option<Spanned> {
        match ch {
            ',' => Some(Spanned {
                tok: Token::Comma,
                start,
                end: start,
            }),
            ';' => Some(Spanned {
                tok: Token::Semicolon,
                start,
                end: start,
            }),
            '{' => Some(Spanned {
                tok: Token::OpenCurly,
                start,
                end: start,
            }),
            '}' => Some(Spanned {
                tok: Token::CloseCurly,
                start,
                end: start,
            }),
            '(' => Some(Spanned {
                tok: Token::OpenParen,
                start,
                end: start,
            }),
            ')' => Some(Spanned {
                tok: Token::CloseParen,
                start,
                end: start,
            }),
            '[' => Some(Spanned {
                tok: Token::OpenSquare,
                start,
                end: start,
            }),
            ']' => Some(Spanned {
                tok: Token::CloseSquare,
                start,
                end: start,
            }),
            '`' => Some(Spanned {
                tok: Token::OpPower,
                start,
                end: start,
            }),
            '>' if self.lookahead_eq('>', 1) => Some(Spanned {
                tok: Token::OpShr,
                start,
                end: start.add(1),
            }),
            '<' if self.lookahead_eq('<', 1) => Some(Spanned {
                tok: Token::OpShl,
                start,
                end: start.add(1),
            }),
            '*' => Some(Spanned {
                tok: Token::OpStar,
                start,
                end: start,
            }),
            '/' => Some(Spanned {
                tok: Token::OpDiv,
                start,
                end: start,
            }),
            '%' => Some(Spanned {
                tok: Token::OpMod,
                start,
                end: start,
            }),
            '&' => Some(Spanned {
                tok: Token::OpBAnd,
                start,
                end: start,
            }),
            '|' => Some(Spanned {
                tok: Token::OpBXor,
                start,
                end: start,
            }),
            '^' => Some(Spanned {
                tok: Token::OpBOr,
                start,
                end: start,
            }),
            '+' => Some(Spanned {
                tok: Token::OpAdd,
                start,
                end: start,
            }),
            '-' => Some(Spanned {
                tok: Token::OpSub,
                start,
                end: start,
            }),
            '<' => Some(Spanned {
                tok: Token::OpLt,
                start,
                end: start,
            }),
            '>' => Some(Spanned {
                tok: Token::OpGt,
                start,
                end: start,
            }),
            '>' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpLe,
                start,
                end: start.add(1),
            }),
            '<' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpGe,
                start,
                end: start.add(1),
            }),
            '=' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpEq,
                start,
                end: start.add(1),
            }),
            '!' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpNe,
                start,
                end: start.add(1),
            }),
            '&' if self.lookahead_eq('&', 1) => Some(Spanned {
                tok: Token::OpAnd,
                start,
                end: start.add(1),
            }),
            '|' if self.lookahead_eq('|', 1) => Some(Spanned {
                tok: Token::OpXor,
                start,
                end: start.add(1),
            }),
            '^' if self.lookahead_eq('^', 1) => Some(Spanned {
                tok: Token::OpOr,
                start,
                end: start.add(1),
            }),
            '=' => Some(Spanned {
                tok: Token::OpAssign,
                start,
                end: start,
            }),
            '<' if self.lookahead_eq('<', 1) && self.lookahead_eq('=', 2) => Some(Spanned {
                tok: Token::OpAShl,
                start,
                end: start.add(2),
            }),
            '>' if self.lookahead_eq('>', 1) && self.lookahead_eq('=', 2) => Some(Spanned {
                tok: Token::OpAShr,
                start,
                end: start.add(2),
            }),
            '*' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpAMul,
                start,
                end: start.add(1),
            }),
            '/' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpADiv,
                start,
                end: start.add(1),
            }),
            '&' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpABAnd,
                start,
                end: start.add(1),
            }),
            '|' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpABOr,
                start,
                end: start.add(1),
            }),
            '^' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpABXor,
                start,
                end: start.add(1),
            }),
            '+' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpAAdd,
                start,
                end: start.add(1),
            }),
            '-' if self.lookahead_eq('=', 1) => Some(Spanned {
                tok: Token::OpASub,
                start,
                end: start.add(1),
            }),
            '!' => Some(Spanned {
                tok: Token::OpNot,
                start,
                end: start,
            }),
            '&' => Some(Spanned {
                tok: Token::OpRef,
                start,
                end: start,
            }),
            '-' if self.lookahead_eq('>', 1) => Some(Spanned {
                tok: Token::OpIndirectMemberAccess,
                start,
                end: start.add(1),
            }),
            '.' => Some(Spanned {
                tok: Token::OpMemberAccess,
                start,
                end: start,
            }),
            _ => None,
        }
    }

    fn number(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut s = ch.to_string();
        s = self
            .text
            .chars()
            .skip(self.location.absolute.to_usize())
            .take_while(|ch| is_digit(ch) || "eE+-.".contains(&ch.to_string()))
            .fold(s, |mut s, ch| {
                s.push(ch);
                s
            });

        self.location = self.location.add(s.len() - 1);

        if is_float(&s) {
            Some(Spanned {
                start,
                end: self.location,
                tok: Token::Float(s.parse::<f64>().unwrap()),
            })
        } else if s.starts_with(vec!['-', '+'].as_slice()) {
            Some(Spanned {
                start,
                end: self.location,
                tok: Token::Signed(s.parse::<i64>().unwrap()),
            })
        } else {
            Some(Spanned {
                start,
                end: self.location,
                tok: Token::Unsigned(s.parse::<u64>().unwrap()),
            })
        }
    }

    fn identifier(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut ident_str = ch.to_string();
        ident_str = self
            .text
            .chars()
            .skip(self.location.absolute.to_usize())
            .take_while(is_ident_continue)
            .fold(ident_str, |mut ident_str, ch| {
                ident_str.push(ch);
                ident_str
            });

        let len = ident_str.len();
        self.location = self.location.add(len - 1);

        match ident_str.as_str() {
            "if" => Some(Spanned {
                start,
                tok: Token::KwIf,
                end: self.location,
            }),
            "else" => Some(Spanned {
                start,
                tok: Token::KwElse,
                end: self.location,
            }),
            "return" => Some(Spanned {
                start,
                tok: Token::KwReturn,
                end: self.location,
            }),
            "class" => Some(Spanned {
                start,
                tok: Token::KwClass,
                end: self.location,
            }),
            "define" => Some(Spanned {
                start,
                tok: Token::KwDefine,
                end: self.location,
            }),
            "extern" => Some(Spanned {
                start,
                tok: Token::KwExtern,
                end: self.location,
            }),
            "while" => Some(Spanned {
                start,
                tok: Token::KwWhile,
                end: self.location,
            }),
            "public" => Some(Spanned {
                start,
                tok: Token::KwPublic,
                end: self.location,
            }),
            "NULL" => Some(Spanned {
                start,
                tok: Token::KwNull,
                end: self.location,
            }),
            "TRUE" => Some(Spanned {
                start,
                tok: Token::KwTrue,
                end: self.location,
            }),
            "FALSE" => Some(Spanned {
                start,
                tok: Token::KwFalse,
                end: self.location,
            }),
            "lastclass" => Some(Spanned {
                start,
                tok: Token::KwLastClass,
                end: self.location,
            }),

            "U0" => Some(Spanned {
                start,
                tok: Token::TypeIdent("U0".into()),
                end: self.location,
            }),
            "I8" => Some(Spanned {
                start,
                tok: Token::TypeIdent("I8".into()),
                end: self.location,
            }),
            "U8" => Some(Spanned {
                start,
                tok: Token::TypeIdent("U8".into()),
                end: self.location,
            }),
            "I16" => Some(Spanned {
                start,
                tok: Token::TypeIdent("I16".into()),
                end: self.location,
            }),
            "U16" => Some(Spanned {
                start,
                tok: Token::TypeIdent("U16".into()),
                end: self.location,
            }),
            "I32" => Some(Spanned {
                start,
                tok: Token::TypeIdent("I32".into()),
                end: self.location,
            }),
            "U32" => Some(Spanned {
                start,
                tok: Token::TypeIdent("U32".into()),
                end: self.location,
            }),
            "I64" => Some(Spanned {
                start,
                tok: Token::TypeIdent("I64".into()),
                end: self.location,
            }),
            "U64" => Some(Spanned {
                start,
                tok: Token::TypeIdent("U64".into()),
                end: self.location,
            }),
            "F64" => Some(Spanned {
                start,
                tok: Token::TypeIdent("F64".into()),
                end: self.location,
            }),
            _ => {
                if let Some(Token::KwClass) = self.last {
                    self.types.push(ident_str.clone());
                    Some(Spanned {
                        start,
                        tok: Token::TypeIdent(ident_str),
                        end: self.location,
                    })
                } else {
                    Some(Spanned {
                        start,
                        tok: if !self.types.contains(&ident_str) {
                            Token::Identifier(ident_str)
                        } else {
                            Token::TypeIdent(ident_str)
                        },
                        end: self.location,
                    })
                }
            }
        }
    }

    fn punct(&mut self, start: Location, ch: char) -> Option<Spanned> {
        match ch {
            ',' => Some(Spanned {
                start,
                tok: Token::Comma,
                end: start.add(1),
            }),
            ';' => Some(Spanned {
                start,
                tok: Token::Semicolon,
                end: start.add(1),
            }),
            '{' => Some(Spanned {
                start,
                tok: Token::OpenCurly,
                end: start.add(1),
            }),
            '}' => Some(Spanned {
                start,
                tok: Token::CloseCurly,
                end: start.add(1),
            }),
            '(' => Some(Spanned {
                start,
                tok: Token::OpenParen,
                end: start.add(1),
            }),
            ')' => Some(Spanned {
                start,
                tok: Token::CloseParen,
                end: start.add(1),
            }),
            '[' => Some(Spanned {
                start,
                tok: Token::OpenSquare,
                end: start.add(1),
            }),
            ']' => Some(Spanned {
                start,
                tok: Token::CloseSquare,
                end: start.add(1),
            }),
            _ => None,
        }
    }

    fn string_literal(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut s = ch.to_string();
        for ch in self.text.chars().skip(self.location.absolute.to_usize()) {
            self.location = self.location.shift(ch);
            s.push(ch);
            if STRING_RE.is_match(&s) {
                break;
            }
        }

        if !STRING_RE.is_match(&s) {
            None
        } else {
            Some(Spanned {
                start,
                tok: Token::String((&s).trim_matches('"').to_owned()),
                end: self.location,
            })
        }
    }
    fn chars_literal(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut s = ch.to_string();
        for ch in self.text.chars().skip(self.location.absolute.to_usize()) {
            self.location = self.location.shift(ch);
            s.push(ch);
            if CHARS_RE.is_match(&s) {
                break;
            }
        }

        if !CHARS_RE.is_match(&s) {
            None
        } else {
            Some(Spanned {
                start,
                tok: Token::Chars((&s).trim_matches('\'').to_owned().as_bytes().to_vec()),
                end: self.location,
            })
        }
    }

    fn bump(&mut self) -> Option<(Location, char)> {
        match self.current() {
            Some(ch) => {
                if (self.location.absolute.to_usize() == 0 && !self.seen_first) {
                    self.seen_first = true;
                } else {
                    self.location = self.location.shift(ch);
                }
                Some((self.location, ch))
            }
            None => None,
        }
    }
}
impl Iterator for Lexer {
    type Item = Result<FlattenedSpanned, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch)) = self.bump() {
            let result = match ch {
                ch if is_operator_char(&ch) => Some(
                    self.operator(start, ch)
                        .ok_or(LexicalError::ExpectedOperator(start, ch)),
                ),
                ch if is_digit(&ch) || "+-".contains(ch) => Some(
                    self.number(start, ch)
                        .ok_or(LexicalError::InvalidDigit(start)),
                ),
                ch if ch.is_whitespace() => continue,
                ch if is_ident_start(&ch) => Some(
                    self.identifier(start, ch)
                        .ok_or(LexicalError::ExpectedIdentOrKw(start)),
                ),
                ch if is_punctuation(&ch) => Some(
                    self.punct(start, ch)
                        .ok_or(LexicalError::ExpectedPunctuation(start)),
                ),
                '"' => Some(
                    self.string_literal(start, ch)
                        .ok_or(LexicalError::InvalidString(start)),
                ),
                '\'' => Some(
                    self.chars_literal(start, ch)
                        .ok_or(LexicalError::InvalidChars(start)),
                ),
                _ => None,
            }
            .map(|res| res.map(Spanned::into_flattened));

            if let Some(Ok((_, tok, _))) = result.clone() {
                self.last = Some(tok)
            } else {
                self.last = None
            }

            return result;
        }
        None
    }
}

pub type FlattenedSpanned = (Location, Token, Location);
impl Spanned {
    fn into_flattened(self) -> FlattenedSpanned {
        (self.start, self.tok, self.end)
    }
}
