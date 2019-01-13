use std::env; //Needed for accessing the environment args
use std::error::Error;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //Accepting command line arguments
    let args: Vec<String> = env::args().collect();

    //Need logic check to make sure there are the correct number of arguments supplied
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }); //Unwrap or else is basically like let if, except it handles the Err and sets variable to Ok inner
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("{}", config.query);
    println!("With text \n {}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if(args.len() < 3){
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone(); //Not the most efficient way to copy a string
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}