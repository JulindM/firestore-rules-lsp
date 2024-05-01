use std::time::Instant;

use chumsky::{
    prelude::*,
    primitive::{choice, just},
    text::{ident, whitespace},
    BoxedParser, Parser,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::{
    errors::SimpleCharError,
    helper_parsers::*,
    models::{EvalExpr, EvalExpression, RuleExpr, UnaryOp},
};

pub fn firestore_expression() -> BoxedParser<'static, char, EvalExpression, SimpleCharError> {
    take_until(semicolon().rewind())
        .map(|(content, _)| content)
        .collect::<String>()
        .map(|res| EvalExpression::new(res))
        .boxed()
}

pub fn expression() -> impl Parser<char, EvalExpr, Error = SimpleCharError> {
    let mut value_expression = Recursive::declare();
    let mut op_expression = Recursive::declare();

    value_expression.define(
        {
            let function_args = choice([
                op_expression.clone().boxed(),
                value_expression.clone().boxed(),
                accessible_path(),
            ])
            .separated_by(just(",").padded())
            .map(EvalExpr::ExprList)
            .boxed();
            let function_call = ident()
                .then(choice([
                    just("(")
                        .then(just(")"))
                        .to(EvalExpr::ExprList(vec![]))
                        .boxed(),
                    function_args
                        .padded()
                        .delimited_by(just("("), just(")"))
                        .boxed(),
                ]))
                .map(|(fname, exprlst)| EvalExpr::FunctionCall(fname, Box::new(exprlst)))
                .debug("function_call")
                .boxed();

            let array = choice([
                just("[")
                    .then(just("]"))
                    .to(EvalExpr::Array(vec![]))
                    .boxed(),
                value_expression.clone().boxed(),
            ])
            .separated_by(just(",").padded())
            .delimited_by(just("["), just("]"))
            .map(|exprsns| EvalExpr::Array(exprsns))
            .debug("array")
            .boxed();

            let primary = choice([array.clone(), function_call.clone(), variable(), literal()])
                .debug("primary")
                .boxed();

            let member = primary
                .then(
                    choice([
                        just(".")
                            .ignore_then(choice([function_call.clone(), variable()]))
                            .boxed(),
                        choice([op_expression.clone(), value_expression.clone()])
                            .delimited_by(just("["), just("]"))
                            .boxed(),
                    ])
                    .repeated()
                    .at_least(1),
                )
                .foldl(|memb, exprsn| EvalExpr::Member(Box::new(memb), Box::new(exprsn)))
                .debug("member")
                .boxed();

            choice([array, function_call, member, variable(), literal()])
        }
        .debug("value expression")
        .boxed(),
    );

    op_expression.define(
        {
            let expression_operand = choice([op_expression.clone(), value_expression.clone()]);

            let grouping = expression_operand
                .clone()
                .padded()
                .delimited_by(just("("), just(")").padded())
                .map(|exprssn| EvalExpr::Atom(Box::new(exprssn)))
                .debug("grouping")
                .boxed();

            let unary_operand = choice([grouping.clone(), expression_operand.clone().boxed()]);
            let unary = choice([
                just("!")
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
                    .then(unary_operand.clone())
                    .map(|(ops, memb)| {
                        EvalExpr::Unary(UnaryOp::NegExclamation, ops.len(), Box::new(memb))
                    })
                    .boxed(),
                just("-")
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
                    .then(unary_operand)
                    .map(|(ops, memb)| {
                        EvalExpr::Unary(UnaryOp::DecrementMinus, ops.len(), Box::new(memb))
                    })
                    .boxed(),
            ])
            .debug("unary operation")
            .boxed();

            let mult_operand = choice([
                grouping.clone(),
                unary.clone(),
                value_expression.clone().boxed(),
            ])
            .boxed();

            let multiplication = mult_operand
                .clone()
                .then(
                    multiplication_op()
                        .padded()
                        .then(mult_operand.clone())
                        .repeated()
                        .at_least(1),
                )
                .foldl(|o1, (op, o2)| EvalExpr::ArithmeticOp(Box::new(o1), Box::new(o2), op))
                .debug("multiplication")
                .boxed();

            let add_operand = choice([
                grouping.clone(),
                unary.clone(),
                multiplication.clone(),
                value_expression.clone().boxed(),
            ])
            .boxed();
            let addition = add_operand
                .clone()
                .then(
                    addition_op()
                        .padded()
                        .then(add_operand)
                        .repeated()
                        .at_least(1),
                )
                .foldl(|o1, (op, o2)| EvalExpr::ArithmeticOp(Box::new(o1), Box::new(o2), op))
                .debug("addition")
                .boxed();

            let rel_operand = choice([
                unary.clone(),
                grouping.clone(),
                addition.clone(),
                multiplication.clone(),
                value_expression.clone().boxed(),
            ]);
            let relation = rel_operand
                .clone()
                .then(
                    relation_op()
                        .padded()
                        .then(rel_operand)
                        .repeated()
                        .at_least(1),
                )
                .foldl(|o1, (op, o2)| EvalExpr::RelationOp(Box::new(o1), Box::new(o2), op))
                .debug("relation")
                .boxed();

            let and_operand = choice([
                unary.clone(),
                grouping.clone(),
                relation.clone(),
                addition.clone(),
                multiplication.clone(),
                value_expression.clone().boxed(),
            ]);
            let conditional_and = and_operand
                .clone()
                .then(
                    whitespace()
                        .then(just("&").then(just("&")))
                        .then(whitespace())
                        .ignore_then(and_operand)
                        .repeated()
                        .at_least(1),
                )
                .foldl(|o1, o2| EvalExpr::ConditionalAnd(Box::new(o1), Box::new(o2)))
                .debug("conditional_and")
                .boxed();

            let or_operand = choice([
                unary.clone(),
                grouping.clone(),
                relation.clone(),
                addition.clone(),
                multiplication.clone(),
                conditional_and.clone(),
                value_expression.clone().boxed(),
            ]);
            let conditional_or = or_operand
                .clone()
                .then(
                    whitespace()
                        .then(just("|").then(just("|")))
                        .then(whitespace())
                        .ignore_then(or_operand)
                        .repeated()
                        .at_least(1),
                )
                .foldl(|o1, o2| EvalExpr::ConditionalOr(Box::new(o1), Box::new(o2)))
                .debug("conditional_or")
                .boxed();

            let ternary = conditional_or
                .clone()
                .then_ignore(just("?").padded())
                .then(conditional_or.clone())
                .then_ignore(just(":").padded())
                .then(expression_operand)
                .map(|((o1, o2), o3)| EvalExpr::Ternary(Box::new(o1), Box::new(o2), Box::new(o3)))
                .debug("ternary")
                .boxed();

            choice([
                grouping,
                unary,
                relation,
                addition,
                multiplication,
                conditional_or,
                conditional_and,
                ternary,
            ])
        }
        .debug("op_expression")
        .boxed(),
    );

    choice([op_expression, value_expression]).boxed()
}

