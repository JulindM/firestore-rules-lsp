mod parsing;
mod repr;

use std::env;

use crate::{parsing::rules_parser::parse, repr::representations::Service};

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
        Ok(tree) => {
            let repr = Service::generate_from_service_definition(tree);
            print!("{:#?}", repr.get_variables());
        }
        Err(errs) => print!("{:#?}", errs),
    }
}
