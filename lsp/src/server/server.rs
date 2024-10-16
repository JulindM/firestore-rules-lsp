use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::*;
use notification::{DidChangeTextDocument, DidOpenTextDocument};
use request::*;

use std::{collections::HashMap, error::Error};
use tree_sitter::{Parser, Point, Tree};

use crate::parser::{
  base::BaseModel,
  evaluation::evaluate_tree,
  extensions::{get_lowest_denominator, EvaluatedTree},
};

pub fn start_server(port: u16, mut parser: Parser) -> Result<(), Box<dyn Error>> {
  let addr = format!("127.0.0.1:{port}");

  let (connection, io_threads) = Connection::listen(addr.to_owned())?;

  let server_capabilities = serde_json::to_value(&ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    hover_provider: Some(HoverProviderCapability::Simple(true)),
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

fn main_loop(connection: Connection, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
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
  let evaluated_tree = evaluate_tree(&parsed_tree, content.as_bytes()).unwrap();

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

  let parsed_tree = parser.parse(content.clone(), Some(old_tree)).unwrap();
  let evaluated_tree = evaluate_tree(&parsed_tree, content.as_bytes()).unwrap();

  evaulated_trees.insert(
    did_change.text_document.uri.to_string(),
    (evaluated_tree, parsed_tree),
  );
}

fn handle_hover(
  hover_r: (RequestId, HoverParams),
  evaulated_trees: &LSPTreeStorage,
  req: Request,
  connection: &Connection,
) {
  let hover_params = hover_r.1.text_document_position_params;
  let (ev_tree, _) = evaulated_trees.find(hover_params.text_document);

  let type_info = get_lowest_denominator(
    to_point(hover_params.position),
    BaseModel::MatchBody(ev_tree.tree().body().clone()),
  )
  .unwrap();

  let hover = Hover {
    contents: HoverContents::Markup(MarkupContent {
      kind: MarkupKind::PlainText,
      // TODO markdown
      value: format!("{type_info:?}"),
    }),
    range: None,
  };

  let msg = Response::new_ok::<Hover>(req.id, hover);

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
  fn find(&self, text_document: TextDocumentIdentifier) -> &(EvaluatedTree, Tree);
  fn find_ver(&self, text_document: VersionedTextDocumentIdentifier) -> &(EvaluatedTree, Tree);
}

impl Find for LSPTreeStorage {
  fn find(&self, text_document: TextDocumentIdentifier) -> &(EvaluatedTree, Tree) {
    let id = text_document.uri.as_str();
    &self[id]
  }

  fn find_ver(&self, text_document: VersionedTextDocumentIdentifier) -> &(EvaluatedTree, Tree) {
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
