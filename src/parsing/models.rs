#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ServiceDefinition(String, Box<Expr>),
    ServiceBody(Vec<Expr>),
    Match(Box<Expr>, Vec<Expr>),
    Path(Vec<Expr>),
    VarPath(String),
    Variable(String),
    SingleSegWildPath(String),
    RecursiveWildPath(String),
    Allow(AllowMethod, Box<Expr>),
    ConditionalAllow(Box<Expr>),
    FunctionDecl(String, Vec<Expr>, Box<Expr>),
    FunctionBody(Vec<Expr>, Box<Expr>),
    Return(Box<Expr>),
    AllAllow,

    //Placeholder during development
    Dummy(String),
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
