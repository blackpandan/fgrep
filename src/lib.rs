use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&config.file_path)?;

    let result = search(&config.query, &contents);

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

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Self { query, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "hello";
        let content: &str = "\
        hello to the world
        the star is not a moon
        but the sun is a star
                             ";

        assert_eq!(vec!["hello to the world"], search(query, content));
    }
}
