use std::{fs::File, io::BufReader};

mod config;
mod parser;
fn main() {
    let file = File::open("data/test.js").unwrap();
    let reader = BufReader::new(file);

    let mut p = parser::Parser::new(config::Language::Javascript);
    p.parse(reader).expect("sudo");

    println!("{:?}", p);
}
