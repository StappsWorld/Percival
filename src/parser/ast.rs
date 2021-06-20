use crate::lexer::{
    tok::{Keyword, Operator},
    Token,
};

#[derive(Debug, Clone)]
pub enum Identifier {
    String(String),
    Pointer(Box<Identifier>),
    Type(String),
}
impl Identifier {
    pub fn pointer(self) -> Self {
        Self::Pointer(Box::new(self))
    }

    pub fn maybe_pointer<T>(self, maybe: Option<T>) -> Self {
        if maybe.is_some() {
            self.pointer()
        } else {
            self
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Identifier(Identifier),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Chars(Vec<u8>),
    Keyword(Keyword),
}

#[derive(Debug)]
pub enum Expr {
    Infix {
        lhs: Box<Expr>,
        op: Operator,
        rhs: Box<Expr>,
    },
    Prefix {
        op: Operator,
        rhs: Box<Expr>,
    },
    Postfix {
        lhs: Box<Expr>,
        op: Operator,
    },
    Value(Value),
    Identifier(Identifier),
    Index(Box<Expr>, Box<Expr>),
    MemberAccess(Box<Expr>, Identifier),
    IndirectMemberAccess(Box<Expr>, Identifier),
    FunctionCall(FunctionCall),
}

impl Expr {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn infix(lhs: Box<Expr>, op: Token, rhs: Box<Expr>) -> Self {
        Self::Infix {
            lhs,
            op: op.expect_operator(),
            rhs,
        }
    }
    pub fn prefix(op: Token, rhs: Box<Expr>) -> Self {
        Self::Prefix {
            op: op.expect_operator(),
            rhs,
        }
    }
    pub fn postfix(lhs: Box<Expr>, op: Token) -> Self {
        Self::Postfix {
            lhs,
            op: op.expect_operator(),
        }
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub ident: Identifier,
    pub args: Vec<Option<Box<Expr>>>,
}

#[derive(Debug)]
pub enum Statement {
    Expr(Box<Expr>),
    Assignment {
        ty: Identifier,
        ident: Identifier,
        value: Option<Box<Expr>>,
    },
    FunctionDeclaration {
        ty: Identifier,
        ident: Identifier,
        args: Vec<FunctionDeclarationArgument>,
        body: Box<Statement>,
    },
    Class(ClassDefinition),
    ExternClass(Identifier),
    Compound(Vec<Box<Statement>>),
    Return(Box<Expr>),
    For {
        init: Option<Box<Expr>>,
        condition: Option<Box<Expr>>,
        each: Option<Box<Expr>>,
        body: Option<Box<Statement>>,
    },
    If {
        condition: Box<Expr>,
        body: Box<Statement>,
        otherwise: Option<Box<Else>>,
    },
}
impl Statement {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug)]
pub struct Else {
    pub condition: Option<Box<Expr>>,
    pub body: Box<Statement>,
    pub otherwise: Option<Box<Else>>,
}

#[derive(Debug)]
pub struct FunctionDeclarationArgument {
    pub ty: Identifier,
    pub ident: Identifier,
    pub default_value: Option<Box<Expr>>,
}

#[derive(Debug)]
pub struct ClassDefinition {
    pub ident: Identifier,
    pub fields: Vec<ClassField>,
}

#[derive(Debug)]
pub struct ClassField {
    pub ty: Identifier,
    pub name: Identifier,
    pub default_value: Option<Box<Expr>>,
}
