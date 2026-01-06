use tree_sitter::{Node, Tree};

use super::{base::*, types::*};

macro_rules! sanitized_children {
  ($node: ident) => {
    $node.children(&mut $node.walk()).filter(|n| !n.is_extra())
  };
}

pub fn evaluate_tree(tree: Tree, source_bytes: &[u8]) -> RulesTree {
  let node = tree.root_node();

  if node.kind() != "source_file" {
    return RulesTree::new(None, None);
  }

  let mut match_body = None;

  let mut service_type = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "service_type" => {
      service_type = match child.utf8_text(source_bytes) {
        Ok(text) => match text {
          "cloud.firestore" => Some(ServiceType::Firestore),
          "firebase.storage" => Some(ServiceType::Storage),
          _ => None,
        },
        Err(_) => None,
      };
    }
    "service_body" => {
      match_body = Some(parse_service_definition(child, source_bytes));
    }
    _ => return,
  });

  RulesTree::new(service_type, match_body)
}

fn parse_service_definition<'a, 'b>(node: Node<'b>, source_bytes: &[u8]) -> ServiceBody {
  let mut matches = vec![];
  let mut functions = vec![];
  let mut rules = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_def" => matches.push(parse_match_def(child, source_bytes)),
    "function_def" => functions.push(parse_function_def(child, source_bytes)),
    "rule_def" => rules.push(parse_rule(child, source_bytes)),
    _ => return,
  });

  ServiceBody::new(functions, matches, rules, node)
}

fn parse_match_body<'a, 'b>(node: Node<'b>, source_bytes: &[u8]) -> MatchBody {
  let mut matches = vec![];
  let mut functions = vec![];
  let mut rules = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_def" => matches.push(parse_match_def(child, source_bytes)),
    "function_def" => functions.push(parse_function_def(child, source_bytes)),
    "rule_def" => rules.push(parse_rule(child, source_bytes)),
    _ => return,
  });

  MatchBody::new(functions, matches, rules, node)
}

fn parse_match_def<'a, 'b>(node: Node<'b>, source_bytes: &[u8]) -> Match {
  let mut path = None;
  let mut body = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "match_path" => path = Some(parse_match_path(child, source_bytes)),
    "match_body" => body = Some(parse_match_body(child, source_bytes)),
    _ => return,
  });

  Match::new(path, body, node)
}

fn parse_function_def<'b>(node: Node<'b>, source_bytes: &[u8]) -> Function {
  let mut name = "";
  let mut params = vec![];
  let mut body = None;

  let mut name_start = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "function_name" => {
      name_start = Some(child.start_position());
      name = child.utf8_text(source_bytes).unwrap()
    }
    "param_list" => params = parse_param_list(child, source_bytes),
    "function_body" => body = Some(parse_function_body(child, source_bytes)),
    _ => return,
  });

  Function::new(
    name,
    params,
    body,
    name_start.unwrap_or(node.start_position()),
    node.end_position(),
  )
}

fn parse_param_list<'a, 'b>(node: Node<'b>, source_bytes: &[u8]) -> Vec<Identifier> {
  let mut idents = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "identifier" => idents.push(Identifier::new(
      child.utf8_text(source_bytes).unwrap(),
      child,
    )),
    _ => return,
  });

  idents
}

fn parse_function_body<'b>(node: Node<'b>, source_bytes: &[u8]) -> FunctionBody {
  let mut variables = vec![];
  let mut ret = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "variable_def" => variables.push(parse_variable_def(child, source_bytes)),
    "fun_return" => ret = parse_fun_return(child, source_bytes),
    _ => return,
  });

  FunctionBody::new(variables, ret, node)
}

fn parse_fun_return<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut expr = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "expr" => expr = parse_expr(child, source_bytes),
    _ => return,
  });

  expr
}

fn parse_variable_def<'b>(node: Node<'b>, source_bytes: &[u8]) -> VariableDefinition {
  let mut name = "";
  let mut expr = None;

  let mut start_pos = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "variable" => {
      name = child.utf8_text(source_bytes).unwrap();
      start_pos = Some(child.start_position());
    }
    "expr" => expr = parse_expr(child, source_bytes),
    _ => return,
  });

  VariableDefinition::new(
    name,
    expr,
    start_pos.unwrap_or(node.start_position()),
    node.end_position(),
  )
}

