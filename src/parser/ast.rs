#![allow(non_snake_case)]
use std::str::FromStr;

use super::expression::Expression;

pub type Identifier = String;

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub enum Type {
    Void,
    Signed(usize),
    Unsigned(usize),
    Float,
}

#[derive(Debug)]
pub enum Argument {
    Value(Value),
    Identifier(Identifier),
    Expression(Box<Expression>),
}
impl Argument {
    pub fn into_expression(self) -> Box<Expression> {
        match self {
            Self::Value(v) => Expression::Value(v).boxed(),
            Self::Identifier(i) => Expression::Identifier(i).boxed(),
            Self::Expression(e) => e,
        }
    }
}
impl<S: ToString> From<S> for Argument {
    fn from(s: S) -> Self {
        Self::Value(Value::String(s.to_string()))
    }
}

#[derive(Debug)]
pub struct Assignment {
    pub identifier: Identifier,
    pub value: Value,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: Identifier,
    pub args: Vec<Box<Expression>>,
}
impl FunctionCall {
    pub fn Print(args: Vec<Box<Expression>>) -> Self {
        Self {
            identifier: "Print".into(),
            args,
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Declaration {
        ty: Type,
        identifier: Identifier,
        value: Option<Box<Expression>>,
    },
    Compound(Vec<Box<Statement>>),
    Loop {
        assignment: Option<Box<Expression>>,
        condition: Option<Box<Expression>>,
        each: Option<Box<Expression>>,
        body: Box<Statement>,
    },
    Return(Box<Expression>),
    Condition(Box<Condition>),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub return_type: Type,
    pub identifier: Identifier,
    pub args: FunctionArguments,
    pub body: Vec<Box<Statement>>,
}
#[derive(Debug)]
pub enum FunctionArguments {
    Varargs,
    Args(Vec<FunctionArgument>),
}

#[derive(Debug)]
pub struct FunctionArgument {
    pub ty: Type,
    pub identifier: Identifier,
    pub default_value: Option<Value>,
}

#[derive(Debug)]
pub struct Condition {
    pub condition: Box<Expression>,
    pub body: Statement,
    pub else_statement: Option<ElseCondition>,
}

#[derive(Debug)]
pub struct ElseCondition {
    pub condition: Option<Box<Expression>>,
    pub body: Statement,
    pub else_statement: Option<Box<ElseCondition>>,
}
