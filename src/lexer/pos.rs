use crate::lexer::tok::Token;
use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

macro_rules! pos_struct {
    (#[$doc:meta] pub struct $Pos:ident($T:ty);) => {
        #[$doc]
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $Pos($T);

        impl fmt::Debug for $Pos {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl $Pos {
            pub fn to_usize(&self) -> usize {
                self.0 as usize
            }
        }

        impl From<usize> for $Pos {
            fn from(src: usize) -> $Pos {
                $Pos(src as $T)
            }
        }

        impl Add for $Pos {
            type Output = $Pos;

            fn add(self, rhs: $Pos) -> $Pos {
                $Pos::from(self.to_usize() + rhs.to_usize())
            }
        }

        impl AddAssign for $Pos {
            fn add_assign(&mut self, rhs: $Pos) {
                self.0 += rhs.0;
            }
        }

        impl Sub for $Pos {
            type Output = $Pos;

            fn sub(self, rhs: $Pos) -> $Pos {
                $Pos::from(self.to_usize() - rhs.to_usize())
            }
        }

        impl SubAssign for $Pos {
            fn sub_assign(&mut self, rhs: $Pos) {
                self.0 -= rhs.0;
            }
        }
    };
}

pos_struct! {
    /// A byte offset in a source string
    pub struct BytePos(u32);
}

impl fmt::Display for BytePos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pos_struct! {
    /// A `0`-indexed column number, displayed externally as if it were offset from `1`.
    pub struct Column(u32);
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0 + 1).fmt(f)
    }
}

pos_struct! {
    /// A `0`-indexed line number, displayed externally as if it were offset from `1`.
    pub struct Line(u32);
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0 + 1).fmt(f)
    }
}

/// A location in a source file
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
pub struct Location {
    pub line: Line,
    pub column: Column,
    pub absolute: BytePos,
}

impl Location {
    pub fn shift(mut self, ch: char) -> Location {
        if ch == '\n' {
            self.line += Line::from(1);
            self.column = Column::from(1);
        } else {
            self.column += Column::from(1);
        }
        self.absolute += BytePos::from(ch.len_utf8());
        self
    }
    pub fn add(mut self, num_bytes: usize) -> Location {
        self.column += Column(num_bytes as u32);
        self.absolute += BytePos(num_bytes as u32);
        self
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Column: {}", self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct Spanned {
    pub start: Location,
    pub tok: Token,
    pub end: Location,
}
impl Spanned {
    pub fn nop(start: Location, end: Location) -> Self {
        Self {
            start,
            end,
            tok: Token::NOP,
        }
    }
}
