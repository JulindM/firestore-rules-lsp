use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::*;
use notification::{DidChangeTextDocument, DidOpenTextDocument};
use request::*;

use std::{collections::HashMap, error::Error};
use tree_sitter::{Parser, Point, Tree};

use crate::{
  parser::{evaluation::evaluate_tree, extensions::EvaluatedTree},
  provider::{
    analysis::{get_lowest_denominator, try_find_definition},
    tokenizer::{get_used_semantic_token_modifiers, get_used_semantic_token_types, tokenize},
  },
};

pub fn start_server(port: u16, mut parser: Parser) -> Result<(), Box<dyn Error>> {
  let addr = format!("127.0.0.1:{port}");

  let (connection, io_threads) = Connection::listen(addr.to_owned())?;

  let server_capabilities = serde_json::to_value(&ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    hover_provider: Some(HoverProviderCapability::Simple(true)),
    definition_provider: Some(OneOf::Left(true)),
    semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
      SemanticTokensOptions {
        work_done_progress_options: WorkDoneProgressOptions {
          work_done_progress: None,
        },
        legend: SemanticTokensLegend {
          token_types: get_used_semantic_token_types(),
          token_modifiers: get_used_semantic_token_modifiers(),
        },
        range: None,
        full: Some(SemanticTokensFullOptions::Bool(true)),
      },
    )),
    ..Default::default()
  })
  .unwrap();

  let _ = match connection.initialize(server_capabilities) {
    Ok(it) => it,
    Err(err) => return Err(Box::new(err)),
  };

  main_loop(connection, &mut parser)?;
  io_threads.join()?;

  Ok(())
}

type LSPTreeStorage = HashMap<String, (EvaluatedTree, Tree)>;

fn main_loop<'a>(connection: Connection, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
  let mut evaulated_trees: LSPTreeStorage = HashMap::new();

  for msg in &connection.receiver {
    match msg {
      Message::Request(req) => {
        if let Ok(_) = cast_req::<Shutdown>(&req) {
          assert_eq!(
            connection.handle_shutdown(&req),
            Ok(true),
            "Error shutting down server"
          );
        }

        if let Ok(hover_r) = cast_req::<HoverRequest>(&req) {
          handle_hover(hover_r, &evaulated_trees, req, &connection);
          continue;
        }

        if let Ok(definition_r) = cast_req::<GotoDefinition>(&req) {
          handle_go_to_definition(definition_r, &evaulated_trees, req, &connection);
          continue;
        }

        if let Ok(tokenize_r) = cast_req::<SemanticTokensFullRequest>(&req) {
          handle_tokenize_request(tokenize_r, &evaulated_trees, req, &connection);
          continue;
        }
      }
      Message::Response(resp) => {
        eprintln!("got response: {resp:?}");
        continue;
      }
      Message::Notification(not) => {
        if let Ok(did_open) = cast_not::<DidOpenTextDocument>(&not) {
          open_doc(&did_open, parser, &mut evaulated_trees);
        }

        if let Ok(did_change) = cast_not::<DidChangeTextDocument>(&not) {
          change_doc(&did_change, parser, &mut evaulated_trees);
        }

        continue;
      }
    }
  }

  Ok(())
}

fn open_doc(
  did_open: &DidOpenTextDocumentParams,
  parser: &mut Parser,
  evaulated_trees: &mut LSPTreeStorage,
) {
  // FIXME What if one these returns none
  let content = did_open.text_document.text.clone();

  let parsed_tree = parser.parse(content.clone(), None).unwrap();
  let evaluated_tree = evaluate_tree(parsed_tree.clone(), content.as_bytes()).unwrap();

  evaulated_trees.insert(
    did_open.text_document.uri.to_string(),
    (evaluated_tree, parsed_tree),
  );
}

fn change_doc(
  did_change: &DidChangeTextDocumentParams,
  parser: &mut Parser,
  evaulated_trees: &mut LSPTreeStorage,
) {
  // FIXME What if one these returns none
  let (_, old_tree) = evaulated_trees.find_ver(did_change.text_document.clone());

  let content = &did_change.content_changes.last().unwrap().text;

  let parsed_tree = parser.parse(content.clone(), None).unwrap();
  let evaluated_tree = evaluate_tree(parsed_tree.clone(), content.as_bytes()).unwrap();

  evaulated_trees.insert(
    did_change.text_document.uri.to_string(),
    (evaluated_tree, parsed_tree),
  );
}

