use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

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
    if let Err(e) =  run(config) {
        println!("Application error  {e}");
        process::exit(1);
    }   
}
