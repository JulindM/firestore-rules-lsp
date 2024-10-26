use tree_sitter::{Node, Point};

use super::base::{BaseModel, Children, FirestoreTree, Function, VariableDefintion};

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
    BaseModel::Function(function) => Some(DefinitionType::Function(function.clone())),
    BaseModel::VariableDefintion(variable_defintion) => {
      Some(DefinitionType::Variable(variable_defintion.clone()))
    }
    _ => None,
  }
}
fn calculate_definitions<'a>(tree: &FirestoreTree<'a>) -> Vec<DefinitionType> {
  filter_children(tree.body(), &is_definition)
}

pub fn get_lowest_denominator<'a>(
  position: Point,
  nestable: &'a dyn Children<'a>,
) -> Option<BaseModel<'a>> {
  if !nestable.contains(position) {
    return None;
  }

  let children = nestable.children();

  let child_hit = children.into_iter().find(|el| el.contains(position));

  if child_hit.is_none() {
    return Some(nestable.to_base_model());
  }

  get_lowest_denominator(position, child_hit.unwrap())
}

pub fn filter_children<'a, FRes>(
  field: &'a dyn Children<'a>,
  filter: &dyn Fn(BaseModel<'a>) -> Option<FRes>,
) -> Vec<FRes> {
  let mut result = vec![];

  let children = field.children();

  if !children.is_empty() {
    for child in children.into_iter() {
      let mut child_res = filter_children(child, filter);
      result.append(&mut child_res);
    }
  }

  result
}
