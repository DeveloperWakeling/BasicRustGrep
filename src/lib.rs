use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
//Library crate for our main app to use

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("{}", config.query);
    println!("With text \n {}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone(); //Not the most efficient way to copy a string
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}