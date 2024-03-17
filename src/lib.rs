pub mod config {
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
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config::Config;

    // use super::*;

    #[test]
    fn constructor_works() {
        let args = [String::from(""), String::from("")];
        let new_config = Config::new(&args);

        assert_eq!(new_config.query, "");
        assert_eq!(new_config.file_path, "");
    }
}
