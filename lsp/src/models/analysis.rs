use std::usize;

use tree_sitter::Point;

use super::{
  base::BaseModel,
  extensions::{get_lowest_denominator, EvaluatedTree},
};

pub fn token_type(tree: &EvaluatedTree, line: usize, offset: usize) -> Option<BaseModel> {
  let point = Point::new(line, offset);
  return get_lowest_denominator(point, BaseModel::MatchBody(tree.tree().body().clone()));
}
