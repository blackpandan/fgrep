use fgrep::Config;
use std::env;
use std::process;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::build(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\nsearching for {}", &config.query);
    println!("in {}\n", &config.file_path);

    if let Err(err) = fgrep::run(config) {
        println!("Problem reading file: {}", err);
        process::exit(1);
    };
}
