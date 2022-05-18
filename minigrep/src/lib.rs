use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // Using clone to allow the Config instance to own
        // these values.
        let query = args[1].clone();
        let filename = args[2].clone();

        // env::var returns a Result that will be the successful Ok
        // variant that contains the value of the env variable if the
        // env variable has been set.

        // is_err is used on the Result to check whether it's an error
        // and therefore unset, which means the search should be case-sensitive.

        // If the env var is set to anything, is_err will return false and
        // the search will be case insensitive.

        // The value of the env var is not important, merely that it is set
        // to something.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { 
            query, 
            filename,
            case_sensitive,
        })
    }
}

// Box<dyn Error> is a trait object. This function will return
// a type that implements the Error trait but the particular type
// need not be specified. dyn is short for dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // expect has been replaced with a ? to avoid panic!.
    // Now it will return the error value from the current function
    // for the caller to handle.

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // println!("With text:\n{}", contents);
    for line in results {
        println!("{}", line);
    }

    // Returns an OK value in the success case. The success type is ().
    Ok(())
}

// Need an explicit lifetime 'a defined in the signature of search.
// The returned vector should contain string slices that reference
// slices of the argument contents (rather than the argument) query.

// The data returned by the search function will live as long as the
// data passed into the search function in the contents argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // query is now a string rather than a slice, because calling
        // to_lowercase creates new data rather than referencing existing
        // data
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

