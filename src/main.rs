use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Problem encountered when parsing arguments: {}",error);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Problem occured: {}",e);
        process::exit(1);
    }
}