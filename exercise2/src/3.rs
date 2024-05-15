use num_bigint::{BigUint};
use std::io;

/*
fn fibonacci_bignum(n: BigUint) -> BigUint {
    let zero: BigUint = "0".parse().unwrap();
    let one: BigUint = "1".parse().unwrap();
    let two: BigUint = "2".parse().unwrap();
    if n == zero {
        zero
    } else if n == one {
        one
    } else {
        fibonacci(&n - &one) + fibonacci(&n - &two)
    }
}
*/

fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}


fn main(){
    /*
    let n: BigUint = "100".parse().unwrap();
    println!("{:?}", n);
    let fibonacci_number = fibonacci(n);
    println!("The recursively determined fibonacci number is: {}", fibonacci_number);
    */
    println!("Fibonacci generator");
    println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();
        println!("\nEnter a positive integer:");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        if n.trim() == "quit" {
            break;
        }
        let n: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", fibonacci(n));
    }
}