use std::vec;

use tree_sitter::{Node, Tree};

use crate::{
  Expr, ExprNode, FirestoreTree, Function, FunctionArgument, FunctionBody, Literal, Match,
  MatchBody, MatchPath, MatchPathPart, Method, Operation, PathSegment, Rule, Variable,
  VariableDefintion,
};

use super::extensions::{ErrorNode, EvaluatedTree};

const SERVICE_NAME: &str = "service_name";
const ROOT_MATCH: &str = "match_body";

pub fn evaluate_tree<'tree>(
  tree: &'tree Tree,
  source_bytes: &[u8],
) -> Result<EvaluatedTree<'tree>, String> {
  let source_node = tree.root_node();

  if source_node.kind() != "source_file" {
    panic!("No source detected")
  }

  let mut cursor = source_node.walk();

  cursor.goto_first_child();
  let mut curr_node = cursor.node();
  let mut curr_kind = curr_node.kind();

  if curr_kind != SERVICE_NAME {
    return Ok(gen_empty_tree(&curr_node));
  }

  cursor.goto_next_sibling();
  curr_node = cursor.node();
  curr_kind = curr_node.kind();

  if curr_kind != ROOT_MATCH {
    return Ok(gen_empty_tree(&curr_node));
  }

  cursor.goto_next_sibling();
  curr_node = cursor.node();
  curr_kind = curr_node.kind();

  if curr_kind != ROOT_MATCH {
    return Ok(gen_empty_tree(&curr_node));
  }

  let (root_match, errors) = parse_match_body(curr_node, source_bytes);
  let tree = FirestoreTree::new(root_match);

  return Ok(EvaluatedTree::new(tree, errors));
}

fn gen_empty_tree<'tree>(service_node: &Node<'tree>) -> EvaluatedTree<'tree> {
  let err = ErrorNode::new(*service_node);
  let empty_tree = FirestoreTree::new(MatchBody::empty());

  return EvaluatedTree::new(empty_tree, vec![err]);
}

fn parse_match_body<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (MatchBody, Vec<ErrorNode<'tree>>) {
  let mut matches = vec![];
  let mut functions = vec![];
  let mut rules = vec![];
  let mut level_errors = vec![];

  node
    .children(&mut node.walk())
    .for_each(|child| match child.kind() {
      "match_def" => {
        let mut res = parse_match_def(child, source_bytes);
        matches.push(res.0);
        level_errors.append(&mut res.1);
      }
      "function_def" => {
        let mut res = parse_function(child, source_bytes);
        functions.push(res.0);
        level_errors.append(&mut res.1);
      }
      "rule_def" => {
        let mut res = parse_rule(child, source_bytes);
        rules.push(res.0);
        level_errors.append(&mut res.1);
      }
      _ if node.is_error() => level_errors.push(ErrorNode::new(node)),
      _ => return,
    });

  (MatchBody::new(functions, matches, rules), level_errors)
}

fn parse_match_def<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Match, Vec<ErrorNode<'tree>>) {
  let mut path = MatchPath::empty();
  let mut body = MatchBody::empty();
  let mut level_errors = vec![];

  node
    .children(&mut node.walk())
    .for_each(|child| match child.kind() {
      "match_path" => {
        let mut res = parse_match_path(child, source_bytes);

        path = res.0;
        level_errors.append(&mut res.1);
      }
      "match_body" => {
        let mut res = parse_match_body(child, source_bytes);
        body = res.0;
        level_errors.append(&mut res.1);
      }
      _ if node.is_error() => level_errors.push(ErrorNode::new(node)),
      _ => return,
    });

  (Match::new(path, body), level_errors)
}

fn parse_function<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Function, Vec<ErrorNode<'tree>>) {
  let mut name = "";
  let mut parms = vec![];
  let mut body = FunctionBody::empty();
  let mut level_errors = vec![];

  let mut cursor = node.walk();

  node
    .children(&mut cursor)
    .for_each(|child| match child.kind() {
      "name" => name = child.utf8_text(source_bytes).unwrap(),
      "param" => parms.push(Variable::new(child.utf8_text(source_bytes).unwrap())),
      "function_body" => {
        let mut res = parse_function_body(child, source_bytes);
        body = res.0;
        level_errors.append(&mut res.1);
      }
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      _ => return,
    });

  (Function::new(name, parms, body), vec![])
}

