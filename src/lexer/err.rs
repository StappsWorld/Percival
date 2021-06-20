use super::Location;

#[derive(Debug, Clone)]
pub enum LexicalError {
    ExpectedOperator(Location, char),
    InvalidDigit(Location),
    ExpectedIdentOrKw(Location),
    ExpectedPunctuation(Location),
    InvalidString(Location),
    InvalidChars(Location),
}
