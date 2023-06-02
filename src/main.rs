use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // If I move this line above the line where I am printing config.file_path
    // then I am getting an error. Although, I am still not sure why I am getting it.
    // Seems like read_to_string moves the string and then I cannot borrow after it gets moved.
    // A question here is doesn't println! macros move the string??
    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments to proceed");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

struct Config {
    query: String,
    file_path: String,
}
