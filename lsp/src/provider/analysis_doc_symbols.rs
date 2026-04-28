use lsp_types::{DocumentSymbol, Range, SymbolKind};

use crate::parser::base::{
  Function, Match, MatchPath, MatchPathPartType, Rule, RulesTree, ServiceType, VariableDefinition,
};

impl Into<DocumentSymbol> for &RulesTree {
  fn into(self) -> DocumentSymbol {
    let name = match self.service_type() {
      Some(ServiceType::Firestore) => "cloud.firestore".to_owned(),
      Some(ServiceType::Storage) => "firebase.storage".to_owned(),
      None => "service".to_owned(),
    };

    let mut children = Vec::new();

    children.extend(self.functions().iter().map(|f| f.into()));

    if let Some(body) = self.service_body() {
      children.extend(body.functions().iter().map(|f| f.into()));
      children.extend(body.matches().iter().map(|f| f.into()));
    }

    let range: Range = self.into();
    let selection_range = range.clone();

    DocumentSymbol {
      name,
      detail: None,
      kind: SymbolKind::NAMESPACE,
      tags: None,
      range: range,
      selection_range: selection_range,
      deprecated: None,
      children: Some(children),
    }
  }
}

impl Into<DocumentSymbol> for &Function {
  fn into(self) -> DocumentSymbol {
    let name = self
      .name()
      .map(|n| n.value().to_owned())
      .unwrap_or_default();

    let mut children = Vec::new();

    if let Some(body) = self.body() {
      children.extend(body.variable_defs().iter().map(|v| v.into()));
    }

    let range: Range = self.into();
    let mut selection_range = range.clone();

    if let Some(name) = self.name() {
      selection_range = name.into();
    }

    DocumentSymbol {
      name,
      detail: None,
      kind: SymbolKind::FUNCTION,
      tags: None,
      range: range,
      selection_range: selection_range,
      deprecated: None,
      children: Some(children),
    }
  }
}

impl Into<DocumentSymbol> for &VariableDefinition {
  fn into(self) -> DocumentSymbol {
    let range = self.into();

    DocumentSymbol {
      name: self.name().to_owned(),
      detail: None,
      kind: SymbolKind::VARIABLE,
      tags: None,
      selection_range: range,
      deprecated: None,
      range,
      children: None,
    }
  }
}

impl Into<DocumentSymbol> for &Match {
  fn into(self) -> DocumentSymbol {
    let name = self
      .path()
      .map(|p| p.into())
      .unwrap_or_else(|| "match".to_owned());

    let mut children = Vec::new();

    if let Some(body) = self.body() {
      children.extend(body.functions().iter().map(|f| f.into()));
      children.extend(body.matches().iter().map(|f| f.into()));
      children.extend(body.rules().iter().map(|r| r.into()))
    }

    let range: Range = self.into();
    let mut selection_range = range.clone();

    if let Some(path) = self.path() {
      selection_range = path.into();
    }

    DocumentSymbol {
      name,
      detail: None,
      kind: SymbolKind::CLASS,
      tags: None,
      range: range,
      selection_range: selection_range,
      deprecated: None,
      children: Some(children),
    }
  }
}

impl Into<String> for &MatchPath {
  fn into(self) -> String {
    self
      .path_parts()
      .iter()
      .map(|p| match p.pathpart_type() {
        MatchPathPartType::Collection => p.value().to_string(),
        MatchPathPartType::Document => format!("{{{:}}}", p.value()),
        MatchPathPartType::MultiPath => format!("{{{:}}}", p.value()),
      })
      .map(|s| s.replace("/", ""))
      .collect::<Vec<_>>()
      .join("/")
  }
}

impl Into<DocumentSymbol> for &Rule {
  fn into(self) -> DocumentSymbol {
    let name = self
      .methods()
      .iter()
      .map(|m| format!("{:?}", m.method_type()).to_lowercase())
      .collect::<Vec<_>>()
      .join(", ");

    let range = self.into();

    DocumentSymbol {
      name,
      detail: None,
      kind: SymbolKind::INTERFACE,
      tags: None,
      selection_range: range,
      deprecated: None,
      range,
      children: None,
    }
  }
}
