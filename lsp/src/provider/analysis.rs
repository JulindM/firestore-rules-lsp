use lsp_types::{CompletionItem, CompletionItemKind, Position};
use tree_sitter::Point;

use crate::parser::{
  base::{BaseModel, DefinableIdentifier, Expr, HasChildren, MatchPathPartType, ToBaseModel},
  types::FirebaseTypeTrait,
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

pub fn get_definable_expr<'a>(model: &BaseModel<'a>) -> Option<&'a Expr> {
  match model {
    BaseModel::ExprNode(expr) => {
      let expression = expr.expr();

      return match &expression {
        Expr::FunctionCall(name_locality, _, _) => match name_locality {
          DefinableIdentifier::Yes(_) => Some(expression),
          _ => None,
        },
        Expr::Variable(variable_locality, _) => match variable_locality {
          DefinableIdentifier::Yes(_) => Some(expression),
          _ => None,
        },
        _ => None,
      };
    }
    _ => None,
  }
}

pub fn try_find_definition<'a>(
  traversing_path: &Vec<BaseModel<'a>>,
) -> Result<Option<BaseModel<'a>>, ()> {
  if traversing_path.is_empty() {
    return Err(());
  }

  let mut reverse_traversal = traversing_path.clone();
  reverse_traversal.reverse();

  let innermost_identifiable = reverse_traversal
    .iter()
    .enumerate()
    .find_map(|(i, el)| get_definable_expr(el).map_or(None, |expr| Some((i, expr))));

  if innermost_identifiable.is_none() {
    return Err(());
  }

  let (hit_idx, to_identify) = innermost_identifiable.unwrap();

  let (_, to_look_in) = reverse_traversal.split_at(hit_idx + 1);

  let hit = match to_identify {
    Expr::FunctionCall(f_identifier, _, _) => to_look_in
      .iter()
      .filter_map(|el| match el {
        BaseModel::ServiceBody(sb) => Some(sb.functions()),
        BaseModel::MatchBody(mb) => Some(mb.functions()),
        _ => None,
      })
      .flatten()
      .find(|f| f.name().eq(f_identifier.name()))
      .and_then(|f| Some(f.to_base_model())),
    Expr::Variable(DefinableIdentifier::Yes(ident), _) => to_look_in
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
  };

  Ok(hit)
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
  let mut expressions = traversing_path.into_iter().rev().filter_map(|b| {
    if let BaseModel::ExprNode(expr) = b {
      return Some(expr);
    } else {
      return None;
    }
  });

  while let Some(val) = expressions.next() {
    if let Expr::MemberField(_) = val.expr() {
      break;
    } else {
      continue;
    }
  }

  let member_opt = expressions.next();

  let type_hit_opt = match member_opt {
    Some(node) => match node.expr() {
      Expr::Member(Some(obj), _) => obj.inferred_type(),
      _ => None,
    },
    _ => None,
  };

  if type_hit_opt.is_none() {
    return vec![];
  }

  let type_hit = type_hit_opt.unwrap();

  let properties = type_hit.get().properties();
  let props = properties.iter().map(|p| CompletionItem {
    label: p.0.to_owned(),
    insert_text: Some(p.0.to_owned()),
    detail: Some(p.1.as_ref().to_owned()),
    kind: Some(CompletionItemKind::PROPERTY),
    ..Default::default()
  });

  let methods = type_hit.get().methods();
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
