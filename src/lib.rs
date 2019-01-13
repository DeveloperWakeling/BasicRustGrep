use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
//Library crate for our main app to use

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
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

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    //Iterate through each line of contents
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

//Tests...
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