fn parse_function_body<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (FunctionBody, Vec<ErrorNode<'tree>>) {
  let mut variables = vec![];
  let mut ret = None;
  let mut level_errors = vec![];

  let mut cursor = node.walk();

  node
    .children(&mut cursor)
    .for_each(|child| match child.kind() {
      "variable_def" => {
        let mut res = parse_variable_def(child, source_bytes);
        variables.push(res.0);
        level_errors.append(&mut res.1);
      }
      "expr" => {
        let mut res = parse_expr(child, source_bytes);
        ret = res.0;
        level_errors.append(&mut res.1);
      }
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      _ => return,
    });

  (FunctionBody::new(variables, ret), level_errors)
}

fn parse_variable_def<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (VariableDefintion, Vec<ErrorNode<'tree>>) {
  let mut name = "";
  let mut expr = None;
  let mut level_errors = vec![];

  let mut cursor = node.walk();

  node
    .children(&mut cursor)
    .for_each(|child| match child.kind() {
      "variable" => name = child.utf8_text(source_bytes).unwrap(),
      "expr" => {
        let mut res = parse_expr(child, source_bytes);
        expr = res.0;
        level_errors.append(&mut res.1);
      }
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      _ => return,
    });

  (VariableDefintion::new(name, expr), level_errors)
}

fn parse_match_path<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (MatchPath, Vec<ErrorNode<'tree>>) {
  let mut path_parts = vec![];
  let mut level_errors = vec![];

  node
    .children(&mut node.walk())
    .for_each(|child| match child.kind() {
      "collection_path_seg" => {
        let name = child.utf8_text(source_bytes).unwrap();
        let part = MatchPathPart::Collection(name.to_owned());
        path_parts.push(part);
      }
      "single_path_seg" => {
        let name = child.utf8_text(source_bytes).unwrap();
        let part = MatchPathPart::SinglePath(name.to_owned());
        path_parts.push(part);
      }
      "multi_path_seg" => {
        let name = child.utf8_text(source_bytes).unwrap();
        let part = MatchPathPart::MultiPath(name.to_owned());
        path_parts.push(part);
      }
      _ if node.is_error() => level_errors.push(ErrorNode::new(node)),
      _ => level_errors.push(ErrorNode::new(node)),
    });

  (MatchPath::new(path_parts), level_errors)
}

fn parse_rule<'tree>(node: Node<'tree>, source_bytes: &[u8]) -> (Rule, Vec<ErrorNode<'tree>>) {
  let mut methods = vec![];
  let mut condition: Option<ExprNode> = None;
  let mut level_errors = vec![];

  node
    .children(&mut node.walk())
    .for_each(|child| match child.kind() {
      "method" => match parse_method(child) {
        Ok(meth) => methods.push(meth),
        Err(err) => level_errors.push(err),
      },
      "expr" => {
        let mut res = parse_expr(child, source_bytes);
        condition = res.0;
        level_errors.append(&mut res.1);
      }
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      _ => return,
    });

  (Rule::new(methods, condition), level_errors)
}

fn parse_primary<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let child = node.child(0).unwrap();

  match child.kind() {
    "literal" => parse_literal(child, source_bytes),
    "variable" => parse_variable(child, source_bytes),
    "expr_group" => parse_expr(child.child(1).unwrap(), source_bytes),
    "function_call" => parse_function_call(child, source_bytes),
    "list" => {
      let expr_list = child.child(1);
      match expr_list {
        None => (None, vec![]),
        Some(node) => parse_expr(node, source_bytes),
      }
    }
    _ if child.is_error() => (None, vec![ErrorNode::new(child)]),
    _ => (None, vec![]),
  }
}

