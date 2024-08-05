use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
};

use crate::parsing::models::{AllowMethod, EvalExpression, RuleExpr};

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    parameters: Vec<String>,
    variables: Vec<String>,
}

impl Function {
    fn generate_from_ast(function_definition: RuleExpr) -> Function {
        match function_definition {
            RuleExpr::FunctionDecl(signature, body) => match *signature {
                RuleExpr::FunctionSig(fname, fparams) => match *body {
                    RuleExpr::FunctionBody(fvariables, _) => Function {
                        name: fname,
                        parameters: fparams,
                        variables: Function::get_variables(fvariables),
                    },
                    _ => panic!("Unexpected variant"),
                },
                _ => panic!("Unexpected variant"),
            },
            _ => panic!("Unexpected variant"),
        }
    }

    fn get_variables(variables: Vec<RuleExpr>) -> Vec<String> {
        variables
            .iter()
            .map(|vardef| match vardef {
                RuleExpr::VariableDef(name, _) => name.clone(),
                _ => panic!("Variable definition expected"),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    rule_types: Vec<AllowMethod>,
    expression: Option<EvalExpression>,
}

impl Rule {
    fn generate_from_ast(rule_definition: RuleExpr) -> Rule {
        match rule_definition {
            RuleExpr::Allow(rmethods, rallow_type) => match *rallow_type {
                RuleExpr::AllAllow => Rule {
                    rule_types: rmethods,
                    expression: None,
                },
                RuleExpr::ConditionalAllow(rexpr) => Rule {
                    rule_types: rmethods,
                    expression: Some(*rexpr),
                },
                _ => panic!("Allow type expected"),
            },
            _ => panic!("Rule type expected"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchBody {
    functions: Vec<Function>,
    rules: Vec<Rule>,
    matches: Vec<Match>,
}

impl MatchBody {
    fn generate_from_definitions(defs: Vec<RuleExpr>) -> MatchBody {
        let mut sfunctions: Vec<Function> = vec![];
        let mut srules: Vec<Rule> = vec![];
        let mut smatches: Vec<Match> = vec![];

        defs.iter().for_each(|part| match part {
            RuleExpr::FunctionDecl(_, __) => {
                sfunctions.push(Function::generate_from_ast(part.clone()))
            }
            RuleExpr::Allow(_, __) => srules.push(Rule::generate_from_ast(part.clone())),
            RuleExpr::Match(_, __) => smatches.push(Match::generate_from_ast(part.clone())),
            _ => panic!(
                "Unexpected variant: Expected allow, match or function got {:#?}",
                part
            ),
        });

        MatchBody {
            functions: sfunctions,
            rules: srules,
            matches: smatches,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pathspec: Vec<String>,
}

impl Path {
    fn generate_from_definition(pathspec: RuleExpr) -> Path {
        let mut spathelements: Vec<String> = vec![];

        match pathspec {
            RuleExpr::Path(path_parts) => {
                path_parts.iter().for_each(|part| match part {
                    RuleExpr::PathPart(spathspec) => spathelements.push(spathspec.clone()),
                    RuleExpr::EvalPathPart(sevalpathspec) => {
                        spathelements.push(sevalpathspec.clone())
                    }
                    RuleExpr::SingleSegWildPath(ssegpathspec) => {
                        spathelements.push(ssegpathspec.clone())
                    }
                    RuleExpr::RecursiveWildPath(ssegpathspec) => {
                        spathelements.push(ssegpathspec.clone())
                    }
                    _ => panic!(
                        "Unexpected variant: Expected pathpart or evalparthpart got {:#?}",
                        part
                    ),
                });
            }
            _ => panic!("Unexpected variant"),
        }

        Path {
            pathspec: spathelements,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Match {
    path: Path,
    match_content: MatchBody,
}

impl Match {
    fn generate_from_ast(match_block: RuleExpr) -> Match {
        match match_block {
            RuleExpr::Match(match_path, definitions) => Match {
                path: Path::generate_from_definition(*match_path),
                match_content: MatchBody::generate_from_definitions(definitions),
            },
            _ => panic!("Expected a match rule"),
        }
    }
}

#[derive(Debug)]
pub struct Service {
    match_content: MatchBody,
}

impl Service {
    pub fn generate_from_service_definition(service: RuleExpr) -> Service {
        match service {
            RuleExpr::ServiceDefinition(_, body) => match *body {
                RuleExpr::ServiceBody(definitions) => Service {
                    match_content: MatchBody::generate_from_definitions(definitions),
                },
                _ => panic!("Expected a service body"),
            },
            _ => panic!("Expected a service definition"),
        }
    }

    pub fn get_variables(&self) -> Vec<String> {
        let mut vars: Vec<String> = vec![];

        for func in self.match_content.functions.iter() {
            vars.append(func.variables.clone().as_mut());
        }

        fn get_funcs_of_match_recrsv(match_def: &mut Match) -> Vec<Function> {
            let mut funcs: Vec<Function> = vec![];

            funcs.append(&mut match_def.match_content.functions);

            if match_def.match_content.functions.is_empty() {
                return funcs;
            }

            funcs.append(
                match_def
                    .match_content
                    .matches
                    .iter_mut()
                    .flat_map(get_funcs_of_match_recrsv)
                    .collect::<Vec<Function>>()
                    .as_mut(),
            );

            return funcs;
        }

        for rule in self.match_content.matches.iter() {
            let funcs: Vec<Function> = get_funcs_of_match_recrsv(&mut rule.clone());

            vars.append(&mut funcs.iter().flat_map(|f| f.variables.clone()).collect())
        }

        return vars;
    }
}
