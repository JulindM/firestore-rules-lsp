use lsp_types::{
  CompletionItem, CompletionItemKind, CompletionItemLabelDetails, Documentation, MarkupContent,
  MarkupKind, Position,
};
use tree_sitter::Point;

use crate::parser::{
  base::{BaseModel, Expr, HasChildren, ToBaseModel, TypeInferenceResult},
  types::{FirebaseTypeTrait, GLOBAL_FUNCTIONS, GLOBAL_VARIABLES, SPECIAL_KEYWORDS, VariableType},
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
) -> Option<(Option<&'a TypeInferenceResult>, BaseModel<'a>)> {
  if traversing_path.is_empty() {
    return None;
  }

  let mut traversal_not_last = traversing_path.clone();
  let last_opt = traversal_not_last.pop();

  if last_opt.is_none() {
    return None;
  }

  let last = last_opt.unwrap();

  let inference_info = match &last {
    BaseModel::ExprNode(expr) => expr.inferred_type(&traversal_not_last),
    BaseModel::Function(fun) => fun.return_type(&traversal_not_last),
    BaseModel::VariableDefinition(vd) => vd.variable_type(&traversal_not_last),
    _ => None,
  };

  Some((inference_info, last))
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

pub fn generate_scoped_autocompletions<'a>(
  traversing_path: &Vec<BaseModel<'a>>,
) -> Vec<CompletionItem> {
  let current_scope_variables = get_scoped_variables(traversing_path);
  let current_scope_functions = get_scoped_functions(traversing_path);
  let firestore_reserved_keywords = SPECIAL_KEYWORDS;

  let var_completions = current_scope_variables.iter().map(|var| CompletionItem {
    label: var.0.to_owned(),
    insert_text: Some(var.0.to_owned()),
    kind: Some(match var.2 {
      VariableType::Variable => CompletionItemKind::VARIABLE,
      VariableType::Module => CompletionItemKind::MODULE,
    }),
    // TODO: Add type to detail
    detail: Some("".to_owned()),
    ..Default::default()
  });

  let func_completions = current_scope_functions.iter().map(|fun| CompletionItem {
    label: fun.0.to_owned(),
    insert_text: Some(fun.0.to_owned()),
    kind: match fun.1 {
      true => Some(CompletionItemKind::FUNCTION),
      false => Some(CompletionItemKind::METHOD),
    },
    // TODO: Add parameters to detail
    detail: Some("()".to_owned()),
    ..Default::default()
  });

  let special_keyword_completions = firestore_reserved_keywords.iter().map(|kw| CompletionItem {
    label: kw.to_string(),
    insert_text: Some(kw.to_string()),
    kind: Some(CompletionItemKind::KEYWORD),
    ..Default::default()
  });

  return Vec::from_iter(
    var_completions.chain(func_completions.chain(special_keyword_completions)),
  );
}

pub fn get_possible_completions<'a>(traversing_path: &Vec<BaseModel<'a>>) -> Vec<CompletionItem> {
  let last_two = traversing_path.last_chunk::<2>();

  if last_two.is_none() {
    // We have no idea where we are
    return vec![];
  }

  let [second_last, last] = last_two.unwrap();

  if last.as_expr_node().is_none() || second_last.as_expr_node().is_none() {
    return generate_scoped_autocompletions(traversing_path);
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

  if request_on_member_object.is_none() {
    // We somehow are not requesting a member but the last two are expr nodes
    // No clue where we are
    return vec![];
  }

  let mut traversal_at_typable = traversing_path[..traversing_path.len() - 2].to_vec();
  traversal_at_typable.push(request_on_member_object.clone().unwrap());

  let definable = try_see_if_typable(&traversal_at_typable);

  if definable.is_none() {
    return vec![];
  }

  let typable = definable.unwrap().0;

  if typable.is_none() {
    return vec![];
  }

  let _type = typable.as_ref().unwrap().type_info().firebase_type();

  let properties = _type.properties();
  let props = properties.iter().map(|p| CompletionItem {
    label: p.0.to_owned(),
    label_details: Some(CompletionItemLabelDetails {
      detail: Some(format!(" {:?}", p.1.firebase_type())),
      description: None,
    }),
    insert_text: Some(p.0.to_owned()),
    documentation: Some(Documentation::MarkupContent(MarkupContent {
      kind: MarkupKind::Markdown,
      value: p.1.docstring().unwrap_or("").to_owned(),
    })),
    kind: Some(CompletionItemKind::FIELD),
    ..Default::default()
  });

  let methods = _type.methods();
  let methods = methods.iter().map(|p| {
    let params_markdown = p
      .1
      .iter()
      .map(|param| format!("{}: {:?}", param.name(), param.param_type().firebase_type()))
      .collect::<Vec<String>>()
      .join(", ");

    let method_doc = p.2.docstring().unwrap_or("").to_owned();

    CompletionItem {
      label: format!("{}", p.0.to_owned(),),
      label_details: Some(CompletionItemLabelDetails {
        detail: Some(format!("({}) → {:?}", params_markdown, p.2.firebase_type())),
        description: None,
      }),
      insert_text: Some(p.0.to_owned()),
      documentation: Some(Documentation::MarkupContent(MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!(
          "{}\n\n---\n\n#### Parameters\n`{}`",
          method_doc, params_markdown
        ),
      })),
      kind: Some(CompletionItemKind::METHOD),
      ..Default::default()
    }
  });

  return Vec::from_iter(props.chain(methods));
}

