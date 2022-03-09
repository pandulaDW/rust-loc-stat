#![allow(dead_code)]

mod config;
mod parser;
mod process;

fn main() {
    let c = config::Config::new("/Users/pandula/Desktop/Projects/node/queries");
    process::process_files(&c).expect("error occurred");
}
