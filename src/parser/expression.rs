use super::{
    ast::{FunctionCall, Identifier, Value},
    operators::Operator,
};

#[derive(Debug)]
pub enum Expression {
    Operation {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
    Value(Value),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
    Index(Box<Expression>, Box<Expression>),
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
}
