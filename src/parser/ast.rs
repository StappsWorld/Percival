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
}

#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Value(Value),
}

impl Expr {
    pub fn add(lhs: Expr, rhs: Expr) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }
    pub fn sub(lhs: Expr, rhs: Expr) -> Self {
        Self::Sub(Box::new(lhs), Box::new(rhs))
    }
    pub fn mul(lhs: Expr, rhs: Expr) -> Self {
        Self::Mul(Box::new(lhs), Box::new(rhs))
    }
    pub fn div(lhs: Expr, rhs: Expr) -> Self {
        Self::Div(Box::new(lhs), Box::new(rhs))
    }
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expr),
    Assignment {
        ty: Identifier,
        ident: Identifier,
        value: Expr,
    },
    Class(Identifier),
}
