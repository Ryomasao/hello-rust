extern crate minigrep;

use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file for {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application Error {}", e);
        process::exit(1);
    }
}

//fn parse_config(args: &[String]) -> Config {
//    println!("searching for {}", query);
//    println!("in file for {}", filename);
//}
