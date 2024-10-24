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

#[derive(Debug, Clone)]
pub enum BaseModel<'a> {
  Function(Function),
  FunctionBody(FunctionBody),
  Rule(Rule),
  VariableDefintion(VariableDefintion),
  MatchPath(MatchPath),
  Match(Match<'a>),
  MatchBody(MatchBody<'a>),
  ExprNode(ExprNode),
  Variable(Variable),
  Literal(Literal),
  RuleMethod(Method),
  MatchPathPart(MatchPathPart),
}

impl<'a> BaseModel<'a> {
  pub fn children(&self) -> Vec<BaseModel<'a>> {
    match self {
      BaseModel::Function(function) => function.children(),
      BaseModel::FunctionBody(function_body) => function_body.children(),
      BaseModel::Rule(rule) => rule.children(),
      BaseModel::VariableDefintion(variable_defintion) => variable_defintion.children(),
      BaseModel::Match(mtch) => mtch.children(),
      BaseModel::MatchPath(match_path) => match_path.children(),
      BaseModel::MatchBody(match_body) => match_body.children(),
      BaseModel::ExprNode(expr_node) => expr_node.children(),
      BaseModel::Variable(_) => vec![],
      BaseModel::Literal(_) => vec![],
      BaseModel::RuleMethod(_) => vec![],
      BaseModel::MatchPathPart(_) => vec![],
    }
  }
}

impl<'a> Contains for BaseModel<'a> {
  fn contains(&self, p: Point) -> bool {
    match self {
      BaseModel::Function(function) => function.contains(p),
      BaseModel::FunctionBody(function_body) => function_body.contains(p),
      BaseModel::Rule(rule) => rule.contains(p),
      BaseModel::VariableDefintion(variable_defintion) => variable_defintion.contains(p),
      BaseModel::Match(mtch) => mtch.contains(p),
      BaseModel::MatchPath(match_path) => match_path.contains(p),
      BaseModel::MatchBody(match_body) => match_body.contains(p),
      BaseModel::ExprNode(expr_node) => expr_node.contains(p),
      BaseModel::Variable(variable) => variable.contains(p),
      BaseModel::Literal(literal) => literal.contains(p),
      BaseModel::RuleMethod(meth) => meth.contains(p),
      BaseModel::MatchPathPart(mpp) => mpp.contains(p),
    }
  }
}

#[derive(Debug, Clone)]
pub struct FirestoreTree<'a> {
  body: MatchBody<'a>,
}

pub trait Contains {
  fn contains(&self, p: Point) -> bool;
}

pub trait Children<'a> {
  fn children(&self) -> Vec<BaseModel<'a>>;
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

impl<'a> Children<'a> for Function {
  fn children(&self) -> Vec<BaseModel<'a>> {
    let mut res = self
      .parameters()
      .iter()
      .map(|var| BaseModel::Variable(var.clone()))
      .collect();

    if self.body().is_none() {
      return res;
    }

    res.push(BaseModel::FunctionBody(self.body().unwrap().clone()));
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

impl<'a> Children<'a> for FunctionBody {
  fn children(&self) -> Vec<BaseModel<'a>> {
    let mut res = self
      .variable_defs()
      .iter()
      .map(|vd| BaseModel::VariableDefintion(vd.clone()))
      .collect();

    if self.ret().is_none() {
      return res;
    }

    res.push(BaseModel::ExprNode(self.ret().unwrap().clone()));
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

impl<'a> Children<'a> for VariableDefintion {
  fn children(&self) -> Vec<BaseModel<'a>> {
    if self.definition().is_none() {
      return vec![];
    };

    vec![BaseModel::ExprNode(self.definition().unwrap().clone())]
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

impl<'a> Children<'a> for MatchPath {
  fn children(&self) -> Vec<BaseModel<'a>> {
    self
      .path_parts()
      .iter()
      .map(|pp| BaseModel::MatchPathPart(pp.clone()))
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

impl<'a> Children<'a> for Match<'a> {
  fn children(&self) -> Vec<BaseModel<'a>> {
    let mut res = vec![];

    if self.path.is_some() {
      res.push(BaseModel::MatchPath(self.path.clone().unwrap()));
    }

    if self.body.is_some() {
      res.push(BaseModel::MatchBody(self.body.clone().unwrap()));
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

impl<'a> Children<'a> for MatchBody<'a> {
  fn children(&self) -> Vec<BaseModel<'a>> {
    let mut res: Vec<Vec<BaseModel<'a>>> = vec![];

    res.push(
      self
        .functions()
        .iter()
        .map(|func| BaseModel::Function(func.clone()))
        .collect(),
    );

    res.push(
      self
        .rules()
        .iter()
        .map(|rule| BaseModel::Rule(rule.clone()))
        .collect(),
    );

    res.push(
      self
        .matches()
        .iter()
        .map(|mtch| BaseModel::Match(mtch.clone()))
        .collect(),
    );

    res.into_iter().flatten().collect()
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

bm_contains!(Method);
bm_span!(Method);

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

impl<'a> Children<'a> for Rule {
  fn children(&self) -> Vec<BaseModel<'a>> {
    let mut res: Vec<BaseModel<'a>> = vec![];

    if self.condition.is_some() {
      res.push(BaseModel::ExprNode(self.condition().unwrap().clone()));
    }

    let mut meths: Vec<BaseModel<'a>> = self
      .methods()
      .iter()
      .map(|meth| BaseModel::RuleMethod(meth.clone()))
      .collect();

    res.append(&mut meths);

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

impl Literal {
  pub fn new<'a>(literal_type: LiteralType, node: Node<'a>) -> Self {
    Self {
      literal_type,
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

bm_contains!(Literal);
bm_span!(Literal);

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

impl<'a> Children<'a> for ExprNode {
  fn children(&self) -> Vec<BaseModel<'a>> {
    match self.expr() {
      Expr::Unary(_, expr_node) => expr_node.as_ref().map_or(vec![], |node| node.children()),
      Expr::Binary(_, expr_node, expr_node1) => vec![
        expr_node.as_ref().map_or(vec![], |node| node.children()),
        expr_node1.as_ref().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::Ternary(expr_node, expr_node1, expr_node2) => vec![
        expr_node.as_ref().map_or(vec![], |node| node.children()),
        expr_node1.as_ref().map_or(vec![], |node| node.children()),
        expr_node2.as_ref().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::Member(expr_node, expr_node1) => vec![
        expr_node.as_ref().map_or(vec![], |node| node.children()),
        expr_node1.as_ref().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::Indexing(expr_node, expr_node1) => vec![
        expr_node.as_ref().map_or(vec![], |node| node.children()),
        expr_node1.as_ref().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::FunctionCall(name, function_argument) => match function_argument {
        // Todo path segments to base models
        Some(FunctionArgument::Path(path)) => vec![],
        Some(FunctionArgument::ExprList(vec)) => {
          vec.iter().map(|expr| expr.children()).flatten().collect()
        }
        None => vec![],
      },
      Expr::Literal(literal) => vec![BaseModel::Literal(literal.clone())],
      Expr::Variable(variable) => vec![BaseModel::Variable(variable.clone())],
    }
  }
}
