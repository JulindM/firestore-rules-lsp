use tree_sitter::{Node, Point};

#[derive(Debug)]
pub enum BaseModel {
  Function(Function),
  FunctionBody(FunctionBody),
  Rule(Rule),
  VariableDefintion(VariableDefintion),
  MatchPath(MatchPath),
  Match(Match),
  MatchBody(MatchBody),
  ExprNode(ExprNode),
  Variable(Variable),
  Literal(Literal),
  RuleMethod(Method),
  MatchPathPart(MatchPathPart),
}

impl BaseModel {
  pub fn children(&self) -> Vec<BaseModel> {
    match self {
      BaseModel::Function(function) => function.children(),
      BaseModel::FunctionBody(function_body) => function_body.children(),
      BaseModel::Rule(rule) => rule.children(),
      BaseModel::VariableDefintion(variable_defintion) => variable_defintion.children(),
      BaseModel::Match(mtch) => mtch.children(),
      BaseModel::MatchPath(match_path) => match_path.children(),
      BaseModel::MatchBody(match_body) => match_body.children(),
      BaseModel::ExprNode(expr_node) => expr_node.children(),
      BaseModel::Variable(variable) => vec![],
      BaseModel::Literal(literal) => vec![],
      BaseModel::RuleMethod(method) => vec![],
      BaseModel::MatchPathPart(match_path_part) => vec![],
    }
  }
}

impl Contains for BaseModel {
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
      BaseModel::Literal(literal) => todo!(),
      BaseModel::RuleMethod(method) => todo!(),
      BaseModel::MatchPathPart(match_path_part) => todo!(),
    }
  }
}

#[derive(Debug)]
pub struct FirestoreTree {
  body: MatchBody,
}

pub trait Contains {
  fn contains(&self, p: Point) -> bool;
}

pub trait Children {
  fn children(&self) -> Vec<BaseModel>;
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

    if (self.0.row == self.1.row && self.0.row != p.row) {
      return false;
    }

    return self.0.column <= p.column && self.1.column >= p.column;
  }
}

impl FirestoreTree {
  pub fn new(body: MatchBody) -> Self {
    Self { body }
  }

