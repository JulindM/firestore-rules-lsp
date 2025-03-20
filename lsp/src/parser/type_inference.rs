use std::collections::HashMap;

use crate::parser::base::HasChildren;

use super::base::{BaseModel, FirestoreTree, MatchBody};

pub fn infer_tree_types(tree: FirestoreTree) -> FirestoreTree {
  return tree;

  if tree.body().is_none() {
    return tree;
  }

  let body = tree.body().unwrap();

  let dependecy_map = infer_children_types_at(body.children(), vec![]);
}

fn infer_children_types_at<'a>(
  children: Vec<&'a dyn HasChildren<'a>>,
  parent_bodies: Vec<&'a MatchBody>,
) -> HashMap<&'a BaseModel<'a>, Vec<&'a BaseModel<'a>>> {
  todo!()
}
