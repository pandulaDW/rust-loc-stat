#![allow(dead_code)]

mod config;
mod parser;
mod process;

fn main() {
    let c = config::Config::new("/Users/pandula/Desktop/react-native-");
    let mut processor = process::Processor::new();

    processor
        .process_files(&c.directory, &c.excluded_dirs)
        .expect("error occurred");

    println!("{:?}", processor.aggregated_result);
}
