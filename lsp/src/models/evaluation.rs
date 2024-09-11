use tree_sitter::{Node, Tree};

use crate::{
  Expr, FirestoreTree, Function, FunctionBody, Match, MatchBody, MatchPath, MatchPathPart, Method,
  Rule, Variable, VariableDefintion,
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
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      //todo
      "function_body" => {
        let mut res = parse_function_body(child, source_bytes);
        body = res.0;
        level_errors.append(&mut res.1);
      }
      _ => return,
    });

  (Function::new(name, parms, body), vec![])
}

fn parse_function_body<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (FunctionBody, Vec<ErrorNode<'tree>>) {
  let mut variables = vec![];
  let mut ret = Expr::new();
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
      "ret_expr" => {
        let mut res = parse_expr(child);
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
  let mut expr = Expr::new();
  let mut level_errors = vec![];

  let mut cursor = node.walk();

  node
    .children(&mut cursor)
    .for_each(|child| match child.kind() {
      "variable" => name = child.utf8_text(source_bytes).unwrap(),
      "ret_expr" => {
        let mut res = parse_expr(child);
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
      _ => todo!(),
    });

  (MatchPath::new(path_parts), level_errors)
}

fn parse_rule<'tree>(node: Node<'tree>, source_bytes: &[u8]) -> (Rule, Vec<ErrorNode<'tree>>) {
  let mut methods = vec![];
  let mut condition: Option<Expr> = None;
  let mut level_errors = vec![];

  node
    .children(&mut node.walk())
    .for_each(|child| match child.kind() {
      "method" => match parse_method(child) {
        Ok(meth) => methods.push(meth),
        Err(err) => level_errors.push(err),
      },
      "condition" => {
        let mut res = parse_expr(child);
        condition = Some(res.0);
        level_errors.append(&mut res.1);
      }
      _ if child.is_error() => level_errors.push(ErrorNode::new(child)),
      _ => return,
    });

  (Rule::new(methods, condition), level_errors)
}

fn parse_expr<'tree>(condition: Node<'tree>) -> (Expr, Vec<ErrorNode>) {
  //TODO
  return (Expr::new(), vec![]);
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
