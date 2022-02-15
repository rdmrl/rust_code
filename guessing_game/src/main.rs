use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println is a macro and not a function.
    // Macros have a ! at the end.
    println!("Guess a number between 1 and 100!");

    // Get a random number generator that is local to the current 
    // thread of execution and seeded by the operating system.

    // The gen_range takes a range and generates a random number
    // within that range. The range is inclusive on the lower bound
    // but exclusive on the upper bound.
    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess.");

        // Create a mutable variable that is currently bound
        // to a new, empty instance of a String.
        let mut guess = String::new();

        // stdin() returns an instance of the Stdin type which 
        // represents a handle to the standard input.

        // read_line takes the user input and appends to the string
        // The string argument needs to be mutable so that read_line
        // can change its contents.

        // The & indicates that this argument is a reference. This
        // allows access to this data without needing copies.

        // read_line also returns a value - io::Result, an enumeration.
        // If the Result instance is an Err value, expect will cause the
        // program to crash and display this message.

        // Not using expect will generate a warning.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Using a match to continue with the loop if there is invalid input.
        // parse returns Ok if able to convert the input string to a number.
        // parse returns Err if not able to parse the string.
        // The underscore in Err() is a catchall value used to match all 
        // Err values, effectively ignoring all errors.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        // {} is a placeholder
        println!("You guessed {}", guess);

        // Use another type cmp::Ordering for comparison.

        // cmp compares two values and returns an Ordering enum.
        // The match expression is used to decide what to do next.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Break out of the loop.
                break;
            }
        }
    }
}
