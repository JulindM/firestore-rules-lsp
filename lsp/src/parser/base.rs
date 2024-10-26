use tree_sitter::{Node, Point};

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

pub trait Children<'a>: Contains + ToBaseModel {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>>;
}

pub trait Contains {
  fn contains<'a>(&'a self, p: Point) -> bool;
}

pub trait ToBaseModel: Contains {
  fn to_base_model<'a>(&'a self) -> BaseModel<'a>;
}

#[derive(Debug, Clone)]
pub enum BaseModel<'a> {
  Function(&'a Function),
  FunctionBody(&'a FunctionBody),
  Rule(&'a Rule),
  VariableDefintion(&'a VariableDefintion),
  MatchPath(&'a MatchPath),
  Match(&'a Match<'a>),
  MatchBody(&'a MatchBody<'a>),
  ExprNode(&'a ExprNode),
  Variable(&'a Variable),
  Literal(&'a Literal),
  MatchPathPart(&'a MatchPathPart),
  Method(&'a Method),
}

pub trait Spanned {
  fn span(&self) -> (Point, Point);
}

impl Contains for (Point, Point) {
  fn contains(&self, p: Point) -> bool {
    if self.0.row < p.row && self.1.row > p.row {
      return true;
    }

    if self.0.row == p.row && self.1.row > p.row {
      return self.0.column <= p.column;
    }

    if self.0.row < p.row && self.1.row == p.row {
      return self.1.column >= p.column;
    }

    if self.0.row == self.1.row && self.0.row != p.row {
      return false;
    }

    return self.0.column <= p.column && self.1.column >= p.column;
  }
}

#[derive(Debug, Clone)]
pub struct FirestoreTree<'a> {
  body: MatchBody<'a>,
}

impl<'a> FirestoreTree<'a> {
  pub fn new(body: MatchBody<'a>) -> Self {
    Self { body }
  }

  pub fn body(&self) -> &MatchBody<'a> {
    &self.body
  }
}

#[derive(Debug, Clone)]
pub struct Function {
  name: String,
  parameters: Vec<Variable>,
  body: Option<FunctionBody>,
  start: Point,
  end: Point,
}

impl Function {
  pub fn new<'a>(
    name: &str,
    parameters: Vec<Variable>,
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

  pub fn parameters(&self) -> &[Variable] {
    &self.parameters
  }

  pub fn body(&self) -> Option<&FunctionBody> {
    self.body.as_ref()
  }
}

bm_contains!(Function);
bm_span!(Function);
bm_to_base_model!(Function);

impl<'a> Children<'a> for Function {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    let mut res = self
      .parameters()
      .iter()
      .map(|val| val as &dyn Children<'a>)
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
  variable_defs: Vec<VariableDefintion>,
  ret: Option<ExprNode>,
  start: Point,
  end: Point,
}

impl FunctionBody {
  pub fn new<'a>(
    variable_defs: Vec<VariableDefintion>,
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

  pub fn variable_defs(&self) -> &[VariableDefintion] {
    &self.variable_defs
  }

  pub fn ret(&self) -> Option<&ExprNode> {
    self.ret.as_ref()
  }
}

bm_contains!(FunctionBody);
bm_span!(FunctionBody);
bm_to_base_model!(FunctionBody);

impl<'a> Children<'a> for FunctionBody {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    let mut res = self
      .variable_defs()
      .iter()
      .map(|el| el as &dyn Children<'a>)
      .collect();

    if self.ret().is_none() {
      return res;
    }

    res.push(self.ret().unwrap());
    res
  }
}

#[derive(Debug, Clone)]
pub struct VariableDefintion {
  name: String,
  definition: Option<ExprNode>,
  start: Point,
  end: Point,
}

impl VariableDefintion {
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

bm_contains!(VariableDefintion);
bm_span!(VariableDefintion);
bm_to_base_model!(VariableDefintion);

impl<'a> Children<'a> for VariableDefintion {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    if self.definition().is_none() {
      return vec![];
    };

    vec![self.definition().unwrap()]
  }
}

#[derive(Debug, Clone)]
pub struct Variable {
  name: String,
  start: Point,
  end: Point,
}

impl Variable {
  pub fn new<'a>(name: &str, node: Node<'a>) -> Self {
    Self {
      name: String::from(name),
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

bm_contains!(Variable);
bm_span!(Variable);
bm_to_base_model!(Variable);

impl<'a> Children<'a> for Variable {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
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
}

bm_contains!(MatchPathPart);
bm_span!(MatchPathPart);
bm_to_base_model!(MatchPathPart);

impl<'a> Children<'a> for MatchPathPart {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    vec![]
  }
}

#[derive(Debug, Clone)]
pub enum MatchPathPartType {
  Collection,
  SinglePath,
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

  pub(crate) fn empty<'a>(node: Node<'a>) -> Self {
    Self {
      path_parts: vec![],
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

impl<'a> Children<'a> for MatchPath {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    self
      .path_parts()
      .iter()
      .map(|val| val as &dyn Children<'a>)
      .collect()
  }
}

#[derive(Debug, Clone)]
pub struct Match<'a> {
  parent_match: Option<&'a Match<'a>>,
  body: Option<MatchBody<'a>>,
  path: Option<MatchPath>,
  end: Point,
  start: Point,
}

impl<'a> Match<'a> {
  pub fn new<'b>(
    parent_match: Option<&'a Match<'a>>,
    path: Option<MatchPath>,
    body: Option<MatchBody<'a>>,
    node: Node<'b>,
  ) -> Self {
    Self {
      parent_match,
      path,
      body,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn path(&self) -> Option<&MatchPath> {
    self.path.as_ref()
  }

  pub fn body(&self) -> Option<&MatchBody<'a>> {
    self.body.as_ref()
  }
}

bm_contains!(Match 'a);
bm_span!(Match 'a);

impl<'a> ToBaseModel for Match<'a> {
  fn to_base_model<'b>(&'b self) -> BaseModel<'b> {
    BaseModel::Match(&self)
  }
}

impl<'a> Children<'a> for Match<'a> {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    let mut res: Vec<&dyn Children<'a>> = vec![];

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
pub struct MatchBody<'a> {
  functions: Vec<Function>,
  matches: Vec<Match<'a>>,
  rules: Vec<Rule>,
  start: Point,
  end: Point,
}

impl<'a> MatchBody<'a> {
  pub fn new<'b>(
    functions: Vec<Function>,
    matches: Vec<Match<'a>>,
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

  pub(crate) fn empty(node: Node<'a>) -> Self {
    Self {
      functions: vec![],
      matches: vec![],
      rules: vec![],
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn functions(&self) -> &[Function] {
    &self.functions
  }

  pub fn matches(&self) -> &[Match<'a>] {
    &self.matches
  }

  pub fn rules(&self) -> &[Rule] {
    &self.rules
  }
}

bm_contains!(MatchBody 'a);
bm_span!(MatchBody 'a);
impl<'a> ToBaseModel for MatchBody<'a> {
  fn to_base_model<'b>(&'b self) -> BaseModel<'b> {
    BaseModel::MatchBody(&self)
  }
}

impl<'a> Children<'a> for MatchBody<'a> {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    let mut res: Vec<&dyn Children<'a>> = vec![];

    for ele in self.functions() {
      res.push(ele);
    }

    for ele in self.matches() {
      res.push(ele);
    }

    for ele in self.rules() {
      res.push(ele);
    }

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

impl<'a> Children<'a> for Method {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
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

impl<'a> Children<'a> for Rule {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    let mut res: Vec<&dyn Children<'a>> = vec![];

    for ele in self.methods() {
      res.push(ele);
    }

    if self.condition.is_some() {
      res.push(self.condition().unwrap());
    }

    res
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
pub enum PathSegment {
  String(String),
  EvalPath(Option<ExprNode>),
}

#[derive(Debug, Clone)]
pub enum FunctionArgument {
  Path(Vec<PathSegment>),
  ExprList(Vec<ExprNode>),
}

#[derive(Debug, Clone)]
pub struct Literal {
  start: Point,
  end: Point,
  literal_type: LiteralType,
}

bm_contains!(Literal);
bm_span!(Literal);
bm_to_base_model!(Literal);

impl Literal {
  pub fn new<'a>(literal_type: LiteralType, node: Node<'a>) -> Self {
    Self {
      literal_type,
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

impl<'a> Children<'a> for Literal {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    vec![]
  }
}

#[derive(Debug, Clone)]
pub enum LiteralType {
  Number(f32),
  Bool(bool),
  Null,
  String(String),
}

#[derive(Debug, Clone)]
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
  Indexing(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
  FunctionCall(String, Option<FunctionArgument>),
  Literal(Literal),
  Variable(Variable),
}

#[derive(Debug, Clone)]
pub struct ExprNode {
  expr: Expr,
  start: Point,
  end: Point,
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
}

bm_contains!(ExprNode);
bm_span!(ExprNode);
bm_to_base_model!(ExprNode);

impl<'a> Children<'a> for ExprNode {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    match self.expr() {
      Expr::Unary(_, expr_node) => resolve_expr_nest(vec![expr_node]),
      Expr::Binary(_, expr_node, expr_node1) => resolve_expr_nest(vec![expr_node, expr_node1]),
      Expr::Ternary(expr_node, expr_node1, expr_node2) => {
        resolve_expr_nest(vec![expr_node, expr_node1, expr_node2])
      }
      Expr::Member(expr_node, expr_node1) => resolve_expr_nest(vec![expr_node, expr_node1]),
      Expr::Indexing(expr_node, expr_node1) => resolve_expr_nest(vec![expr_node, expr_node1]),
      Expr::FunctionCall(name, function_argument) => match function_argument {
        // Todo path segments to base models
        Some(FunctionArgument::Path(path)) => vec![],
        Some(FunctionArgument::ExprList(vec)) => {
          vec.iter().map(|el| el as &dyn Children<'a>).collect()
        }
        None => vec![],
      },
      Expr::Literal(literal) => vec![literal],
      Expr::Variable(variable) => vec![variable],
      _ => vec![],
    }
  }
}

fn resolve_expr_nest<'a>(expr_node: Vec<&'a Option<Box<ExprNode>>>) -> Vec<&'a dyn Children<'a>> {
  expr_node
    .into_iter()
    .map(|el| {
      el.as_ref()
        .map_or_else(|| vec![], |node| vec![node.as_ref() as &dyn Children<'a>])
    })
    .flatten()
    .collect()
}
