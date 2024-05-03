use std::ops::Range;

use chumsky::{prelude::*, text::*};

use super::{
    errors::*,
    expression_parser::{firestore_expression, recrsv_par_evalexpr_parse},
    helper_parsers::*,
    models::RuleExpr,
};

pub fn generate_parser() -> impl Parser<char, RuleExpr, Error = Simple<char>> {
    let let_statement = {
        let let_content = keyword("let")
            .map_err(merge_custom_in_simple("let expected"))
            .padded()
            .ignore_then(ident().map_err(merge_custom_in_simple("variable name expected")))
            .then_ignore(assignment().padded())
            .then(firestore_expression())
            .map(|(var_name, expr)| RuleExpr::VariableDef(var_name, Box::new(expr)))
            .boxed();

        let_content.then_ignore(semicolon()).boxed()
    };

    let return_statement = {
        let return_content = keyword("return")
            .map_err(merge_custom_in_simple("return expected"))
            .padded()
            .ignore_then(firestore_expression())
            .boxed();

        return_content.then_ignore(semicolon()).boxed()
    };

    let function_body = let_statement
        .separated_by(newline_separator())
        .or_not()
        .then(return_statement)
        .map(|(stmts, ret)| {
            RuleExpr::FunctionBody(
                stmts.unwrap_or(vec![]),
                Box::new(RuleExpr::Return(Box::new(ret))),
            )
        })
        .boxed();

    let function_signature = ident()
        .map_err(custom_simple("function name expected"))
        .padded_by(inline_whitespace())
        .then(
            (ident()
                .separated_by(just(",").padded())
                .or_not()
                .delimited_by(just("("), just(")")))
            .map(|vars| vars.unwrap_or(vec![]))
            .or_not(),
        )
        .map(|(fname, fargs)| RuleExpr::FunctionSig(fname, fargs.unwrap_or(vec![])))
        .boxed();

    let function = keyword("function")
        .ignore_then(function_signature)
        .then(
            function_body
                .delimited_by(block_start(), block_end())
                .padded(),
        )
        .map(|(fsig, fbody)| RuleExpr::FunctionDecl(Box::new(fsig), Box::new(fbody)))
        .boxed();

    let allow_content = keyword("if")
        .map_err(custom_simple("if expected"))
        .padded()
        .ignore_then(firestore_expression())
        .map(|c| RuleExpr::ConditionalAllow(Box::new(c)))
        .debug("allow_content")
        .boxed();

    let allow = keyword("allow")
        .ignore_then(inline_whitespace())
        .ignore_then(
            method()
                .map_err(custom_simple("unknown rule"))
                .separated_by(just(","))
                .then(colon().ignore_then(allow_content).or_not())
                .map(|(m, expr)| match expr {
                    Some(expr) => RuleExpr::Allow(m, Box::new(expr)),
                    None => RuleExpr::AllAllow,
                })
                .boxed(),
        )
        .then_ignore(semicolon())
        .boxed();

    let match_declaration = keyword("match")
        .then(inline_whitespace())
        .ignore_then(rule_path())
        .boxed();

    let match_block = recursive(|match_expr| {
        match_declaration
            .then_ignore(whitespace().then(block_start()))
            .then(
                choice([allow, function.clone(), match_expr.boxed()])
                    .separated_by(whitespace())
                    .validate(at_least("allow, function or match expected", 1))
                    .validate(rule_existence_validator),
            )
            .then_ignore(block_end())
            .padded()
            .map(|(path, exprsns)| RuleExpr::Match(Box::new(path), exprsns))
    })
    .boxed();

    let service_body = choice([function, match_block])
        .separated_by(whitespace())
        .collect()
        .validate(at_least("allow or function expected", 1))
        .validate(rule_existence_validator)
        .map(RuleExpr::ServiceBody)
        .boxed();

    let service_decl = keyword("service")
        .map_err(custom_simple("service expected"))
        .then(inline_whitespace())
        .ignore_then(
            dotted_ident()
                .map_err(custom_simple("service name expected (cloud.firestore)"))
                .then_ignore(whitespace()),
        )
        .then(service_body.delimited_by(block_start(), block_end()))
        .padded()
        .map(|(name, body)| RuleExpr::ServiceDefinition(name, Box::new(body)));

    let rules_parsing = service_decl.then_ignore(whitespace().then(end()));

    rules_parsing
}

fn at_least(
    msg: &str,
    min_len: usize,
) -> impl Fn(Vec<RuleExpr>, Range<usize>, &mut dyn FnMut(Simple<char>)) -> Vec<RuleExpr> + '_ {
    move |exprsns, span, emit| {
        if exprsns.len() < min_len {
            let err = custom_span(msg)(span);
            emit(err)
        }

        exprsns
    }
}

fn rule_existence_validator(
    exprsns: Vec<RuleExpr>,
    span: Range<usize>,
    emit: &mut dyn FnMut(Simple<char>),
) -> Vec<RuleExpr> {
    let has_allow = exprsns.iter().any(|s| match s {
        RuleExpr::Match(_, _) => true,
        RuleExpr::Allow(_, _) => true,
        RuleExpr::AllAllow => true,
        _ => false,
    });

    if !has_allow {
        let err = custom_span("A rule block must have at least a rule or match block")(span);

        emit(err)
    };

    exprsns
}

pub fn parse(stream: Vec<char>, debug: bool) -> Result<RuleExpr, Vec<Simple<char>>> {
    let ast = {
        let rules_parser = generate_parser();

        if debug {
            let res = rules_parser.parse_recovery_verbose(stream);

            match res.0 {
                None => Err(res.1),
                _ => Ok(res.0.unwrap()),
            }
        } else {
            rules_parser.parse(stream)
        }
    };

    match ast {
        Ok(expr) => Ok(recrsv_par_evalexpr_parse(expr)),
        e => e,
    }
}
