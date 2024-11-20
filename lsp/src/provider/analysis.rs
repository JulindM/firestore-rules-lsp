use lsp_types::{Diagnostic, Position};
use tree_sitter::{Point, Tree};

use crate::parser::base::{
  BaseModel, Children, Expr, FirestoreTree, MatchPathPartType, ToBaseModel,
};

use super::diagnoser::{diagnose_linting_errors, diagnose_syntax_errors};

pub fn to_point(position: Position) -> Point {
  Point::new(
    position.line.try_into().unwrap(),
    position.character.try_into().unwrap(),
  )
}

pub fn to_position(point: Point) -> Position {
  Position::new(
    point.row.try_into().unwrap(),
    point.column.try_into().unwrap(),
  )
}

pub fn try_find_definition<'a>(traversing_path: &Vec<&BaseModel<'a>>) -> Option<BaseModel<'a>> {
  if traversing_path.len() < 2 {
    return None;
  }

  let mut reverse_traversal = traversing_path.clone();
  reverse_traversal.reverse();

  let innermost_identifiable = reverse_traversal.iter().enumerate().find(|el| match el {
    (_, BaseModel::ExprNode(expr)) => match expr.expr() {
      Expr::FunctionCall(_, _) => true,
      Expr::Variable(_) => true,
      _ => false,
    },
    _ => false,
  });

  if innermost_identifiable.is_none() {
    return None;
  }

  let (hit_idx, to_identify) = innermost_identifiable.unwrap();

  let (_, to_look_in) = reverse_traversal.split_at(hit_idx + 1);

  match to_identify {
    BaseModel::ExprNode(node) => match node.expr() {
      Expr::FunctionCall(fname, _) => to_look_in
        .iter()
        .filter_map(|el| match el {
          BaseModel::MatchBody(mb) => Some(mb),
          _ => None,
        })
        .map(|el| el.functions())
        .flatten()
        .find(|f| f.name().eq(fname.value()))
        .and_then(|f| Some(f.to_base_model())),
      Expr::Variable(ident) => to_look_in
        .iter()
        .filter_map(|el| match el {
          BaseModel::Match(mb) => mb.path().map(|p| {
            p.path_parts()
              .iter()
              .filter(|p| *p.pathpart_type() == MatchPathPartType::Document)
              .map(|val| val.to_base_model())
              .collect::<Vec<BaseModel<'a>>>()
          }),
          BaseModel::FunctionBody(fb) if fb.variable_defs().len() > 0 => Some(
            fb.variable_defs()
              .iter()
              .map(|vd| vd.to_base_model())
              .collect::<Vec<BaseModel<'a>>>(),
          ),
          _ => None,
        })
        .flatten()
        .find(|definition| match definition {
          BaseModel::VariableDefintion(vd) => vd.name().eq(ident.value()),
          BaseModel::MatchPathPart(mpp) => mpp.value().eq(ident.value()),
          _ => false,
        }),
      _ => None,
    },
    _ => None,
  }
}

pub fn get_lowest_denominator<'a>(
  position: Position,
  nestable: &'a dyn Children<'a>,
) -> Vec<BaseModel<'a>> {
  let point = to_point(position);

  if !nestable.contains(point) {
    return vec![];
  }

  let mut res = vec![nestable.to_base_model()];

  let children = nestable.children();

  let child_hit = children.into_iter().find(|el| el.contains(point));

  if child_hit.is_none() {
    return res;
  }

  let mut inner_hit = get_lowest_denominator(position, child_hit.unwrap());

  res.append(&mut inner_hit);

  res
}

pub fn build_diagnostics(tree: &Tree, firestore_tree: Option<&FirestoreTree>) -> Vec<Diagnostic> {
  let mut diagnostics: Vec<Diagnostic> = vec![];

  let mut syntax_errors = diagnose_syntax_errors(tree.root_node());

  let mut linting_warnings = vec![];
  if firestore_tree.is_some() {
    linting_warnings = diagnose_linting_errors(firestore_tree.unwrap());
  }

  diagnostics.append(&mut syntax_errors);
  diagnostics.append(&mut linting_warnings);

  diagnostics
}
