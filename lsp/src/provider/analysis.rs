use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use tree_sitter::Point;

use crate::parser::{
  base::{BaseModel, Children, Expr, MatchPathPartType, ToBaseModel},
  extensions::EvaluatedTree,
};

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

pub fn try_find_definition<'a>(traversal: &Vec<BaseModel<'a>>) -> Option<BaseModel<'a>> {
  if traversal.len() < 2 {
    return None;
  }

  let mut reverse_traversal = traversal.clone();
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

pub fn build_diagnostics(tree: &EvaluatedTree) -> Vec<Diagnostic> {
  let mut errors: Vec<Diagnostic> = vec![];

  for err_node in tree.error_nodes() {
    errors.push(Diagnostic {
      range: Range {
        start: to_position(err_node.start()),
        end: to_position(err_node.end()),
      },
      severity: Some(DiagnosticSeverity::ERROR),
      code: None,
      code_description: None,
      source: None,
      message: "Error".to_string(),
      related_information: None,
      tags: None,
      data: None,
    });
  }

  errors
}
