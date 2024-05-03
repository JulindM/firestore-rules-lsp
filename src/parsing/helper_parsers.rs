use chumsky::{
    error::Simple,
    primitive::{choice, filter, just, take_until},
    text::{ident, int, keyword, newline, whitespace, Character},
    BoxedParser, Parser,
};

use super::{
    errors::custom_span,
    models::{AllowMethod, ArithmeticOp, EvalExpr, RelationOp, RuleExpr},
};

pub fn inline_whitespace() -> BoxedParser<'static, char, (), Simple<char>> {
    filter(|c: &char| c.is_inline_whitespace())
        .repeated()
        .ignored()
        .boxed()
}

pub fn dotted_ident() -> BoxedParser<'static, char, String, Simple<char>> {
    ident()
        .separated_by(just("."))
        .at_least(1)
        .collect::<Vec<String>>()
        .map(|parts| parts.join("."))
        .boxed()
}

pub fn semicolon() -> BoxedParser<'static, char, (), Simple<char>> {
    just(";").ignored().boxed()
}

pub fn colon() -> BoxedParser<'static, char, (), Simple<char>> {
    just(":").ignored().boxed()
}

pub fn block_start() -> BoxedParser<'static, char, (), Simple<char>> {
    just("{").then(whitespace()).ignored().boxed()
}

pub fn block_end() -> BoxedParser<'static, char, (), Simple<char>> {
    whitespace().then(just("}")).ignored().boxed()
}
pub fn variable() -> BoxedParser<'static, char, EvalExpr, Simple<char>> {
    ident().map(EvalExpr::Variable).boxed()
}

pub fn pathspec() -> BoxedParser<'static, char, String, Simple<char>> {
    ident().boxed()
}

pub fn path_part() -> BoxedParser<'static, char, RuleExpr, Simple<char>> {
    pathspec().map(RuleExpr::PathPart).boxed()
}

pub fn eval_path_part() -> BoxedParser<'static, char, RuleExpr, Simple<char>> {
    dotted_ident()
        .delimited_by(just("$("), just(")"))
        .map(RuleExpr::EvalPathPart)
        .boxed()
}
pub fn single_seg_wild_path() -> BoxedParser<'static, char, RuleExpr, Simple<char>> {
    pathspec()
        .delimited_by(just("{"), just("}"))
        .map(RuleExpr::SingleSegWildPath)
        .boxed()
}

pub fn rec_wild_path() -> BoxedParser<'static, char, RuleExpr, Simple<char>> {
    pathspec()
        .then_ignore(just("="))
        .then_ignore(just("*"))
        .then_ignore(just("*"))
        .delimited_by(just("{"), just("}"))
        .map(RuleExpr::RecursiveWildPath)
        .boxed()
}

pub fn path_separator() -> BoxedParser<'static, char, (), Simple<char>> {
    just("/").ignored().boxed()
}

pub fn rule_path() -> BoxedParser<'static, char, RuleExpr, Simple<char>> {
    choice([path_part(), single_seg_wild_path(), rec_wild_path()])
        .separated_by(path_separator())
        .allow_leading()
        .at_least(1)
        .collect()
        .map(RuleExpr::Path)
        .boxed()
}

pub fn accessible_path() -> BoxedParser<'static, char, EvalExpr, Simple<char>> {
    choice([path_part(), eval_path_part()])
        .separated_by(path_separator())
        .allow_leading()
        .at_least(1)
        .collect()
        .map(EvalExpr::AccessiblePath)
        .boxed()
}

pub fn newline_separator() -> BoxedParser<'static, char, (), Simple<char>> {
    newline().ignored().boxed()
}

pub fn addition_op() -> BoxedParser<'static, char, ArithmeticOp, Simple<char>> {
    choice([
        just("+").to(ArithmeticOp::Add),
        just("-").to(ArithmeticOp::Sub),
    ])
    .boxed()
}

pub fn multiplication_op() -> BoxedParser<'static, char, ArithmeticOp, Simple<char>> {
    choice([
        just("*").to(ArithmeticOp::Mult),
        just("/").to(ArithmeticOp::Div),
        just("%").to(ArithmeticOp::Mod),
    ])
    .boxed()
}

pub fn relation_op() -> BoxedParser<'static, char, RelationOp, Simple<char>> {
    choice([
        just(">").then(just("=")).to(RelationOp::GreaterEq).boxed(),
        just(">").to(RelationOp::Greater).boxed(),
        just("<").then(just("=")).to(RelationOp::LessEq).boxed(),
        just("<").to(RelationOp::Less).boxed(),
        just("!").then(just("=")).to(RelationOp::Ineq).boxed(),
        just("=").then(just("=")).to(RelationOp::Eq).boxed(),
        keyword("in").to(RelationOp::In).boxed(),
    ])
    .boxed()
}

pub fn assignment() -> BoxedParser<'static, char, (), Simple<char>> {
    just("=").ignored().boxed()
}

pub fn number() -> BoxedParser<'static, char, EvalExpr, Simple<char>> {
    just("-")
        .to(-1)
        .or_not()
        .to(1)
        .then(int(10).try_map(|s: String, span| {
            s.parse::<i32>()
                .map_err(|_| custom_span("not a valid integer")(span))
        }))
        .map(|(negation, digits)| EvalExpr::Number(negation * digits))
        .boxed()
}

pub fn stringval() -> BoxedParser<'static, char, EvalExpr, Simple<char>> {
    choice([
        just("\"").ignore_then(take_until(just("\""))),
        just("\'").ignore_then(take_until(just("\'"))),
    ])
    .map(|(strval, _)| strval)
    .collect::<String>()
    .map(EvalExpr::String)
    .boxed()
}

pub fn literal() -> BoxedParser<'static, char, EvalExpr, Simple<char>> {
    choice([number(), stringval()]).boxed()
}

pub fn method() -> BoxedParser<'static, char, AllowMethod, Simple<char>> {
    choice([
        keyword("read").to(AllowMethod::Read),
        keyword("write").to(AllowMethod::Write),
        keyword("get").to(AllowMethod::Get),
        keyword("list").to(AllowMethod::List),
        keyword("create").to(AllowMethod::Create),
        keyword("update").to(AllowMethod::Update),
        keyword("delete").to(AllowMethod::Delete),
    ])
    .boxed()
}
