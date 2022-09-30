use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents =
        fs::read_to_string(file_path).expect("Couldn't read the file from the path provided");

    println!("With text:\n{contents}");

    // Run code using `cargo run --example cli-tutorial the README.md`
    // would ideally search for text "the" in the file "README.md"
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
