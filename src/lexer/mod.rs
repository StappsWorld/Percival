lazy_static::lazy_static! {
    pub static ref FLOAT_RE: Regex = Regex::new(r"[-+]?[0-9]*\.[0-9]+([eE][-+]?[0-9]+)?").unwrap();
    pub static ref STRING_RE: Regex = Regex::new(r#""(?:[^"\\]|\\.)*""#).unwrap();
    pub static ref CHARS_RE: Regex = Regex::new(r#"'(?:[^"\\]|\\.)*'"#).unwrap();
}

use regex::Regex;

use crate::lexer::tok::{Keyword, Operator};

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
    "-+&.>/^|<*!%=`".contains(&ch.to_string())
}

fn is_kw_char(ch: &char) -> bool {
    "dplwnubhsfeatxric".contains(&ch.to_string())
}

fn is_keyword(s: &String) -> bool {
    [
        "if",
        "else",
        "return",
        "class",
        "define",
        "extern",
        "while",
        "for",
        "public",
        "NULL",
        "TRUE",
        "FALSE",
        "lastclass",
    ]
    .contains(&s.as_str())
}

fn is_builtin_type(s: &String) -> bool {
    [
        "U0", "I8", "U8", "I16", "U16", "I32", "U32", "I64", "U64", "F64",
    ]
    .contains(&s.as_str())
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

    fn operator(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let operators = vec![
            "`", ">>", "<<", "*", "/", "%", "^", "|", "+", "++", "-", "--", "<", ">", "<=", ">=",
            "==", "!=", "&&", "^^", "||", "=", "<<=", ">>=", "*=", "/=", "&=", "|=", "^=", "+=",
            "-=", "!", "&", "->", ".",
        ];

        let mut buffer = ch.to_string();
        let mut i = 0;
        while let Some(ch) = self.lookahead_offset(i) {
            if !is_operator_char(&ch) {
                break;
            } else {
                buffer.push(ch);
            }
            i += 1;
        }

        self.location = self.location.add(buffer.len() - 1);
        if operators.contains(&buffer.as_str()) {
            Some(Spanned {
                start,
                tok: Token::Operator(Operator::from(buffer)),
                end: self.location,
            })
        } else {
            None
        }
    }

    fn number(&mut self, start: Location, ch: char) -> Option<Spanned> {
        let mut s = self
            .text
            .chars()
            .skip(self.location.absolute.to_usize())
            .take_while(|ch| is_digit(ch) || "eE+-.".contains(&ch.to_string()))
            .fold(ch.to_string(), |mut s, ch| {
                s.push(ch);
                s
            });

        if (s.ends_with(vec!['-', '+'].as_slice())) {
            s.pop();
        }

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

        self.location = self.location.add(ident_str.len() - 1);

        if is_keyword(&ident_str) {
            Some(Spanned {
                start,
                tok: Token::Keyword(Keyword::from(ident_str)),
                end: self.location,
            })
        } else if is_builtin_type(&ident_str) {
            Some(Spanned {
                start,
                tok: Token::TypeIdent(ident_str),
                end: self.location,
            })
        } else {
            match &self.last {
                Some(Token::Keyword(Keyword::Class)) => {
                    self.types.push(ident_str.clone());
                    Some(Spanned {
                        start,
                        tok: Token::TypeIdent(ident_str),
                        end: self.location,
                    })
                }
                _ => Some(Spanned {
                    start,
                    tok: if !self.types.contains(&ident_str) {
                        Token::Identifier(ident_str)
                    } else {
                        Token::TypeIdent(ident_str)
                    },
                    end: self.location,
                }),
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
                self.location = self.location.shift(ch);
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
