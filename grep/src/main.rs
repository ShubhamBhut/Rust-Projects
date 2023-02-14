use std::env;
use std::process;

use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err: &str| {
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


