use minigrep::config::Config;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_results = Config::new(&args);

    println!("Searching for {}", parsed_results.query);
    println!("In file {}", parsed_results.file_path);

    let contents = fs::read_to_string(parsed_results.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
