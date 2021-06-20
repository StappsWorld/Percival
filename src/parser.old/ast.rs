#![allow(non_snake_case)]

use super::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    String(String),
    Pointer(Box<Identifier>),
    Array(Box<Identifier>, u64),
}
impl Identifier {
    pub fn pointer(self) -> Self {
        Self::Pointer(Box::new(self))
    }
    pub fn array(self, index: u64) -> Self {
        Self::Array(Box::new(self), index)
    }
}
impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Chars(Vec<u8>),
    Null,
    LastClass,
}

#[derive(Debug)]
pub enum Type {
    Void,
    Signed(usize),
    Unsigned(usize),
    Float,
    Pointer(Box<Type>),
    Identifier(Identifier),
}
impl Type {
    pub fn pointer(self) -> Self {
        Self::Pointer(Box::new(self))
    }
}

#[derive(Debug)]
pub enum Directive {
    Define(Identifier, Box<Expression>),
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
    pub args: Vec<Option<Box<Expression>>>,
}
impl FunctionCall {
    pub fn Print(args: Vec<Box<Expression>>) -> Self {
        Self {
            identifier: "Print".into(),
            args: args.into_iter().map(|a| Some(a)).collect(),
        }
    }

    pub fn PutChars(chars: Box<Expression>) -> Self {
        Self {
            identifier: "PutChars".into(),
            args: vec![Some(chars)],
        }
    }
}

#[derive(Debug)]
pub enum Extern {
    Class(Identifier),
    Identifier(Identifier),
}

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Declaration(Declaration),
    Compound(Vec<Box<Statement>>),
    For {
        assignment: Option<Box<Expression>>,
        condition: Option<Box<Expression>>,
        each: Option<Box<Expression>>,
        body: Box<Statement>,
    },
    While {
        condition: Option<Box<Expression>>,
        body: Box<Statement>,
    },
    Return(Option<Box<Expression>>),
    Condition(Box<Condition>),
    FunctionDeclaration(FunctionDeclaration),
    Comment(String),
    Directive(Directive),
    Definition(Definition),
    Extern(Extern),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub is_public: bool,
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

#[derive(Debug)]
pub struct Definition {
    pub ident: Identifier,
    pub feilds: Vec<Field>,
}
impl Definition {
    pub fn new(ident: Identifier, feilds: Vec<Field>) -> Self {
        Self { ident, feilds }
    }
}

#[derive(Debug)]
pub struct Meta {
    pub identifier: Identifier,
    pub expr: Box<Expression>,
}
impl Meta {
    pub fn new(identifier: Identifier, expr: Box<Expression>) -> Self {
        Self { identifier, expr }
    }
}

#[derive(Debug)]
pub struct Field {
    pub ty: Type,
    pub identifiers: Vec<Identifier>,
    pub initial_values: Vec<Option<Box<Expression>>>,
    pub meta: Vec<Meta>,
}
impl Field {
    pub fn new(
        ty: Type,
        identifiers: Vec<Identifier>,
        initial_values: Vec<Option<Box<Expression>>>,
        meta: Vec<Meta>,
    ) -> Self {
        Self {
            ty,
            identifiers,
            initial_values,
            meta,
        }
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub ty: Type,
    pub identifiers: Vec<Identifier>,
    pub initial_values: Vec<Option<Box<Expression>>>,
}
impl Declaration {
    pub fn new(
        ty: Type,
        identifiers: Vec<Identifier>,
        initial_values: Vec<Option<Box<Expression>>>,
    ) -> Self {
        Self {
            ty,
            identifiers,
            initial_values,
        }
    }
}
