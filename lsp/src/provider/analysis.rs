use lsp_types::{Diagnostic, Position};
use tree_sitter::{Point, Tree};

use crate::parser::base::{
  BaseModel, Expr, FirestoreTree, HasChildren, MatchPathPartType, ToBaseModel,
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

pub fn is_definable<'a>(model: &BaseModel<'a>) -> bool {
  match model {
    BaseModel::ExprNode(expr) => match expr.expr() {
      Expr::FunctionCall(_, _) => true,
      Expr::Variable(_) => true,
      _ => false,
    },
    _ => false,
  }
}

pub fn try_find_definition<'a>(traversing_path: &Vec<BaseModel<'a>>) -> Option<BaseModel<'a>> {
  if traversing_path.is_empty() {
    return None;
  }

  let mut reverse_traversal = traversing_path.clone();
  reverse_traversal.reverse();

  let innermost_identifiable = reverse_traversal
    .iter()
    .enumerate()
    .find(|(_, el)| is_definable(el));

  if innermost_identifiable.is_none() {
    return None;
  }

  let (hit_idx, to_identify) = innermost_identifiable.unwrap();

  let (_, to_look_in) = reverse_traversal.split_at(hit_idx + 1);

  let member_hits = to_look_in
    .iter()
    .filter(|el| match el {
      BaseModel::ExprNode(expr) => match expr.expr() {
        Expr::Member(_, _) => true,
        _ => false,
      },
      _ => false,
    })
    .count();

  if member_hits > 2 {
    return None;
  }

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
          BaseModel::Match(m) if m.path().is_some() => Some(
            m.path()
              .unwrap()
              .path_parts()
              .iter()
              .filter(|p| *p.pathpart_type() == MatchPathPartType::Document)
              .map(|val| val.to_base_model())
              .collect::<Vec<BaseModel<'a>>>(),
          ),
          BaseModel::MatchBody(mb) if !mb.functions().is_empty() => Some(
            mb.functions()
              .iter()
              .map(|f| f.to_base_model())
              .collect::<Vec<BaseModel<'a>>>(),
          ),
          BaseModel::ServiceBody(sb) => Some(
            [
              sb.functions()
                .iter()
                .map(|f| f.to_base_model())
                .collect::<Vec<BaseModel<'a>>>(),
              sb.variables().iter().map(|v| v.to_base_model()).collect(),
            ]
            .concat(),
          ),
          BaseModel::Function(f) if !f.parameters().is_empty() => Some(
            f.parameters()
              .iter()
              .map(|vd| vd.to_base_model())
              .collect::<Vec<BaseModel<'a>>>(),
          ),
          BaseModel::FunctionBody(fb) if !fb.variable_defs().is_empty() => Some(
            fb.variable_defs()
              .iter()
              .map(|vd| vd.to_base_model())
              .collect::<Vec<BaseModel<'a>>>(),
          ),
          _ => None,
        })
        .flatten()
        .find(|definition| match definition {
          BaseModel::VariableDefinition(vd) => vd.name().eq(ident.value()),
          BaseModel::Identifier(i) => i.value().eq(ident.value()),
          BaseModel::MatchPathPart(mpp) => mpp.value().eq(ident.value()),
          _ => false,
        }),
      _ => None,
    },
    _ => None,
  }
}

pub fn get_path_traversal<'a>(
  position: Position,
  starting: &'a dyn HasChildren<'a>,
) -> Vec<BaseModel<'a>> {
  let point = to_point(position);

  if !starting.contains(point) {
    return vec![];
  }

  let mut res = vec![starting.to_base_model()];

  let children = starting.children();

  let child_hit = children.into_iter().find(|el| el.contains(point));

  if child_hit.is_none() {
    return res;
  }

  let mut inner_hit = get_path_traversal(position, child_hit.unwrap());

  res.append(&mut inner_hit);

  res
}

pub fn build_diagnostics(tree: &Tree, firestore_tree: &FirestoreTree) -> Vec<Diagnostic> {
  let mut diagnostics: Vec<Diagnostic> = vec![];

  let mut syntax_errors = diagnose_syntax_errors(tree.root_node());
  let mut linting_warnings = diagnose_linting_errors(firestore_tree);

  diagnostics.append(&mut syntax_errors);
  diagnostics.append(&mut linting_warnings);

  diagnostics
}
