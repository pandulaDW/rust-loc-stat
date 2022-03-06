#![allow(dead_code)]

mod config;
mod parser;
mod process;

fn main() {
    let c = config::Config::new("/Users/pandula/Desktop/react-native-owner-app");
    process::process_files(&c).expect("error occurred");
}
