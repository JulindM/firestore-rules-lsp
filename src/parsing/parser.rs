use chumsky::{prelude::*, text::*};

use crate::parsing::models::{ArithmeticOp, RelationOp, UnaryOp};

use super::{
    errors::{gen_error, SimpleCharError},
    models::{AllowMethod, Expr},
};

pub fn generate_parser() -> impl Parser<char, Expr, Error = SimpleCharError> {
    let semicolon = just(";").debug("semicolon").boxed();

    let colon = just(":").debug("colon").boxed();

    let comment = whitespace()
        .then(just("/"))
        .then(just("/"))
        .then(take_until(newline().rewind()))
        .map(|_| Expr::Comment)
        .boxed();

    macro_rules! statement {
        ($p: expr) => {
            $p.then_ignore(semicolon.clone())
        };
    }

    let dotted_ident = ident()
        .separated_by(just("."))
        .collect::<Vec<String>>()
        .map(|parts| parts.join("."))
        .debug("dotted_ident")
        .boxed();

    let block_start = just("{").then(newline()).debug("block_start").boxed();

    let block_end = newline()
        .or_not()
        .then(whitespace())
        .then(just("}"))
        .debug("block_end")
        .boxed();

    let pathspec = ident().debug("path_spec").boxed();

    let path_part = pathspec
        .clone()
        .map(Expr::PathPart)
        .debug("path_part")
        .boxed();

    let eval_path_part = dotted_ident
        .clone()
        .delimited_by(just("$("), just(")"))
        .map(Expr::EvalPathPart)
        .debug("eval_path")
        .boxed();

    let single_seg_wild_path = pathspec
        .clone()
        .delimited_by(just("{"), just("}"))
        .map(Expr::SingleSegWildPath)
        .debug("single_segment_wildcard")
        .boxed();

    let rec_wild_path = pathspec
        .clone()
        .then_ignore(just("="))
        .then_ignore(just("*"))
        .then_ignore(just("*"))
        .delimited_by(just("{"), just("}"))
        .map(Expr::RecursiveWildPath)
        .debug("wildcard_pattern")
        .boxed();

    let path_separator = just("/").debug("path_separator").boxed();

    let rule_path = choice([path_part.clone(), single_seg_wild_path, rec_wild_path])
        .separated_by(path_separator.clone())
        .allow_leading()
        .at_least(1)
        .collect()
        .map(Expr::Path)
        .debug("rule_path")
        .boxed();

    let accessible_path = choice([path_part, eval_path_part])
        .separated_by(path_separator)
        .allow_leading()
        .at_least(1)
        .collect()
        .map(Expr::Path)
        .debug("accessible_path")
        .boxed();

    let newline_separator = newline().debug("whitespace_delimiter").boxed();

    let addition_op = choice([
        just("+").to(ArithmeticOp::Add),
        just("-").to(ArithmeticOp::Sub),
    ])
    .debug("addition_op")
    .boxed();

    let multiplication_op = choice([
        just("*").to(ArithmeticOp::Mult),
        just("/").to(ArithmeticOp::Div),
        just("%").to(ArithmeticOp::Mod),
    ])
    .debug("mutliplication_op")
    .boxed();

    let relation_op = choice([
        just(">").then(just("=")).to(RelationOp::GreaterEq).boxed(),
        just(">").to(RelationOp::Greater).boxed(),
        just("<").then(just("=")).to(RelationOp::LessEq).boxed(),
        just("<").to(RelationOp::Less).boxed(),
        just("!").then(just("=")).to(RelationOp::Ineq).boxed(),
        just("=").then(just("=")).to(RelationOp::Eq).boxed(),
        keyword("in").to(RelationOp::In).boxed(),
    ])
    .debug("relation op")
    .boxed();

    let assignment = just("=").debug("assignment").boxed();

    let number = just("-")
        .to(-1)
        .or_not()
        .to(1)
        .then(int(10).try_map(|s: String, span| {
            s.parse::<i32>()
                .map_err(|_| gen_error("not a valid integer")(span))
        }))
        .map(|(negation, digits)| Expr::Number(negation * digits))
        .debug("number")
        .boxed();

    let stringval = choice([
        just("\"").ignore_then(take_until(just("\""))),
        just("\'").ignore_then(take_until(just("\'"))),
    ])
    .debug("string")
    .map(|(strval, _)| strval)
    .collect::<String>()
    .map(Expr::String)
    .boxed();

    let literal = choice([number, stringval.clone()]).debug("literal").boxed();

    let mut value_expression = Recursive::declare();
    let mut op_expression = Recursive::declare();

    let expression = choice([op_expression.clone(), value_expression.clone()]).boxed();

    value_expression.define(
        {
            let variable = ident().debug("variable").map(Expr::Variable).boxed();

            let function_args = choice([
                value_expression.clone().boxed(),
                op_expression.clone().boxed(),
                accessible_path,
            ])
            .clone()
            .separated_by(just(",").padded())
            .map(Expr::ExprList)
            .boxed();
            let function_call = ident()
                .then(choice([
                    just("(").then(just(")")).to(Expr::ExprList(vec![])).boxed(),
                    function_args
                        .clone()
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
            .clone()
            .separated_by(just(",").padded())
            .delimited_by(just("["), just("]"))
            .map(|exprsns| Expr::Array(exprsns))
            .debug("array")
            .boxed();

            let primary = choice([
                function_call.clone(),
                variable.clone(),
                array.clone(),
                literal.clone(),
            ])
            .debug("primary")
            .boxed();

            let member = primary
                .clone()
                .then(
                    choice([
                        just(".")
                            .ignore_then(choice([function_call.clone(), variable.clone()]))
                            .boxed(),
                        choice([op_expression.clone(), value_expression.clone()])
                            .clone()
                            .delimited_by(just("["), just("]"))
                            .boxed(),
                    ])
                    .repeated()
                    .at_least(1),
                )
                .foldl(|memb, exprsn| Expr::Member(Box::new(memb), Box::new(exprsn)))
                .debug("member")
                .boxed();

            choice([function_call, member, variable, array, literal])
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
                    multiplication_op
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
                    addition_op
                        .padded()
                        .then(add_operand.clone())
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
                    relation_op
                        .padded()
                        .then(rel_operand.clone())
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
                        .ignore_then(and_operand.clone())
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
                        .ignore_then(or_operand.clone())
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
                .then(expression_operand.clone())
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

    let let_statement = {
        let let_content = keyword("let")
            .padded()
            .ignore_then(ident().clone().map_err(gen_error("variable name expected")))
            .then_ignore(assignment.padded().map_err(gen_error("equals expected")))
            .then(expression.clone().map_err(gen_error("expression expected")))
            .map(|(var_name, expr)| Expr::VariableDef(var_name, Box::new(expr.clone())))
            .boxed();

        statement!(let_content).debug("let_statement").boxed()
    };

    let return_statement = {
        let return_content = keyword("return")
            .padded()
            .ignore_then(expression.clone().map_err(gen_error("expression expected")))
            .debug("return")
            .boxed();

        statement!(return_content).debug("return_statement").boxed()
    };

    let function_body = let_statement
        .separated_by(newline_separator.clone())
        .or_not()
        .then(return_statement)
        .map(|(stmts, ret)| {
            Expr::FunctionBody(
                stmts.unwrap_or(vec![]),
                Box::new(Expr::Return(Box::new(ret))),
            )
        })
        .debug("function_body")
        .boxed();

    let function_signature = ident()
        .then(
            ident()
                .clone()
                .separated_by(just(",").padded())
                .delimited_by(just("("), just(")"))
                .padded()
                .or_not()
                .map(|vars| vars.unwrap_or(vec![]))
                .or_not(),
        )
        .map_err(gen_error(
            "function signature expected (call or definition)",
        ))
        .map(|(fname, fargs)| Expr::FunctionSig(fname, fargs.unwrap_or(vec![])))
        .debug("function_signature")
        .boxed();

    let function_declaration = (keyword("function").padded())
        .ignore_then(function_signature)
        .then(
            function_body
                .padded()
                .delimited_by(block_start.clone(), block_end.clone()),
        )
        .map(|(fsig, fbody)| Expr::FunctionDecl(Box::new(fsig), Box::new(fbody)))
        .debug("function_declaration")
        .boxed();

    let allow_content = (keyword("if").padded().map_err(gen_error("if expected")))
        .ignore_then(expression)
        .map(|c| Expr::ConditionalAllow(Box::new(c)))
        .debug("allow_content")
        .boxed();

    let methods = choice([
        keyword("read").to(AllowMethod::Read),
        keyword("write").to(AllowMethod::Write),
        keyword("get").to(AllowMethod::Get),
        keyword("list").to(AllowMethod::List),
        keyword("create").to(AllowMethod::Create),
        keyword("update").to(AllowMethod::Update),
        keyword("delete").to(AllowMethod::Delete),
    ])
    .separated_by(just(",").padded())
    .debug("method")
    .boxed();

    let allow = (keyword("allow").padded())
        .ignore_then(choice([
            statement!(methods.clone())
                .map(|m| (m, Expr::AllAllow))
                .boxed(),
            statement!(methods.then_ignore(colon).then(allow_content)).boxed(),
        ]))
        .map(|(meth, con)| Expr::Allow(meth, Box::new(con)))
        .debug("allow_rule")
        .boxed();

    let match_declaration = whitespace()
        .then(keyword("match"))
        .then(whitespace())
        .ignore_then(rule_path)
        .debug("match_declaration")
        .boxed();

    let match_block = recursive(|expr| {
        match_declaration
            .then_ignore(whitespace().then(block_start.clone()))
            .then(
                choice([comment.clone(), allow, function_declaration, expr.boxed()])
                    .separated_by(newline_separator.clone())
                    .at_least(1)
                    .boxed()
                    // There has to be a smarter way than validating
                    // but I cant seem to be smart enough to find it
                    .validate(|exprsns, span, emit| {
                        let has_allow = exprsns.iter().any(|s| match s {
                            Expr::Match(_, _) => true,
                            Expr::Allow(_, _) => true,
                            Expr::AllAllow => true,
                            _ => false,
                        });

                        if !has_allow {
                            emit(gen_error(
                                "An allow block must have at least 1 allow rule or match block",
                            )(span))
                        };
                        exprsns
                    }),
            )
            .then_ignore(block_end.clone())
            .map(|(path, exprsns)| Expr::Match(Box::new(path), exprsns))
    })
    .debug("match_block")
    .boxed();

    let service_body = choice([comment, match_block])
        .separated_by(newline_separator)
        .collect()
        .debug("service_body")
        .map(Expr::ServiceBody)
        .boxed();

    let service_decl = keyword("service")
        .ignore_then(dotted_ident.padded())
        .then(service_body.delimited_by(block_start, block_end))
        .map(|(name, body)| Expr::ServiceDefinition(name, Box::new(body)))
        .debug("service_definition")
        .boxed();

    let rules_parsing = service_decl.then_ignore(end());

    rules_parsing
}

pub fn parse_debug(stream: Vec<char>) -> Option<Expr> {
    let (res, errors) = generate_parser().parse_recovery_verbose(stream);

    print!("{:?}", errors);

    res
}

pub fn parse(stream: Vec<char>) -> Result<Expr, Vec<SimpleCharError>> {
    generate_parser().parse(stream)
}
