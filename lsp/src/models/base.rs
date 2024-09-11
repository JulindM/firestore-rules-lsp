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
  ret: Expr,
}

impl FunctionBody {
  pub fn new(variable_defs: Vec<VariableDefintion>, ret: Expr) -> Self {
    Self { variable_defs, ret }
  }

  pub(crate) fn empty() -> FunctionBody {
    Self {
      variable_defs: vec![],
      ret: Expr::new(),
    }
  }
}

#[derive(Debug)]
pub struct VariableDefintion {
  name: String,
  definition: Expr,
}

impl VariableDefintion {
  pub fn new(name: &str, definition: Expr) -> Self {
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
  condition: Option<Expr>,
}

impl Rule {
  pub fn new(methods: Vec<Method>, condition: Option<Expr>) -> Self {
    Self { methods, condition }
  }
}

#[derive(Debug)]
pub struct Expr {}

impl Expr {
  pub fn new() -> Self {
    Self {}
  }
}
