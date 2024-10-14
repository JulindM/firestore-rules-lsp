use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::*;
use request::*;

use serde::ser::StdError;
use std::error::Error;
use tree_sitter::Parser;

pub fn start_server(port: u16, parser: Parser) -> Result<(), Box<dyn StdError + Send + Sync>> {
  let addr = format!("127.0.0.1:{port}");

  let (connection, io_threads) = Connection::listen(addr.to_owned())?;

  let server_capabilities = serde_json::to_value(&ServerCapabilities {
    type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
    ..Default::default()
  })
  .unwrap();

  let initialization_params = match connection.initialize(server_capabilities) {
    Ok(it) => it,
    Err(e) => {
      if e.channel_is_disconnected() {
        io_threads.join()?;
      }
      return Err(e.into());
    }
  };

  main_loop(connection, initialization_params)?;
  io_threads.join()?;

  Ok(())
}

fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  let _params: InitializeParams = serde_json::from_value(params).unwrap();

  for msg in &connection.receiver {
    match msg {
      Message::Request(req) => {
        if connection.handle_shutdown(&req)? {
          return Ok(());
        }
        match cast::<GotoDefinition>(req) {
          Ok((id, params)) => {
            let result = Some(GotoDefinitionResponse::Array(Vec::new()));
            let result = serde_json::to_value(&result).unwrap();
            let resp = Response {
              id,
              result: Some(result),
              error: None,
            };
            connection.sender.send(Message::Response(resp))?;
            continue;
          }
          Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
          Err(ExtractError::MethodMismatch(req)) => req,
        };
      }
      Message::Response(resp) => {
        eprintln!("got response: {resp:?}");
      }
      Message::Notification(not) => {
        eprintln!("got notification: {not:?}");
      }
    }
  }
  Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
  R: lsp_types::request::Request,
  R::Params: serde::de::DeserializeOwned,
{
  req.extract(R::METHOD)
}
