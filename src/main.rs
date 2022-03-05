use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod config;
mod parser;
fn main() {
    let file = File::open("foo.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {}
}
