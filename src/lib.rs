use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents= fs::read_to_string(config.file_path)?;
    let response = search(&config.query, &contents);
    for line in response {
        println!("{line}");
    }
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments to proceed");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut response_vector = Vec::new();
    for line in contents.lines() {
      if line.contains(query) {
        response_vector.push(line);
      }
    }
    response_vector
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut response_vector = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
      if line.to_lowercase().contains(&query) {
        response_vector.push(line);
      }
    }
    response_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}