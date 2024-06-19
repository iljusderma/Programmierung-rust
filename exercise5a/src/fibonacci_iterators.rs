use std::io;

struct Fibonacci {
    a: u128,
    b: u128,
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(c)
    }
}

fn main() {
    let fib = Fibonacci { a: 0, b: 1 };

    println!("Enter number to calculate Fibonacci for:");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let number = buffer.trim().parse::<u128>().unwrap_or(10_u128);

    // For the loop, we need to make use of a special method offered by Iterator trait
    // @TODO Implement the loop below
    for x in fib.take(number as usize){
        println!("{x}")
    }
}
