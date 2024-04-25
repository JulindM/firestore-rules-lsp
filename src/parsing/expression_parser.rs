use chumsky::{
    prelude::*,
    primitive::{choice, just},
    text::{ident, whitespace},
    BoxedParser, Parser,
};

use super::{
    errors::SimpleCharError,
    helper_parsers::*,
    models::{Expr, UnaryOp},
};

pub fn firestore_expression() -> BoxedParser<'static, char, Expr, SimpleCharError> {
    let mut value_expression = Recursive::declare();
    let mut op_expression = Recursive::declare();

    value_expression.define(
        {
            let function_args = choice([
                value_expression.clone().boxed(),
                op_expression.clone().boxed(),
                accessible_path(),
            ])
            .separated_by(just(",").padded())
            .map(Expr::ExprList)
            .boxed();
            let function_call = ident()
                .then(choice([
                    just("(").then(just(")")).to(Expr::ExprList(vec![])).boxed(),
                    function_args
                        .padded()
                        .delimited_by(just("("), just(")"))
                        .boxed(),
                ]))
                .map(|(fname, exprlst)| Expr::FunctionCall(fname, Box::new(exprlst)))
                .debug("function_call")
                .boxed();

            let array = choice([
                just("[").then(just("]")).to(Expr::Array(vec![])).boxed(),
                value_expression.clone().boxed(),
            ])
            .separated_by(just(",").padded())
            .delimited_by(just("["), just("]"))
            .map(|exprsns| Expr::Array(exprsns))
            .debug("array")
            .boxed();

            let primary = choice([function_call.clone(), variable(), array.clone(), literal()])
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
                .foldl(|memb, exprsn| Expr::Member(Box::new(memb), Box::new(exprsn)))
                .debug("member")
                .boxed();

            choice([function_call, member, variable(), array, literal()])
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
                .map(|exprssn| Expr::Atom(Box::new(exprssn)))
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
                        Expr::Unary(UnaryOp::NegExclamation, ops.len(), Box::new(memb))
                    })
                    .boxed(),
                just("-")
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
                    .then(unary_operand)
                    .map(|(ops, memb)| {
                        Expr::Unary(UnaryOp::DecrementMinus, ops.len(), Box::new(memb))
                    })
                    .boxed(),
            ])
            .debug("unary operation")
            .boxed();

            let mult_operand = choice([
                unary.clone(),
                grouping.clone(),
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
                .foldl(|o1, (op, o2)| Expr::ArithmeticOp(Box::new(o1), Box::new(o2), op))
                .debug("multiplication")
                .boxed();

            let add_operand = choice([
                multiplication.clone(),
                unary.clone(),
                grouping.clone(),
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
                .foldl(|o1, (op, o2)| Expr::ArithmeticOp(Box::new(o1), Box::new(o2), op))
                .debug("addition")
                .boxed();

            let rel_operand = choice([
                addition.clone(),
                multiplication.clone(),
                unary.clone(),
                grouping.clone(),
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
                .foldl(|o1, (op, o2)| Expr::RelationOp(Box::new(o1), Box::new(o2), op))
                .debug("relation")
                .boxed();

            let and_operand = choice([
                relation.clone(),
                addition.clone(),
                multiplication.clone(),
                unary.clone(),
                grouping.clone(),
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
                .foldl(|o1, o2| Expr::ConditionalAnd(Box::new(o1), Box::new(o2)))
                .debug("conditional_and")
                .boxed();

            let or_operand = choice([
                conditional_and.clone(),
                relation.clone(),
                addition.clone(),
                multiplication.clone(),
                unary.clone(),
                grouping.clone(),
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
                .foldl(|o1, o2| Expr::ConditionalOr(Box::new(o1), Box::new(o2)))
                .debug("conditional_or")
                .boxed();

            let ternary = conditional_or
                .clone()
                .then_ignore(just("?").padded())
                .then(conditional_or.clone())
                .then_ignore(just(":").padded())
                .then(expression_operand)
                .map(|((o1, o2), o3)| Expr::Ternary(Box::new(o1), Box::new(o2), Box::new(o3)))
                .debug("ternary")
                .boxed();

            choice([
                ternary,
                conditional_or,
                conditional_and,
                relation,
                addition,
                multiplication,
                unary,
                grouping,
            ])
        }
        .debug("op_expression")
        .boxed(),
    );

    choice([op_expression, value_expression]).boxed()
}
