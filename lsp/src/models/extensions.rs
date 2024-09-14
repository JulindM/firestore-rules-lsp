use tree_sitter::{Node, Point};

use crate::FirestoreTree;

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
}
