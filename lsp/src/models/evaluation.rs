use tree_sitter::{Node, Tree};

use crate::{FirestoreTree, Function, Match, MatchBody, MatchPath, MatchPathPart, Variable};

use super::extensions::{ErrorNode, EvaluatedTree};

const SERVICE_NAME: &str = "service_name";
const ROOT_MATCH: &str = "match_body";

pub fn evaluate_tree<'tree>(
  tree: &'tree Tree,
  source_bytes: &[u8],
) -> Result<EvaluatedTree<'tree>, String> {
  let source_node = tree.root_node();

  if source_node.kind() != "source_file" {
    panic!("No source detected")
  }

  let mut cursor = source_node.walk();

  cursor.goto_first_child();
  let mut curr_node = cursor.node();
  let mut curr_kind = curr_node.kind();

  if curr_kind != SERVICE_NAME {
    return Ok(gen_empty_tree(&curr_node));
  }

  cursor.goto_next_sibling();
  curr_node = cursor.node();
  curr_kind = curr_node.kind();

  if curr_kind != ROOT_MATCH {
    return Ok(gen_empty_tree(&curr_node));
  }

  cursor.goto_next_sibling();
  curr_node = cursor.node();
  curr_kind = curr_node.kind();

  if curr_kind != ROOT_MATCH {
    return Ok(gen_empty_tree(&curr_node));
  }

  let (root_match, errors) = parse_match_body(curr_node, source_bytes);
  let tree = FirestoreTree::new(root_match);

  return Ok(EvaluatedTree::new(tree, errors));
}

fn gen_empty_tree<'tree>(service_node: &Node<'tree>) -> EvaluatedTree<'tree> {
  let err = ErrorNode::new(*service_node);
  let empty_tree = FirestoreTree::new(MatchBody::empty());

  return EvaluatedTree::new(empty_tree, vec![err]);
}

fn parse_match_body<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (MatchBody, Vec<ErrorNode<'tree>>) {
  debug_entry("match_body", node);

  let (matches, match_errors) = node
    .children_by_field_name("match_def", &mut node.walk())
    .map(|m_node| parse_match_def(m_node, source_bytes))
    .fold((vec![], vec![]), |mut acc, mut val| {
      acc.0.push(val.0);
      acc.1.append(&mut val.1);
      acc
    });

  let (functions, function_errors) = node
    .children_by_field_name("function_def", &mut node.walk())
    .map(|f_node| parse_function(f_node, source_bytes))
    .fold((vec![], vec![]), |mut acc, mut val| {
      acc.0.push(val.0);
      acc.1.append(&mut val.1);
      acc
    });

  let errors = vec![match_errors, function_errors]
    .into_iter()
    .flatten()
    .collect();

  let root_match_body = MatchBody::new(functions, matches);

  (root_match_body, errors)
}

fn parse_match_def<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Match, Vec<ErrorNode<'tree>>) {
  debug_entry("match_def", node);
  let path;

  let path_node = node
    .children_by_field_name("path", &mut node.walk())
    .next()
    .unwrap();

  path = parse_match_path(path_node, source_bytes);

  let match_node = node
    .children_by_field_name("match_body", &mut node.walk())
    .next()
    .unwrap();

  let (match_body, errors) = parse_match_body(match_node, source_bytes);

  (Match::new(path, match_body), errors)
}

fn parse_function<'tree>(
  node: Node<'tree>,
  source_bytes: &[u8],
) -> (Function, Vec<ErrorNode<'tree>>) {
  debug_entry("function", node);

  let name = node
    .children_by_field_name("name", &mut node.walk())
    .next()
    .unwrap()
    .utf8_text(source_bytes)
    .unwrap();

  let parameters = node
    .children_by_field_name("param", &mut node.walk())
    .map(|p_node| Variable::new(p_node.utf8_text(source_bytes).unwrap()))
    .collect();

  (Function::new(String::from(name), parameters), vec![])
}

fn parse_match_path<'tree>(node: Node<'tree>, source_bytes: &[u8]) -> MatchPath {
  debug_entry("path", node);

  let mut path_parts = vec![];
  let mut cursor = node.walk();

  cursor.goto_first_child();

  loop {
    let curr_node = cursor.node();

    println!("{:#?}", curr_node.kind());

    match curr_node.kind() {
      "collection_path_seg" => {
        let name = curr_node.utf8_text(source_bytes).unwrap();
        let part = MatchPathPart::Collection(name.to_owned());
        path_parts.push(part);
      }
      "single_path_seg" => {
        let name = curr_node.utf8_text(source_bytes).unwrap();
        let part = MatchPathPart::SinglePath(name.to_owned());
        path_parts.push(part);
      }
      "multi_path_seg" => {
        let name = curr_node.utf8_text(source_bytes).unwrap();
        let part = MatchPathPart::MultiPath(name.to_owned());
        path_parts.push(part);
      }
      _ => continue,
    }

    if !cursor.goto_next_sibling() {
      break;
    }
  }

  MatchPath::new(path_parts)
}

fn debug_entry<'a>(parser: &str, node: Node<'a>) {
  println!(
    "Entering {:?} node type {:#} at {:#}",
    parser,
    node.kind(),
    node.start_position()
  );
}
