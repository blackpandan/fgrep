use fgrep::Config;
use std::env;
use std::process;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::build(&arguments).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("\nSearching for {}", &config.query);
    println!("in {}\n", &config.file_path);

    if let Err(err) = fgrep::run(config) {
        println!("{}", err);
        process::exit(1);
    };
}
