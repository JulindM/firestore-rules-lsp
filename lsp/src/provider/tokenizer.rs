use lsp_types::{SemanticToken, SemanticTokenModifier, SemanticTokenType};
use tree_sitter::Node;

pub fn tokenize<'a>(node: &Node<'a>) -> Vec<SemanticToken> {
  let mut level_cursor = node.walk();
  let mut semantic_tokens: Vec<SemanticToken> = vec![];

  loop {
    let curr_node = level_cursor.node();

    let curr_tokenization = build_semantic_token(&curr_node);

    if curr_tokenization.is_none() {
      let mut curr_node_cursor = curr_node.walk();
      let curr_node_children = curr_node.children(&mut curr_node_cursor);

      if curr_node_children.len() != 0 {
        for child in curr_node_children {
          let mut child_tokenizations = tokenize(&child);
          semantic_tokens.append(&mut child_tokenizations);
        }
      }
    } else {
      semantic_tokens.push(curr_tokenization.unwrap());
    }

    let moved = level_cursor.goto_next_sibling();

    if !moved {
      break;
    }
  }

  semantic_tokens
}

fn build_semantic_token<'a>(node: &Node<'a>) -> Option<SemanticToken> {
  let token_type = get_semantic_type(node.kind());

  if token_type.is_none() {
    return None;
  }

  let start = node.start_position();
  let length = (node.end_byte() - node.start_byte()) / size_of::<usize>();
  let (token_type, modifier) = token_type.unwrap();

  Some(SemanticToken {
    delta_line: start.column.try_into().unwrap(),
    delta_start: start.row.try_into().unwrap(),
    length: length.try_into().unwrap(),
    token_type,
    token_modifiers_bitset: modifier.unwrap_or(0),
  })
}

pub fn get_used_semantic_token_types() -> Vec<SemanticTokenType> {
  vec![
    SemanticTokenType::COMMENT,  //1
    SemanticTokenType::NUMBER,   //2
    SemanticTokenType::STRING,   //3
    SemanticTokenType::VARIABLE, //4
    SemanticTokenType::OPERATOR, //5
    SemanticTokenType::KEYWORD,  //6
    SemanticTokenType::FUNCTION, //7
    SemanticTokenType::VARIABLE, //8
  ]
}

pub fn get_used_semantic_token_modifiers() -> Vec<SemanticTokenModifier> {
  vec![
    SemanticTokenModifier::DEFINITION, //1
  ]
}

fn get_semantic_type(type_str: &str) -> Option<(u32, Option<u32>)> {
  let token_type = match type_str {
    "comment" => Some(1),
    "number" => Some(2),
    "string" | "path" => Some(3),
    "variable" => Some(4),
    "!" | "-" | "+" | "=" | "mult_op" | "relation_op" | "&&" | "||" | "?" | ":" => Some(5),
    "false" | "true" | "null" | "let" | "return" | "function" | "method" | "create" | "update"
    | "delete" | "match" | "allow" | "if" => Some(6),
    "function_name" => Some(7),
    "single_path_seg" | "multi_path_seg" => Some(8),
    _ => None,
  };

  if token_type.is_none() {
    return None;
  }

  let modifier = match type_str {
    "function" => Some(1),
    _ => None,
  };

  //TODO Implement modifiers
  Some((token_type.unwrap(), modifier))
}
