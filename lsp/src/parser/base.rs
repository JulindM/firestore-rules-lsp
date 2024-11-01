use std::fmt::Debug;

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

pub trait Children<'a>: Contains + ToBaseModel + Debug {
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
  Match(&'a Match),
  MatchBody(&'a MatchBody),
  ExprNode(&'a ExprNode),
  Identifier(&'a Identifier),
  Literal(&'a Literal),
  MatchPathPart(&'a MatchPathPart),
  Method(&'a Method),
}

impl<'a> BaseModel<'a> {
  pub fn type_str(&self) -> &str {
    match self {
      BaseModel::Function(_) => "Function",
      BaseModel::FunctionBody(_) => "FunctionBody",
      BaseModel::Rule(_) => "Rule",
      BaseModel::VariableDefintion(_) => "VarDef",
      BaseModel::MatchPath(_) => "MatchPath",
      BaseModel::Match(_) => "Match",
      BaseModel::MatchBody(_) => "MatchBody",
      BaseModel::ExprNode(node) => match node.expr() {
        Expr::Unary(_, _) => "Unary",
        Expr::Binary(_, _, _) => "Binary",
        Expr::Ternary(_, _, _) => "Ternary",
        Expr::Member(_, _) => "Member",
        Expr::Indexing(_, _) => "Indexing",
        Expr::FunctionCall(_, _) => "FunctionCall",
        Expr::Literal(_) => "Literal",
        Expr::Variable(_) => "Variable",
      },
      BaseModel::Identifier(_) => "Identifier",
      BaseModel::Literal(_) => "Literal",
      BaseModel::MatchPathPart(_) => "MatchPathPart",
      BaseModel::Method(_) => "Method",
    }
  }

  pub fn span(&self) -> (Point, Point) {
    match self {
      BaseModel::Function(f) => f.span(),
      BaseModel::FunctionBody(fb) => fb.span(),
      BaseModel::Rule(r) => r.span(),
      BaseModel::VariableDefintion(vd) => vd.span(),
      BaseModel::MatchPath(mp) => mp.span(),
      BaseModel::Match(m) => m.span(),
      BaseModel::MatchBody(mb) => mb.span(),
      BaseModel::ExprNode(en) => en.span(),
      BaseModel::Identifier(v) => v.span(),
      BaseModel::Literal(l) => l.span(),
      BaseModel::MatchPathPart(mpp) => mpp.span(),
      BaseModel::Method(m) => m.span(),
    }
  }
}

pub trait Spanned {
  fn span(&self) -> (Point, Point);
}

impl Contains for (Point, Point) {
  fn contains(&self, p: Point) -> bool {
    if self.0.row < p.row && self.1.row > p.row {
      return true;
    }

    if self.0.row == p.row {
      return self.0.column <= p.column;
    }

    if self.1.row == p.row {
      return p.column <= self.1.column;
    }

    return false;
  }
}

#[derive(Debug, Clone)]
pub struct FirestoreTree {
  body: Option<MatchBody>,
}

impl<'a> FirestoreTree {
  pub fn new(body: Option<MatchBody>) -> Self {
    Self { body }
  }

  pub fn body(&self) -> Option<&MatchBody> {
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

impl<'a> Children<'a> for Identifier {
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

  pub fn pathpart_type(&self) -> &MatchPathPartType {
    &self.pathpart_type
  }

  pub fn value(&self) -> &str {
    match self.pathpart_type {
      MatchPathPartType::SinglePath => self
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

impl<'a> Children<'a> for MatchPathPart {
  fn children(&'a self) -> Vec<&'a dyn Children<'a>> {
    vec![]
  }
}

#[derive(Debug, Clone, PartialEq)]
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

impl<'a> ToBaseModel for Match {
  fn to_base_model<'b>(&'b self) -> BaseModel<'b> {
    BaseModel::Match(&self)
  }
}

impl<'a> Children<'a> for Match {
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

  pub fn matches(&self) -> &[Match] {
    &self.matches
  }

  pub fn rules(&self) -> &[Rule] {
    &self.rules
  }
}

bm_contains!(MatchBody);
bm_span!(MatchBody);
impl<'a> ToBaseModel for MatchBody {
  fn to_base_model<'b>(&'b self) -> BaseModel<'b> {
    BaseModel::MatchBody(&self)
  }
}

impl<'a> Children<'a> for MatchBody {
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
  String(Identifier),
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
  FunctionCall(Identifier, Option<FunctionArgument>),
  Literal(Literal),
  Variable(Identifier),
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
      Expr::FunctionCall(name, function_argument) => {
        let mut f_arg_children = match function_argument {
          Some(FunctionArgument::Path(path_vec)) => path_vec
            .iter()
            .map(|el| match el {
              PathSegment::String(identifier) => vec![identifier as &dyn Children<'a>],
              PathSegment::EvalPath(expr_node) => resolve_expr_nest_non_box(vec![expr_node]),
            })
            .flatten()
            .collect(),
          Some(FunctionArgument::ExprList(vec)) => {
            vec.iter().map(|el| el as &dyn Children<'a>).collect()
          }
          None => vec![],
        };

        f_arg_children.push(name);

        f_arg_children
      }
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

fn resolve_expr_nest_non_box<'a>(
  expr_node: Vec<&'a Option<ExprNode>>,
) -> Vec<&'a dyn Children<'a>> {
  expr_node
    .into_iter()
    .map(|el| {
      el.as_ref()
        .map_or_else(|| vec![], |node| vec![node as &dyn Children<'a>])
    })
    .flatten()
    .collect()
}
