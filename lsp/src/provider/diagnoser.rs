use lsp_types::{Diagnostic, DiagnosticSeverity, Range};
use tree_sitter::{Node, Point, Tree};

use crate::{
  parser::{
    base::{BaseModel, RulesTree, ServiceType, Spanned},
    types::FirebaseType,
  },
  provider::analysis::try_see_if_typable,
};

use super::analysis::{bfs_execute_at, to_position};

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

type Diagnoser = fn(&Vec<BaseModel<'_>>) -> Option<Diagnostic>;

pub fn diagnose_linting_errors<'a>(tree: &'a RulesTree) -> Vec<Diagnostic> {
  if tree.body().is_none() || tree.service_type() != Some(&ServiceType::Firestore) {
    // Disable linting for non-firestore rules for now
    return vec![];
  }

  let body = tree.body().unwrap();

  let diagnosers: Vec<Diagnoser> = vec![find_missing_definitions, find_if_rule_expr_not_bool];

  bfs_execute_at(body, &vec![], &diagnosers)
}

fn find_if_rule_expr_not_bool(traversal_list: &Vec<BaseModel<'_>>) -> Option<Diagnostic> {
  let last_node = traversal_list.last();

  if last_node.is_none() {
    return None;
  }

  let node = last_node.unwrap();

  fn gen_diagnostic(fir_type: Option<FirebaseType>, span: (Point, Point)) -> Diagnostic {
    if fir_type.is_none() {
      return Diagnostic {
        range: Range {
          start: to_position(span.0),
          end: to_position(span.1),
        },
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: None,
        message: "Expected a boolean expression".to_owned(),
        related_information: None,
        tags: None,
        data: None,
      };
    } else {
      return Diagnostic {
        range: Range {
          start: to_position(span.0),
          end: to_position(span.1),
        },
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: None,
        message: format!(
          "Expected a boolean expression, found {:?}",
          fir_type.unwrap()
        ),
        related_information: None,
        tags: None,
        data: None,
      };
    }
  }

  match node {
    BaseModel::Rule(rule) => {
      let expr_type = rule.condition();

      if expr_type.is_none() {
        return None;
      }

      let expr = expr_type.unwrap();

      let expr_type = expr
        .inferred_type(traversal_list)
        .and_then(|v| Some(v.type_info().firebase_type()));

      if expr_type != Some(FirebaseType::Boolean) && expr_type != Some(FirebaseType::Any) {
        return Some(gen_diagnostic(expr_type, expr.span()));
      }

      None
    }
    _ => None,
  }
}

fn find_missing_definitions(traversal_list: &Vec<BaseModel<'_>>) -> Option<Diagnostic> {
  let typable = try_see_if_typable(traversal_list);

  if typable.is_none() {
    return None;
  }

  let typeable = typable.unwrap();

  if typeable.0.is_none() {
    return None;
  }

  let definition_node = &typeable.0.as_ref().unwrap().definition_location();

  if definition_node.is_none() {
    return None;
  }

  let definition_result = definition_node.as_ref().unwrap();

  if definition_result.is_ok() {
    return None;
  }

  let model = typeable.1;

  let err_str = definition_result.as_ref().err().unwrap();

  Some(Diagnostic {
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
  })
}

pub fn build_diagnostics(tree: &Tree, firestore_tree: &RulesTree) -> Vec<Diagnostic> {
  let mut diagnostics: Vec<Diagnostic> = vec![];

  let mut syntax_errors = diagnose_syntax_errors(tree.root_node());
  let mut linting_warnings = diagnose_linting_errors(firestore_tree);

  diagnostics.append(&mut syntax_errors);
  diagnostics.append(&mut linting_warnings);

  diagnostics
}
