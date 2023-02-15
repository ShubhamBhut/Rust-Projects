use std::env;
use std::process;

use grep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing the arguments, {}", err);
        process::exit(1);
    });
   
    println!("Searching for {}", config.query);
    println!("The file path is {}", config.file_path);
    
    if let Err(e) = grep::run(config){
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}


