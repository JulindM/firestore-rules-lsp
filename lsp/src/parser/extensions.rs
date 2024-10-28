use tree_sitter::{Node, Point};

use super::base::{
  BaseModel, Children, Expr, ExprNode, FirestoreTree, Function, ToBaseModel, VariableDefintion,
};

#[derive(Debug)]
pub struct ErrorNode {
  content: String,
  start: Point,
  end: Point,
}

impl ErrorNode {
  pub fn new<'b>(node: Node<'b>, source: &[u8]) -> Self {
    Self {
      content: node.utf8_text(source).unwrap_or("").to_owned(),
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

#[derive(Debug)]
pub struct EvaluatedTree {
  tree: FirestoreTree,
  error_nodes: Vec<ErrorNode>,
}

impl EvaluatedTree {
  pub fn new(tree: FirestoreTree, errors: Vec<ErrorNode>) -> Self {
    Self {
      tree: tree.clone(),
      error_nodes: errors,
    }
  }

  pub fn error_nodes(&self) -> &[ErrorNode] {
    &self.error_nodes
  }

  pub fn tree(&self) -> &FirestoreTree {
    &self.tree
  }
}

#[derive(Debug)]
pub enum DefinitionType {
  Function(Function),
  Variable(VariableDefintion),
  GlobalVariables(String),
}

pub fn get_lowest_denominator<'a>(
  position: Point,
  nestable: &'a dyn Children<'a>,
) -> Vec<BaseModel<'a>> {
  if !nestable.contains(position) {
    return vec![];
  }

  let mut res = vec![nestable.to_base_model()];

  let children = nestable.children();
  let child_hit = children.into_iter().find(|el| el.contains(position));

  if child_hit.is_none() {
    return res;
  }

  let mut inner_hit = get_lowest_denominator(position, child_hit.unwrap());

  res.append(&mut inner_hit);

  res
}

pub fn filter_children<'a, FRes>(
  field: &'a dyn Children<'a>,
  filter: &dyn Fn(BaseModel<'a>) -> Option<FRes>,
) -> Vec<FRes> {
  let mut result = vec![];

  let children = field.children();

  if !children.is_empty() {
    for child in children.into_iter() {
      let mut child_res = filter_children(child, filter);
      result.append(&mut child_res);
    }
  }

  result
}

pub fn try_find_definition<'a>(traversal: Vec<BaseModel<'a>>) -> Option<BaseModel<'a>> {
  if traversal.len() < 2 {
    return None;
  }

  let (left, child) = traversal.split_at(traversal.len() - 1);

  match child.first().unwrap() {
    BaseModel::ExprNode(node) => match node.expr() {
      Expr::FunctionCall(fname, _) => left
        .iter()
        .filter_map(|el| match el {
          BaseModel::MatchBody(mb) => Some(mb),
          _ => None,
        })
        .map(|el| el.functions())
        .flatten()
        .find(|f| f.name().eq(fname))
        .and_then(|f| Some(f.to_base_model())),
      _ => None,
    },
    _ => None,
  }
}
