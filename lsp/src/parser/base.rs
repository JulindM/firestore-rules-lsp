use std::{
  cell::Cell,
  fmt::{Debug, Display},
};

use strum::{AsRefStr, Display};
use tree_sitter::{Node, Point};

use super::types::FirebaseType;

macro_rules! bm_span(
  ($clazz:ident $($life:lifetime),*) => (
    impl<$($life),*> Spanned for $clazz<$($life),*> {
      fn span(&self) -> (Point, Point) {
        (self.start, self.end)
      }
    }
));

macro_rules! bm_contains(
  ($clazz:ident $($life:lifetime),*) => (
    impl<$($life),*> Contains for $clazz<$($life),*> {
      fn contains(&self, p: Point) -> bool {
        (self.start, self.end).contains(p)
      }
    }
));

macro_rules! bm_to_base_model(
  ($clazz:ident) => (
    impl ToBaseModel for $clazz {
      fn to_base_model<'a>(&'a self) -> BaseModel<'a>{
        BaseModel::$clazz(&self)
      }
    }
));

pub trait HasChildren<'a>: Contains + ToBaseModel + Debug {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>>;
}

pub trait Contains {
  fn contains<'a>(&'a self, p: Point) -> bool;
}

pub trait ToBaseModel: Contains {
  fn to_base_model<'a>(&'a self) -> BaseModel<'a>;
}

