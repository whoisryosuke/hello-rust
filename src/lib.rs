use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Question mark (?) at end lets us return any errors from `fs`
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    // If we got the file, return Ok type an empty tuple (which functions usually return?)
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if user passed enough args into CLI command
        if args.len() < 3 {
            return Err("Not enough arguments. Try `cargo run search-term path/to/file.md`");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
