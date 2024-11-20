use std::iter::Inspect;

use lsp_types::{Diagnostic, DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::parser::base::{BaseModel, Children, FirestoreTree, ToBaseModel};

use super::analysis::{to_position, try_find_definition};

pub fn diagnose_syntax_errors<'a>(node: Node<'a>) -> Vec<Diagnostic> {
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
          let mut child_errors = diagnose_syntax_errors(child);
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

pub fn diagnose_linting_errors<'a>(tree: &FirestoreTree) -> Vec<Diagnostic> {
  if tree.body().is_none() {
    return vec![];
  }

  let body = tree.body().unwrap();
  diagnose(body, vec![])
}

pub fn diagnose<'a, 'b>(
  nestable: &'b dyn Children<'b>,
  traversal_list: Vec<&BaseModel<'a>>,
) -> Vec<Diagnostic> {
  let mut curr_diagnostics: Vec<Diagnostic> = vec![];

  let inspection = inspect(&nestable.to_base_model(), &traversal_list);

  if let Some(diagnostic) = inspection {
    curr_diagnostics.push(diagnostic);
  }

  for child in nestable.children() {
    let mut new_traversal = traversal_list.clone();
    let base_model = child.to_base_model();

    new_traversal.push(&base_model);

    let mut child_diagnostic = diagnose(child, new_traversal);
    curr_diagnostics.append(&mut child_diagnostic);
  }

  curr_diagnostics
}

fn inspect<'a, 'b>(
  model: &BaseModel<'b>,
  traversal_list: &'a Vec<&BaseModel<'a>>,
) -> Option<Diagnostic> {
  let mut path_traversal = traversal_list.clone();
  path_traversal.push(model);

  match model.type_str() {
    "FunctionCall" => {
      let def_hit = try_find_definition(&path_traversal);

      if let Some(_) = def_hit {
        return None;
      }

      let span = model.span();

      return Some(Diagnostic {
        range: Range {
          start: to_position(span.0),
          end: to_position(span.1),
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: None,
        message: "No function definition found".to_owned(),
        related_information: None,
        tags: None,
        data: None,
      });
    }
    _ => None,
  }
}
