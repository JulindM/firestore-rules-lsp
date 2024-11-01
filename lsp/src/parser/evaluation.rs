use std::vec;

use tree_sitter::{Node, Tree};

use crate::parser::base::MethodType;

use super::{
  base::{
    Expr, ExprNode, FirestoreTree, Function, FunctionArgument, FunctionBody, Identifier, Literal,
    LiteralType, Match, MatchBody, MatchPath, MatchPathPart, MatchPathPartType, Method, Operation,
    PathSegment, Rule, VariableDefintion,
  },
  extensions::{ErrorNode, EvaluatedTree},
};

macro_rules! sanitized_children {
  ($node: ident) => {
    $node
      .children(&mut $node.walk())
      .filter(|n| n.kind() != "comment")
  };
}

pub fn evaluate_tree<'a>(tree: Tree, source_bytes: &[u8]) -> EvaluatedTree {
  let node = tree.root_node();

  if node.kind() != "source_file" {
    return EvaluatedTree::new(None, vec![]);
  }

  let mut level_errors = vec![];
  let mut firestore_tree: Option<FirestoreTree> = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_body" => {
      let (root_match, mut errors) = parse_match_body(None, child, source_bytes);
      firestore_tree = Some(FirestoreTree::new(Some(root_match)));
      level_errors.append(&mut errors);
    }
    _ if node.is_error() => level_errors.push(ErrorNode::new(node, source_bytes)),
    _ => return,
  });

  EvaluatedTree::new(firestore_tree, level_errors)
}

fn parse_match_body<'a, 'b>(
  curr_match_parent: Option<&'a Match>,
  node: Node<'b>,
  source_bytes: &[u8],
) -> (MatchBody, Vec<ErrorNode>) {
  let mut matches = vec![];
  let mut functions = vec![];
  let mut rules = vec![];
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_def" => {
      let mut res = parse_match_def(curr_match_parent, child, source_bytes);
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
    _ if node.is_error() => level_errors.push(ErrorNode::new(node, source_bytes)),
    _ => return,
  });

  (
    MatchBody::new(functions, matches, rules, node),
    level_errors,
  )
}

fn parse_match_def<'a, 'b>(
  curr_parent_match: Option<&'a Match>,
  node: Node<'b>,
  source_bytes: &[u8],
) -> (Match, Vec<ErrorNode>) {
  let mut path = None;
  let mut body = None;
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_path" => {
      let mut res = parse_match_path(child, source_bytes);
      path = Some(res.0);
      level_errors.append(&mut res.1);
    }
    "match_body" => {
      let mut res = parse_match_body(curr_parent_match, child, source_bytes);
      body = Some(res.0);
      level_errors.append(&mut res.1);
    }
    _ if node.is_error() => level_errors.push(ErrorNode::new(node, source_bytes)),
    _ => return,
  });

  (Match::new(path, body, node), level_errors)
}

fn parse_function<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Function, Vec<ErrorNode>) {
  let mut name = "";
  let mut parms = vec![];
  let mut body = None;
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    _ if child.is_missing() => level_errors.push(ErrorNode::new(child, source_bytes)),
    "function_name" => name = child.utf8_text(source_bytes).unwrap(),
    "param" => parms.push(Identifier::new(
      child.utf8_text(source_bytes).unwrap(),
      child,
    )),
    "function_body" => {
      let mut res = parse_function_body(child, source_bytes);
      body = Some(res.0);
      level_errors.append(&mut res.1);
    }
    _ if child.is_error() => level_errors.push(ErrorNode::new(child, source_bytes)),
    _ => return,
  });

  (Function::new(name, parms, body, node), vec![])
}

fn parse_function_body<'b>(node: Node<'b>, source_bytes: &[u8]) -> (FunctionBody, Vec<ErrorNode>) {
  let mut variables = vec![];
  let mut ret = None;
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    _ if child.is_missing() => level_errors.push(ErrorNode::new(child, source_bytes)),
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
    _ if child.is_error() => level_errors.push(ErrorNode::new(child, source_bytes)),
    _ => return,
  });

  (FunctionBody::new(variables, ret, node), level_errors)
}

