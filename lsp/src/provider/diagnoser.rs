use lsp_types::{Diagnostic, DiagnosticSeverity, Range};
use tree_sitter::{Node, Tree};

use crate::parser::base::{BaseModel, Expr, FirestoreTree};

use super::analysis::{bfs_execute_at, get_definable_expr, to_position, try_find_definition};

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

pub fn diagnose_linting_errors<'a>(tree: &'a FirestoreTree) -> Vec<Diagnostic> {
  if tree.body().is_none() {
    return vec![];
  }

  let body = tree.body().unwrap();
  bfs_execute_at(body, &vec![], find_missing_definitions)
}

fn find_missing_definitions<'a>(traversal_list: &'a Vec<BaseModel<'a>>) -> Option<Diagnostic> {
  match traversal_list.last() {
    Some(model) => {
      let expr = get_definable_expr(model);

      if expr.is_none() {
        return None;
      }

      let err_str = match expr.unwrap() {
        Expr::FunctionCall(f_ident, _, _) => {
          format!("No function definition found for {}", f_ident.name())
        }
        Expr::Variable(var_ident, _) => {
          format!("No variable definition found for {}", var_ident.name())
        }
        _ => "".to_owned(),
      };

      let hit = try_find_definition(traversal_list);

      match hit {
        Ok(None) => Some(Diagnostic {
          range: Range {
            start: to_position(model.span().0),
            end: to_position(model.span().1),
          },
          severity: Some(DiagnosticSeverity::ERROR),
          code: None,
          code_description: None,
          source: None,
          message: err_str.to_owned(),
          related_information: None,
          tags: None,
          data: None,
        }),
        _ => None,
      }
    }
    _ => None,
  }
}

pub fn build_diagnostics(tree: &Tree, firestore_tree: &FirestoreTree) -> Vec<Diagnostic> {
  let mut diagnostics: Vec<Diagnostic> = vec![];

  let mut syntax_errors = diagnose_syntax_errors(tree.root_node());
  let mut linting_warnings = diagnose_linting_errors(firestore_tree);

  diagnostics.append(&mut syntax_errors);
  diagnostics.append(&mut linting_warnings);

  diagnostics
}
