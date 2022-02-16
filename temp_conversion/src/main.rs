use std::io;

fn main() {

    // The conversion factor.
    const CENTI_MULT: f32 = 1.8;

    // The number of degrees to add in the conversion.
    const DEG_TO_ADD: f32 = 32.0;

    // Ask the user to enter the value to be converted.
    println!("Enter the temperature:");

    let mut input_temp = String::new();

    // Read the user input.
    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    // Convert the string input into a floating point number.
    let input_temp: f32 = match input_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    // Ask the user to enter the scale to convert to.
    println!("Enter f to convert to Fahrenheit, or c to convert to Centigrade.");

    let mut input_type = String::new();
    io::stdin()
        .read_line(&mut input_type)
        .expect("Failed to read line");

    // Set to true if the conversion was done successfully.
    let conversion_done;

    // The converted temperature value.
    let output_temp: f32;

    // The units for the converted value used for display.
    let mut output_type_display = 'C';

    // The units for the input value used for display.
    let mut input_type_display = 'F';

    // Match on the input scale; trim to remove newlines.
    match input_type.trim() {
        "f" => {
            output_temp = ( input_temp * CENTI_MULT ) + DEG_TO_ADD;
            // The output scale.
            output_type_display = 'F';

            // The input scale.
            input_type_display = 'C';
            conversion_done = true;
        },
        "c" => {
            output_temp = ( input_temp - DEG_TO_ADD ) / CENTI_MULT;
            // The output scale.
            output_type_display = 'C';

            // The input scale.
            input_type_display = 'F';
            conversion_done = true;
        },
        _ => {
            // Error condition, when the scale input is incorrect.
            conversion_done = false;
            output_temp = 0.0;
            // Show an error message.
            println!("Please enter either f for Fahrenheit, or c for Centigrade.");
        }
    }

    if conversion_done {
        // If the conversion was successful, show both the input and converted values.
        println!("{}{} has been converted to {}{}.", input_temp, input_type_display, output_temp, output_type_display);
    }
}