fn handle_go_to_definition(
  definition_r: (RequestId, GotoDefinitionParams),
  evaulated_trees: &HashMap<String, (EvaluatedTree, Tree)>,
  req: Request,
  connection: &Connection,
) {
  let definition_param = definition_r.1.text_document_position_params;
  let (ev_tree, _) = evaulated_trees.find(&definition_param.text_document);

  let traversal =
    get_lowest_denominator(to_point(definition_param.position), ev_tree.tree().body());

  let definition = try_find_definition(traversal);

  if definition.is_none() {
    let _ = connection.sender.send(Message::Response(Response::new_ok(
      req.id,
      "No definition found",
    )));
    return;
  }

  let span = definition.unwrap().span();

  let definition_resp = GotoDefinitionResponse::Scalar(Location::new(
    definition_param.text_document.uri,
    Range::new(to_position(span.0), to_position(span.1)),
  ));

  let msg = Response::new_ok::<GotoDefinitionResponse>(req.id, definition_resp);

  // FIXME What if this errors out
  let _ = connection.sender.send(Message::Response(msg));
}

fn handle_hover(
  hover_r: (RequestId, HoverParams),
  evaulated_trees: &LSPTreeStorage,
  req: Request,
  connection: &Connection,
) {
  let hover_params = hover_r.1.text_document_position_params;
  let (ev_tree, _) = evaulated_trees.find(&hover_params.text_document);

  let traversal_list =
    get_lowest_denominator(to_point(hover_params.position), ev_tree.tree().body());

  let traversal: String = traversal_list
    .into_iter()
    .map(|v| v.type_str().to_string())
    .collect::<Vec<String>>()
    .join("->");

  let hover = Hover {
    contents: HoverContents::Markup(MarkupContent {
      kind: MarkupKind::PlainText,
      // TODO markdown
      value: format!("{traversal:?}"),
    }),
    range: None,
  };

  let msg = Response::new_ok::<Hover>(req.id, hover);

  // FIXME What if this errors out
  let _ = connection.sender.send(Message::Response(msg));
}

fn handle_tokenize_request(
  tokenize_r: (RequestId, SemanticTokensParams),
  evaulated_trees: &HashMap<String, (EvaluatedTree, Tree)>,
  req: Request,
  connection: &Connection,
) -> () {
  let tokenize_params = tokenize_r.1;
  let (_, tree) = evaulated_trees.find(&tokenize_params.text_document);

  let tokenization_result = tokenize(&tree.root_node());

  let tokenize_msg = SemanticTokensResult::Tokens(SemanticTokens {
    result_id: None,
    data: tokenization_result,
  });

  let msg = Response::new_ok::<SemanticTokensResult>(req.id, tokenize_msg);

  // FIXME What if this errors out
  let _ = connection.sender.send(Message::Response(msg));
}

fn cast_req<R>(req: &Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
  R: lsp_types::request::Request,
  R::Params: serde::de::DeserializeOwned,
{
  req.clone().extract::<R::Params>(R::METHOD)
}

fn cast_not<N>(not: &Notification) -> Result<N::Params, ExtractError<Notification>>
where
  N: lsp_types::notification::Notification,
  N::Params: serde::de::DeserializeOwned,
{
  not.clone().extract(N::METHOD)
}

trait Find {
  fn find<'a>(&'a self, text_document: &TextDocumentIdentifier) -> &'a (EvaluatedTree, Tree);
  fn find_ver<'a>(
    &'a self,
    text_document: VersionedTextDocumentIdentifier,
  ) -> &'a (EvaluatedTree, Tree);
}

impl Find for LSPTreeStorage {
  fn find<'a>(&'a self, text_document: &TextDocumentIdentifier) -> &'a (EvaluatedTree, Tree) {
    let id = text_document.uri.as_str();
    &self[id]
  }

  fn find_ver<'a>(
    &'a self,
    text_document: VersionedTextDocumentIdentifier,
  ) -> &'a (EvaluatedTree, Tree) {
    let id = text_document.uri.as_str();
    &self[id]
  }
}

fn to_point(position: Position) -> Point {
  Point::new(
    position.line.try_into().unwrap(),
    position.character.try_into().unwrap(),
  )
}

fn to_position(point: Point) -> Position {
  Position::new(
    point.row.try_into().unwrap(),
    point.column.try_into().unwrap(),
  )
}
