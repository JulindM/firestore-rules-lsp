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
    LogicExpression(Box<Expr>, Box<Expr>, BinaryLogicOp),
    UnaryLogicExpression(Box<Expr>, UnaryLogicOp),
    ArithmeticExpr(Box<Expr>, Box<Expr>, ArithmeticOp),
    Return(Box<Expr>),
    AllAllow,
    Nested(Box<Expr>, Box<Expr>, NestedOperator),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryLogicOp {
    And,
    Or,
    Eq,
    Ineq,
    Greater,
    GreaterEq,
    In,
    Is,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryLogicOp {
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NestedOperator {
    FieldAcess,
    Indexing,
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
