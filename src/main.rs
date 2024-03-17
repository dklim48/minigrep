use minigrep::config::Config;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // let parsed_results = Config::new(&args);
    // let mut parsed_results: Config = Config {
    //     query: String::from(""),
    //     file_path: String::from(""),
    // };

    // if let Ok(configration) = Config::build(&args) {
    //     parsed_results = configration;
    // }

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
