use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    fs::read_to_string(config.file_path).expect("Something went wrong reading the file");
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