fn parse_function_call<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let name = node.child(0).unwrap().utf8_text(source_bytes).unwrap();
  let mut level_errors = vec![];

  let mut arg = None;

  node
    .children(&mut node.walk())
    .for_each(|child| match child.kind() {
      "expr_list" => {
        let mut res = parse_expr_list(child, source_bytes);
        arg = Some(FunctionArgument::ExprList(res.0));
        level_errors.append(&mut res.1);
      }
      "path" => {
        let mut res = parse_path(child, source_bytes);
        arg = Some(FunctionArgument::Path(res.0));
        level_errors.append(&mut res.1);
      }
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      _ => return,
    });

  (
    Some(ExprNode::new(Expr::FunctionCall(name.to_owned(), arg))),
    level_errors,
  )
}

fn parse_path<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Vec<PathSegment>, Vec<ErrorNode<'tree>>) {
  let mut path_segments = vec![];
  let mut level_errors = vec![];

  let mut cursor = node.walk();

  node
    .children(&mut cursor)
    .for_each(|child| match child.kind() {
      "path_segment" => {
        let mut c_cursor = child.walk();

        child
          .children(&mut c_cursor)
          .for_each(|child| match child.kind() {
            "identifier" => {
              let name = child.utf8_text(source_bytes).unwrap();
              path_segments.push(PathSegment::String(name.to_owned()))
            }
            "expr" => {
              let mut expr = parse_expr(child, source_bytes);
              path_segments.push(PathSegment::EvalPath(expr.0));
              level_errors.append(&mut expr.1);
            }
            _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
            _ => return,
          });
      }
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      _ => return,
    });

  (path_segments, level_errors)
}

fn parse_expr_list<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Vec<ExprNode>, Vec<ErrorNode<'tree>>) {
  let mut expr_nodes = vec![];
  let mut level_errors = vec![];

  let mut cursor = node.walk();

  node
    .children(&mut cursor)
    .for_each(|child| match child.kind() {
      "expr" => {
        let mut res = parse_expr(child, source_bytes);
        let _ = res.0.and_then(|expr| Some(expr_nodes.push(expr)));
        level_errors.append(&mut res.1);
      }
      "," => return,
      _ => level_errors.push(ErrorNode::new(child)),
    });

  (expr_nodes, level_errors)
}

fn parse_literal<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let child = node.child(0).unwrap();

  let literal = match child.kind() {
    "number" => Literal::Number(
      child
        .utf8_text(source_bytes)
        .unwrap()
        .parse::<f32>()
        .unwrap(),
    ),
    "true" => Literal::Bool(true),
    "false" => Literal::Bool(false),
    "null" => Literal::Null,
    "string" => Literal::String(child.utf8_text(source_bytes).unwrap().to_owned()),
    _ if child.is_error() => return (None, vec![ErrorNode::new(child)]),
    _ => return (None, vec![]),
  };

  (Some(ExprNode::new(Expr::Literal(literal))), vec![])
}

fn parse_variable<'tree>(
  child: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let name = child.utf8_text(source_bytes).unwrap();
  let variable = Variable::new(name);

  (Some(ExprNode::new(Expr::Variable(variable))), vec![])
}

fn parse_indexing<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let mut cursor = node.walk();
  let children = node.children(&mut cursor);

  if children.len() != 4 {
    return (None, vec![ErrorNode::new(node)]);
  }

  let object;
  let index;
  let mut level_errors = vec![];

  object = parse_expr(node.child(0).unwrap(), source_bytes);
  index = parse_expr(node.child(2).unwrap(), source_bytes);

  let expr = Expr::Indexing(Box::new(object.0), Box::new(index.0));

  level_errors = [level_errors, object.1, index.1]
    .into_iter()
    .flatten()
    .collect();

  (Some(ExprNode::new(expr)), level_errors)
}

fn parse_expr<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let child = match node.kind() {
    "expr" => node.child(0).unwrap(),
    _ => node,
  };

  return match child.kind() {
    "ternary" => parse_ternary(child, source_bytes),
    "conditional_or" | "conditional_and" | "relation" | "addition" | "multiplication" => {
      parse_binary(child, source_bytes)
    }
    "unary" => parse_unary(child, source_bytes),
    "member" => parse_member(child, source_bytes),
    "indexing" => parse_indexing(child, source_bytes),
    "primary" => parse_primary(child, source_bytes),
    "function_call" => parse_function_call(child, source_bytes),
    "variable" => parse_variable(child, source_bytes),
    _ if child.is_error() => (None, vec![ErrorNode::new(child)]),
    _ => (None, vec![]),
  };
}

