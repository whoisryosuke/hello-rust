use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // Run code using `cargo run --example cli-tutorial the README.md`
    // would ideally search for text "the" in the file "README.md"
}