fn parse_match_path<'b>(node: Node<'b>, source_bytes: &[u8]) -> MatchPath {
  let mut path_parts = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "collection_path_seg" => {
      let name = child.utf8_text(source_bytes).unwrap().to_owned();
      let part = MatchPathPart::new(name, MatchPathPartType::Collection, child);
      path_parts.push(part);
    }
    "single_path_seg" => {
      let name = child.utf8_text(source_bytes).unwrap().to_owned();
      let part = MatchPathPart::new(name, MatchPathPartType::Document, child);
      path_parts.push(part);
    }
    "multi_path_seg" => {
      let name = child.utf8_text(source_bytes).unwrap().to_owned();
      let part = MatchPathPart::new(name, MatchPathPartType::MultiPath, child);
      path_parts.push(part);
    }
    _ => return,
  });

  MatchPath::new(path_parts, node)
}

fn parse_rule<'b>(node: Node<'b>, source_bytes: &[u8]) -> Rule {
  let mut methods = vec![];
  let mut condition: Option<ExprNode> = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "method" => match parse_method(child) {
      Some(meth) => methods.push(meth),
      None => return,
    },
    "expr" => condition = parse_expr(child, source_bytes),
    _ => return,
  });

  Rule::new(methods, condition, node)
}

fn parse_primary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let child = node.child(0).unwrap();

  match child.kind() {
    "literal" => parse_literal(child, source_bytes),
    "variable" => parse_variable(child, source_bytes),
    "expr_group" => parse_expr_group(child, source_bytes),
    "function_call" => parse_function_call(child, source_bytes),
    "list" => parse_list(child, source_bytes),
    "map" => parse_map(child, source_bytes),
    _ => None,
  }
}

fn parse_list<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut list_elements = vec![];

  for child in sanitized_children!(node) {
    match child.kind() {
      "expr" => {
        let res = parse_expr(child, source_bytes);
        if res.is_some() {
          list_elements.push(res.unwrap());
        }
      }
      _ => continue,
    }
  }

  Some(ExprNode::new(Expr::List(list_elements.clone()), node))
}

fn parse_map<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut map_entries = vec![];

  for child in sanitized_children!(node) {
    match child.kind() {
      "map_entry" => {
        let entry = parse_map_entry(child, source_bytes);
        if entry.is_some() {
          map_entries.push(entry.unwrap());
        }
      }
      _ => continue,
    }
  }

  Some(ExprNode::new(Expr::Map(map_entries.clone()), node))
}

fn parse_map_entry<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut key = None;
  let mut value = None;

  for child in sanitized_children!(node) {
    match child.kind() {
      "map_key" => key = parse_expr(child, source_bytes),
      "map_value" => value = parse_expr(child, source_bytes),
      _ => continue,
    }
  }

  Some(ExprNode::new(
    Expr::MapEntry(key.map(Box::new), value.map(Box::new)),
    node,
  ))
}

fn parse_function_call<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut args = vec![];
  let mut name = None;

  sanitized_children!(node).for_each(|child| match child.kind() {
    "function_calling_name" => name = parse_function_calling_name(child, source_bytes),
    "function_argument" => {
      let res = parse_function_arg(child, source_bytes);
      if res.is_some() {
        args.push(res.unwrap());
      }
    }
    _ => return,
  });

  Some(ExprNode::new(Expr::FunctionCall(name.unwrap(), args), node))
}

fn parse_function_calling_name<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<Identifier> {
  let fnode = node.child(0);

  if fnode.is_none() {
    return None;
  }

  let name = fnode.unwrap().utf8_text(source_bytes).unwrap();

  match fnode.unwrap().kind() {
    "identifier" => Some(Identifier::new(name, node)),
    _ => None,
  }
}

fn parse_function_arg<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  for child in sanitized_children!(node) {
    match child.kind() {
      "expr" => {
        let res = parse_expr(child, source_bytes);
        return res;
      }
      "path" => {
        let res = parse_path(child, source_bytes);
        return res;
      }
      _ => return None,
    }
  }

  None
}

