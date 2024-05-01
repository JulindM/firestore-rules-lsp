use chumsky::Parser;

use super::{errors::SimpleCharError, expression_parser::expression};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ServiceDefinition(String, Box<Expr>),
    ServiceBody(Vec<Expr>),
    Match(Box<Expr>, Vec<Expr>),
    Path(Vec<Expr>),
    PathPart(String),
    EvalPathPart(String),
    Variable(String),
    VariableDef(String, Box<FireExpression>),
    SingleSegWildPath(String),
    RecursiveWildPath(String),
    Allow(Vec<AllowMethod>, Box<Expr>),
    ConditionalAllow(Box<FireExpression>),
    FunctionSig(String, Vec<String>),
    FunctionDecl(Box<Expr>, Box<Expr>),
    FunctionBody(Vec<Expr>, Box<Expr>),
    ConditionalAnd(Box<Expr>, Box<Expr>),
    ConditionalOr(Box<Expr>, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, usize, Box<Expr>),
    ArithmeticOp(Box<Expr>, Box<Expr>, ArithmeticOp),
    RelationOp(Box<Expr>, Box<Expr>, RelationOp),
    Return(Box<FireExpression>),
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

#[derive(Clone, PartialEq)]
pub struct FireExpression {
    content: String,
    parsed_content: Option<Result<Expr, Vec<SimpleCharError>>>,
}

impl std::fmt::Debug for FireExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FireExpression")
            .field("content", &self.content)
            .field("parsed_content", &self.parsed_content)
            .finish()
    }
}

impl FireExpression {
    pub fn new(content_in: String) -> FireExpression {
        FireExpression {
            content: content_in,
            parsed_content: None,
        }
    }

    pub fn parse_content(&mut self) {
        self.parsed_content = Some(expression().debug("Expr").parse(self.content.clone()));
    }
}
