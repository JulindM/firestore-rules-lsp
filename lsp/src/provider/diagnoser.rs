use lsp_types::{Diagnostic, DiagnosticSeverity, Range};
use tree_sitter::Node;

use super::analysis::to_position;

pub fn diagnose_synatx_errors<'a>(node: Node<'a>) -> Vec<Diagnostic> {
  let mut errors: Vec<Diagnostic> = vec![];
  let mut level_cursor = node.walk();

  loop {
    let curr_node = level_cursor.node();

    let curr_node_error = is_parse_error(curr_node);

    if curr_node_error.is_none() {
      let mut curr_node_cursor = curr_node.walk();
      let curr_node_children = curr_node.children(&mut curr_node_cursor);

      if curr_node_children.len() != 0 {
        for child in curr_node_children {
          let mut child_errors = diagnose_synatx_errors(child);
          errors.append(&mut child_errors);
        }
      }
    } else {
      errors.push(
        curr_node_error
          .map(|err_node| Diagnostic {
            range: Range {
              start: to_position(err_node.start_position()),
              end: to_position(err_node.end_position()),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: None,
            message: get_err_msg(node),
            related_information: None,
            tags: None,
            data: None,
          })
          .unwrap(),
      );
    }

    let moved = level_cursor.goto_next_sibling();

    if !moved {
      break;
    }
  }

  errors
}

fn get_err_msg<'a>(node: Node<'a>) -> String {
  if node.is_error() {
    return "Error".to_owned();
  };

  if node.is_missing() {
    return format!("Missing {}", node.kind()).to_owned();
  };

  String::new()
}

fn is_parse_error<'a>(node: Node<'a>) -> Option<Node<'a>> {
  if node.is_error() {
    return Some(node);
  };

  if node.is_missing() {
    return Some(node);
  };

  None
}
