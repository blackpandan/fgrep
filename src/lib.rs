use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&config.file_path)?;

    println!("\n{contents}",);

    Ok(())
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
