#![deny(elided_lifetimes_in_paths)]

mod parser;
mod provider;
mod server;

use std::error::Error;

use clap::{Command, arg};
use server::server::start_server;
use tree_sitter_firestore_rules;

pub fn main() -> Result<(), Box<dyn Error>> {
  let args = Command::new("firestore-rules-lsp").args(&[
    arg!(socket: --socket <NUMBER> "port if starting the lsp as a server").conflicts_with("stdio"),
    arg!(stdio: --stdio "flag to start over stdio").conflicts_with("socket"),
  ]);

  let arg_result = args.try_get_matches();

  if arg_result.is_err() {
    let err = arg_result.err().unwrap();
    print!("{}", err);
    return Ok(());
  }

  let matches = arg_result.unwrap();

  let port_str = matches.get_one::<String>("socket");

  let mut startup_type = StartUpType::STDIO;

  if port_str.is_some() {
    startup_type = StartUpType::TCP((*port_str.unwrap()).parse::<u16>().unwrap())
  }

  let language = tree_sitter_firestore_rules::LANGUAGE;

  let mut parser = tree_sitter::Parser::new();

  parser
    .set_language(&language.into())
    .expect("Error loading FirestoreRules parser");

  start_server(startup_type, parser)?;
  Ok(())
}

#[derive(Debug)]
pub enum StartUpType {
  STDIO,
  TCP(u16),
}
