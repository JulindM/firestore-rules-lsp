use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::*;
use lsp_types::*;
use request::*;

use std::{collections::HashMap, error::Error};
use tree_sitter::{Parser, Tree};

use crate::{
  parser::{
    base::{FirestoreTree, MatchBody},
    evaluation::evaluate_tree,
  },
  provider::{
    analysis::{build_diagnostics, get_lowest_denominator, to_position, try_find_definition},
    tokenizer::{get_used_semantic_token_modifiers, get_used_semantic_token_types, tokenize},
  },
  StartUpType,
};

pub fn start_server(startup_type: StartUpType, mut parser: Parser) -> Result<(), Box<dyn Error>> {
  eprintln!("Starting server over {:?}", startup_type);

  let (connection, io_threads) = match startup_type {
    StartUpType::STDIO => Connection::stdio(),
    StartUpType::TCP(port) => {
      let addr = format!("0.0.0.0:{port}");
      Connection::connect(addr)?
    }
  };

  let server_capabilities = serde_json::to_value(&ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    hover_provider: Some(HoverProviderCapability::Simple(true)),
    definition_provider: Some(OneOf::Left(true)),
    semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
      SemanticTokensOptions {
        work_done_progress_options: WorkDoneProgressOptions {
          work_done_progress: Some(false),
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

  eprintln!("LSP Initialized");

  main_loop(connection, &mut parser)?;
  io_threads.join()?;

  Ok(())
}

type LSPTreeStorage<'a> = HashMap<String, (FirestoreTree, Tree)>;

fn main_loop<'a>(connection: Connection, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
  let mut evaulated_trees: LSPTreeStorage<'a> = HashMap::new();

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
        if let Ok(did_open) = cast_notif::<DidOpenTextDocument>(&not) {
          open_doc(&did_open, parser, &mut evaulated_trees);
          publish_diagnostics(&did_open.text_document.uri, &evaulated_trees, &connection);
        }

        if let Ok(did_change) = cast_notif::<DidChangeTextDocument>(&not) {
          change_doc(&did_change, parser, &mut evaulated_trees);
          publish_diagnostics(&did_change.text_document.uri, &evaulated_trees, &connection);
        }

        continue;
      }
    }
  }

  Ok(())
}

fn publish_diagnostics<'a>(
  text_document_uri: &Uri,
  firestore_trees: &HashMap<String, (FirestoreTree, Tree)>,
  connection: &Connection,
) -> () {
  let find = firestore_trees.get(text_document_uri.as_str());

  if find.is_none() {
    return;
  }

  let (mut firestore_tree, tree) = find.unwrap().to_owned();

  let diagnostics = build_diagnostics(&tree, &mut firestore_tree);

  let _ = connection
    .sender
    .try_send(Message::Notification(Notification::new(
      "textDocument/publishDiagnostics".to_owned(),
      PublishDiagnosticsParams::new(text_document_uri.to_owned(), diagnostics.clone(), None),
    )));
}

fn open_doc<'a>(
  did_open: &DidOpenTextDocumentParams,
  parser: &mut Parser,
  evaulated_trees: &mut LSPTreeStorage<'a>,
) {
  let text = &did_open.text_document.text;

  let parsed_tree_opt = parser.parse(text.clone(), None);

  if parsed_tree_opt.is_none() {
    return;
  }

  let tree = parsed_tree_opt.unwrap();

  let evaluated_tree = evaluate_tree(tree.clone(), text.as_bytes());

  evaulated_trees.insert(
    did_open.text_document.uri.to_string(),
    (evaluated_tree, tree),
  );
}

fn change_doc<'a>(
  did_change: &DidChangeTextDocumentParams,
  parser: &mut Parser,
  evaulated_trees: &mut LSPTreeStorage<'a>,
) {
  let content = &did_change.content_changes.last();

  if content.is_none() {
    return;
  }

  let text = content.cloned().unwrap().text;

  let parsed_tree_opt = parser.parse(text.clone(), None);

  if parsed_tree_opt.is_none() {
    return;
  }

  let tree = parsed_tree_opt.unwrap();

  let evaluated_tree = evaluate_tree(tree.clone(), text.as_bytes());

  evaulated_trees.insert(
    did_change.text_document.uri.to_string(),
    (evaluated_tree, tree),
  );
}

fn handle_go_to_definition<'a>(
  definition_r: (RequestId, GotoDefinitionParams),
  evaulated_trees: &HashMap<String, (FirestoreTree, Tree)>,
  req: Request,
  connection: &Connection,
) {
  let definition_param = definition_r.1.text_document_position_params;

  let body = match try_get_body(evaulated_trees, &definition_param.text_document) {
    Some(value) => value,
    None => {
      Message::Response(Response::new_ok(req.id.clone(), "No body found"));
      return;
    }
  };

  let traversal = get_lowest_denominator(definition_param.position, body);

  let definition = try_find_definition(&traversal.iter().collect());

  if definition.is_none() {
    let _ = connection
      .sender
      .try_send(Message::Response(Response::new_ok(
        req.id.clone(),
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

  let _ = connection.sender.try_send(Message::Response(msg));
}

fn try_get_body<'a>(
  evaulated_trees: &'a HashMap<String, (FirestoreTree, Tree)>,
  doc: &TextDocumentIdentifier,
) -> Option<&'a MatchBody> {
  let find = evaulated_trees.get(doc.uri.as_str());

  if find.is_none() {
    return None;
  }

  let (firestore_tree, _) = find.unwrap();

  firestore_tree.body()
}

fn handle_hover<'a>(
  hover_r: (RequestId, HoverParams),
  evaulated_trees: &LSPTreeStorage<'a>,
  req: Request,
  connection: &Connection,
) {
  let hover_params = hover_r.1.text_document_position_params;

  let body = match try_get_body(evaulated_trees, &hover_params.text_document) {
    Some(value) => value,
    None => {
      Message::Response(Response::new_ok(req.id.clone(), "No body found"));
      return;
    }
  };

  let traversal_list = get_lowest_denominator(hover_params.position, body);

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

  let _ = connection.sender.try_send(Message::Response(msg));
}

fn handle_tokenize_request<'a>(
  tokenize_r: (RequestId, SemanticTokensParams),
  evaulated_trees: &HashMap<String, (FirestoreTree, Tree)>,
  req: Request,
  connection: &Connection,
) -> () {
  let tokenize_params = tokenize_r.1;

  let find = evaulated_trees.get(tokenize_params.text_document.uri.as_str());

  if find.is_none() {
    return;
  }

  let (_, tree) = find.unwrap();

  let tokenization_result = tokenize(&tree);

  let tokenize_msg = SemanticTokensResult::Tokens(SemanticTokens {
    result_id: None,
    data: tokenization_result,
  });

  let msg = Response::new_ok::<SemanticTokensResult>(req.id, tokenize_msg);

  let _ = connection.sender.try_send(Message::Response(msg));
}

fn cast_req<R>(req: &Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
  R: lsp_types::request::Request,
  R::Params: serde::de::DeserializeOwned,
{
  req.clone().extract::<R::Params>(R::METHOD)
}

fn cast_notif<N>(not: &Notification) -> Result<N::Params, ExtractError<Notification>>
where
  N: lsp_types::notification::Notification,
  N::Params: serde::de::DeserializeOwned,
{
  not.clone().extract(N::METHOD)
}
