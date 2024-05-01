use std::io;

fn convert(value: f64, mode: u64) -> f64 {
    if mode == 1 {
        // celsius to fahrenheit
        (value * 1.8) + 32.0
    } else if mode == 2 {
        // fahrenheit to celsius
        (value - 32.0) / 1.8
    } else {
        println!("Unknown mode.");
        value
    }
}

fn main() {
    let exit_msg = loop {
        println!("Enter the conversion direction. \n
            1: Celsius -> Fahrenheit \n
            2: Fahrenheit -> Celsius");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let mode: u64 = input.trim().parse().unwrap();
        println!("Insert the temperature in Celsius or exit with q:");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let input = input.trim();
        if input.eq("q") {
            println!("pass");
            break "Program exited."
        }
        let value: f64 = input.parse().unwrap();
        let result = convert(value, mode);
        println!("{result}")
        //println!("The temperature {}°C is equal to {fahrenheit}°F.");
    };
    println!("{exit_msg}");
}