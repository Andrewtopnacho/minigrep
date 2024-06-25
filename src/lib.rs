use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Config { query, file_path };
    }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    search(&config.query, &contents).iter().for_each(|j| println!("{}", j));

    return Ok(());
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    contents.lines().for_each(|j| if j.contains(query) {results.push(j)});
    
    return results;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
pick three.";
        
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

}