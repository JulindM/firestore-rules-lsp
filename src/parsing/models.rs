use chumsky::Parser;

use super::{errors::SimpleCharError, expression_parser::expression};

#[derive(Debug, Clone, PartialEq)]
pub enum RuleExpr {
    ServiceDefinition(String, Box<RuleExpr>),
    ServiceBody(Vec<RuleExpr>),
    Match(Box<RuleExpr>, Vec<RuleExpr>),
    Path(Vec<RuleExpr>),
    PathPart(String),
    EvalPathPart(String),
    VariableDef(String, Box<EvalExpression>),
    SingleSegWildPath(String),
    RecursiveWildPath(String),
    Allow(Vec<AllowMethod>, Box<RuleExpr>),
    ConditionalAllow(Box<EvalExpression>),
    FunctionSig(String, Vec<String>),
    FunctionDecl(Box<RuleExpr>, Box<RuleExpr>),
    FunctionBody(Vec<RuleExpr>, Box<RuleExpr>),
    Return(Box<EvalExpression>),
    AllAllow,
    Comment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalExpr {
    Array(Vec<EvalExpr>),
    ExprList(Vec<EvalExpr>),
    Member(Box<EvalExpr>, Box<EvalExpr>),
    FunctionCall(String, Box<EvalExpr>),
    ConditionalAnd(Box<EvalExpr>, Box<EvalExpr>),
    ConditionalOr(Box<EvalExpr>, Box<EvalExpr>),
    Ternary(Box<EvalExpr>, Box<EvalExpr>, Box<EvalExpr>),
    Unary(UnaryOp, usize, Box<EvalExpr>),
    ArithmeticOp(Box<EvalExpr>, Box<EvalExpr>, ArithmeticOp),
    RelationOp(Box<EvalExpr>, Box<EvalExpr>, RelationOp),
    Atom(Box<EvalExpr>),
    Variable(String),
    Number(i32),
    String(String),
    AccessiblePath(Vec<RuleExpr>),
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
pub struct EvalExpression {
    content: String,
    parsed_content: Option<Result<EvalExpr, Vec<SimpleCharError>>>,
}

impl std::fmt::Debug for EvalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FireExpression")
            .field("content", &self.content)
            .field("parsed_content", &self.parsed_content)
            .finish()
    }
}

impl EvalExpression {
    pub fn new(content_in: String) -> EvalExpression {
        EvalExpression {
            content: content_in,
            parsed_content: None,
        }
    }

    pub fn parse(&mut self) {
        self.parsed_content = Some(expression().debug("Expr").parse(self.content.clone()));
    }
}
