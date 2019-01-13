extern crate minigrep;

use std::process;
use std::env; //Needed for accessing the environment args
use minigrep::Config;

fn main() {
    //Accepting command line arguments
    let args: Vec<String> = env::args().collect();

    //Need logic check to make sure there are the correct number of arguments supplied
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    }); //Unwrap or else is basically like let if, except it handles the Err and sets variable to Ok inner
    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}