fn get_scoped_variables<'a>(
  traversing_path: &Vec<BaseModel<'a>>,
) -> Vec<(String, bool, VariableType)> {
  let mut scoped_vars = vec![];

  for node in traversing_path.iter().rev() {
    match node {
      BaseModel::FunctionBody(fun) => scoped_vars.extend(
        fun
          .variable_defs()
          .iter()
          .map(|def| (def.name().to_owned(), false, VariableType::Variable)),
      ),
      _ => continue,
    }
  }

  scoped_vars.extend(
    GLOBAL_VARIABLES
      .iter()
      .map(|def| (def.0.to_owned(), true, def.2.clone())),
  );

  scoped_vars
}

fn get_scoped_functions<'a>(traversing_path: &Vec<BaseModel<'a>>) -> Vec<(String, bool)> {
  let mut scoped_funs: Vec<(String, bool)> = vec![];

  for node in traversing_path.iter().rev() {
    match node {
      BaseModel::MatchBody(body) => scoped_funs.extend(
        body
          .functions()
          .iter()
          .map(|def| (def.name().to_owned(), false)),
      ),
      _ => continue,
    }
  }

  scoped_funs.extend(GLOBAL_FUNCTIONS.iter().map(|def| (def.0.to_owned(), true)));

  scoped_funs
}

type TraversableConsuming<'a, T> = fn(&Vec<BaseModel<'a>>) -> Option<T>;

pub fn bfs_execute_at<'a, T>(
  nestable: &'a dyn HasChildren<'a>,
  existing_traversal: &Vec<BaseModel<'a>>,
  consumers: &Vec<TraversableConsuming<'a, T>>,
) -> Vec<T> {
  let mut curr_traversal = existing_traversal.clone();

  curr_traversal.push(nestable.to_base_model());

  let mut curr_diagnostics: Vec<T> = vec![];

  consumers.iter().for_each(|consumer| {
    let result = consumer(&curr_traversal);
    if result.is_some() {
      curr_diagnostics.push(result.unwrap());
    }
  });

  for child in nestable.children() {
    let mut child_results = bfs_execute_at(child, &curr_traversal, consumers);

    if !child_results.is_empty() {
      curr_diagnostics.append(&mut child_results);
    }
  }

  curr_diagnostics
}

pub fn get_hover_result<'a>(traversing_path: &Vec<BaseModel<'a>>) -> Option<MarkupContent> {
  let hover_result = try_see_if_typable(traversing_path)
    .and_then(|res| res.0)
    .and_then(|t| Some((t.type_info().firebase_type(), t.type_info().docstring())));

  if hover_result.is_none() {
    return None;
  }

  let (fir_type, docstr) = hover_result.unwrap();

  Some(MarkupContent {
    kind: MarkupKind::Markdown,
    value: format!("`{:?}`\n\n---\n{}", fir_type, docstr.unwrap_or("")),
  })
}
