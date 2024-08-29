use std::io::stdin;

fn main() {
    println!("Welcome to the fahrenheit conversion program!");
    println!("Please enter the temperature in Fahrenheit: ");
    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number or decimal!");
                continue;
            }
        };

        println!(
            "The temperature in Celsius is: {}",
            (input - 32.0) * 5.0 / 9.0
        );
        break;
    }
}
