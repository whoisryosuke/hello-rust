use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Question mark (?) at end lets us return any errors from `fs`
    let contents = fs::read_to_string(config.file_path)?;

    // Run the search function
    // And loop through the results to print out each "match"
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // If we got the file, return Ok type an empty tuple (which functions usually return?)
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Storage for lines of strings that match our search term
    let mut results = Vec::new();

    // Go through each "line" inside the body of text
    for line in contents.lines() {
        // Check if the line "contains" the search term
        if line.contains(query) {
            // Add it to the results vector
            results.push(line);
        }
    }

    results
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
