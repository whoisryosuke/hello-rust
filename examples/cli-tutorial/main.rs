use std::env;
use std::process;

use super::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse CLI arguments
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Read from user's file
    // Run the function (and does anything inside)
    // then checks if we return an error
    if let Err(e) = config::run(config) {
        // Got an error? Let the user know and crash app gracefully
        println!("Application error!: {e}");
        process::exit(1);
    }

    // Run code using `cargo run --example cli-tutorial the README.md`
    // would ideally search for text "the" in the file "README.md"
}
