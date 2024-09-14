use tree_sitter::{Node, Point};

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
  start: Point,
  end: Point,
}

impl Function {
  pub fn new(name: &str, parameters: Vec<Variable>, body: FunctionBody, node: Node) -> Self {
    Self {
      name: name.to_owned(),
      parameters,
      body,
      start: node.start_position(),
      end: node.end_position(),
    }
  }
}

#[derive(Debug)]
pub struct FunctionBody {
  variable_defs: Vec<VariableDefintion>,
  ret: Option<ExprNode>,
  start: Option<Point>,
  end: Option<Point>,
}

impl FunctionBody {
  pub fn new(variable_defs: Vec<VariableDefintion>, ret: Option<ExprNode>, node: Node) -> Self {
    Self {
      variable_defs,
      ret,
      start: Some(node.start_position()),
      end: Some(node.end_position()),
    }
  }

  pub(crate) fn empty() -> FunctionBody {
    Self {
      variable_defs: vec![],
      ret: None,
      start: None,
      end: None,
    }
  }
}

#[derive(Debug)]
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
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum MatchPathPart {
  Collection(String),
  SinglePath(String),
  MultiPath(String),
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct Match {
  path: MatchPath,
  body: MatchBody,
  start: Point,
  end: Point,
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
}

#[derive(Debug)]
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
}
