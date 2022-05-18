use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Config now contains owned String values.
    // The args variable is the owner of the argument values
    // and is only letting the parse_config method borrow them.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.filename);

    // run returns an () in the success case.
    // Only need to detect an error so unwrap_or_else is not needed.
    // as with Config::new.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
