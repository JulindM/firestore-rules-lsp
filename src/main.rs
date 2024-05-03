mod parsing;

use std::env;

use crate::parsing::rules_parser::parse;

pub fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build_global()
        .unwrap();

    let path: String = env::args().nth(1).expect("A path is needed");

    println!("Running with: {:?}", path);

    let src = std::fs::read_to_string(path).unwrap();

    let stream = Vec::from_iter(src.chars());

    let ast = parse(stream, false);

    match ast {
        Ok(_) => (),
        Err(errs) => print!("{:#?}", errs),
    }
}
