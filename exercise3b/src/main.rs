fn main() {
    let x = 243_f64;

    println!("{} {} {}",x / 16.0, (x / 16.0).floor() , (x / 16.0).fract());
    let x = 4;
    let y = match x {
        3 => 50,
        other => other
    };
    println!("{y}")
}