mod models;

use std::{fs::File, io::Read};

use models::{analysis::token_type, evaluation::evaluate_tree};
use tree_sitter_firestore_rules;

pub fn main() {
  const PATH: &str = "../tree-sitter-firestore_rules/examples/example.rules";

  let file_res = File::open(PATH);

  let mut file;

  match file_res {
    Ok(f) => file = f,
    Err(_) => panic!("File could not be read"),
  }

  let mut contents = vec![];

  let _ = file.read_to_end(&mut contents);

  let mut parser = tree_sitter::Parser::new();

  let language = tree_sitter_firestore_rules::LANGUAGE;

  parser
    .set_language(&language.into())
    .expect("Error loading FirestoreRules parser");

  let tree = parser.parse(contents.clone(), None).unwrap();
  let eval_tree = evaluate_tree(&tree, &contents).unwrap();

  let test = token_type(&eval_tree, 38, 14);

  println!("{:?}", test.unwrap())
}
