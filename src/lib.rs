use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&config.file_path)?;

    let result: Vec<&str>;

    if config.insensitive == true {
        result = search_case_insensitive(&config.query, &contents);
    } else {
        result = search(&config.query, &contents);
    }

    println!("Results: \n");
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub insensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Arguments");
        }

        let mut insensitive: bool = false;
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        if env::var("INSENSITIVE_CASE").is_ok() {
            insensitive = true;
        }

        Ok(Self {
            query,
            file_path,
            insensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_normal() {
        let query: &str = "star";
        let content: &str = "\
Hello to the world
the star is not a moon
but the sun is a star
                             ";

        assert_eq!(
            vec!["the star is not a moon", "but the sun is a star"],
            search(query, content)
        );
    }

    #[test]
    fn search_insensitive() {
        let query: &str = "hello";
        let content: &str = "\
Hello to the world
the star is not a moon
but the sun is a star
                             ";

        assert_eq!(
            vec!["Hello to the world"],
            search_case_insensitive(query, content)
        );
    }

    #[test]
    fn search_sensitive() {
        let query: &str = "hello";
        let content: &str = "\
Hello to the world
the star is not a moon
but the sun is a star
                             ";

        let result: Vec<&str> = Vec::new();
        assert_eq!(result, search(query, content));
    }
}
