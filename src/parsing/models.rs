#[derive(Debug)]
pub enum Expr {
    ServiceDefinition(String, Box<Expr>),
    ServiceBody(Vec<Expr>),
    Match(Box<Expr>, Vec<Expr>),
    Path(Vec<Expr>),
    VarPath(String),
    SingleSegWildPath(String),
    RecursiveWildPath(String),

    Rule(String),
}
