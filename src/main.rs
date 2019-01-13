use std::env; //Needed for accessing the environment args
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //Accepting command line arguments
    let args: Vec<String> = env::args().collect();

    //Need logic check to make sure there are the correct number of arguments supplied
    let config = Config::new(&args); //Technically could give up ownership here???
    let mut f = File::open(config.filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    println!("{}", config.query);
    println!("With text \n {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone(); //Not the most efficient way to copy a string
        let filename = args[2].clone();
        Config { query, filename }
    }
}