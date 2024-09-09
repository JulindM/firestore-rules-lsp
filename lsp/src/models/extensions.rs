use tree_sitter::Node;

use crate::FirestoreTree;

#[derive(Debug)]
pub struct ErrorNode<'a> {
  node: Node<'a>,
}

impl<'a> ErrorNode<'a> {
  pub fn new(node: Node<'a>) -> Self {
    Self { node: node }
  }
}

#[derive(Debug)]
pub struct EvaluatedTree<'a> {
  tree: FirestoreTree,
  error_nodes: Vec<ErrorNode<'a>>,
}

impl<'a> EvaluatedTree<'a> {
  pub fn new(tree: FirestoreTree, errors: Vec<ErrorNode<'a>>) -> Self {
    Self {
      tree,
      error_nodes: errors,
    }
  }
}
