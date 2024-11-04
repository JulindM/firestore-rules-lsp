use tree_sitter::{Node, Point};

use super::base::FirestoreTree;

#[derive(Debug, Clone)]
pub enum ErrorNodeType {
  Error,
  Missing,
}

#[derive(Debug, Clone)]
pub struct SemanticError {
  content: String,
  error_type: ErrorNodeType,
  start: Point,
  end: Point,
}

impl SemanticError {
  pub fn new<'b>(node: Node<'b>, error_type: ErrorNodeType, source: &[u8]) -> Self {
    Self {
      content: node.utf8_text(source).unwrap_or("").to_owned(),
      error_type,
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
  semantic_errors: Vec<SemanticError>,
}

impl EvaluatedTree {
  pub fn new(tree: Option<FirestoreTree>, errors: Vec<SemanticError>) -> Self {
    Self {
      tree: tree.clone(),
      semantic_errors: errors,
    }
  }

  pub fn semantic_errors(&self) -> &[SemanticError] {
    &self.semantic_errors
  }

  pub fn tree(&self) -> Option<&FirestoreTree> {
    self.tree.as_ref()
  }
}
