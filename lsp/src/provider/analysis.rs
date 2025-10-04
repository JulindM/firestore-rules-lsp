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
  Option<(FirebaseType, Option<Result<(Point, Point), String>>)>,
  BaseModel<'a>,
)> {
  if traversing_path.is_empty() {
    return None;
  }

  let mut traversal_not_last = traversing_path.clone();
  let last = traversal_not_last.pop();

  match last {
    Some(BaseModel::ExprNode(expr)) => {
      Some((expr.inferred_type(traversal_not_last), last.unwrap()))
    }
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
  let last_two = traversing_path.last_chunk::<2>();

  if last_two.is_none() {
    return vec![];
  }

  let [second_last, last] = last_two.unwrap();

  if last.as_expr_node().is_none() || second_last.as_expr_node().is_none() {
    // Handle non expr node auto-completions later
    return vec![];
  }

  let last_expr = last.as_expr_node().unwrap().expr();
  let second_last_expr = second_last.as_expr_node().unwrap().expr();

  let request_on_member_object = match second_last_expr {
    Expr::Member(obj, _) => {
      if obj.is_none() {
        None
      } else {
        match last_expr {
          Expr::MemberFunction(_, _) | Expr::MemberVariable(_) => {
            Some(obj.as_ref().unwrap().to_base_model())
          }
          _ => None,
        }
      }
    }
    _ => None,
  };

  eprintln!("{:?}", traversing_path);

  if request_on_member_object.is_none() {
    // Handle non member field auto-completions later
    return vec![];
  }

  let mut traversal_at_typable = traversing_path[..traversing_path.len() - 2].to_vec();
  traversal_at_typable.push(request_on_member_object.unwrap());

  eprintln!("{:?}", traversal_at_typable);

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
