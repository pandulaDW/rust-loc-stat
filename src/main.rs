#![allow(dead_code)]

mod config;
mod parser;
mod process;

fn main() {
    let c = config::Config::new("/Users/pandula/Desktop/api");
    let _handlers = process::process_files(&c).expect("error occurred");
    println!("{}", _handlers.len());
}
