use lsp_types::{SemanticToken, SemanticTokenModifier, SemanticTokenType};
use tree_sitter::{Node, Tree};

pub fn tokenize<'a>(tree: &Tree) -> Vec<SemanticToken> {
  let mut semantic_tokens = vec![];
  let absolute_tokenization = absolute_tokenize(&tree.root_node());

  if absolute_tokenization.len() != 0 {
    let first = absolute_tokenization.first().unwrap();
    semantic_tokens.push(SemanticToken {
      delta_line: first.line,
      delta_start: first.col,
      length: first.length,
      token_modifiers_bitset: first.token_modifiers_bitset,
      token_type: first.token_type,
    });
  }

  if absolute_tokenization.len() <= 1 {
    return semantic_tokens;
  }

  for i in 1..=absolute_tokenization.len() - 1 {
    let to_add =
      absolute_tokenization[i].get_semantic_token_from_prev(&absolute_tokenization[i - 1]);
    semantic_tokens.push(to_add);
  }

  semantic_tokens
}

fn absolute_tokenize<'a>(node: &Node<'a>) -> Vec<AbsoluteToken> {
  let mut level_cursor = node.walk();
  let mut absolute_tokens: Vec<AbsoluteToken> = vec![];

  loop {
    let curr_node = level_cursor.node();

    let curr_tokenization = build_absolute_token(&curr_node);

    if curr_tokenization.is_none() {
      let mut curr_node_cursor = curr_node.walk();
      let curr_node_children = curr_node.children(&mut curr_node_cursor);

      if curr_node_children.len() != 0 {
        for child in curr_node_children {
          let mut child_tokenizations = absolute_tokenize(&child);
          absolute_tokens.append(&mut child_tokenizations);
        }
      }
    } else {
      absolute_tokens.push(curr_tokenization.unwrap());
    }

    let moved = level_cursor.goto_next_sibling();

    if !moved {
      break;
    }
  }

  absolute_tokens
}

struct AbsoluteToken {
  line: u32,
  col: u32,
  length: u32,
  token_type: u32,
  token_modifiers_bitset: u32,
}
impl AbsoluteToken {
  fn get_semantic_token_from_prev(&self, prev: &AbsoluteToken) -> SemanticToken {
    let line_delta = self.line - prev.line;

    SemanticToken {
      delta_line: line_delta,
      delta_start: if line_delta == 0 {
        self.col - prev.col
      } else {
        self.col
      },
      token_type: self.token_type,
      length: self.length,
      token_modifiers_bitset: self.token_modifiers_bitset,
    }
  }
}

fn build_absolute_token<'a>(node: &Node<'a>) -> Option<AbsoluteToken> {
  let token_type = get_semantic_type(
    node.kind(),
    node.parent().and_then(|p| Some(p.kind())).unwrap_or(""),
  );

  if token_type.is_none() {
    return None;
  }

  let start = node.start_position();
  let length = node.end_byte() - node.start_byte();
  let (token_type, modifier) = token_type.unwrap();

  Some(AbsoluteToken {
    col: start.column.try_into().unwrap(),
    line: start.row.try_into().unwrap(),
    length: length.try_into().unwrap(),
    token_type,
    token_modifiers_bitset: modifier.unwrap_or(0),
  })
}

pub fn get_used_semantic_token_types() -> Vec<SemanticTokenType> {
  vec![
    SemanticTokenType::COMMENT,  //0
    SemanticTokenType::NUMBER,   //1
    SemanticTokenType::STRING,   //2
    SemanticTokenType::VARIABLE, //3
    SemanticTokenType::OPERATOR, //4
    SemanticTokenType::KEYWORD,  //5
    SemanticTokenType::FUNCTION, //6
    SemanticTokenType::VARIABLE, //7
    SemanticTokenType::PROPERTY, //8
    SemanticTokenType::TYPE,     //9
  ]
}

pub fn get_used_semantic_token_modifiers() -> Vec<SemanticTokenModifier> {
  vec![
    SemanticTokenModifier::DECLARATION, //0
  ]
}

fn get_semantic_type(type_str: &str, parent_type: &str) -> Option<(u32, Option<u32>)> {
  let token_type = match type_str {
    "comment" => Some(0),
    "number" => Some(1),
    "string" => Some(2),
    "variable" => match parent_type {
      "member_field" => Some(8),
      _ => Some(7),
    },
    "identifier" => match parent_type {
      "path_segment" => Some(2),
      _ => None,
    },
    "!" | "-" | "+" | "=" | "mult_op" | "relation_op" | "&&" | "||" | "?" | ":" | "is" | "in" => {
      Some(4)
    }
    "false" | "true" | "null" | "let" | "return" | "function" | "method" | "create" | "update"
    | "delete" | "match" | "allow" | "if" | "service" | "cloud.firestore" => Some(5),
    "function_name" | "function_calling_name" => Some(6),
    "single_path_seg" | "multi_path_seg" => Some(7),
    "type" => Some(8),
    _ => None,
  };

  if token_type.is_none() {
    return None;
  }

  let modifier = match type_str {
    "function" => Some(1),
    "function_name" => Some(1),
    _ => None,
  };

  //TODO Implement modifiers
  Some((token_type.unwrap(), modifier))
}
