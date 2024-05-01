use std::env;

fn fibonacci(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n + 1)
    }
}

fn fibonacci_iterative(n: usize) -> usize {
    let mut fnumbers = vec![0; n + 1];
    let mut fnumber: usize;
    for i in 0..=n {
        fnumber = if i == 0 {
                0
            } else if i == 1 {
                1
            } else {
                fnumbers[i - 1] + fnumbers[i - 2]
            };
        // println!("i: {}, fibnum: {}", i, fnumber);
        fnumbers[i] = fnumber;
        // println!("{:?}", fnumbers);
    }
    fnumbers[n]
    
}

fn main(){
    let n: Vec<_> = env::args().collect();
    let n: usize = n[1].parse().unwrap();
    println!("{:?}", n);
    let fibonacci_number = fibonacci_iterative(n);
    println!("The iteratively determined fibonacci number is: {}", fibonacci_number);
    let fibonacci_number = fibonacci(n);
    println!("The recursively determined fibonacci number is: {}", fibonacci_number);
}