#[derive(Clone, Display)]
pub enum BaseModel<'a> {
  Function(&'a Function),
  FunctionBody(&'a FunctionBody),
  Rule(&'a Rule),
  VariableDefinition(&'a VariableDefinition),
  MatchPath(&'a MatchPath),
  Match(&'a Match),
  MatchBody(&'a MatchBody),
  ServiceBody(&'a ServiceBody),
  #[strum(to_string = "{0}")]
  ExprNode(&'a ExprNode),
  Identifier(&'a Identifier),
  Literal(&'a Literal),
  MatchPathPart(&'a MatchPathPart),
  Method(&'a Method),
}

impl<'a> BaseModel<'a> {
  pub fn span(&self) -> (Point, Point) {
    match self {
      BaseModel::Function(f) => f.span(),
      BaseModel::FunctionBody(fb) => fb.span(),
      BaseModel::Rule(r) => r.span(),
      BaseModel::VariableDefinition(vd) => vd.span(),
      BaseModel::MatchPath(mp) => mp.span(),
      BaseModel::Match(m) => m.span(),
      BaseModel::MatchBody(mb) => mb.span(),
      BaseModel::ExprNode(en) => en.span(),
      BaseModel::Identifier(v) => v.span(),
      BaseModel::Literal(l) => l.span(),
      BaseModel::MatchPathPart(mpp) => mpp.span(),
      BaseModel::Method(m) => m.span(),
      BaseModel::ServiceBody(body) => body.span(),
    }
  }
}

impl<'a> Debug for BaseModel<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Function(arg0) => f.debug_tuple("Function").finish(),
      Self::FunctionBody(arg0) => f.debug_tuple("FunctionBody").finish(),
      Self::Rule(arg0) => f.debug_tuple("Rule").finish(),
      Self::VariableDefinition(arg0) => f.debug_tuple("VariableDefinition").finish(),
      Self::MatchPath(arg0) => f.debug_tuple("MatchPath").finish(),
      Self::Match(arg0) => f.debug_tuple("Match").finish(),
      Self::MatchBody(arg0) => f.debug_tuple("MatchBody").finish(),
      Self::ServiceBody(arg0) => f.debug_tuple("ServiceBody").finish(),
      Self::ExprNode(arg0) => f.debug_tuple("ExprNode").field(arg0).finish(),
      Self::Identifier(arg0) => f.debug_tuple("Identifier").finish(),
      Self::Literal(arg0) => f.debug_tuple("Literal").finish(),
      Self::MatchPathPart(arg0) => f.debug_tuple("MatchPathPart").finish(),
      Self::Method(arg0) => f.debug_tuple("Method").finish(),
    }
  }
}

pub trait Spanned {
  fn span(&self) -> (Point, Point);
}

impl Contains for (Point, Point) {
  fn contains(&self, p: Point) -> bool {
    if p.row < self.0.row || p.row > self.1.row {
      // p not in (start line, end line)
      return false;
    }

    if self.0.row < self.1.row {
      // Mutliline spanning block
      if p.row > self.0.row && p.row < self.1.row {
        // p in (start line, end line)
        return true;
      }

      if p.row == self.0.row {
        // p in start line
        return p.column >= self.0.column;
      }

      // p in end line
      return p.column <= self.1.column;
    } else {
      // Oneline spanning
      return self.0.column <= p.column && p.column <= self.1.column;
    }
  }
}

#[derive(Debug, Clone)]
pub struct FirestoreTree {
  body: Option<ServiceBody>,
}

impl FirestoreTree {
  pub fn new(body: Option<ServiceBody>) -> Self {
    Self { body }
  }

  pub fn body(&self) -> Option<&ServiceBody> {
    self.body.as_ref()
  }
}

#[derive(Debug, Clone)]
pub struct Function {
  name: String,
  parameters: Vec<Identifier>,
  body: Option<FunctionBody>,
  start: Point,
  end: Point,
}

impl Function {
  pub fn new<'a>(
    name: &str,
    parameters: Vec<Identifier>,
    body: Option<FunctionBody>,
    node: Node<'a>,
  ) -> Self {
    Self {
      name: name.to_owned(),
      parameters,
      body,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn parameters(&self) -> &[Identifier] {
    &self.parameters
  }

  pub fn body(&self) -> Option<&FunctionBody> {
    self.body.as_ref()
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}

bm_contains!(Function);
bm_span!(Function);
bm_to_base_model!(Function);

impl<'a> HasChildren<'a> for Function {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    let mut res = self
      .parameters()
      .iter()
      .map(|val| val as &dyn HasChildren<'a>)
      .collect();

    if self.body().is_none() {
      return res;
    }

    res.push(self.body().unwrap());

    res
  }
}

#[derive(Debug, Clone)]
pub struct FunctionBody {
  variable_defs: Vec<VariableDefinition>,
  ret: Option<ExprNode>,
  start: Point,
  end: Point,
}

impl FunctionBody {
  pub fn new<'a>(
    variable_defs: Vec<VariableDefinition>,
    ret: Option<ExprNode>,
    node: Node<'a>,
  ) -> Self {
    Self {
      variable_defs,
      ret,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn variable_defs(&self) -> &[VariableDefinition] {
    &self.variable_defs
  }

  pub fn ret(&self) -> Option<&ExprNode> {
    self.ret.as_ref()
  }
}

bm_contains!(FunctionBody);
bm_span!(FunctionBody);
bm_to_base_model!(FunctionBody);

impl<'a> HasChildren<'a> for FunctionBody {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    let mut res = self
      .variable_defs()
      .iter()
      .map(|el| el as &dyn HasChildren<'a>)
      .collect();

    if self.ret().is_none() {
      return res;
    }

    res.push(self.ret().unwrap());
    res
  }
}

#[derive(Debug, Clone)]
pub struct VariableDefinition {
  name: String,
  definition: Option<ExprNode>,
  start: Point,
  end: Point,
}

impl VariableDefinition {
  pub fn new<'a>(name: &str, definition: Option<ExprNode>, node: Node<'a>) -> Self {
    Self {
      name: name.to_owned(),
      definition,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn definition(&self) -> Option<&ExprNode> {
    self.definition.as_ref()
  }
}

bm_contains!(VariableDefinition);
bm_span!(VariableDefinition);
bm_to_base_model!(VariableDefinition);

impl<'a> HasChildren<'a> for VariableDefinition {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    if self.definition().is_none() {
      return vec![];
    };

    vec![self.definition().unwrap()]
  }
}

#[derive(Debug, Clone)]
pub struct Identifier {
  value: String,
  start: Point,
  end: Point,
}

impl Identifier {
  pub fn new<'a>(name: &str, node: Node<'a>) -> Self {
    Self {
      value: String::from(name),
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn value(&self) -> &str {
    &self.value
  }
}

bm_contains!(Identifier);
bm_span!(Identifier);
bm_to_base_model!(Identifier);

impl<'a> HasChildren<'a> for Identifier {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    vec![]
  }
}

#[derive(Debug, Clone)]
pub struct MatchPathPart {
  value: String,
  pathpart_type: MatchPathPartType,
  start: Point,
  end: Point,
}

impl MatchPathPart {
  pub fn new<'a>(value: String, pathpart_type: MatchPathPartType, node: Node<'a>) -> Self {
    Self {
      value,
      pathpart_type,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn pathpart_type(&self) -> &MatchPathPartType {
    &self.pathpart_type
  }

  pub fn value(&self) -> &str {
    match self.pathpart_type {
      MatchPathPartType::Document => self
        .value
        .strip_prefix("/{")
        .unwrap()
        .strip_suffix("}")
        .unwrap(),
      // TODO Handle multi path parts
      _ => self.value.as_ref(),
    }
  }
}

bm_contains!(MatchPathPart);
bm_span!(MatchPathPart);
bm_to_base_model!(MatchPathPart);

impl<'a> HasChildren<'a> for MatchPathPart {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    vec![]
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPathPartType {
  Collection,
  Document,
  MultiPath,
}

#[derive(Debug, Clone)]
pub struct MatchPath {
  path_parts: Vec<MatchPathPart>,
  start: Point,
  end: Point,
}

impl MatchPath {
  pub fn new<'a>(path_parts: Vec<MatchPathPart>, node: Node<'a>) -> Self {
    Self {
      path_parts,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn path_parts(&self) -> &[MatchPathPart] {
    &self.path_parts
  }
}

bm_contains!(MatchPath);
bm_span!(MatchPath);
bm_to_base_model!(MatchPath);

impl<'a> HasChildren<'a> for MatchPath {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    self
      .path_parts
      .iter()
      .map(|val| val as &dyn HasChildren<'a>)
      .collect()
  }
}

#[derive(Debug, Clone)]
pub struct Match {
  body: Option<MatchBody>,
  path: Option<MatchPath>,
  end: Point,
  start: Point,
}

impl Match {
  pub fn new<'b>(path: Option<MatchPath>, body: Option<MatchBody>, node: Node<'b>) -> Self {
    Self {
      path,
      body,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn path(&self) -> Option<&MatchPath> {
    self.path.as_ref()
  }

  pub fn body(&self) -> Option<&MatchBody> {
    self.body.as_ref()
  }
}

bm_contains!(Match);
bm_span!(Match);
bm_to_base_model!(Match);

impl<'a> HasChildren<'a> for Match {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    let mut res: Vec<&dyn HasChildren<'a>> = vec![];

    if self.path.is_some() {
      res.push(self.path().unwrap());
    }

    if self.body.is_some() {
      res.push(self.body().unwrap());
    }

    res
  }
}

#[derive(Debug, Clone)]
pub struct MatchBody {
  functions: Vec<Function>,
  matches: Vec<Match>,
  rules: Vec<Rule>,
  start: Point,
  end: Point,
}

impl MatchBody {
  pub fn new<'b>(
    functions: Vec<Function>,
    matches: Vec<Match>,
    rules: Vec<Rule>,
    node: Node<'b>,
  ) -> Self {
    Self {
      functions,
      matches,
      rules,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn functions(&self) -> &[Function] {
    &self.functions
  }
}

bm_contains!(MatchBody);
bm_span!(MatchBody);
bm_to_base_model!(MatchBody);

impl<'a> HasChildren<'a> for MatchBody {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    let mut res: Vec<&dyn HasChildren<'a>> = vec![];

    self.functions.iter().for_each(|f| res.push(f));
    self.matches.iter().for_each(|f| res.push(f));
    self.rules.iter().for_each(|f| res.push(f));

    res
  }
}

#[derive(Debug, Clone)]
pub struct ServiceBody {
  functions: Vec<Function>,
  matches: Vec<Match>,
  rules: Vec<Rule>,
  start: Point,
  end: Point,
}

impl ServiceBody {
  pub fn new<'b>(
    functions: Vec<Function>,
    matches: Vec<Match>,
    rules: Vec<Rule>,
    node: Node<'b>,
  ) -> Self {
    Self {
      matches,
      functions,
      rules,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn functions(&self) -> &[Function] {
    &self.functions
  }
}

bm_contains!(ServiceBody);
bm_span!(ServiceBody);
bm_to_base_model!(ServiceBody);

impl<'a> HasChildren<'a> for ServiceBody {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    let mut res: Vec<&dyn HasChildren<'a>> = vec![];

    self.functions.iter().for_each(|f| res.push(f));
    self.matches.iter().for_each(|f| res.push(f));
    self.rules.iter().for_each(|f| res.push(f));

    res
  }
}

#[derive(Debug, Clone)]
pub struct Method {
  start: Point,
  end: Point,
  method_type: MethodType,
}

impl Method {
  pub fn new<'a>(method_type: MethodType, node: Node<'a>) -> Self {
    Self {
      method_type,
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

impl<'a> HasChildren<'a> for Method {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    vec![]
  }
}

bm_contains!(Method);
bm_span!(Method);
bm_to_base_model!(Method);

#[derive(Debug, Clone)]
pub enum MethodType {
  Read,
  Write,
  Get,
  List,
  Create,
  Update,
  Delete,
  Unknown,
}

#[derive(Debug, Clone)]
pub struct Rule {
  methods: Vec<Method>,
  condition: Option<ExprNode>,
  start: Point,
  end: Point,
}

impl Rule {
  pub fn new<'a>(methods: Vec<Method>, condition: Option<ExprNode>, node: Node<'a>) -> Self {
    Self {
      methods,
      condition,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn methods(&self) -> &[Method] {
    &self.methods
  }

  pub fn condition(&self) -> Option<&ExprNode> {
    self.condition.as_ref()
  }
}

bm_span!(Rule);
bm_contains!(Rule);
bm_to_base_model!(Rule);

impl<'a> HasChildren<'a> for Rule {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    let mut res: Vec<&dyn HasChildren<'a>> = vec![];

    if self.condition.is_some() {
      res.push(self.condition().unwrap());
    }

    res
  }
}

#[derive(Debug, Clone)]
pub struct Literal {
  start: Point,
  end: Point,
  literal_type: Cell<FirebaseType>,
}

bm_contains!(Literal);
bm_span!(Literal);
bm_to_base_model!(Literal);

impl Literal {
  pub fn new<'a>(literal_type: FirebaseType, node: Node<'a>) -> Self {
    Self {
      start: node.start_position(),
      end: node.end_position(),
      literal_type: Cell::new(literal_type),
    }
  }

  pub fn firebase_type(&self) -> &Cell<FirebaseType> {
    &self.literal_type
  }
}

impl<'a> HasChildren<'a> for Literal {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    vec![]
  }
}

#[derive(Debug, Clone)]
pub enum Operation {
  Negation,
  Addition,
  Multiplication,
  Division,
  Relation,
  And,
  Or,
  Substraction,
  Modulo,
}

#[derive(Debug, Clone)]
pub enum DefinableIdentifier {
  Yes(Identifier),
  No(Identifier),
}

impl DefinableIdentifier {
  pub fn name(&self) -> &str {
    match &self {
      DefinableIdentifier::Yes(identifier) => identifier.value(),
      DefinableIdentifier::No(identifier) => identifier.value(),
    }
  }
}

#[derive(Debug, Clone, AsRefStr)]
pub enum Expr {
  Unary(Option<Operation>, Option<Box<ExprNode>>),
  Binary(
    Option<Operation>,
    Option<Box<ExprNode>>,
    Option<Box<ExprNode>>,
  ),
  Ternary(
    Option<Box<ExprNode>>,
    Option<Box<ExprNode>>,
    Option<Box<ExprNode>>,
  ),
  Member(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
  MemberObject(Option<Box<ExprNode>>),
  MemberField(Option<Box<ExprNode>>),
  Indexing(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
  FunctionCall(DefinableIdentifier, Vec<ExprNode>, Cell<FirebaseType>),
  Literal(Literal),
  Path(Vec<ExprNode>, Cell<FirebaseType>),
  Variable(DefinableIdentifier, Cell<FirebaseType>),
  List(Vec<ExprNode>, Cell<FirebaseType>),
  Map(Vec<ExprNode>, Cell<FirebaseType>),
  MapEntry(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
  ExprGroup(Option<Box<ExprNode>>),
  TypeComparison(Option<Box<ExprNode>>, Cell<FirebaseType>),
}

#[derive(Clone)]
pub struct ExprNode {
  expr: Expr,
  start: Point,
  end: Point,
}

impl Display for ExprNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.expr.as_ref())
  }
}

impl Debug for ExprNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct(&self.expr.as_ref())
      .field("start", &self.start)
      .field("end", &self.end)
      .finish()
  }
}

impl ExprNode {
  pub fn new<'a>(expr: Expr, node: Node<'a>) -> Self {
    Self {
      expr,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn expr(&self) -> &Expr {
    &self.expr
  }

  pub fn inferred_type(&self) -> Option<&Cell<FirebaseType>> {
    match &self.expr {
      Expr::Member(_, member) => member.as_ref().and_then(|f| f.inferred_type()),
      Expr::MemberObject(node) => node.as_ref().and_then(|o| o.inferred_type()),
      Expr::MemberField(node) => node.as_ref().and_then(|f| f.inferred_type()),
      Expr::FunctionCall(_, _, fir_type) => Some(fir_type),
      Expr::Literal(literal) => Some(literal.firebase_type()),
      Expr::Variable(_, fir_type) => Some(fir_type),
      Expr::List(_, fir_type) => Some(fir_type),
      Expr::Unary(_, _) => None,
      Expr::Binary(_, _, _) => None,
      Expr::Ternary(_, _, _) => None,
      Expr::Indexing(_, _) => None,
      Expr::Map(_, fir_type) => Some(fir_type),
      Expr::MapEntry(_, _) => None,
      Expr::Path(_, fir_type) => Some(fir_type),
      Expr::ExprGroup(expr) => expr.as_ref().and_then(|e| e.inferred_type()),
      Expr::TypeComparison(_, fir_type) => Some(fir_type),
    }
  }
}

bm_contains!(ExprNode);
bm_span!(ExprNode);
bm_to_base_model!(ExprNode);

impl<'a> HasChildren<'a> for ExprNode {
  fn children(&'a self) -> Vec<&'a dyn HasChildren<'a>> {
    match &self.expr {
      Expr::Unary(_, expr_node) => resolve_expr_nest(vec![expr_node]),
      Expr::Binary(_, expr_node, expr_node1) => resolve_expr_nest(vec![expr_node, expr_node1]),
      Expr::Ternary(expr_node, expr_node1, expr_node2) => {
        resolve_expr_nest(vec![expr_node, expr_node1, expr_node2])
      }
      Expr::Member(expr_node, expr_node1) => resolve_expr_nest(vec![expr_node, expr_node1]),
      Expr::Indexing(expr_node, expr_node1) => resolve_expr_nest(vec![expr_node, expr_node1]),
      Expr::FunctionCall(_, function_arguments, _) => function_arguments
        .iter()
        .map(|e| e as &dyn HasChildren<'a>)
        .collect(),
      Expr::MemberObject(expr_node) => resolve_expr_nest(vec![expr_node]),
      Expr::MemberField(expr_node) => resolve_expr_nest(vec![expr_node]),
      Expr::List(expr_nodes, _) | Expr::Map(expr_nodes, _) => expr_nodes
        .iter()
        .map(|e| e as &dyn HasChildren<'a>)
        .collect(),
      Expr::Literal(_) => vec![],
      Expr::Variable(_, _) => vec![],
      Expr::MapEntry(key, value) => resolve_expr_nest(vec![key, value]),
      Expr::Path(path_segments, _) => path_segments
        .iter()
        .map(|n| n as &dyn HasChildren<'a>)
        .collect(),
      Expr::ExprGroup(expr_node) => resolve_expr_nest(vec![expr_node]),
      Expr::TypeComparison(expr_node, _) => resolve_expr_nest(vec![expr_node]),
    }
  }
}

fn resolve_expr_nest<'a>(
  expr_node: Vec<&'a Option<Box<ExprNode>>>,
) -> Vec<&'a dyn HasChildren<'a>> {
  expr_node
    .into_iter()
    .filter_map(|el| el.as_ref())
    .map(|el| el.as_ref() as &dyn HasChildren<'a>)
    .collect()
}
