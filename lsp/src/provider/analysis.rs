use lsp_types::{CompletionItem, CompletionItemKind, Position};
use tree_sitter::Point;

use crate::parser::{
  base::{BaseModel, Expr, HasChildren, ToBaseModel},
  types::{FirebaseType, FirebaseTypeTrait},
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

pub fn try_see_if_typable<'a>(
  traversing_path: &Vec<BaseModel<'a>>,
) -> Option<(
  Option<(FirebaseType, Option<Result<BaseModel<'a>, String>>)>,
  BaseModel<'a>,
)> {
  if traversing_path.is_empty() {
    return None;
  }

  let last = traversing_path.last().unwrap();

  let mut traversal_not_last = traversing_path.clone();
  traversal_not_last.pop();

  match last {
    BaseModel::ExprNode(expr) => Some((expr.inferred_type(traversal_not_last), last.to_owned())),
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

pub fn get_possible_completions<'a>(traversing_path: &Vec<BaseModel<'a>>) -> Vec<CompletionItem> {
  let request_on_memeber_object = traversing_path
    .last_chunk::<3>()
    .and_then(|chunk| match (&chunk[0], &chunk[1], &chunk[2]) {
      (BaseModel::ExprNode(e1), BaseModel::ExprNode(e2), _) => {
        if let Expr::Member(object, _) = e1.expr() {
          if let Expr::MemberField { .. } = e2.expr() {
            return Some(object);
          }
        }
        None
      }
      _ => None,
    })
    .unwrap_or(&None)
    .to_owned();

  if request_on_memeber_object.is_none() {
    // Handle non member field auto-completions later
    return vec![];
  }

  let mut traversal_at_typable = traversing_path[..traversing_path.len() - 3].to_vec();
  let binding = request_on_memeber_object.unwrap();
  traversal_at_typable.push(binding.to_base_model());

  let definable = try_see_if_typable(&traversal_at_typable);

  if definable.is_none() {
    return vec![];
  }

  let typable = definable.unwrap().0;

  if typable.is_none() {
    return vec![];
  }

  let _type = typable.unwrap().0;

  let properties = _type.properties();
  let props = properties.iter().map(|p| CompletionItem {
    label: p.0.to_owned(),
    insert_text: Some(p.0.to_owned()),
    detail: Some(p.1.as_ref().to_owned()),
    kind: Some(CompletionItemKind::PROPERTY),
    ..Default::default()
  });

  let methods = _type.methods();
  let methods = methods.iter().map(|p| CompletionItem {
    label: format!(
      "{}{}",
      p.0.to_owned(),
      // TODO when parameters are there
      if p.2.is_empty() { "()" } else { "(...)" }
    ),
    insert_text: Some(format!(
      "{}{}",
      p.0.to_owned(),
      // TODO when parameters are there
      if p.2.is_empty() { "()" } else { "()" }
    )),
    detail: Some(p.1.as_ref().to_owned()),
    kind: Some(CompletionItemKind::FUNCTION),
    ..Default::default()
  });

  return Vec::from_iter(props.chain(methods));
}

type TraversableConsuming<'a, T> = fn(&Vec<BaseModel<'a>>) -> Option<T>;

pub fn bfs_execute_at<'a, T>(
  nestable: &'a dyn HasChildren<'a>,
  existing_traversal: &Vec<BaseModel<'a>>,
  function: TraversableConsuming<'a, T>,
) -> Vec<T> {
  // TODO avoid cloning here - use some sort of slice
  let mut curr_traversal = existing_traversal.clone();

  curr_traversal.push(nestable.to_base_model());

  let mut curr_diagnostics: Vec<T> = vec![];

  let result_opt = function(&curr_traversal);

  if let Some(result) = result_opt {
    curr_diagnostics.push(result);
  }

  for child in nestable.children() {
    let mut child_results = bfs_execute_at(child, &curr_traversal, function);

    if !child_results.is_empty() {
      curr_diagnostics.append(&mut child_results);
    }
  }

  curr_diagnostics
}
