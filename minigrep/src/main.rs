use std::env;
use std::fs;
use std::process;
use std::error::Error;

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

    run(config);
}

// Box<dyn Error> is a trait object. This function will return
// a type that implements the Error trait but the particular type
// need not be specified. dyn is short for dynamic.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // expect has been replaced with a ? to avoid panic!.
    // Now it will return the error value from the current function
    // for the caller to handle.

    println!("With text:\n{}", contents);

    // Returns an OK value in the success case. The success type is ().
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // Using clone to allow the Config instance to own
        // these values.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
