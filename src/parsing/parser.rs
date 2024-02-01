use chumsky::{prelude::*, text::*};

use super::models::Expr;

type SimpleCharError = Simple<char>;

fn is_path_spec_char(a: &char) -> bool {
    a.is_ascii_alphabetic()
}

pub fn generate_parser() -> impl Parser<char, Expr, Error = SimpleCharError> {
    let dotted_ident = ident()
        .separated_by(just("."))
        .collect::<Vec<String>>()
        .map(|parts| parts.join("."))
        .labelled("dotted_ident")
        .boxed();

    let block_start = just("{").then(newline()).labelled("block_start").boxed();

    let block_end = newline()
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

    let semicolon = just(";").labelled("semicolon").boxed();

    let rule = whitespace()
        .ignore_then(keyword("allow"))
        .then_ignore(semicolon)
        .map(|()| Expr::Rule("allow".to_string()))
        .labelled("rule_match")
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
                choice([rule, expr.boxed()])
                    .separated_by(newline_separator)
                    .at_least(1),
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
