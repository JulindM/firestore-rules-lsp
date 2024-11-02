use tree_sitter::{Node, Point};

use super::base::FirestoreTree;

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

  pub fn start(&self) -> Point {
    self.start
  }

  pub fn end(&self) -> Point {
    self.end
  }
}

#[derive(Debug)]
pub struct EvaluatedTree {
  tree: Option<FirestoreTree>,
  error_nodes: Vec<ErrorNode>,
}

impl EvaluatedTree {
  pub fn new(tree: Option<FirestoreTree>, errors: Vec<ErrorNode>) -> Self {
    Self {
      tree: tree.clone(),
      error_nodes: errors,
    }
  }

  pub fn error_nodes(&self) -> &[ErrorNode] {
    &self.error_nodes
  }

  pub fn tree(&self) -> Option<&FirestoreTree> {
    self.tree.as_ref()
  }
}
