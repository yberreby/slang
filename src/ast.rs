#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    Ref(Ident),
    Arith(Op, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Num(i32),
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnaryOp {
    Not,
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lte,
    Gte,
    And,
    Or,
    Lt,
    Gt,
    Ne,
    BinOr,
    BinAnd,
    LShift,
    RShift,
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ident(pub String);


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Decl {
    Let(Ident, Expr)
}

pub struct Func(pub Ident, pub Vec<ParamDecl>, pub Vec<Statement>);

pub enum Statement {
    Empty
}

// "type name"
pub struct ParamDecl {
    pub typ: Ident,
    pub name: Ident,
}
