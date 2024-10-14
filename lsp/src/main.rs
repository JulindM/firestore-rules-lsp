mod server;

use std::error::Error;

use clap::Parser;
use server::server::start_server;
use tree_sitter_firestore_rules;

pub fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let args = Args::parse();

  let language = tree_sitter_firestore_rules::LANGUAGE;

  let mut parser = tree_sitter::Parser::new();

  parser
    .set_language(&language.into())
    .expect("Error loading FirestoreRules parser");

  start_server(args.port, parser)
}

#[derive(clap::Parser)]
struct Args {
  port: u16,
}