fn parse_member<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let mut cursor = node.walk();
  let children = node.children(&mut cursor);

  if children.len() != 3 {
    return (None, vec![ErrorNode::new(node)]);
  }

  let object;
  let field;
  let mut level_errors = vec![];

  object = parse_expr(node.child(0).unwrap(), source_bytes);
  field = parse_expr(node.child(2).unwrap(), source_bytes);

  let expr = Expr::Member(Box::new(object.0), Box::new(field.0));

  level_errors = [level_errors, object.1, field.1]
    .into_iter()
    .flatten()
    .collect();

  (Some(ExprNode::new(expr)), level_errors)
}

fn parse_ternary<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let mut cursor = node.walk();
  let children = node.children(&mut cursor);

  if children.len() != 5 {
    return (None, vec![ErrorNode::new(node)]);
  }

  let condition;
  let on_true;
  let on_false;
  let level_errors;

  condition = parse_expr(node.child(0).unwrap(), source_bytes);
  on_true = parse_expr(node.child(2).unwrap(), source_bytes);
  on_false = parse_expr(node.child(4).unwrap(), source_bytes);

  let expr = Expr::Ternary(
    Box::new(condition.0),
    Box::new(on_true.0),
    Box::new(on_false.0),
  );

  level_errors = [condition.1, on_true.1, on_false.1]
    .into_iter()
    .flatten()
    .collect();

  (Some(ExprNode::new(expr)), level_errors)
}

fn parse_binary<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let mut cursor = node.walk();
  let children = node.children(&mut cursor);

  if children.len() != 3 {
    return (None, vec![ErrorNode::new(node)]);
  }

  let operator1;
  let operation;
  let operator2;
  let mut level_errors = vec![];

  operator1 = parse_expr(node.child(0).unwrap(), source_bytes);
  operator2 = parse_expr(node.child(2).unwrap(), source_bytes);

  let op_node = node.child(1).unwrap();

  operation = match op_node.kind() {
    "+" => Some(Operation::Addition),
    "-" => Some(Operation::Substraction),
    "&&" => Some(Operation::And),
    "||" => Some(Operation::Or),
    "*" => Some(Operation::Multiplication),
    "/" => Some(Operation::Division),
    "%" => Some(Operation::Modulo),
    "<" | "<=" | ">=" | ">" | "==" | "!=" | "in" => Some(Operation::Relation),
    _ => {
      level_errors.push(ErrorNode::new(op_node));
      None
    }
  };

  let expr = Expr::Binary(operation, Box::new(operator1.0), Box::new(operator2.0));

  level_errors = [level_errors, operator1.1, operator2.1]
    .into_iter()
    .flatten()
    .collect();

  (Some(ExprNode::new(expr)), level_errors)
}

fn parse_unary<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode<'tree>>) {
  let mut cursor = node.walk();
  let children = node.children(&mut cursor);

  if children.len() != 2 {
    return (None, vec![ErrorNode::new(node)]);
  }

  let operation;
  let operator;
  let mut level_errors = vec![];

  operator = parse_expr(node.child(1).unwrap(), source_bytes);

  let op_node = node.child(0).unwrap();

  operation = match op_node.kind() {
    "negation" => Some(Operation::Negation),
    "substraction" => Some(Operation::Substraction),
    "&&" => Some(Operation::And),
    _ => {
      level_errors.push(ErrorNode::new(op_node));
      None
    }
  };

  let expr = Expr::Unary(operation, Box::new(operator.0));

  level_errors = [level_errors, operator.1].into_iter().flatten().collect();

  (Some(ExprNode::new(expr)), level_errors)
}

fn parse_method<'tree>(m_node: Node<'tree>) -> Result<Method, ErrorNode> {
  let mut cursor = m_node.walk();

  cursor.goto_first_child();

  match cursor.node().kind() {
    "read" => Ok(Method::Read),
    "write" => Ok(Method::Write),
    "get" => Ok(Method::Get),
    "list" => Ok(Method::List),
    "create" => Ok(Method::Create),
    "update" => Ok(Method::Update),
    "delete" => Ok(Method::Delete),
    _ => Err(ErrorNode::new(cursor.node())),
  }
}