fn parse_variable_def<'b>(
  node: Node<'b>,
  source_bytes: &[u8],
) -> (VariableDefintion, Vec<ErrorNode>) {
  let mut name = "";
  let mut expr = None;
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    _ if child.is_missing() => level_errors.push(ErrorNode::new(child, source_bytes)),
    "variable" => name = child.utf8_text(source_bytes).unwrap(),
    "expr" => {
      let mut res = parse_expr(child, source_bytes);
      expr = res.0;
      level_errors.append(&mut res.1);
    }
    _ if child.is_error() => level_errors.push(ErrorNode::new(child, source_bytes)),
    _ => return,
  });

  (VariableDefintion::new(name, expr, node), level_errors)
}

fn parse_match_path<'b>(node: Node<'b>, source_bytes: &[u8]) -> (MatchPath, Vec<ErrorNode>) {
  let mut path_parts = vec![];
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "collection_path_seg" => {
      let name = child.utf8_text(source_bytes).unwrap().to_owned();
      let part = MatchPathPart::new(name, MatchPathPartType::Collection, child);
      path_parts.push(part);
    }
    "single_path_seg" => {
      let name = child.utf8_text(source_bytes).unwrap().to_owned();
      let part = MatchPathPart::new(name, MatchPathPartType::Document, child);
      path_parts.push(part);
    }
    "multi_path_seg" => {
      let name = child.utf8_text(source_bytes).unwrap().to_owned();
      let part = MatchPathPart::new(name, MatchPathPartType::MultiPath, child);
      path_parts.push(part);
    }
    _ if node.is_error() => level_errors.push(ErrorNode::new(node, source_bytes)),
    _ => level_errors.push(ErrorNode::new(node, source_bytes)),
  });

  (MatchPath::new(path_parts, node), level_errors)
}

fn parse_rule<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Rule, Vec<ErrorNode>) {
  let mut methods = vec![];
  let mut condition: Option<ExprNode> = None;
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    _ if child.is_missing() => level_errors.push(ErrorNode::new(child, source_bytes)),
    "method" => match parse_method(child, source_bytes) {
      Ok(meth) => methods.push(meth),
      Err(err) => level_errors.push(err),
    },
    "expr" => {
      let mut res = parse_expr(child, source_bytes);
      condition = res.0;
      level_errors.append(&mut res.1);
    }
    _ if child.is_error() => level_errors.push(ErrorNode::new(child, source_bytes)),
    _ => return,
  });

  (Rule::new(methods, condition, node), level_errors)
}

fn parse_primary<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let child = node.child(0).unwrap();

  match child.kind() {
    _ if child.is_missing() => (None, vec![ErrorNode::new(child, source_bytes)]),
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
    _ if child.is_error() => (None, vec![ErrorNode::new(child, source_bytes)]),
    _ => (None, vec![]),
  }
}

fn parse_function_call<'b>(
  node: Node<'b>,
  source_bytes: &[u8],
) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let mut level_errors = vec![];

  let mut args = vec![];
  let mut name = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    _ if child.is_missing() => level_errors.push(ErrorNode::new(child, source_bytes)),
    "function_calling_name" => {
      name = Some(Identifier::new(
        child.utf8_text(source_bytes).unwrap(),
        child,
      ));
    }
    "function_argument" => {
      let mut res = parse_function_arg(child, source_bytes);
      if res.0.is_some() {
        args.push(res.0.unwrap());
      }
      level_errors.append(&mut res.1);
    }
    _ if child.is_error() => level_errors.push(ErrorNode::new(child, source_bytes)),
    _ => return,
  });

  (
    Some(ExprNode::new(Expr::FunctionCall(name.unwrap(), args), node)),
    level_errors,
  )
}

