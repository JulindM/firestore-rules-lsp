use std::{fs::File, io::Read, usize};

use tree_sitter::Point;

use crate::models::evaluation::evaluate_tree;

use super::{
  base::BaseModel,
  extensions::{get_lowest_denominator, EvaluatedTree},
};

pub fn token_type(tree: &EvaluatedTree, line: usize, offset: usize) -> Option<BaseModel> {
  let point = Point::new(line, offset);
  return get_lowest_denominator(point, BaseModel::MatchBody(tree.tree().body().clone()));
}

pub fn consume_file(parser: &mut tree_sitter::Parser, path: String) {
  let file_res = File::open(path);

  let mut file;

  match file_res {
    Ok(f) => file = f,
    Err(_) => panic!("File could not be read"),
  }

  let mut contents = vec![];

  let _ = file.read_to_end(&mut contents);

  let tree = parser.parse(contents.clone(), None).unwrap();
  let eval_tree = evaluate_tree(&tree, &contents).unwrap();

  println!("{}", eval_tree.definitions().len())
}
