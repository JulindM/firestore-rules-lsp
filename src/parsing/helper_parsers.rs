use chumsky::{
    primitive::{choice, filter, just, take_until},
    text::{ident, int, keyword, newline, whitespace, Character},
    BoxedParser, Parser,
};

use super::{
    errors::{gen_error, SimpleCharError},
    models::{ArithmeticOp, Expr, RelationOp},
};

pub fn inline_whitespace() -> BoxedParser<'static, char, (), SimpleCharError> {
    filter(|c: &char| c.is_inline_whitespace())
        .repeated()
        .ignored()
        .boxed()
}

pub fn dotted_ident() -> BoxedParser<'static, char, String, SimpleCharError> {
    ident()
        .separated_by(just("."))
        .collect::<Vec<String>>()
        .map(|parts| parts.join("."))
        .debug("dotted_ident")
        .boxed()
}

pub fn semicolon() -> BoxedParser<'static, char, (), SimpleCharError> {
    just(";").ignored().debug("semicolon").boxed()
}

pub fn colon() -> BoxedParser<'static, char, (), SimpleCharError> {
    just(":").ignored().debug("colon").boxed()
}

pub fn block_start() -> BoxedParser<'static, char, (), SimpleCharError> {
    just("{")
        .then(whitespace())
        .ignored()
        .debug("block_start")
        .boxed()
}

pub fn block_end() -> BoxedParser<'static, char, (), SimpleCharError> {
    whitespace()
        .then(just("}"))
        .ignored()
        .debug("block_end")
        .boxed()
}
pub fn variable() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    ident().debug("variable").map(Expr::Variable).boxed()
}

pub fn pathspec() -> BoxedParser<'static, char, String, SimpleCharError> {
    ident().debug("path_spec").boxed()
}

pub fn path_part() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    pathspec().map(Expr::PathPart).debug("path_part").boxed()
}

pub fn eval_path_part() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    dotted_ident()
        .delimited_by(just("$("), just(")"))
        .map(Expr::EvalPathPart)
        .debug("eval_path")
        .boxed()
}
pub fn single_seg_wild_path() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    pathspec()
        .delimited_by(just("{"), just("}"))
        .map(Expr::SingleSegWildPath)
        .debug("single_segment_wildcard")
        .boxed()
}

pub fn rec_wild_path() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    pathspec()
        .then_ignore(just("="))
        .then_ignore(just("*"))
        .then_ignore(just("*"))
        .delimited_by(just("{"), just("}"))
        .map(Expr::RecursiveWildPath)
        .debug("wildcard_pattern")
        .boxed()
}

pub fn path_separator() -> BoxedParser<'static, char, (), SimpleCharError> {
    just("/").ignored().debug("path_separator").boxed()
}

pub fn rule_path() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    choice([path_part(), single_seg_wild_path(), rec_wild_path()])
        .separated_by(path_separator())
        .allow_leading()
        .at_least(1)
        .collect()
        .map(Expr::Path)
        .debug("rule_path")
        .boxed()
}

pub fn accessible_path() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    choice([path_part(), eval_path_part()])
        .separated_by(path_separator())
        .allow_leading()
        .at_least(1)
        .collect()
        .map(Expr::Path)
        .debug("accessible_path")
        .boxed()
}

pub fn newline_separator() -> BoxedParser<'static, char, (), SimpleCharError> {
    newline().ignored().debug("whitespace_delimiter").boxed()
}

pub fn addition_op() -> BoxedParser<'static, char, ArithmeticOp, SimpleCharError> {
    choice([
        just("+").to(ArithmeticOp::Add),
        just("-").to(ArithmeticOp::Sub),
    ])
    .debug("addition_op")
    .boxed()
}

pub fn multiplication_op() -> BoxedParser<'static, char, ArithmeticOp, SimpleCharError> {
    choice([
        just("*").to(ArithmeticOp::Mult),
        just("/").to(ArithmeticOp::Div),
        just("%").to(ArithmeticOp::Mod),
    ])
    .debug("mutliplication_op")
    .boxed()
}

pub fn relation_op() -> BoxedParser<'static, char, RelationOp, SimpleCharError> {
    choice([
        just(">").then(just("=")).to(RelationOp::GreaterEq).boxed(),
        just(">").to(RelationOp::Greater).boxed(),
        just("<").then(just("=")).to(RelationOp::LessEq).boxed(),
        just("<").to(RelationOp::Less).boxed(),
        just("!").then(just("=")).to(RelationOp::Ineq).boxed(),
        just("=").then(just("=")).to(RelationOp::Eq).boxed(),
        keyword("in").to(RelationOp::In).boxed(),
    ])
    .debug("relation op")
    .boxed()
}

pub fn assignment() -> BoxedParser<'static, char, (), SimpleCharError> {
    just("=").ignored().debug("assignment").boxed()
}

pub fn number() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    just("-")
        .to(-1)
        .or_not()
        .to(1)
        .then(int(10).try_map(|s: String, span| {
            s.parse::<i32>()
                .map_err(|_| gen_error("not a valid integer")(span))
        }))
        .map(|(negation, digits)| Expr::Number(negation * digits))
        .debug("number")
        .boxed()
}

pub fn stringval() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    choice([
        just("\"").ignore_then(take_until(just("\""))),
        just("\'").ignore_then(take_until(just("\'"))),
    ])
    .debug("string")
    .map(|(strval, _)| strval)
    .collect::<String>()
    .map(Expr::String)
    .boxed()
}

pub fn literal() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    choice([number(), stringval()]).debug("literal").boxed()
}

pub fn comment() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    whitespace()
        .then(just("/"))
        .then(just("/"))
        .then(take_until(newline().rewind()))
        .map(|_| Expr::Comment)
        .boxed()
}
