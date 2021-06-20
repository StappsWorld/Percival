use super::{
    ast::{FunctionCall, Identifier, Type, Value},
    operators::Operator,
};

#[derive(Debug)]
pub enum Expression {
    Operation {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
    Cast(Type),
    UnaryOperation(Operator, Box<Expression>),
    Value(Value),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    Index(Box<Expression>, Box<Expression>),
    IndirectMemberAccess(Box<Expression>, Identifier),
    MemberAccess(Box<Expression>, Identifier),
    Type(Type),
}
impl Expression {
    pub fn operation(lhs: Box<Expression>, op: Operator, rhs: Box<Expression>) -> Self {
        Self::Operation { lhs, op, rhs }
    }

    pub fn one() -> Box<Self> {
        Box::new(Self::Value(Value::Unsigned(1)))
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn string_literal(s: String) -> Self {
        Self::Value(Value::String(s))
    }

    pub fn chars_literal(chars: Vec<u8>) -> Self {
        Self::Value(Value::Chars(chars))
    }
}
