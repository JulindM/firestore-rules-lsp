use chumsky::{prelude::*, text::*};

use crate::parsing::models::{ArithmeticOp, BinaryLogicOp, NestedOperator, UnaryLogicOp};

use super::{
    errors::{gen_error, SimpleCharError},
    models::{AllowMethod, Expr},
};

fn is_path_spec_char(a: &char) -> bool {
    a.is_ascii_alphabetic()
}

fn generate_nested_expr(expr: Expr, nested_ops: Vec<(NestedOperator, Expr)>) -> Expr {
    if nested_ops.is_empty() {
        return expr;
    };

    if nested_ops.len() == 1 {
        let (op, nested_expr) = nested_ops.last().unwrap();
        return Expr::Nested(Box::new(expr), Box::new(nested_expr.clone()), op.clone());
    }

    let ((op, nested_expr), remaining) = nested_ops.split_first().unwrap();

    Expr::Nested(
        Box::new(expr),
        Box::new(generate_nested_expr(
            nested_expr.clone(),
            remaining.to_vec(),
        )),
        op.clone(),
    )
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

    let arithmetic_op = choice([
        just("+").to(ArithmeticOp::Add),
        just("-").to(ArithmeticOp::Sub),
        just("*").to(ArithmeticOp::Mult),
        just("/").to(ArithmeticOp::Div),
        just("%").to(ArithmeticOp::Mod),
    ])
    .labelled("arithmetic_op")
    .boxed();

    // TODO fix this mess of justs
    let binary_logic_op = choice([
        just("&").then(just("&")).to(BinaryLogicOp::And).boxed(),
        just("|").then(just("|")).to(BinaryLogicOp::Or).boxed(),
        just("=").then(just("=")).to(BinaryLogicOp::Eq).boxed(),
        just("!").then(just("=")).to(BinaryLogicOp::Ineq).boxed(),
        just(">").to(BinaryLogicOp::Greater).boxed(),
        just(">")
            .then(just("="))
            .to(BinaryLogicOp::GreaterEq)
            .boxed(),
        keyword("in").to(BinaryLogicOp::In).boxed(),
        keyword("is").to(BinaryLogicOp::Is).boxed(),
    ])
    .labelled("binary_logic_op")
    .boxed();

    let unary_logic_op = choice([
        just("!").to(UnaryLogicOp::Neg).boxed(),
        just("-").to(UnaryLogicOp::Neg).boxed(),
    ])
    .labelled("unary_logic_op")
    .boxed();

    let assignment = just("=").labelled("assignment").boxed();

    // Test if _ is allowed in variable names
    let variable = ident().labelled("variable").map(Expr::Variable).boxed();

    let number = just("-")
        .to(-1)
        .or_not()
        .to(1)
        .then(int(10).try_map(|s: String, span| {
            s.parse::<i32>()
                .map_err(|_| gen_error("not a valid integer")(span))
        }))
        .map(|(negation, digits)| Expr::Number(negation * digits))
        .labelled("number")
        .boxed();

    let function_signature = ident()
        .then(
            variable
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
        .labelled("function_signature")
        .boxed();

    let indexing = just("[")
        .ignore_then(int(10).try_map(|s: String, span| {
            s.parse::<u32>()
                .map_err(|_| gen_error("not a valid positive integer")(span))
        }))
        .then_ignore(just("]"))
        .labelled("indexing")
        .map(|num| (NestedOperator::Indexing, Expr::Number(num as i32)))
        .boxed();

    let field_access = just(".")
        .ignore_then(variable.clone())
        .map(|var| (NestedOperator::FieldAcess, var))
        .labelled("indexing")
        .boxed();

    let atomic_valuable = choice([function_signature.clone(), variable.clone()])
        .labelled("atomic_valuable")
        .boxed();

    let valuable = atomic_valuable
        .then(choice([indexing, field_access]).repeated().or_not())
        .map(|(atomic, nested_ops)| {
            if nested_ops.is_none() {
                return atomic;
            }

            generate_nested_expr(atomic, nested_ops.unwrap())
        })
        .labelled("valuable")
        .boxed();

    let arithmetic_expr = recursive(|expr| {
        let operand = choice([
            number.clone(),
            valuable.clone(),
            expr.delimited_by(just("("), just(")")).boxed(),
        ]);

        operand
            .clone()
            .map_err(gen_error("arithmetic operand expected"))
            .then(arithmetic_op.padded())
            .map_err(gen_error("arithmetic operator expected"))
            .then(operand)
            .map_err(gen_error("arithmetic operand expected"))
            .map(|((o1, op), o2)| Expr::ArithmeticExpr(Box::new(o1), Box::new(o2), op))
    })
    .labelled("arithmetic_expr")
    .boxed();

    let logic_expr = recursive(|expr| {
        let truthy_operand = choice([
            valuable.clone(),
            keyword("true").to(Expr::Bool(true)).boxed(),
            keyword("false").to(Expr::Bool(false)).boxed(),
            expr.delimited_by(just("("), just(")")).boxed(),
        ])
        .boxed();

        let binary_truthy_operand = choice([truthy_operand.clone(), number]).boxed();
        let binary_expression = binary_truthy_operand
            .clone()
            .map_err(gen_error("truthy operand expected"))
            .then(binary_logic_op.padded())
            .map_err(gen_error("truthy operator expected"))
            .then(binary_truthy_operand.clone())
            .map_err(gen_error("truthy operand expected"))
            .map(|((o1, op), o2)| Expr::LogicExpression(Box::new(o1), Box::new(o2), op))
            .boxed();

        let unary_expression = unary_logic_op
            .map_err(gen_error("unary logic operator expected"))
            .then(truthy_operand.clone())
            .map_err(gen_error("truthy operand expected"))
            .map(|(op, o1)| Expr::UnaryLogicExpression(Box::new(o1), op))
            .boxed();

        choice([binary_expression, unary_expression, truthy_operand])
            .labelled("logic expression")
            .boxed()
    })
    .labelled("arithmetic_expr")
    .boxed();

    let expression = choice([logic_expr.clone(), arithmetic_expr, valuable])
        .map_err(gen_error("expression expected"))
        .labelled("expression")
        .boxed();

    //TODO let body
    let let_statement = {
        let let_content = keyword("let")
            .padded()
            .ignore_then(variable.clone())
            .map_err(gen_error("variable name expected"))
            .then_ignore(assignment.padded())
            .map_err(gen_error("equals expected"))
            .then(expression.clone())
            .map_err(gen_error("arithmetic or logic expression expected"))
            .map(|(var_name, expr)| {
                Expr::VariableDef(Box::new(var_name.clone()), Box::new(expr.clone()))
            })
            .boxed();

        statement!(let_content).labelled("let_statement").boxed()
    };

    let return_statement = {
        //TODO return body
        let return_content = keyword("return")
            .padded()
            .map_err(gen_error("return expected"))
            .ignore_then(expression)
            .labelled("return")
            .boxed();

        statement!(return_content)
            .labelled("return_statement")
            .boxed()
    };

    let function_body = let_statement
        .separated_by(newline_separator.clone().repeated().at_least(1))
        .or_not()
        .then(return_statement.then_ignore(newline_separator.clone()))
        .map_err(gen_error("let or return expected"))
        .map(|(stmts, ret)| {
            Expr::FunctionBody(
                stmts.unwrap_or(vec![]),
                Box::new(Expr::Return(Box::new(ret))),
            )
        })
        .labelled("function_body")
        .boxed();

    let function_declaration = (keyword("function").padded())
        .ignore_then(function_signature)
        .then(
            function_body
                .padded()
                .delimited_by(block_start.clone(), block_end.clone()),
        )
        .map(|(fsig, fbody)| Expr::FunctionDecl(Box::new(fsig), Box::new(fbody)))
        .labelled("function_declaration")
        .boxed();

    // TODO content
    let allow_content = (keyword("if").padded().map_err(gen_error("if expected")))
        .ignore_then(logic_expr)
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
                            emit(gen_error("An allow block must have at least 1 allow rule")(
                                span,
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
        .map(Expr::ServiceBody)
        .boxed();

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
