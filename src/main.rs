mod parsing;

use std::env;

use crate::parsing::rules_parser::{parse, parse_debug};

pub fn main() {
    let path: String = env::args().nth(1).expect("A path is needed");

    println!("Running with: {:?}", path);

    let src = std::fs::read_to_string(path).unwrap();

    let stream = Vec::from_iter(src.chars());

    let ast = parse(stream);

    println!("{:?}", ast);
}
