#[derive(Debug)]
pub struct FirestoreTree {
  body: MatchBody,
}

impl FirestoreTree {
  pub fn new(body: MatchBody) -> Self {
    Self { body }
  }
}

#[derive(Debug)]
pub struct Function {
  name: String,
  parameters: Vec<Variable>,
  body: FunctionBody,
}

impl Function {
  pub fn new(name: &str, parameters: Vec<Variable>, body: FunctionBody) -> Self {
    Self {
      name: name.to_owned(),
      parameters,
      body,
    }
  }
}

#[derive(Debug)]
pub struct FunctionBody {
  variable_defs: Vec<VariableDefintion>,
  ret: Option<ExprNode>,
}

impl FunctionBody {
  pub fn new(variable_defs: Vec<VariableDefintion>, ret: Option<ExprNode>) -> Self {
    Self { variable_defs, ret }
  }

  pub(crate) fn empty() -> FunctionBody {
    Self {
      variable_defs: vec![],
      ret: None,
    }
  }
}

#[derive(Debug)]
pub struct VariableDefintion {
  name: String,
  definition: Option<ExprNode>,
}

impl VariableDefintion {
  pub fn new(name: &str, definition: Option<ExprNode>) -> Self {
    Self {
      name: name.to_owned(),
      definition,
    }
  }
}

#[derive(Debug)]
pub struct Variable {
  name: String,
}

impl Variable {
  pub fn new(name: &str) -> Self {
    Self {
      name: String::from(name),
    }
  }
}

#[derive(Debug)]
pub enum MatchPathPart {
  Collection(String),
  SinglePath(String),
  MultiPath(String),
}

#[derive(Debug)]
pub struct MatchPath {
  path_parts: Vec<MatchPathPart>,
}

impl MatchPath {
  pub fn new(path_parts: Vec<MatchPathPart>) -> Self {
    Self { path_parts }
  }

  pub(crate) fn empty() -> Self {
    Self { path_parts: vec![] }
  }
}

#[derive(Debug)]
pub struct Match {
  path: MatchPath,
  body: MatchBody,
}

impl Match {
  pub fn new(path: MatchPath, body: MatchBody) -> Self {
    Self { path, body }
  }

  pub(crate) fn empty() -> Self {
    Self {
      path: MatchPath::empty(),
      body: MatchBody::empty(),
    }
  }
}

#[derive(Debug)]
pub struct MatchBody {
  functions: Vec<Function>,
  matches: Vec<Match>,
  rules: Vec<Rule>,
}

impl MatchBody {
  pub fn new(functions: Vec<Function>, matches: Vec<Match>, rules: Vec<Rule>) -> Self {
    Self {
      functions,
      matches,
      rules,
    }
  }

  pub(crate) fn empty() -> Self {
    Self {
      functions: vec![],
      matches: vec![],
      rules: vec![],
    }
  }
}

#[derive(Debug)]
pub enum Method {
  Read,
  Write,
  Get,
  List,
  Create,
  Update,
  Delete,
}

#[derive(Debug)]
pub struct Rule {
  methods: Vec<Method>,
  condition: Option<ExprNode>,
}

impl Rule {
  pub fn new(methods: Vec<Method>, condition: Option<ExprNode>) -> Self {
    Self { methods, condition }
  }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum PathSegment {
  String(String),
  EvalPath(Option<ExprNode>),
}

#[derive(Debug)]
pub enum FunctionArgument {
  Path(Vec<PathSegment>),
  ExprList(Vec<ExprNode>),
}

#[derive(Debug)]
pub enum Literal {
  Number(f32),
  Bool(bool),
  Null,
  String(String),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ExprNode {
  expr: Expr,
}

impl ExprNode {
  pub fn new(expr: Expr) -> Self {
    Self { expr }
  }
}