pub fn recrsv_par_evalexpr_parse(expression: RuleExpr) -> RuleExpr {
    let start = Instant::now();

    let res = match expression {
        RuleExpr::VariableDef(v, mut fexpr) => {
            fexpr.parse();
            RuleExpr::VariableDef(v, fexpr)
        }
        RuleExpr::Return(mut fexpr) => {
            fexpr.parse();
            RuleExpr::Return(fexpr)
        }
        RuleExpr::ConditionalAllow(mut fexpr) => {
            fexpr.parse();
            RuleExpr::ConditionalAllow(fexpr)
        }
        RuleExpr::ServiceDefinition(v, expr) => {
            RuleExpr::ServiceDefinition(v, Box::new(recrsv_par_evalexpr_parse(*expr)))
        }
        RuleExpr::ServiceBody(exprsns) => RuleExpr::ServiceBody(
            exprsns
                .into_par_iter()
                .map(recrsv_par_evalexpr_parse)
                .collect(),
        ),
        RuleExpr::Match(v, exprsns) => RuleExpr::Match(
            v,
            exprsns
                .into_par_iter()
                .map(recrsv_par_evalexpr_parse)
                .collect(),
        ),
        RuleExpr::Path(exprsns) => RuleExpr::Path(
            exprsns
                .into_par_iter()
                .map(recrsv_par_evalexpr_parse)
                .collect(),
        ),
        RuleExpr::Allow(v, expr) => RuleExpr::Allow(v, Box::new(recrsv_par_evalexpr_parse(*expr))),
        RuleExpr::FunctionDecl(e1, e2) => RuleExpr::FunctionDecl(
            Box::new(recrsv_par_evalexpr_parse(*e1)),
            Box::new(recrsv_par_evalexpr_parse(*e2)),
        ),
        RuleExpr::FunctionBody(exprsns, expr) => RuleExpr::FunctionBody(
            exprsns
                .into_par_iter()
                .map(recrsv_par_evalexpr_parse)
                .collect(),
            Box::new(recrsv_par_evalexpr_parse(*expr)),
        ),
        expr => expr,
    };

    let elapsed_time = start.elapsed().as_secs();

    if elapsed_time > 2 {
        println!("-------");
        println!("Worker done in {:?}s", elapsed_time);
        println!("The offending epxr:\n${:?}", res.clone());
        println!("-------");
    }

    return res;
}
