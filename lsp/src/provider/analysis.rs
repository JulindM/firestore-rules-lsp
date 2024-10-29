use tree_sitter::Point;

use crate::parser::base::{BaseModel, Children, Expr, MatchPathPartType, ToBaseModel};

pub fn try_find_definition<'a>(traversal: Vec<BaseModel<'a>>) -> Option<BaseModel<'a>> {
  if traversal.len() < 2 {
    return None;
  }

  let (left, child) = traversal.split_at(traversal.len() - 1);

  match child.first().unwrap() {
    BaseModel::ExprNode(node) => match node.expr() {
      Expr::FunctionCall(fname, _) => left
        .iter()
        .filter_map(|el| match el {
          BaseModel::MatchBody(mb) => Some(mb),
          _ => None,
        })
        .map(|el| el.functions())
        .flatten()
        .find(|f| f.name().eq(fname))
        .and_then(|f| Some(f.to_base_model())),
      Expr::Variable(ident) => left
        .iter()
        .filter_map(|el| match el {
          BaseModel::Match(mb) => mb.path().map(|p| {
            p.path_parts()
              .iter()
              .filter(|p| *p.pathpart_type() == MatchPathPartType::SinglePath)
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
          BaseModel::VariableDefintion(vd) => vd.name().eq(ident.name()),
          BaseModel::MatchPathPart(mpp) => mpp.value().eq(ident.name()),
          _ => false,
        }),
      _ => None,
    },
    _ => None,
  }
}

pub fn get_lowest_denominator<'a>(
  position: Point,
  nestable: &'a dyn Children<'a>,
) -> Vec<BaseModel<'a>> {
  if !nestable.contains(position) {
    return vec![];
  }

  let mut res = vec![nestable.to_base_model()];

  let children = nestable.children();
  let child_hit = children.into_iter().find(|el| el.contains(position));

  if child_hit.is_none() {
    return res;
  }

  let mut inner_hit = get_lowest_denominator(position, child_hit.unwrap());

  res.append(&mut inner_hit);

  res
}