fn parse_path<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut path_segments = vec![];

  sanitized_children!(node).for_each(|child| match child.kind() {
    "path_segment" => {
      sanitized_children!(child).for_each(|child| match child.kind() {
        "path_part" => path_segments.push(ExprNode::new(
          Expr::Literal(Literal::new(FirebaseType::Any, child)),
          child,
        )),
        "expr_group" => {
          let expr = parse_expr_group(child, source_bytes);
          if expr.is_some() {
            path_segments.push(expr.unwrap());
          }
        }
        _ => return,
      });
    }
    _ => return,
  });

  Some(ExprNode::new(Expr::Path(path_segments), node))
}

fn parse_literal<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return None;
  }

  let child = children[0];

  let literal = match child.kind() {
    "number" => Some(Literal::new(
      child
        .utf8_text(source_bytes)
        .unwrap()
        .parse::<f32>()
        .map_or(FirebaseType::Integer, |_| FirebaseType::Float),
      child,
    )),
    "true" => Some(Literal::new(FirebaseType::Boolean, node)),
    "false" => Some(Literal::new(FirebaseType::Boolean, node)),
    "null" => Some(Literal::new(FirebaseType::Null, child)),
    "string" => Some(Literal::new(FirebaseType::String, child)),
    _ => None,
  };

  literal.and_then(|lit| Some(ExprNode::new(Expr::Literal(lit.clone()), node)))
}

fn parse_variable<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let var_node = node.child(0);

  if var_node.is_none() {
    return None;
  }

  match var_node.unwrap().kind() {
    "identifier" => {
      let vname = var_node?.utf8_text(source_bytes).unwrap();

      let ident = Identifier::new(vname, node);

      Some(ExprNode::new(Expr::Variable(ident), node))
    }
    _ => None,
  }
}

fn parse_indexing<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 4 {
    return None;
  }

  let object_node = &children[0];

  let object = match object_node.kind() {
    "variable" => parse_variable(*object_node, source_bytes),
    "function_call" => parse_function_call(*object_node, source_bytes),
    "expr_group" => parse_expr_group(*object_node, source_bytes),
    "list" => parse_list(*object_node, source_bytes),
    "map" => parse_map(*object_node, source_bytes),
    _ => None,
  };

  let field_node = &children[2];
  let field = match field_node.kind() {
    "expr" => parse_expr(*field_node, source_bytes),
    "range" => parse_range(*field_node, source_bytes),
    _ => None,
  };

  let expr = Expr::Indexing(object.clone().map(Box::new), field.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_range<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return None;
  }

  let start = parse_expr(children[0], source_bytes);
  let end = parse_expr(children[2], source_bytes);

  let expr = Expr::Range(start.map(Box::new), end.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_expr<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return None;
  }

  let child = children[0];

  return match child.kind() {
    "ternary" => parse_ternary(child, source_bytes),
    "conditional_or" | "conditional_and" | "relation" | "addition" | "multiplication" => {
      parse_binary(child, source_bytes)
    }
    "contains" => parse_contains(child, source_bytes),
    "type_comparison" => parse_type_comparison(child, source_bytes),
    "unary" => parse_unary(child, source_bytes),
    "member" => parse_member(child, source_bytes),
    "indexing" => parse_indexing(child, source_bytes),
    "primary" => parse_primary(child, source_bytes),
    _ => None,
  };
}

fn parse_expr_group<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return None;
  }

  let child = children[1];

  let expr = match child.kind() {
    "expr" => parse_expr(child, source_bytes),
    "path" => parse_path(child, source_bytes),
    _ => None,
  };

  return Some(ExprNode::new(Expr::ExprGroup(expr.map(Box::new)), node));
}

fn parse_member<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let mut object_expr = None;
  let mut field_expr = None;

  for child in sanitized_children!(node) {
    match child.kind() {
      "member_object" => object_expr = parse_member_object(child, source_bytes),
      _ => continue,
    }
  }

  for child in sanitized_children!(node) {
    match child.kind() {
      "member_field" => field_expr = parse_member_field(child, source_bytes),
      _ => continue,
    }
  }

  Some(ExprNode::new(
    Expr::Member(object_expr.map(Box::new), field_expr.clone().map(Box::new)),
    node,
  ))
}