fn parse_function_arg<'b>(
  node: Node<'b>,
  source_bytes: &[u8],
) -> (Option<FunctionArgument>, Vec<ErrorNode>) {
  for child in sanitized_children!(node) {
    match child.kind() {
      _ if child.is_missing() => return (None, vec![ErrorNode::new(child, source_bytes)]),
      "expr" => {
        let res = parse_expr(child, source_bytes);
        return (
          res.0.and_then(|node| Some(FunctionArgument::Expr(node))),
          res.1,
        );
      }
      "path" => {
        let res = parse_path(child, source_bytes);
        return (Some(FunctionArgument::Path(res.0)), res.1);
      }
      _ if child.is_error() => return (None, vec![ErrorNode::new(child, source_bytes)]),
      _ => continue,
    }
  }

  (None, vec![])
}

fn parse_path<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Vec<PathSegment>, Vec<ErrorNode>) {
  let mut path_segments = vec![];
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    _ if child.is_missing() => level_errors.push(ErrorNode::new(child, source_bytes)),
    "path_segment" => {
      child
        .children(&mut child.walk())
        .for_each(|child| match child.kind() {
          _ if child.is_missing() => level_errors.push(ErrorNode::new(child, source_bytes)),
          "identifier" => {
            let name = child.utf8_text(source_bytes).unwrap();
            path_segments.push(PathSegment::String(Identifier::new(name, child)))
          }
          "expr" => {
            let mut expr = parse_expr(child, source_bytes);
            if expr.0.is_some() {
              path_segments.push(PathSegment::EvalPath(expr.0.unwrap()));
            }
            level_errors.append(&mut expr.1);
          }
          _ if child.is_error() => level_errors.push(ErrorNode::new(child, source_bytes)),
          _ => return,
        });
    }
    _ if child.is_error() => level_errors.push(ErrorNode::new(child, source_bytes)),
    _ => return,
  });

  (path_segments, level_errors)
}

fn parse_expr_list<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Vec<ExprNode>, Vec<ErrorNode>) {
  let mut expr_nodes = vec![];
  let mut level_errors = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "expr" => {
      let mut res = parse_expr(child, source_bytes);
      if res.0.is_some() {
        expr_nodes.push(res.0.unwrap());
      }
      level_errors.append(&mut res.1);
    }
    _ => level_errors.push(ErrorNode::new(child, source_bytes)),
  });

  (expr_nodes, level_errors)
}

fn parse_literal<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return (None, vec![ErrorNode::new(node, source_bytes)]);
  }

  let child = children[0];

  let literal = match child.kind() {
    _ if child.is_missing() => return (None, vec![ErrorNode::new(child, source_bytes)]),
    "number" => Literal::new(
      LiteralType::Number(
        child
          .utf8_text(source_bytes)
          .unwrap()
          .parse::<f32>()
          .unwrap(),
      ),
      child,
    ),
    "true" => Literal::new(LiteralType::Bool(true), node),
    "false" => Literal::new(LiteralType::Bool(false), node),
    "null" => Literal::new(LiteralType::Null, child),
    "string" => Literal::new(
      LiteralType::String(child.utf8_text(source_bytes).unwrap().to_owned()),
      child,
    ),
    _ if child.is_error() => return (None, vec![ErrorNode::new(child, source_bytes)]),
    _ => return (None, vec![]),
  };

  (Some(ExprNode::new(Expr::Literal(literal), node)), vec![])
}

fn parse_variable<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let name = node.utf8_text(source_bytes).unwrap();
  let variable = Identifier::new(name, node);

  (Some(ExprNode::new(Expr::Variable(variable), node)), vec![])
}

fn parse_indexing<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 4 {
    return (None, vec![ErrorNode::new(node, source_bytes)]);
  }

  let object;
  let index;
  let mut level_errors = vec![];

  object = parse_expr(children[0], source_bytes);
  index = parse_expr(children[2], source_bytes);

  let expr = Expr::Indexing(object.0.map(Box::new), index.0.map(Box::new));

  level_errors = [level_errors, object.1, index.1]
    .into_iter()
    .flatten()
    .collect();

  (Some(ExprNode::new(expr, node)), level_errors)
}

fn parse_expr<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return (None, vec![ErrorNode::new(node, source_bytes)]);
  }

  let child = children[0];

  return match child.kind() {
    _ if child.is_missing() => (None, vec![ErrorNode::new(child, source_bytes)]),
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
    _ if child.is_error() => (None, vec![ErrorNode::new(child, source_bytes)]),
    _ => (None, vec![]),
  };
}

