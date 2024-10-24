use tree_sitter::{Node, Point};

use super::base::{BaseModel, Contains, FirestoreTree, Function, VariableDefintion};

#[derive(Debug)]
pub struct ErrorNode {
  content: String,
  start: Point,
  end: Point,
}

impl ErrorNode {
  pub fn new<'b>(node: Node<'b>, source: &[u8]) -> Self {
    Self {
      content: node.utf8_text(source).unwrap_or("").to_owned(),
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

#[derive(Debug)]
pub struct EvaluatedTree<'a> {
  tree: FirestoreTree<'a>,
  error_nodes: Vec<ErrorNode>,
  definitions: Vec<DefinitionType>,
}

impl<'a> EvaluatedTree<'a> {
  pub fn new(tree: FirestoreTree<'a>, errors: Vec<ErrorNode>) -> Self {
    Self {
      tree: tree.clone(),
      error_nodes: errors,
      definitions: calculate_definitions(&tree),
    }
  }

  pub fn error_nodes(&self) -> &[ErrorNode] {
    &self.error_nodes
  }

  pub fn tree(&self) -> &FirestoreTree<'a> {
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

fn is_definition<'a>(field: BaseModel<'a>) -> Option<DefinitionType> {
  match field {
    BaseModel::Function(function) => Some(DefinitionType::Function(function)),
    BaseModel::VariableDefintion(variable_defintion) => {
      Some(DefinitionType::Variable(variable_defintion))
    }
    _ => None,
  }
}
fn calculate_definitions<'a>(tree: &FirestoreTree<'a>) -> Vec<DefinitionType> {
  let body_base = BaseModel::MatchBody(tree.body().clone());
  get_children(body_base, &is_definition)
}

pub fn get_lowest_denominator<'a>(position: Point, field: BaseModel<'a>) -> Option<BaseModel<'a>> {
  if !field.contains(position) {
    return None;
  }

  let children = field.children();

  if children.is_empty() {
    return Some(field.clone());
  }

  field
    .children()
    .into_iter()
    .find(|el| el.contains(position))
    .map(|el| get_lowest_denominator(position, el).unwrap())
}

pub fn get_children<'a, F>(field: BaseModel<'a>, filter: &F) -> Vec<DefinitionType>
where
  F: Fn(BaseModel<'a>) -> Option<DefinitionType>,
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
