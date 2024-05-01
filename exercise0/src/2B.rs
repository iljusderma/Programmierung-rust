use std::env;

fn main() {
    println!("{:?}", env::var("RUST_INPUT"));
    let key = "RUST_INPUT";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }
}