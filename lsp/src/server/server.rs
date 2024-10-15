use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::*;
use notification::DidOpenTextDocument;
use request::*;

use std::{collections::HashMap, error::Error, fs::File, io, path::Path};
use tree_sitter::{Parser, Point};

use crate::parser::{
  base::BaseModel,
  evaluation::evaluate_tree,
  extensions::{get_lowest_denominator, EvaluatedTree},
};

pub fn start_server(port: u16, mut parser: Parser) -> Result<(), Box<dyn Error>> {
  let addr = format!("127.0.0.1:{port}");

  let (connection, io_threads) = Connection::listen(addr.to_owned())?;

  let server_capabilities = serde_json::to_value(&ServerCapabilities {
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

fn open_file(uri: Uri) -> Result<File, std::io::Error> {
  let p = Path::new(uri.path().as_str());

  if p.is_dir() {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      "Expected a file not a folder",
    ));
  };

  File::open(p)
}

fn main_loop(connection: Connection, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
  let mut evaulated_trees: HashMap<String, EvaluatedTree> = HashMap::new();

  for msg in &connection.receiver {
    match msg {
      Message::Request(req) => {
        if let Ok(sd) = cast_req::<Shutdown>(&req) {
          assert_eq!(
            connection.handle_shutdown(&req),
            Ok(true),
            "Error shutting down server"
          );
        }

        if let Ok(hover_r) = cast_req::<HoverRequest>(&req) {
          let hover_params = hover_r.1.text_document_position_params;
          let ev_tree = evaulated_trees.find(hover_params.text_document);

          let type_info = get_lowest_denominator(
            to_point(hover_params.position),
            BaseModel::MatchBody(ev_tree.tree().body().clone()),
          )
          .unwrap();

          let hover = Hover {
            contents: HoverContents::Markup(MarkupContent {
              kind: MarkupKind::PlainText,
              value: format!("{type_info:?}"),
            }),
            range: None,
          };

          let msg = Response::new_ok::<Hover>(req.id, hover);

          // FIXME What if this errors out
          let _ = connection.sender.send(Message::Response(msg));

          continue;
        }
      }
      Message::Response(resp) => {
        eprintln!("got response: {resp:?}");
        continue;
      }
      Message::Notification(not) => {
        if let Ok(dc) = cast_not::<DidOpenTextDocument>(&not) {
          let content = dc.text_document.text;

          // FIXME What if one these returns none
          let parsed_tree = parser.parse(content.clone(), None).unwrap();
          let evaluated_tree = evaluate_tree(&parsed_tree, content.as_bytes()).unwrap();
          evaulated_trees.insert(dc.text_document.uri.to_string(), evaluated_tree);
        }
        continue;
      }
    }
  }

  Ok(())
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
  fn find(&self, text_document: TextDocumentIdentifier) -> &EvaluatedTree;
}

impl Find for HashMap<String, EvaluatedTree> {
  fn find(&self, text_document: TextDocumentIdentifier) -> &EvaluatedTree {
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
