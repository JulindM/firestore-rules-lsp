#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ServiceDefinition(String, Box<Expr>),
    ServiceBody(Vec<Expr>),
    Match(Box<Expr>, Vec<Expr>),
    Path(Vec<Expr>),
    PathPart(String),
    EvalPathPart(String),
    Variable(String),
    VariableDef(String, Box<Expr>),
    SingleSegWildPath(String),
    RecursiveWildPath(String),
    Allow(Vec<AllowMethod>, Box<Expr>),
    ConditionalAllow(Box<Expr>),
    FunctionSig(String, Vec<String>),
    FunctionDecl(Box<Expr>, Box<Expr>),
    FunctionBody(Vec<Expr>, Box<Expr>),
    ConditionalAnd(Box<Expr>, Box<Expr>),
    ConditionalOr(Box<Expr>, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, usize, Box<Expr>),
    ArithmeticOp(Box<Expr>, Box<Expr>, ArithmeticOp),
    RelationOp(Box<Expr>, Box<Expr>, RelationOp),
    Return(Box<Expr>),
    AllAllow,
    FunctionCall(String, Box<Expr>),
    Atom(Box<Expr>),
    Number(i32),
    String(String),
    Array(Vec<Expr>),
    ExprList(Vec<Expr>),
    Member(Box<Expr>, Box<Expr>),
    Comment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationOp {
    Eq,
    Ineq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    In,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    NegExclamation,
    DecrementMinus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AllowMethod {
    Read,
    Write,
    Get,
    List,
    Create,
    Update,
    Delete,
}