  pub fn body(&self) -> &MatchBody {
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
  pub fn new(
    name: &str,
    parameters: Vec<Variable>,
    body: Option<FunctionBody>,
    node: Node,
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

impl Spanned for Function {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for Function {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for Function {
  fn children(&self) -> Vec<BaseModel> {
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
  pub fn new(variable_defs: Vec<VariableDefintion>, ret: Option<ExprNode>, node: Node) -> Self {
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

impl Spanned for FunctionBody {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for FunctionBody {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for FunctionBody {
  fn children(&self) -> Vec<BaseModel> {
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
  pub fn new(name: &str, definition: Option<ExprNode>, node: Node) -> Self {
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

impl Spanned for VariableDefintion {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for VariableDefintion {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for VariableDefintion {
  fn children(&self) -> Vec<BaseModel> {
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
  pub fn new(name: &str, node: Node) -> Self {
    Self {
      name: String::from(name),
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

impl Contains for Variable {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Spanned for Variable {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

#[derive(Debug, Clone)]
pub enum MatchPathPart {
  Collection(String),
  SinglePath(String),
  MultiPath(String),
}

#[derive(Debug, Clone)]
pub struct MatchPath {
  path_parts: Vec<MatchPathPart>,
  start: Point,
  end: Point,
}

impl MatchPath {
  pub fn new(path_parts: Vec<MatchPathPart>, node: Node) -> Self {
    Self {
      path_parts,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub(crate) fn empty(node: Node) -> Self {
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

impl Spanned for MatchPath {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for MatchPath {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for MatchPath {
  fn children(&self) -> Vec<BaseModel> {
    self
      .path_parts()
      .iter()
      .map(|pp| BaseModel::MatchPathPart(pp.clone()))
      .collect()
  }
}

#[derive(Debug, Clone)]
pub struct Match {
  body: MatchBody,
  path: MatchPath,
  end: Point,
  start: Point,
}

impl Match {
  pub fn new(path: MatchPath, body: MatchBody, node: Node) -> Self {
    Self {
      path,
      body,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub(crate) fn empty(node: Node) -> Self {
    Self {
      path: MatchPath::empty(node),
      body: MatchBody::empty(node),
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub fn body(&self) -> &MatchBody {
    &self.body
  }

  pub fn path(&self) -> &MatchPath {
    &self.path
  }
}

impl Spanned for Match {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for Match {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for Match {
  fn children(&self) -> Vec<BaseModel> {
    vec![
      BaseModel::MatchPath(self.path().clone()),
      BaseModel::MatchBody(self.body().clone()),
    ]
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
  pub fn new(functions: Vec<Function>, matches: Vec<Match>, rules: Vec<Rule>, node: Node) -> Self {
    Self {
      functions,
      matches,
      rules,
      start: node.start_position(),
      end: node.end_position(),
    }
  }

  pub(crate) fn empty(node: Node) -> Self {
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

  pub fn matches(&self) -> &[Match] {
    &self.matches
  }

  pub fn rules(&self) -> &[Rule] {
    &self.rules
  }
}

impl Spanned for MatchBody {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for MatchBody {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for MatchBody {
  fn children(&self) -> Vec<BaseModel> {
    let mut res: Vec<Vec<BaseModel>> = vec![];

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
pub enum Method {
  Read,
  Write,
  Get,
  List,
  Create,
  Update,
  Delete,
}

#[derive(Debug, Clone)]
pub struct Rule {
  methods: Vec<Method>,
  condition: Option<ExprNode>,
  start: Point,
  end: Point,
}

impl Rule {
  pub fn new(methods: Vec<Method>, condition: Option<ExprNode>, node: Node) -> Self {
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

impl Spanned for Rule {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for Rule {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for Rule {
  fn children(&self) -> Vec<BaseModel> {
    let mut res: Vec<BaseModel> = vec![];

    if self.condition.is_some() {
      res.push(BaseModel::ExprNode(self.condition().unwrap().clone()));
    }

    let mut meths: Vec<BaseModel> = self
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
pub enum Literal {
  Number(f32),
  Bool(bool),
  Null,
  String(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
  Unary(Option<Operation>, Box<Option<ExprNode>>),
  Binary(
    Option<Operation>,
    Box<Option<ExprNode>>,
    Box<Option<ExprNode>>,
  ),
  Ternary(
    Box<Option<ExprNode>>,
    Box<Option<ExprNode>>,
    Box<Option<ExprNode>>,
  ),
  Member(Box<Option<ExprNode>>, Box<Option<ExprNode>>),
  Indexing(Box<Option<ExprNode>>, Box<Option<ExprNode>>),
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
  pub fn new(expr: Expr, node: Node) -> Self {
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

impl Spanned for ExprNode {
  fn span(&self) -> (Point, Point) {
    (self.start, self.end)
  }
}

impl Contains for ExprNode {
  fn contains(&self, p: Point) -> bool {
    self.span().contains(p)
  }
}

impl Children for ExprNode {
  fn children(&self) -> Vec<BaseModel> {
    match self.expr() {
      Expr::Unary(_, expr_node) => expr_node.clone().map_or(vec![], |node| node.children()),
      Expr::Binary(_, expr_node, expr_node1) => vec![
        expr_node.clone().map_or(vec![], |node| node.children()),
        expr_node1.clone().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::Ternary(expr_node, expr_node1, expr_node2) => vec![
        expr_node.clone().map_or(vec![], |node| node.children()),
        expr_node1.clone().map_or(vec![], |node| node.children()),
        expr_node2.clone().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::Member(expr_node, expr_node1) => vec![
        expr_node.clone().map_or(vec![], |node| node.children()),
        expr_node1.clone().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::Indexing(expr_node, expr_node1) => vec![
        expr_node.clone().map_or(vec![], |node| node.children()),
        expr_node1.clone().map_or(vec![], |node| node.children()),
      ]
      .into_iter()
      .flatten()
      .collect(),
      Expr::FunctionCall(name, function_argument) => match function_argument {
        Some(FunctionArgument::Path(path)) => todo!(),
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
