use std::io;


fn fahrenheit_to_celsius(value: i32) -> i32 {
     (value - 32) * 5/9
}

fn celsius_to_fahrenheit(value: i32) -> i32 {
    (value * 9/5) + 32
}

fn main() {
    println!("Hello! I can convert Fahrenheit temperatures to Celsius for you! Americans can be really silly!");

    // prompt the user to enter the variable and parse it
    // ensure the number entered is actually a decimal point number
    // and not some other data type
    println!("Please enter the fahrenheit temperature:");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read your input");

    // create a new variable and apply the function to it
    let temperature: i32 = temperature.trim().parse().expect("Please enter a number");
    // display the result
    let result = fahrenheit_to_celsius(temperature);
    println!("The temperature in Celsius is: {result}");
}
