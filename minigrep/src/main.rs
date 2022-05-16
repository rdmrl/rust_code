use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}\n", args);

    // Config now contains owned String values.
    // The args variable is the owner of the argument values
    // and is only letting the parse_config method borrow them.
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        // Using clone to allow the Config instance to own
        // these values.
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
