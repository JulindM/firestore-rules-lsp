use std::vec;

use tree_sitter::{Node, Tree};

use crate::parser::base::MethodType;

use super::base::{
  Expr, ExprNode, FirestoreTree, Function, FunctionArgument, FunctionBody, Identifier, Literal,
  LiteralType, Match, MatchBody, MatchPath, MatchPathPart, MatchPathPartType, Method, Operation,
  PathSegment, Rule, VariableDefintion,
};

macro_rules! sanitized_children {
  ($node: ident) => {
    $node.children(&mut $node.walk()).filter(|n| !n.is_extra())
  };
}

pub fn evaluate_tree(tree: Tree, source_bytes: &[u8]) -> FirestoreTree {
  let node = tree.root_node();

  if node.kind() != "source_file" {
    return FirestoreTree::new(None);
  }

  let mut match_body = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_body" => {
      match_body = Some(parse_match_body(None, child, source_bytes));
    }
    _ => return,
  });

  FirestoreTree::new(match_body)
}

fn parse_match_body<'a, 'b>(
  curr_match_parent: Option<&'a Match>,
  node: Node<'b>,
  source_bytes: &[u8],
) -> MatchBody {
  let mut matches = vec![];
  let mut functions = vec![];
  let mut rules = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_def" => matches.push(parse_match_def(curr_match_parent, child, source_bytes)),
    "function_def" => functions.push(parse_function_def(child, source_bytes)),
    "rule_def" => rules.push(parse_rule(child, source_bytes)),
    _ => return,
  });

  MatchBody::new(functions, matches, rules, node)
}

fn parse_match_def<'a, 'b>(
  curr_parent_match: Option<&'a Match>,
  node: Node<'b>,
  source_bytes: &[u8],
) -> Match {
  let mut path = None;
  let mut body = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_path" => path = Some(parse_match_path(child, source_bytes)),
    "match_body" => body = Some(parse_match_body(curr_parent_match, child, source_bytes)),
    _ => return,
  });

  Match::new(path, body, node)
}

fn parse_function_def<'b>(node: Node<'b>, source_bytes: &[u8]) -> Function {
  let mut name = "";
  let mut parms = vec![];
  let mut body = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "function_name" => name = child.utf8_text(source_bytes).unwrap(),
    "param" => parms.push(Identifier::new(
      child.utf8_text(source_bytes).unwrap(),
      child,
    )),
    "function_body" => body = Some(parse_function_body(child, source_bytes)),
    _ => return,
  });

  Function::new(name, parms, body, node)
}

fn parse_function_body<'b>(node: Node<'b>, source_bytes: &[u8]) -> FunctionBody {
  let mut variables = vec![];
  let mut ret = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "variable_def" => variables.push(parse_variable_def(child, source_bytes)),
    "fun_return" => ret = parse_fun_return(child, source_bytes),
    _ => return,
  });

  FunctionBody::new(variables, ret, node)
}

fn parse_fun_return<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut expr = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "expr" => expr = parse_expr(child, source_bytes),
    _ => return,
  });

  expr
}

fn parse_variable_def<'b>(node: Node<'b>, source_bytes: &[u8]) -> VariableDefintion {
  let mut name = "";
  let mut expr = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "variable" => name = child.utf8_text(source_bytes).unwrap(),
    "expr" => expr = parse_expr(child, source_bytes),
    _ => return,
  });

  VariableDefintion::new(name, expr, node)
}

fn parse_match_path<'b>(node: Node<'b>, source_bytes: &[u8]) -> MatchPath {
  let mut path_parts = vec![];

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
    _ => return,
  });

  MatchPath::new(path_parts, node)
}

fn parse_rule<'b>(node: Node<'b>, source_bytes: &[u8]) -> Rule {
  let mut methods = vec![];
  let mut condition: Option<ExprNode> = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "method" => match parse_method(child) {
      Some(meth) => methods.push(meth),
      None => return,
    },
    "expr" => condition = parse_expr(child, source_bytes),
    _ => return,
  });

  Rule::new(methods, condition, node)
}

fn parse_primary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let child = node.child(0).unwrap();

  match child.kind() {
    "literal" => parse_literal(child, source_bytes),
    "variable" => parse_variable(child, source_bytes),
    "expr_group" => parse_expr(child.child(1).unwrap(), source_bytes),
    "function_call" => parse_function_call(child, source_bytes),
    "list" => parse_list(child, source_bytes),
    _ => None,
  }
}

fn parse_list<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut list_elements = vec![];

  for child in sanitized_children!(node) {
    match child.kind() {
      "expr" => {
        let res = parse_expr(child, source_bytes);
        if res.is_some() {
          list_elements.push(res.unwrap());
        }
      }
      _ => return None,
    }
  }

  Some(ExprNode::new(Expr::List(list_elements), node))
}

fn parse_function_call<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut args = vec![];
  let mut name = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "function_calling_name" => {
      name = Some(Identifier::new(
        child.utf8_text(source_bytes).unwrap(),
        child,
      ));
    }
    "function_argument" => {
      let res = parse_function_arg(child, source_bytes);
      if res.is_some() {
        args.push(res.unwrap());
      }
    }
    _ => return,
  });

  Some(ExprNode::new(Expr::FunctionCall(name.unwrap(), args), node))
}

