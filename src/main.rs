use minigrep::Config;
use std::{env, process};

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
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
