use num_bigint::BigUint;
use std::io;

struct Fibonacci {
    a: BigUint,
    b: BigUint,
}

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let c = &self.a + &self.b;
        self.a = self.b.clone();
        self.b = c.clone();
        Some(c)
    }
}

fn main() {
    let fib = Fibonacci { a: BigUint::from(0 as u64), b: BigUint::from(1 as u64) };

    println!("Enter number to calculate Fibonacci for:");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let number = buffer.trim().parse::<u128>().unwrap_or(10_u128);

    // For the loop, we need to make use of a special method offered by Iterator trait
    for x in fib.take(number as usize){
        println!("{x}")
    }
}
