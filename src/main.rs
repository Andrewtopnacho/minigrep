use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::*;

fn main() {
    // Return command line arguments and collect them into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Create an instance of Config with the second and third elements of args being assigned to the two fields of the struct
    // let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}

