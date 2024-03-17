use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut query = "";
    if let Some(val) = &args.get(1) {
        query = val;
    }

    let mut file_path = "";
    if let Some(val) = &args.get(2) {
        file_path = val;
    }

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
