use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
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

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguements");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

#[cfg(test)]
mod config_tests {
    use crate::Config;

    // use super::*;

    #[test]
    fn constructor_works() {
        let args = [String::from(""), String::from("")];
        let new_config = Config::new(&args);

        assert_eq!(new_config.query, "");
        assert_eq!(new_config.file_path, "");
    }
}
