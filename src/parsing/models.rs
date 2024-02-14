#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ServiceDefinition(String, Box<Expr>),
    ServiceBody(Vec<Expr>),
    Match(Box<Expr>, Vec<Expr>),
    Path(Vec<Expr>),
    VarPath(String),
    Variable(String),
    VariableDef(Box<Expr>, Box<Expr>),
    SingleSegWildPath(String),
    RecursiveWildPath(String),
    Allow(AllowMethod, Box<Expr>),
    ConditionalAllow(Box<Expr>),
    FunctionSig(String, Vec<Expr>),
    FunctionDecl(Box<Expr>, Box<Expr>),
    FunctionBody(Vec<Expr>, Box<Expr>),
    LogicExpression(Box<Expr>, Box<Expr>, LogicOp),
    ArithmeticExpr(Box<Expr>, Box<Expr>, ArithmeticOp),
    Return(Box<Expr>),
    AllAllow,

    //Placeholder during development
    Dummy(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicOp {
    And,
    Or,
    Eq,
    Ineq,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mult,
    Div,
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
