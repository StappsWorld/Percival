use super::expression::Expression;

pub type Identifier = String;

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    String(LiteralString),
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
}

#[derive(Debug)]
pub enum LiteralString {
    Raw(String),
    Format {
        literal: String,
        args: Vec<Argument>,
    },
}
impl LiteralString {
    pub fn format<S: ToString>(literal: S, args: Vec<Argument>) -> Self {
        Self::Format {
            literal: literal.to_string(),
            args,
        }
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