fn parse_function_arg<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<FunctionArgument> {
  for child in sanitized_children!(node) {
    match child.kind() {
      "expr" => {
        let res = parse_expr(child, source_bytes);
        return res.and_then(|node| Some(FunctionArgument::Expr(node)));
      }
      "path" => {
        let res = parse_path(child, source_bytes);
        return Some(FunctionArgument::Path(res));
      }
      _ => return None,
    }
  }

  None
}

fn parse_path<'b>(node: Node<'b>, source_bytes: &[u8]) -> Vec<PathSegment> {
  let mut path_segments = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "path_segment" => {
      sanitized_children!(child).for_each(|child| match child.kind() {
        "identifier" => {
          let name = child.utf8_text(source_bytes).unwrap();
          path_segments.push(PathSegment::String(Identifier::new(name, child)))
        }
        "expr" => {
          let expr = parse_expr(child, source_bytes);
          if expr.is_some() {
            path_segments.push(PathSegment::EvalPath(expr.unwrap()));
          }
        }
        _ => return,
      });
    }
    _ => return,
  });

  path_segments
}

fn parse_literal<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return None;
  }

  let child = children[0];

  let literal = match child.kind() {
    "number" => Some(Literal::new(
      LiteralType::Number(
        child
          .utf8_text(source_bytes)
          .unwrap()
          .parse::<f32>()
          .unwrap(),
      ),
      child,
    )),
    "true" => Some(Literal::new(LiteralType::Bool(true), node)),
    "false" => Some(Literal::new(LiteralType::Bool(false), node)),
    "null" => Some(Literal::new(LiteralType::Null, child)),
    "string" => Some(Literal::new(
      LiteralType::String(child.utf8_text(source_bytes).unwrap().to_owned()),
      child,
    )),
    _ => None,
  };

  literal.and_then(|lit| Some(ExprNode::new(Expr::Literal(lit), node)))
}

fn parse_variable<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let name = node.utf8_text(source_bytes).unwrap();
  let variable = Identifier::new(name, node);

  Some(ExprNode::new(Expr::Variable(variable), node))
}

fn parse_indexing<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 4 {
    return None;
  }

  let object_node = &children[0];

  let object = match object_node.kind() {
    "primary" => parse_primary(*object_node, source_bytes),
    "member" => parse_member(*object_node, source_bytes),
    _ => None,
  };

  let field_node = &children[2];
  let field = match field_node.kind() {
    "expr" => parse_variable(*field_node, source_bytes),
    _ => None,
  };

  let expr = Expr::Indexing(object.map(Box::new), field.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_expr<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return None;
  }

  let child = children[0];

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
    _ => None,
  };
}

fn parse_member<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return None;
  }

  let object_node = &children[0];

  let object = match object_node.kind() {
    "primary" => parse_primary(*object_node, source_bytes),
    "member" => parse_member(*object_node, source_bytes),
    _ => None,
  };

  let field_node = &children[2];
  let field = match field_node.kind() {
    "variable" => parse_variable(*field_node, source_bytes),
    "function_call" => parse_function_call(*field_node, source_bytes),
    _ => None,
  };

  let expr = Expr::Member(object.map(Box::new), field.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_ternary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 5 {
    return None;
  }

  let condition;
  let on_true;
  let on_false;

  condition = parse_expr(children[0], source_bytes);
  on_true = parse_expr(children[2], source_bytes);
  on_false = parse_expr(children[4], source_bytes);

  let expr = Expr::Ternary(
    condition.map(Box::new),
    on_true.map(Box::new),
    on_false.map(Box::new),
  );

  Some(ExprNode::new(expr, node))
}

fn parse_binary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return None;
  }

  let operator1 = parse_expr(children[0], source_bytes);
  let operator2 = parse_expr(children[2], source_bytes);

  let op_node = children[1];

  let operation = match op_node.kind() {
    "+" => Some(Operation::Addition),
    "-" => Some(Operation::Substraction),
    "&&" => Some(Operation::And),
    "||" => Some(Operation::Or),
    "*" => Some(Operation::Multiplication),
    "/" => Some(Operation::Division),
    "%" => Some(Operation::Modulo),
    "<" | "<=" | ">=" | ">" | "==" | "!=" | "in" => Some(Operation::Relation),
    _ => None,
  };

  let expr = Expr::Binary(operation, operator1.map(Box::new), operator2.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_unary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 2 {
    return None;
  }

  let expr_res = parse_expr(children[1], source_bytes);

  let op_node = children[0];

  let operation = match op_node.kind() {
    "!" => Some(Operation::Negation),
    "-" => Some(Operation::Substraction),
    _ => None,
  };

  let expr = Expr::Unary(operation, expr_res.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_method<'b>(node: Node<'b>) -> Option<Method> {
  let mut children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return None;
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

  Some(Method::new(m_type, child))
}
