use tree_sitter::{Node, Point};

use super::base::{BaseModel, Contains, FirestoreTree};

#[derive(Debug)]
pub struct ErrorNode {
  content: String,
  start: Point,
  end: Point,
}

impl ErrorNode {
  pub fn new(node: Node, source: &[u8]) -> Self {
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
      tree,
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

pub fn get_lowest_denominator(position: Point, field: BaseModel) -> Option<BaseModel> {
  if !field.contains(position) {
    return None;
  }

  let children = field.children();

  if children.is_empty() {
    return Some(field);
  }

  for child in children.into_iter() {
    if child.contains(position) {
      return get_lowest_denominator(position, child);
    }
  }

  return None;
}
