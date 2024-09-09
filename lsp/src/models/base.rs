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
}

impl Function {
  pub fn new(name: String, parameters: Vec<Variable>) -> Self {
    Self { name, parameters }
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
}

impl MatchBody {
  pub fn new(functions: Vec<Function>, matches: Vec<Match>) -> Self {
    Self { functions, matches }
  }

  pub(crate) fn empty() -> Self {
    Self {
      functions: vec![],
      matches: vec![],
    }
  }
}
