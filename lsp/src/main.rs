use tree_sitter_firestore_rules;

pub fn main() {
  let code = r#"
  service cloud.firestore {}
  "#;

  let mut parser = tree_sitter::Parser::new();

  let language = tree_sitter_firestore_rules::LANGUAGE;

  parser
    .set_language(&language.into())
    .expect("Error loading FirestoreRules parser");
  let tree = parser.parse(code, None).unwrap();

  print!("{:#?}", tree.root_node());
}
