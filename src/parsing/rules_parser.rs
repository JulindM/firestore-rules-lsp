use chumsky::{prelude::*, text::*};

use crate::parsing::models::AllowMethod;

use super::{
    errors::*,
    expression_parser::{firestore_expression, recursive_parallel_expr_resolve},
    helper_parsers::*,
    models::Expr,
};

pub fn generate_parser() -> impl Parser<char, Expr, Error = SimpleCharError> {
    macro_rules! statement {
        ($p: expr) => {
            $p.then_ignore(semicolon())
        };
    }

    let let_statement = {
        let let_content = keyword("let")
            .padded()
            .ignore_then(ident().map_err(gen_error("variable name expected")))
            .then_ignore(assignment().padded().map_err(gen_error("equals expected")))
            .then(firestore_expression().map_err(gen_error("expression expected")))
            .map(|(var_name, expr)| Expr::VariableDef(var_name, Box::new(expr)))
            .boxed();

        statement!(let_content).debug("let_statement").boxed()
    };

    let return_statement = {
        let return_content = keyword("return")
            .padded()
            .ignore_then(firestore_expression().map_err(gen_error("expression expected")))
            .debug("return")
            .boxed();

        statement!(return_content).debug("return_statement").boxed()
    };

    let function_body = let_statement
        .separated_by(newline_separator())
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
            (ident()
                .separated_by(just(",").padded())
                .delimited_by(just("("), just(")")))
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

    let function = (keyword("function").then_ignore(inline_whitespace()))
        .ignore_then(function_signature)
        .then(
            function_body
                .delimited_by(block_start(), block_end())
                .padded(),
        )
        .map(|(fsig, fbody)| Expr::FunctionDecl(Box::new(fsig), Box::new(fbody)))
        .debug("function_declaration")
        .boxed();

    let allow_content = (keyword("if").padded().map_err(gen_error("if expected")))
        .ignore_then(firestore_expression())
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

    let allow = (keyword("allow").then(inline_whitespace()))
        .ignore_then(choice([
            statement!(methods.clone())
                .map(|m| (m, Expr::AllAllow))
                .boxed(),
            statement!(methods.then_ignore(colon()).then(allow_content)).boxed(),
        ]))
        .map(|(meth, con)| Expr::Allow(meth, Box::new(con)))
        .debug("allow_rule")
        .boxed();

    let match_declaration = keyword("match")
        .then(inline_whitespace())
        .ignore_then(rule_path())
        .debug("match_declaration")
        .boxed();

    let match_block = recursive(|match_expr| {
        match_declaration
            .then_ignore(whitespace().then(block_start()))
            .then(
                choice([allow, function.clone(), match_expr.boxed()])
                    .separated_by(whitespace())
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
            .then_ignore(block_end())
            .padded()
            .map(|(path, exprsns)| Expr::Match(Box::new(path), exprsns))
    })
    .debug("match_block")
    .boxed();

    let service_body = choice([comment(), match_block, function])
        .separated_by(whitespace())
        .collect()
        .debug("service_body")
        .map(Expr::ServiceBody)
        .boxed();

    let service_decl = keyword("service")
        .then(inline_whitespace())
        .ignore_then(dotted_ident().then_ignore(whitespace()))
        .then(service_body.delimited_by(block_start(), block_end()))
        .padded()
        .map(|(name, body)| Expr::ServiceDefinition(name, Box::new(body)))
        .debug("service_definition")
        .boxed();

    let rules_parsing = service_decl.then_ignore(whitespace().then(end()));

    rules_parsing
}

pub fn parse(stream: Vec<char>, debug: bool) -> Result<Expr, Vec<SimpleCharError>> {
    let ast = {
        if debug {
            let res = generate_parser().parse_recovery_verbose(stream);
            Ok(res.0.expect("Some expression expected"))
        } else {
            generate_parser().parse(stream)
        }
    };

    match ast {
        Ok(expr) => Ok(recursive_parallel_expr_resolve(expr)),
        e => e,
    }
}