fn parse_member<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return (None, vec![ErrorNode::new(node, source_bytes)]);
  }

  let mut level_errors = vec![];

  let object_node = &children[0];

  let object = match object_node.kind() {
    "primary" => {
      let mut res = parse_primary(*object_node, source_bytes);

      level_errors.append(&mut res.1);
      res.0
    }
    "member" => {
      let mut res = parse_member(*object_node, source_bytes);
      level_errors.append(&mut res.1);
      res.0
    }
    _ => None,
  };

  let field_node = &children[2];
  let field = match field_node.kind() {
    "variable" => {
      let mut res = parse_variable(*field_node, source_bytes);
      level_errors.append(&mut res.1);
      res.0
    }
    "function_call" => {
      let mut res = parse_function_call(*field_node, source_bytes);
      level_errors.append(&mut res.1);
      res.0
    }
    _ => None,
  };

  let expr = Expr::Member(object.map(Box::new), field.map(Box::new));

  (Some(ExprNode::new(expr, node)), level_errors)
}

fn parse_ternary<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 5 {
    return (None, vec![ErrorNode::new(node, source_bytes)]);
  }

  let condition;
  let on_true;
  let on_false;
  let level_errors;

  condition = parse_expr(children[0], source_bytes);
  on_true = parse_expr(children[2], source_bytes);
  on_false = parse_expr(children[4], source_bytes);

  let expr = Expr::Ternary(
    condition.0.map(Box::new),
    on_true.0.map(Box::new),
    on_false.0.map(Box::new),
  );

  level_errors = [condition.1, on_true.1, on_false.1]
    .into_iter()
    .flatten()
    .collect();

  (Some(ExprNode::new(expr, node)), level_errors)
}

fn parse_binary<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return (None, vec![ErrorNode::new(node, source_bytes)]);
  }

  let operator1;
  let operation;
  let operator2;
  let mut level_errors = vec![];

  operator1 = parse_expr(children[0], source_bytes);
  operator2 = parse_expr(children[2], source_bytes);

  let op_node = children[1];

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
      level_errors.push(ErrorNode::new(op_node, source_bytes));
      None
    }
  };

  let expr = Expr::Binary(
    operation,
    operator1.0.map(Box::new),
    operator2.0.map(Box::new),
  );

  level_errors = [level_errors, operator1.1, operator2.1]
    .into_iter()
    .flatten()
    .collect();

  (Some(ExprNode::new(expr, node)), level_errors)
}

fn parse_unary<'b>(node: Node<'b>, source_bytes: &[u8]) -> (Option<ExprNode>, Vec<ErrorNode>) {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 2 {
    return (None, vec![ErrorNode::new(node, source_bytes)]);
  }

  let operation;
  let operator;
  let mut level_errors = vec![];

  operator = parse_expr(children[1], source_bytes);

  let op_node = children[0];

  operation = match op_node.kind() {
    "negation" => Some(Operation::Negation),
    "substraction" => Some(Operation::Substraction),
    "&&" => Some(Operation::And),
    _ => {
      level_errors.push(ErrorNode::new(op_node, source_bytes));
      None
    }
  };

  let expr = Expr::Unary(operation, operator.0.map(Box::new));

  level_errors = [level_errors, operator.1].into_iter().flatten().collect();

  (Some(ExprNode::new(expr, node)), level_errors)
}

fn parse_method<'b>(node: Node<'b>, source_bytes: &[u8]) -> Result<Method, ErrorNode> {
  let mut children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return Err(ErrorNode::new(node, source_bytes));
  }

  let child = children.pop().unwrap();

  let m_type = match child.kind() {
    "read" => MethodType::Read,
    "write" => MethodType::Write,
    "get" => MethodType::Get,
    "list" => MethodType::List,
    "create" => MethodType::Create,
    "update" => MethodType::Update,
    "delete" => MethodType::Delete,
    _ => MethodType::Unknown,
  };

  Ok(Method::new(m_type, child))
}
