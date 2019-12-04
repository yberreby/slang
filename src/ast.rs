#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TopLevel {
    Statement(Statement),

}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    Ref(Ident),
    Arith(Op, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Num(i32),
    StringLiteral(String),
    FnCall(Ident, Vec<Expr>),
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Func {
    pub name: Ident,
    pub params: Vec<ParamDecl>,
    pub ret_type: Option<Ident>,
    pub body: Vec<Statement>
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Statement {
    Empty,
    LocalAssignment(Ident, Expr),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
// "type name"
pub struct ParamDecl {
    pub typ: Ident,
    pub name: Ident,
}

