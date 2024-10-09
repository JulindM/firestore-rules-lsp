use tree_sitter::{Node, Point};

use super::base::{BaseModel, Contains, FirestoreTree, Function, VariableDefintion};

#[derive(Debug)]
pub struct ErrorNode {
  content: String,
  start: Point,
  end: Point,
}

impl ErrorNode {
  pub fn new(node: Node, source: &[u8]) -> Self {
    Self {
      content: node.utf8_text(source).unwrap_or("").to_owned(),
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

#[derive(Debug)]
pub struct EvaluatedTree {
  tree: FirestoreTree,
  error_nodes: Vec<ErrorNode>,
  definitions: Vec<DefinitionType>,
}

impl EvaluatedTree {
  pub fn new(tree: FirestoreTree, errors: Vec<ErrorNode>) -> Self {
    Self {
      tree: tree.clone(),
      error_nodes: errors,
      definitions: calculate_definitions(&tree),
    }
  }

  pub fn error_nodes(&self) -> &[ErrorNode] {
    &self.error_nodes
  }

  pub fn tree(&self) -> &FirestoreTree {
    &self.tree
  }

  pub fn definitions(&self) -> &[DefinitionType] {
    &self.definitions
  }
}

#[derive(Debug)]
pub enum DefinitionType {
  Function(Function),
  Variable(VariableDefintion),
  GlobalVariables(String),
}

fn is_definition(field: BaseModel) -> Option<DefinitionType> {
  match field {
    BaseModel::Function(function) => Some(DefinitionType::Function(function)),
    BaseModel::VariableDefintion(variable_defintion) => {
      Some(DefinitionType::Variable(variable_defintion))
    }
    _ => None,
  }
}
fn calculate_definitions(tree: &FirestoreTree) -> Vec<DefinitionType> {
  let body_base = BaseModel::MatchBody(tree.body().clone());
  get_children(body_base, &is_definition)
}

pub fn get_lowest_denominator(position: Point, field: BaseModel) -> Option<BaseModel> {
  if !field.contains(position) {
    return None;
  }

  let children = field.children();

  if children.is_empty() {
    return Some(field);
  }

  for child in children.into_iter() {
    if child.contains(position) {
      return get_lowest_denominator(position, child);
    }
  }

  return None;
}

pub fn get_children<F>(field: BaseModel, filter: &F) -> Vec<DefinitionType>
where
  F: Fn(BaseModel) -> Option<DefinitionType>,
{
  let mut result = vec![];

  let top_level_result = filter(field.clone());

  if top_level_result.is_some() {
    result.push(top_level_result.unwrap());
  }

  let children = field.children();

  if !children.is_empty() {
    for child in children.into_iter() {
      let mut child_res = get_children(child, filter);
      result.append(&mut child_res);
    }
  }

  result
}