fn parse_member_object<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return None;
  }

  let object_node = children.first().unwrap();

  let object = match object_node.kind() {
    "primary" => parse_primary(*object_node, source_bytes),
    "member" => parse_member(*object_node, source_bytes),
    _ => None,
  };

  object.map(|val| ExprNode::new(Expr::MemberObject(Some(val.clone()).map(Box::new)), node))
}

fn parse_member_field<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 2 {
    return None;
  }

  let field_node = &children[1];

  match field_node.kind() {
    "variable" => {
      let var = parse_variable(*field_node, source_bytes);

      if var.is_none() {
        return None;
      }

      match var.unwrap().expr() {
        Expr::Variable(ident) => Some(ExprNode::new(Expr::MemberVariable(ident.to_owned()), node)),
        _ => None,
      }
    }
    "function_call" => {
      let func_call = parse_function_call(*field_node, source_bytes);

      if func_call.is_none() {
        return None;
      }

      match func_call.unwrap().expr() {
        Expr::FunctionCall(ident, args) => Some(ExprNode::new(
          Expr::MemberFunction(ident.to_owned(), args.to_owned()),
          node,
        )),
        _ => None,
      }
    }
    "field_indexing" => parse_indexing(node, source_bytes),
    _ => None,
  }
}

fn parse_ternary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 5 {
    return None;
  }

  let condition;
  let on_true;
  let on_false;

  condition = parse_expr(children[0], source_bytes);
  on_true = parse_expr(children[2], source_bytes);
  on_false = parse_expr(children[4], source_bytes);

  let expr = Expr::Ternary(
    condition.map(Box::new),
    on_true.clone().map(Box::new),
    on_false.clone().map(Box::new),
  );

  Some(ExprNode::new(expr, node))
}

fn parse_type_comparison<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return None;
  }

  let operator = parse_expr(children[0], source_bytes);

  let expr = Expr::TypeComparison(operator.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_binary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 3 {
    return None;
  }

  let operator1 = parse_expr(children[0], source_bytes);
  let operator2 = parse_expr(children[2], source_bytes);

  let op_node = children[1];

  let operation = match op_node.kind() {
    "+" => Some(Operation::Addition),
    "-" => Some(Operation::Substraction),
    "&&" => Some(Operation::And),
    "||" => Some(Operation::Or),
    "*" => Some(Operation::Multiplication),
    "/" => Some(Operation::Division),
    "%" => Some(Operation::Modulo),
    "<" | "<=" | ">=" | ">" | "==" | "!=" => Some(Operation::Relation),
    "in" => Some(Operation::Contains),
    _ => None,
  };

  let expr = Expr::Binary(
    operation.clone(),
    operator1.map(Box::new),
    operator2.map(Box::new),
  );

  Some(ExprNode::new(expr, node))
}

fn parse_contains<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 4 {
    return None;
  }

  let operator1 = parse_expr(children[0], source_bytes);
  let operator2 = parse_expr(children[3], source_bytes);

  let expr = Expr::Binary(
    Some(Operation::Contains),
    operator1.map(Box::new),
    operator2.map(Box::new),
  );

  Some(ExprNode::new(expr, node))
}

fn parse_unary<'b>(node: Node<'b>, source_bytes: &[u8]) -> Option<ExprNode> {
  let children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 2 {
    return None;
  }

  let expr_res = parse_expr(children[1], source_bytes);

  let op_node = children[0];

  let operation = match op_node.kind() {
    "!" => Some(Operation::Negation),
    "-" => Some(Operation::Substraction),
    _ => None,
  };

  let expr = Expr::Unary(operation.clone(), expr_res.map(Box::new));

  Some(ExprNode::new(expr, node))
}

fn parse_method<'b>(node: Node<'b>) -> Option<Method> {
  let mut children: Vec<Node<'b>> = sanitized_children!(node).collect();

  if children.len() != 1 {
    return None;
  }

  let child = children.pop().unwrap();

  let m_type = match child.kind() {
    "read" => MethodType::Read,
    "write" => MethodType::Write,
    "get" => MethodType::Get,
    "list" => MethodType::List,
    "create" => MethodType::Create,
    "update" => MethodType::Update,
    "delete" => MethodType::Delete,
    _ => MethodType::Unknown,
  };

  Some(Method::new(m_type, child))
}
