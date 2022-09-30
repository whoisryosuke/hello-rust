use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse CLI arguments
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Read from user's file
    let contents = fs::read_to_string(config.file_path)
        .expect("Couldn't read the file from the path provided");

    println!("With text:\n{contents}");

    // Run code using `cargo run --example cli-tutorial the README.md`
    // would ideally search for text "the" in the file "README.md"
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // Check if user passed enough args into CLI command
        if args.len() < 3 {
            panic!("Not enough arguments. Try `cargo run search-term path/to/file.md`")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
