use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
    if let Err(e) = run(config) {
        // Got an error? Let the user know and crash app gracefully
        println!("Application error!: {e}");
        process::exit(1);
    }

    // Run code using `cargo run --example cli-tutorial the README.md`
    // would ideally search for text "the" in the file "README.md"
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Question mark (?) at end lets us return any errors from `fs`
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    // If we got the file, return Ok type an empty tuple (which functions usually return?)
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if user passed enough args into CLI command
        if args.len() < 3 {
            return Err("Not enough arguments. Try `cargo run search-term path/to/file.md`");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
