use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_results = parse_config(&args);

    println!("Searching for {}", parsed_results.query);
    println!("In file {}", parsed_results.file_path);

    let contents = fs::read_to_string(parsed_results.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let mut query = String::from("");
    let mut file_path = String::from("");

    if let Some(val) = args.get(1) {
        query = val.clone();
    }

    if let Some(val) = args.get(2) {
        file_path = val.clone();
    }

    Config { query, file_path }
}
