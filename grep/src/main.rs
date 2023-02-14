use std::env;
use std::process;

use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing the arguments, {}", err);
        process::exit(1);
    });
   
    println!("Searching for {}", config.query);
    println!("The file path is {}", config.file_path);
    
    if let Err(e) = grep::run(config){
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{

    let mut results = Vec::new();
    
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn one_result() {

        let query = "duct";
        let contents = "/
Rust:
safe, fast, productive.
Pick three";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
