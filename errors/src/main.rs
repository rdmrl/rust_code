use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    /*
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    */

    /*
    // With closures.
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
    */

    let res = read_username_from_file();
    println!("Result: {:?}", res);
}

fn read_username_from_file() -> Result<String, io::Error> {
/*
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

    // 2. Using the ? Operator.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
*/

    // 3. Using ? and chaining.
    let mut s = String::new();

    File::open("Hello.txt")?.read_to_string(&mut s)?;

    Ok(s);

    // 4. Single call.
    fs::read_to_string("hello.txt");
}
