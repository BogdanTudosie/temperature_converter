use std::io;


fn fahrenheit_to_celsius(value: i32) -> i32 {
    println!("Converting from Fahrenheit to Celsius");
     (value - 32) * 5/9
}

fn celsius_to_fahrenheit(value: i32) -> i32 {
    println!("Converting from Celsius to Fahrenheit");
    (value * 9/5) + 32
}

fn main() {
    println!("Hello! I can convert Fahrenheit temperatures to Celsius for you! Americans can be really silly!");

    // add capability to choose which conversion the user wants;
    println!("What kind of conversion would you like to perform?");

    let mut conversion = String::new();
    io::stdin()
        .read_line(&mut conversion)
        .expect("Please enter a value");

    // Shadow the conversion and use this to verify what the user wants 
    let conversion = conversion.trim();
    let conversion = match conversion {
        "1" => 1,
        "2" => 2,
        _ => {
            0;
            return;
        }
    };

    // prompt the user to enter the variable and parse it
    // ensure the number entered is actually a decimal point number
    // and not some other data type
    println!("Please enter the temperature:");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read your input");

    // create a new variable and apply the function to it
    let temperature: i32 = temperature.trim().parse().expect("Please enter a number");
    // display the result
    // add processing of the conversion type!
    let mut result = 0;
    if conversion == 1 {
        result = fahrenheit_to_celsius(temperature);
    }
    else if conversion == 2 {
        result = celsius_to_fahrenheit(temperature);
    }
    println!("The temperature is: {result}");
}
