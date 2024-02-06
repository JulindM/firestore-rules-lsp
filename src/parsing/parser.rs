use chumsky::{prelude::*, text::*};

use super::models::{AllowMethod, Expr};

type SimpleCharError = Simple<char>;

fn is_path_spec_char(a: &char) -> bool {
    a.is_ascii_alphabetic()
}

pub fn generate_parser() -> impl Parser<char, Expr, Error = SimpleCharError> {
    let semicolon = just(";").labelled("semicolon").boxed();
    let colon = just(":").labelled("colon").boxed();

    macro_rules! statement {
        ($p: expr) => {
            $p.then_ignore(semicolon.clone())
        };
    }

    let dotted_ident = ident()
        .separated_by(just("."))
        .collect::<Vec<String>>()
        .map(|parts| parts.join("."))
        .labelled("dotted_ident")
        .boxed();

    let block_start = just("{").then(newline()).labelled("block_start").boxed();

    let block_end = newline()
        .or_not()
        .then(whitespace())
        .then(just("}"))
        .labelled("block_end")
        .boxed();

    // TODO fix this ugly hack
    let pathspec = filter(|c| is_path_spec_char(c))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|spec| spec)
        .labelled("path_spec")
        .boxed();

    let var_path = pathspec
        .clone()
        .map(Expr::VarPath)
        .labelled("path_part")
        .boxed();

    let single_seg_wild_path = pathspec
        .clone()
        .delimited_by(just("{"), just("}"))
        .map(Expr::SingleSegWildPath)
        .labelled("single_segment_wildcard")
        .boxed();

    let rec_wild_path = pathspec
        .clone()
        .then_ignore(just("="))
        .then_ignore(just("*"))
        .then_ignore(just("*"))
        .delimited_by(just("{"), just("}"))
        .map(Expr::RecursiveWildPath)
        .labelled("wildcard_pattern")
        .boxed();

    let path_separator = just("/").labelled("path_separator").boxed();

    let path = path_separator
        .clone()
        .ignore_then(
            choice([var_path, single_seg_wild_path, rec_wild_path])
                .separated_by(path_separator)
                .collect(),
        )
        .map(Expr::Path);

    let newline_separator = newline().labelled("whitespace_delimiter").boxed();

    let method = choice([
        keyword("read").to(AllowMethod::Read),
        keyword("write").to(AllowMethod::Write),
        keyword("get").to(AllowMethod::Get),
        keyword("list").to(AllowMethod::List),
        keyword("create").to(AllowMethod::Create),
        keyword("update").to(AllowMethod::Update),
        keyword("delete").to(AllowMethod::Delete),
    ])
    .labelled("method")
    .boxed();

    // Test if _ is allowed in variable names
    let variable = ident().labelled("variable").map(Expr::Variable).boxed();

    // TODO Function body
    let function_body = statement!(variable.clone())
        .separated_by(newline_separator.clone())
        .map(Expr::FunctionBody)
        .labelled("function_body")
        .boxed();

    let function_declaration = (keyword("function").padded())
        .ignore_then(ident())
        .then(
            variable
                .clone()
                .separated_by(just(",").padded())
                .delimited_by(just("("), just(")"))
                .padded(),
        )
        .then(
            function_body
                .padded()
                .delimited_by(block_start.clone(), block_end.clone()),
        )
        .map(|((fname, fargs), fbody)| Expr::FunctionDecl(fname, fargs, Box::new(fbody)))
        .labelled("function_declaration")
        .boxed();

    // TODO content
    let allow_content = (keyword("if")
        .padded()
        .map_err(|e: SimpleCharError| Simple::custom(e.span(), "if expected")))
    .ignore_then(variable)
    .map(|c| Expr::ConditionalAllow(Box::new(c)))
    .labelled("allow_content")
    .boxed();

    let allow = (keyword("allow").padded())
        .ignore_then(choice([
            statement!(method.clone())
                .map(|m| (m, Expr::AllAllow))
                .boxed(),
            statement!(method.then_ignore(colon).then(allow_content)).boxed(),
        ]))
        .map(|(meth, con)| Expr::Allow(meth, Box::new(con)))
        .labelled("allow_rule")
        .boxed();

    let match_declaration = whitespace()
        .then(keyword("match"))
        .then(whitespace())
        .ignore_then(path)
        .labelled("match_declaration")
        .boxed();

    let match_block = recursive(|expr| {
        match_declaration
            .then_ignore(whitespace().then(block_start.clone()))
            .then(
                choice([allow, function_declaration, expr.boxed()])
                    .separated_by(newline_separator)
                    .at_least(1)
                    .boxed()
                    // There has to be a smarter way than validating
                    // but I cant seem to be smart enough to find it
                    .validate(|exprsns, span, emit| {
                        let has_allow = exprsns.iter().any(|s| match s {
                            Expr::Allow(_, _) => true,
                            _ => false,
                        });

                        if !has_allow {
                            emit(Simple::custom(
                                span,
                                "An allow block must have at least 1 allow rule",
                            ))
                        };
                        exprsns
                    }),
            )
            .then_ignore(block_end.clone())
            .map(|(path, exprsns)| Expr::Match(Box::new(path), exprsns))
    })
    .labelled("match_block")
    .boxed();

    let service_body = match_block
        .repeated()
        .collect()
        .labelled("service_body")
        .map(Expr::ServiceBody);

    let service_decl = keyword("service")
        .ignore_then(dotted_ident.padded())
        .then(service_body.delimited_by(block_start, block_end))
        .map(|(name, body)| Expr::ServiceDefinition(name, Box::new(body)))
        .labelled("service_definition")
        .boxed();

    service_decl.then_ignore(end())
}

pub fn parse(stream: Vec<char>) -> Result<Expr, Vec<Simple<char>>> {
    generate_parser().parse(stream)
}
