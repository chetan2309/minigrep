use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
    fs::read_to_string(config.file_path).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
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